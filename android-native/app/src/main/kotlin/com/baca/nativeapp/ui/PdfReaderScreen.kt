package com.baca.nativeapp.ui

import android.graphics.Bitmap
import android.graphics.pdf.PdfRenderer
import android.os.ParcelFileDescriptor
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.pager.HorizontalPager
import androidx.compose.foundation.pager.rememberPagerState
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.rounded.ArrowBack
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.asImageBitmap
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import uniffi.baca_core.NativeBook
import java.io.File

@Composable
fun PdfReaderScreen(
    book: NativeBook,
    palette: BacaPalette,
    onProgressCommit: (Float) -> Unit,
    onBack: () -> Unit,
) {
    var renderer by remember { mutableStateOf<PdfRenderer?>(null) }
    var pageCount by remember { mutableIntStateOf(0) }
    var errorMsg by remember { mutableStateOf<String?>(null) }

    DisposableEffect(book.filePath) {
        var fd: ParcelFileDescriptor? = null
        var pdf: PdfRenderer? = null
        try {
            fd = ParcelFileDescriptor.open(File(book.filePath), ParcelFileDescriptor.MODE_READ_ONLY)
            pdf = PdfRenderer(fd)
            renderer = pdf
            pageCount = pdf.pageCount
        } catch (e: Exception) {
            errorMsg = "Gagal membuka PDF: ${e.message}"
        }
        onDispose {
            try { pdf?.close() } catch (_: Exception) {}
            try { fd?.close() } catch (_: Exception) {}
        }
    }

    Column(modifier = Modifier.fillMaxSize().background(palette.bg)) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .statusBarsPadding()
                .padding(horizontal = 12.dp, vertical = 8.dp),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            IconButton(onClick = onBack) {
                Icon(Icons.AutoMirrored.Rounded.ArrowBack, contentDescription = "Kembali", tint = palette.textPrimary)
            }
            Text(book.title, color = palette.textPrimary, fontSize = 16.sp, modifier = Modifier.weight(1f))
        }

        if (errorMsg != null) {
            Box(Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
                Text(errorMsg!!, color = palette.textSecondary)
            }
        } else if (pageCount > 0 && renderer != null) {
            val initialPage = (book.progressPercent * pageCount).toInt().coerceIn(0, pageCount - 1)
            val pagerState = rememberPagerState(initialPage = initialPage, pageCount = { pageCount })

            LaunchedEffect(pagerState.settledPage) {
                val frac = (pagerState.settledPage + 1).toFloat() / pageCount.toFloat()
                onProgressCommit(frac)
            }

            HorizontalPager(state = pagerState, modifier = Modifier.weight(1f).fillMaxWidth()) { index ->
                PdfPageImage(renderer = renderer!!, pageIndex = index)
            }

            Text(
                "Hal. ${pagerState.currentPage + 1} / $pageCount",
                color = palette.textSecondary,
                fontSize = 12.sp,
                modifier = Modifier
                    .align(Alignment.CenterHorizontally)
                    .navigationBarsPadding()
                    .padding(12.dp),
            )
        }
    }
}

@Composable
private fun PdfPageImage(renderer: PdfRenderer, pageIndex: Int) {
    var bitmap by remember(pageIndex) { mutableStateOf<Bitmap?>(null) }

    LaunchedEffect(pageIndex) {
        bitmap = withContext(Dispatchers.IO) {
            synchronized(renderer) {
                renderer.openPage(pageIndex).use { page ->
                    val bmp = Bitmap.createBitmap(page.width * 2, page.height * 2, Bitmap.Config.ARGB_8888)
                    bmp.eraseColor(android.graphics.Color.WHITE)
                    page.render(bmp, null, null, PdfRenderer.Page.RENDER_MODE_FOR_DISPLAY)
                    bmp
                }
            }
        }
    }

    Box(modifier = Modifier.fillMaxSize().padding(12.dp), contentAlignment = Alignment.Center) {
        bitmap?.let {
            Image(
                bitmap = it.asImageBitmap(),
                contentDescription = "Halaman ${pageIndex + 1}",
                contentScale = ContentScale.Fit,
                modifier = Modifier.fillMaxSize(),
            )
        } ?: CircularProgressIndicator()
    }
}
