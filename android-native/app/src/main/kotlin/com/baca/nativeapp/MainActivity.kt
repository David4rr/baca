package com.baca.nativeapp

import android.content.Intent
import android.net.Uri
import android.os.Bundle
import android.provider.OpenableColumns
import androidx.activity.ComponentActivity
import androidx.activity.compose.BackHandler
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.animation.AnimatedContent
import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.core.tween
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.animation.slideInVertically
import androidx.compose.animation.slideOutVertically
import androidx.compose.animation.togetherWith
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.rounded.BarChart
import androidx.compose.material.icons.rounded.Explore
import androidx.compose.material.icons.rounded.Home
import androidx.compose.material.icons.rounded.Person
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.baca.nativeapp.ui.*
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import uniffi.baca_core.*

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            MaterialTheme {
                BacaNativeApp(activity = this)
            }
        }
    }
}

private enum class MainTab(val icon: ImageVector, val label: String) {
    Shelf(Icons.Rounded.Home, "Library"),
    Stats(Icons.Rounded.BarChart, "Stats"),
    Explore(Icons.Rounded.Explore, "Explore"),
    Settings(Icons.Rounded.Person, "Settings"),
}

private sealed interface NativeRoute {
    data class Tab(val tab: MainTab) : NativeRoute
    data class EpubReader(val book: NativeBook, val epub: NativeEpubBook) : NativeRoute
    data class PdfReader(val book: NativeBook) : NativeRoute
}

