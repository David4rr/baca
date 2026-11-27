package com.baca.nativeapp.ui

import android.graphics.Bitmap
import android.graphics.pdf.PdfRenderer
import android.os.ParcelFileDescriptor
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.pager.HorizontalPager
import androidx.compose.foundation.pager.rememberPagerState
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.rounded.KeyboardArrowLeft
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.asImageBitmap
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.style.TextOverflow
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
    onSessionEnd: (Int) -> Unit,
    onBack: () -> Unit,
) {
    val startTimeMs = remember(book.id) { System.currentTimeMillis() }
    DisposableEffect(book.id) {
        onDispose {
            val elapsedSec = ((System.currentTimeMillis() - startTimeMs) / 1000L).toInt().coerceAtLeast(1)
            onSessionEnd(elapsedSec)
        }
    }

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
                .padding(horizontal = 16.dp, vertical = 10.dp),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            Box(
                modifier = Modifier
                    .size(40.dp)
                    .shadow(4.dp, CircleShape, ambientColor = Color.Black.copy(alpha = 0.08f))
                    .clip(CircleShape)
                    .background(palette.surface)
                    .border(0.5.dp, palette.border, CircleShape)
                    .applePressScale()
                    .clickable(onClick = onBack),
                contentAlignment = Alignment.Center,
            ) {
                Icon(
                    Icons.AutoMirrored.Rounded.KeyboardArrowLeft,
                    contentDescription = "Kembali",
                    tint = palette.textPrimary,
                )
            }
            Text(
                text = book.title,
                fontFamily = FontFamily.Serif,
                color = palette.textPrimary,
                fontSize = 17.sp,
                maxLines = 1,
                overflow = TextOverflow.Ellipsis,
                modifier = Modifier.weight(1f),
            )
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
                "Page ${pagerState.currentPage + 1} of $pageCount",
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
