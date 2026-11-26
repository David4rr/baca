package com.baca.nativeapp

import android.net.Uri
import android.os.Bundle
import android.provider.OpenableColumns
import androidx.activity.ComponentActivity
import androidx.activity.compose.BackHandler
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import com.baca.nativeapp.ui.*
import kotlinx.coroutines.Dispatchers
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

private sealed interface NativeRoute {
    data object Shelf : NativeRoute
    data class EpubReader(val book: NativeBook, val epub: NativeEpubBook) : NativeRoute
    data class PdfReader(val book: NativeBook) : NativeRoute
}

@Composable
private fun BacaNativeApp(activity: ComponentActivity) {
    val scope = rememberCoroutineScope()
    var core by remember { mutableStateOf<BacaCore?>(null) }
    var books by remember { mutableStateOf<List<NativeBook>>(emptyList()) }
    var settings by remember { mutableStateOf<NativeSettings?>(null) }
    var route by remember { mutableStateOf<NativeRoute>(NativeRoute.Shelf) }
    var initError by remember { mutableStateOf<String?>(null) }
    var bookmarkedChapters by remember { mutableStateOf<Set<Int>>(emptySet()) }

    LaunchedEffect(Unit) {
        withContext(Dispatchers.IO) {
            try {
                val c = BacaCore.open(activity.filesDir.absolutePath)
                core = c
                books = c.getAllBooks()
                settings = c.getSettings()
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
            runCatching { c.importBytes(name, bytes) }.onSuccess {
                books = c.getAllBooks()
            }
        }
    }

    val palette = BacaPalettes.fromName(settings?.theme ?: "warm")

    if (initError != null) {
        Box(Modifier.fillMaxSize(), contentAlignment = Alignment.Center) { Text(initError!!) }
        return
    }
    if (core == null || settings == null) {
        Box(Modifier.fillMaxSize(), contentAlignment = Alignment.Center) { CircularProgressIndicator() }
        return
    }

    BackHandler(enabled = route != NativeRoute.Shelf) {
        route = NativeRoute.Shelf
    }

    when (val current = route) {
        NativeRoute.Shelf -> ShelfScreen(
            books = books,
            palette = palette,
            onImportClick = { importLauncher.launch(arrayOf("application/epub+zip", "application/pdf")) },
            onOpenBook = { book ->
                val c = core!!
                scope.launch(Dispatchers.IO) {
                    c.touchLastRead(book.id)
                    if (book.format.equals("pdf", ignoreCase = true)) {
                        route = NativeRoute.PdfReader(book)
                    } else {
                        val parsed = c.parseEpubBlocks(book.filePath)
                        bookmarkedChapters = c.getBookmarks(book.id).map { it.chapterIndex.toInt() }.toSet()
                        route = NativeRoute.EpubReader(book, parsed)
                    }
                    books = c.getAllBooks()
                }
            },
            onDeleteBook = { book ->
                val c = core!!
                scope.launch(Dispatchers.IO) {
                    c.deleteBook(book.id, book.filePath)
                    books = c.getAllBooks()
                }
            },
        )

        is NativeRoute.EpubReader -> ReaderScreen(
            book = current.book,
            epub = current.epub,
            palette = palette,
            settings = settings!!,
            bookmarkedChapters = bookmarkedChapters,
            onToggleBookmark = { chIdx, title ->
                val c = core!!
                scope.launch(Dispatchers.IO) {
                    c.toggleBookmark(current.book.id, chIdx.toUInt(), title, "")
                    bookmarkedChapters = c.getBookmarks(current.book.id).map { it.chapterIndex.toInt() }.toSet()
                }
            },
            onProgressCommit = { ch, frac ->
                val c = core!!
                val total = current.epub.chapters.size.coerceAtLeast(1)
                val pct = ((ch + frac) / total).coerceIn(0f, 1f)
                scope.launch(Dispatchers.IO) {
                    c.updateProgress(current.book.id, ch.toUInt(), pct)
                }
            },
            onChangeTheme = { themeId ->
                val c = core!!
                val updated = settings!!.copy(theme = themeId)
                settings = updated
                scope.launch(Dispatchers.IO) { c.saveSettings(updated) }
            },
            onBack = {
                val c = core!!
                scope.launch(Dispatchers.IO) { books = c.getAllBooks() }
                route = NativeRoute.Shelf
            },
        )

        is NativeRoute.PdfReader -> PdfReaderScreen(
            book = current.book,
            palette = palette,
            onProgressCommit = { frac ->
                val c = core!!
                scope.launch(Dispatchers.IO) { c.updateProgress(current.book.id, 0u, frac) }
            },
            onBack = {
                val c = core!!
                scope.launch(Dispatchers.IO) { books = c.getAllBooks() }
                route = NativeRoute.Shelf
            },
        )
    }
}

private fun queryFileName(activity: ComponentActivity, uri: Uri): String? {
    activity.contentResolver.query(uri, null, null, null, null)?.use { cursor ->
        val idx = cursor.getColumnIndex(OpenableColumns.DISPLAY_NAME)
        if (idx >= 0 && cursor.moveToFirst()) return cursor.getString(idx)
    }
    return uri.lastPathSegment
}
