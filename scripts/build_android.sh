#!/usr/bin/env bash
set -e

# Export Android SDK & NDK paths
export ANDROID_HOME="${ANDROID_HOME:-/Users/mac/Library/Android/sdk}"
export ANDROID_NDK_HOME="${ANDROID_NDK_HOME:-$ANDROID_HOME/ndk/28.2.13676358}"
export ANDROID_NDK_ROOT="$ANDROID_NDK_HOME"

echo "==> 1. Compiling Tailwind CSS bundle..."
npx -y tailwindcss@3 -i assets/styles/main.css -o assets/styles/bundle.css --minify

echo "==> 2. Building Rust shared libraries via dx for arm64..."
dx build --arch arm64 --platform android --release

echo "==> 3. Building Rust shared libraries via dx for x64..."
dx build --arch x64 --platform android --release

echo "==> 4. Injecting Apple-standard transparent edge-to-edge styles and Kotlin activity..."
cat << 'EOF' > target/dx/baca/release/android/app/app/src/main/res/values/styles.xml
<resources>
    <!-- Base application theme with Apple-standard transparent edge-to-edge status bar -->
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

cat << 'EOF' > target/dx/baca/release/android/app/app/src/main/kotlin/dev/dioxus/main/MainActivity.kt
package dev.dioxus.main;

import android.graphics.Color
import android.os.Build
import android.os.Bundle
import android.view.View
import android.view.WindowManager
import androidx.core.view.WindowCompat
import com.baca.Baca.BuildConfig;
typealias BuildConfig = BuildConfig;

class MainActivity : WryActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        
        // Enable true edge-to-edge layout so WebView draws directly behind the status bar
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

echo "sdk.dir=$ANDROID_HOME" > target/dx/baca/release/android/app/local.properties

echo "==> 5. Assembling Universal APK via Gradle..."
(cd target/dx/baca/release/android/app && ./gradlew assembleDebug)

echo "==> 6. Copying Universal APK to dist/release/..."
mkdir -p dist/release
cp target/dx/baca/release/android/app/app/build/outputs/apk/debug/app-debug.apk dist/release/baca-v3.3.3-android-universal.apk

echo "==> Done! Output: dist/release/baca-v3.3.3-android-universal.apk"