@Composable
private fun BacaNativeApp(activity: ComponentActivity) {
    val scope = rememberCoroutineScope()
    var core by remember { mutableStateOf<BacaCore?>(null) }
    var books by remember { mutableStateOf<List<NativeBook>>(emptyList()) }
    var settings by remember { mutableStateOf<NativeSettings?>(null) }
    var stats by remember { mutableStateOf<List<NativeDailyStat>>(emptyList()) }
    var totalMinutes by remember { mutableIntStateOf(0) }
    var route by remember { mutableStateOf<NativeRoute>(NativeRoute.Tab(MainTab.Shelf)) }
    var initError by remember { mutableStateOf<String?>(null) }
    var bookmarks by remember { mutableStateOf<List<NativeBookmark>>(emptyList()) }
    var toastMsg by remember { mutableStateOf<String?>(null) }
    var lastBackPressMs by remember { mutableLongStateOf(0L) }

    fun showToast(msg: String) {
        toastMsg = msg
    }

    LaunchedEffect(toastMsg) {
        if (toastMsg != null) {
            delay(2600)
            toastMsg = null
        }
    }

    LaunchedEffect(Unit) {
        withContext(Dispatchers.IO) {
            try {
                val c = BacaCore.open(activity.filesDir.absolutePath)
                core = c
                books = c.getAllBooks()
                settings = c.getSettings()
                stats = c.get7DayStats()
                totalMinutes = c.getTotalReadingMinutes().toInt()
            } catch (e: Exception) {
                initError = e.message ?: "Gagal membuka perpustakaan"
            }
        }
    }

    val importLauncher = rememberLauncherForActivityResult(
        ActivityResultContracts.OpenDocument()
    ) { uri: Uri? ->
        val c = core ?: return@rememberLauncherForActivityResult
        if (uri == null) return@rememberLauncherForActivityResult
        scope.launch(Dispatchers.IO) {
            val bytes = activity.contentResolver.openInputStream(uri)?.use { it.readBytes() } ?: return@launch
            val name = queryFileName(activity, uri) ?: "book.epub"
            runCatching { c.importBytes(name, bytes) }
                .onSuccess {
                    books = c.getAllBooks()
                    showToast("Buku \"${it.title}\" ditambahkan")
                }
                .onFailure { showToast(it.message ?: "Gagal mengimpor buku") }
        }
    }

    val exportBackupLauncher = rememberLauncherForActivityResult(
        ActivityResultContracts.CreateDocument("application/json")
    ) { uri: Uri? ->
        val c = core ?: return@rememberLauncherForActivityResult
        if (uri == null) return@rememberLauncherForActivityResult
        scope.launch(Dispatchers.IO) {
            runCatching {
                val json = c.exportBackupJson()
                activity.contentResolver.openOutputStream(uri)?.use { it.write(json.toByteArray()) }
            }.onSuccess {
                showToast("Backup JSON tersimpan")
            }.onFailure {
                showToast(it.message ?: "Gagal menyimpan backup")
            }
        }
    }

    val importBackupLauncher = rememberLauncherForActivityResult(
        ActivityResultContracts.OpenDocument()
    ) { uri: Uri? ->
        val c = core ?: return@rememberLauncherForActivityResult
        if (uri == null) return@rememberLauncherForActivityResult
        scope.launch(Dispatchers.IO) {
            runCatching {
                val json = activity.contentResolver.openInputStream(uri)?.bufferedReader()?.use { it.readText() }
                    ?: error("Berkas kosong")
                val count = c.importBackupJson(json)
                books = c.getAllBooks()
                settings = c.getSettings()
                stats = c.get7DayStats()
                totalMinutes = c.getTotalReadingMinutes().toInt()
                count
            }.onSuccess { count ->
                showToast("$count buku dipulihkan dari backup")
            }.onFailure {
                showToast(it.message ?: "Gagal memulihkan backup")
            }
        }
    }

    val palette = BacaPalettes.fromName(settings?.theme ?: "warm")

    if (initError != null) {
        Box(Modifier.fillMaxSize().background(palette.bg), contentAlignment = Alignment.Center) {
            Text(initError!!, color = palette.textPrimary)
        }
        return
    }
    if (core == null || settings == null) {
        Box(Modifier.fillMaxSize().background(palette.bg), contentAlignment = Alignment.Center) {
            CircularProgressIndicator(color = palette.accent)
        }
        return
    }

    fun openBook(book: NativeBook) {
        val c = core ?: return
        scope.launch(Dispatchers.IO) {
            c.touchLastRead(book.id)
            if (book.format.equals("pdf", ignoreCase = true)) {
                route = NativeRoute.PdfReader(book)
            } else {
                runCatching { c.parseEpubBlocks(book.filePath) }
                    .onSuccess { parsed ->
                        bookmarks = c.getBookmarks(book.id)
                        route = NativeRoute.EpubReader(book, parsed)
                    }
                    .onFailure { showToast(it.message ?: "Gagal membuka EPUB") }
            }
            books = c.getAllBooks()
        }
    }

    BackHandler {
        when (val r = route) {
            is NativeRoute.EpubReader, is NativeRoute.PdfReader -> {
                route = NativeRoute.Tab(MainTab.Shelf)
            }
            is NativeRoute.Tab -> {
                if (r.tab != MainTab.Shelf) {
                    route = NativeRoute.Tab(MainTab.Shelf)
                } else {
                    val now = System.currentTimeMillis()
                    if (now - lastBackPressMs < 2000L) {
                        activity.finish()
                    } else {
                        lastBackPressMs = now
                        showToast("Tekan sekali lagi untuk keluar")
                    }
                }
            }
        }
    }

    Box(modifier = Modifier.fillMaxSize().background(palette.bg)) {
        AnimatedContent(
            targetState = route,
            transitionSpec = {
                fadeIn(tween(220)) togetherWith fadeOut(tween(180))
            },
            label = "rootNavigation",
        ) { current ->
            when (current) {
                is NativeRoute.Tab -> when (current.tab) {
                    MainTab.Shelf -> ShelfScreen(
                        books = books,
                        palette = palette,
                        onImportClick = { importLauncher.launch(arrayOf("application/epub+zip", "application/pdf")) },
                        onOpenBook = { openBook(it) },
                        onDeleteBook = { book ->
                            val c = core!!
                            scope.launch(Dispatchers.IO) {
                                c.deleteBook(book.id, book.filePath)
                                books = c.getAllBooks()
                                showToast("Buku dihapus")
                            }
                        },
                    )

                    MainTab.Stats -> StatsScreen(
                        palette = palette,
                        books = books,
                        stats = stats,
                        totalMinutes = totalMinutes,
                    )

                    MainTab.Explore -> ExploreScreen(
                        palette = palette,
                        libraryBooks = books,
                        onSearch = { q ->
                            withContext(Dispatchers.IO) {
                                runCatching { core!!.searchOnline(q, 1u) }
                            }
                        },
                        onDownload = { item, fmt ->
                            withContext(Dispatchers.IO) {
                                val direct = if (fmt == "pdf") item.pdfUrl else item.epubUrl
                                runCatching {
                                    val saved = core!!.downloadOnlineBook(
                                        item.identifier,
                                        item.title,
                                        item.creator,
                                        fmt,
                                        direct,
                                    )
                                    books = core!!.getAllBooks()
                                    showToast("\"${saved.title}\" siap dibaca")
                                    saved
                                }
                            }
                        },
                        onOpenDownloaded = { openBook(it) },
                    )

                    MainTab.Settings -> SettingsScreen(
                        palette = palette,
                        settings = settings!!,
                        appVersion = "3.5.1",
                        onSaveSettings = { updated ->
                            settings = updated
                            scope.launch(Dispatchers.IO) {
                                core!!.saveSettings(updated)
                                showToast("Pengaturan disimpan")
                            }
                        },
                        onExportBackup = { exportBackupLauncher.launch("baca-backup.json") },
                        onImportBackup = { importBackupLauncher.launch(arrayOf("application/json")) },
                        onCheckUpdate = {
                            withContext(Dispatchers.IO) {
                                runCatching { core!!.checkForUpdate("3.5.1") }
                            }
                        },
                        onOpenUrl = { url ->
                            runCatching {
                                activity.startActivity(Intent(Intent.ACTION_VIEW, Uri.parse(url)))
                            }
                        },
                    )
                }

                is NativeRoute.EpubReader -> ReaderScreen(
                    book = current.book,
                    epub = current.epub,
                    palette = palette,
                    settings = settings!!,
                    bookmarks = bookmarks,
                    onToggleBookmark = { chIdx, title, snippet ->
                        val c = core!!
                        scope.launch(Dispatchers.IO) {
                            val added = c.toggleBookmark(current.book.id, chIdx.toUInt(), title, snippet)
                            bookmarks = c.getBookmarks(current.book.id)
                            showToast(if (added) "Markah disimpan" else "Markah dihapus")
                        }
                    },
                    onDeleteBookmark = { bmId ->
                        val c = core!!
                        scope.launch(Dispatchers.IO) {
                            c.deleteBookmark(bmId)
                            bookmarks = c.getBookmarks(current.book.id)
                        }
                    },
                    onProgressCommit = { ch, frac ->
                        val c = core!!
                        val total = current.epub.chapters.size.coerceAtLeast(1)
                        val pct = ((ch + frac) / total).coerceIn(0f, 1f)
                        scope.launch(Dispatchers.IO) { c.updateProgress(current.book.id, ch.toUInt(), pct) }
                    },
                    onUpdateSettings = { updated ->
                        settings = updated
                        scope.launch(Dispatchers.IO) { core!!.saveSettings(updated) }
                    },
                    onAskAi = { prompt ->
                        withContext(Dispatchers.IO) {
                            val key = settings?.geminiApiKey ?: ""
                            val model = settings?.geminiModel ?: "gemini-2.5-flash"
                            runCatching { core!!.askAi(prompt, key, model) }
                        }
                    },
                    onSessionEnd = { sec ->
                        val c = core ?: return@ReaderScreen
                        scope.launch(Dispatchers.IO) {
                            c.recordReadingSession(current.book.id, sec.toUInt())
                            stats = c.get7DayStats()
                            totalMinutes = c.getTotalReadingMinutes().toInt()
                            books = c.getAllBooks()
                        }
                    },
                    onBack = { route = NativeRoute.Tab(MainTab.Shelf) },
                )

                is NativeRoute.PdfReader -> PdfReaderScreen(
                    book = current.book,
                    palette = palette,
                    onProgressCommit = { frac ->
                        val c = core!!
                        scope.launch(Dispatchers.IO) { c.updateProgress(current.book.id, 0u, frac) }
                    },
                    onSessionEnd = { sec ->
                        val c = core ?: return@PdfReaderScreen
                        scope.launch(Dispatchers.IO) {
                            c.recordReadingSession(current.book.id, sec.toUInt())
                            stats = c.get7DayStats()
                            totalMinutes = c.getTotalReadingMinutes().toInt()
                            books = c.getAllBooks()
                        }
                    },
                    onBack = { route = NativeRoute.Tab(MainTab.Shelf) },
                )
            }
        }

        // Floating Apple Island Bottom Dock matching reference
        val activeTab = (route as? NativeRoute.Tab)?.tab
        AnimatedVisibility(
            visible = activeTab != null,
            enter = fadeIn() + slideInVertically { it },
            exit = fadeOut() + slideOutVertically { it },
            modifier = Modifier
                .align(Alignment.BottomCenter)
                .navigationBarsPadding()
                .padding(bottom = 16.dp),
        ) {
            FloatingAppleDock(
                activeTab = activeTab ?: MainTab.Shelf,
                palette = palette,
                onSelect = { tab -> route = NativeRoute.Tab(tab) },
            )
        }

        // Dynamic Island style pill toast
        AnimatedVisibility(
            visible = toastMsg != null,
            enter = fadeIn() + slideInVertically { -it },
            exit = fadeOut() + slideOutVertically { -it },
            modifier = Modifier
                .align(Alignment.TopCenter)
                .statusBarsPadding()
                .padding(top = 12.dp),
        ) {
            toastMsg?.let { msg ->
                Box(
                    modifier = Modifier
                        .shadow(8.dp, CircleShape)
                        .clip(CircleShape)
                        .background(palette.textPrimary)
                        .padding(horizontal = 20.dp, vertical = 10.dp),
                ) {
                    Text(msg, fontSize = 13.sp, fontWeight = FontWeight.Medium, color = palette.bg)
                }
            }
        }
    }
}

