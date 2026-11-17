#!/usr/bin/env bash
set -e

# Export Android SDK & NDK paths
export ANDROID_HOME="${ANDROID_HOME:-/Users/mac/Library/Android/sdk}"
export ANDROID_NDK_HOME="${ANDROID_NDK_HOME:-$ANDROID_HOME/ndk/28.2.13676358}"
export ANDROID_NDK_ROOT="$ANDROID_NDK_HOME"

VERSION=$(grep '^version = ' Cargo.toml | head -n 1 | cut -d '"' -f 2)
VERSION_CODE=$(echo "$VERSION" | awk -F. '{printf "%d%02d%02d", $1, $2, $3}')

echo "==> 1. Compiling Tailwind CSS bundle..."
npx -y tailwindcss@3 -i assets/styles/main.css -o assets/styles/bundle.css --minify

echo "==> 2. Building Rust shared libraries via dx for arm64..."
dx build --arch arm64 --platform android --release

echo "==> 3. Building Rust shared libraries via dx for x64..."
dx build --arch x64 --platform android --release

APP_DIR=target/dx/baca/release/android/app/app

echo "==> 4. Injecting transparent edge-to-edge styles and Kotlin activity..."
cat << 'EOF' > "$APP_DIR/src/main/res/values/styles.xml"
<resources>
    <style name="AppTheme" parent="@style/Theme.AppCompat.Light.NoActionBar">
        <item name="android:windowDrawsSystemBarBackgrounds">true</item>
        <item name="android:statusBarColor">@android:color/transparent</item>
        <item name="android:navigationBarColor">@android:color/transparent</item>
        <item name="android:windowLightStatusBar">true</item>
        <item name="android:windowTranslucentStatus">false</item>
        <item name="android:windowTranslucentNavigation">false</item>
    </style>
</resources>
EOF

mkdir -p "$APP_DIR/src/main/res/xml"
cat << 'EOF' > "$APP_DIR/src/main/res/xml/file_paths.xml"
<?xml version="1.0" encoding="utf-8"?>
<paths>
    <files-path name="files" path="." />
    <cache-path name="cache" path="." />
    <external-path name="downloads" path="." />
</paths>
EOF

cat << 'EOF' > "$APP_DIR/src/main/kotlin/dev/dioxus/main/MainActivity.kt"
package dev.dioxus.main;

import android.graphics.Color
import android.os.Bundle
import androidx.core.view.WindowCompat
import com.baca.Baca.BuildConfig;
typealias BuildConfig = BuildConfig;

class MainActivity : WryActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        WindowCompat.setDecorFitsSystemWindows(window, false)
        window.statusBarColor = Color.TRANSPARENT
        window.navigationBarColor = Color.TRANSPARENT
        try {
            val controller = WindowCompat.getInsetsController(window, window.decorView)
            controller?.isAppearanceLightStatusBars = true
            controller?.isAppearanceLightNavigationBars = true
        } catch (_: Exception) {}
    }
}
EOF

echo "==> 5. Patching manifest, provider, orientation handling, and signing..."
VERSION_NAME="$VERSION" VERSION_CODE="$VERSION_CODE" python3 - "$APP_DIR" <<'PY'
import os, pathlib, re, sys
app = pathlib.Path(sys.argv[1])
manifest = app / "src/main/AndroidManifest.xml"
s = manifest.read_text()
if "REQUEST_INSTALL_PACKAGES" not in s:
    s = s.replace('<uses-permission android:name="android.permission.INTERNET" />',
                  '<uses-permission android:name="android.permission.INTERNET" />\n'
                  '    <uses-permission android:name="android.permission.REQUEST_INSTALL_PACKAGES" />')
s = re.sub(r'android:configChanges="[^"]*"',
           'android:configChanges="orientation|screenSize|smallestScreenSize|screenLayout|keyboardHidden|uiMode|density|fontScale|locale|layoutDirection"', s)
if "FileProvider" not in s:
    provider = ('\n        <provider android:name="androidx.core.content.FileProvider"\n'
                '            android:authorities="${applicationId}.fileprovider"\n'
                '            android:exported="false" android:grantUriPermissions="true">\n'
                '            <meta-data android:name="android.support.FILE_PROVIDER_PATHS"\n'
                '                android:resource="@xml/file_paths" />\n'
                '        </provider>\n')
    s = s.replace('</application>', provider + '    </application>')
s = s.replace('android:theme="@style/AppTheme">',
              'android:theme="@style/AppTheme" android:usesCleartextTraffic="false">')
manifest.write_text(s)

gradle = app / "build.gradle.kts"
g = gradle.read_text()
version_name = os.environ["VERSION_NAME"]
version_code = os.environ["VERSION_CODE"]
g = re.sub(r'versionCode = \d+', f'versionCode = {version_code}', g)
g = re.sub(r'versionName = "[^"]*"', f'versionName = "{version_name}"', g)
if "signingConfigs" not in g:
    signing = '''
    signingConfigs {
        create("shared") {
            storeFile = file(System.getProperty("user.home") + "/.android/debug.keystore")
            storePassword = "android"
            keyAlias = "androiddebugkey"
            keyPassword = "android"
        }
    }
'''
    g = g.replace("    buildTypes {", signing + "    buildTypes {")
g = g.replace('''        getByName("release") {
            isMinifyEnabled = true''', '''        getByName("release") {
            signingConfig = signingConfigs.getByName("shared")
            isDebuggable = false
            isMinifyEnabled = false''')
gradle.write_text(g)
PY

if [ ! -f "$HOME/.android/debug.keystore" ]; then
  mkdir -p "$HOME/.android"
  keytool -genkeypair -keystore "$HOME/.android/debug.keystore" -storepass android \
    -alias androiddebugkey -keypass android -dname "CN=Android Debug,O=Android,C=US" \
    -keyalg RSA -keysize 2048 -validity 10000
fi

echo "sdk.dir=$ANDROID_HOME" > target/dx/baca/release/android/app/local.properties

echo "==> 6. Assembling signed release APK..."
(cd target/dx/baca/release/android/app && ./gradlew assembleRelease)

echo "==> 7. Copying APK to dist/release/..."
mkdir -p dist/release
cp "$APP_DIR/build/outputs/apk/release/app-release.apk" "dist/release/baca-v${VERSION}-android-universal.apk"
(cd dist/release && if command -v sha256sum >/dev/null; then sha256sum "baca-v${VERSION}-android-universal.apk" > "baca-v${VERSION}-android-universal.apk.sha256"; else shasum -a 256 "baca-v${VERSION}-android-universal.apk" > "baca-v${VERSION}-android-universal.apk.sha256"; fi)

echo "==> Done! Output: dist/release/baca-v${VERSION}-android-universal.apk"