@Composable
private fun FloatingAppleDock(
    activeTab: MainTab,
    palette: BacaPalette,
    onSelect: (MainTab) -> Unit,
) {
    Row(
        modifier = Modifier
            .shadow(14.dp, CircleShape, ambientColor = Color.Black.copy(alpha = 0.14f))
            .clip(CircleShape)
            .background(palette.surface.copy(alpha = 0.96f))
            .border(0.5.dp, palette.border, CircleShape)
            .padding(horizontal = 10.dp, vertical = 8.dp),
        horizontalArrangement = Arrangement.spacedBy(8.dp),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        MainTab.entries.forEach { tab ->
            val selected = activeTab == tab
            Box(
                modifier = Modifier
                    .clip(CircleShape)
                    .background(if (selected) palette.surfaceMuted else Color.Transparent)
                    .applePressScale()
                    .clickable { onSelect(tab) }
                    .padding(horizontal = 18.dp, vertical = 10.dp),
                contentAlignment = Alignment.Center,
            ) {
                Icon(
                    imageVector = tab.icon,
                    contentDescription = tab.label,
                    tint = if (selected) palette.textPrimary else palette.textSecondary,
                    modifier = Modifier.size(22.dp),
                )
            }
        }
    }
}

private fun queryFileName(activity: ComponentActivity, uri: Uri): String? {
    activity.contentResolver.query(uri, null, null, null, null)?.use { cursor ->
        val idx = cursor.getColumnIndex(OpenableColumns.DISPLAY_NAME)
        if (idx >= 0 && cursor.moveToFirst()) return cursor.getString(idx)
    }
    return uri.lastPathSegment
}
