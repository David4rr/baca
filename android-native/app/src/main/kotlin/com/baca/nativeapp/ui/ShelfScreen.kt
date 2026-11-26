package com.baca.nativeapp.ui

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.grid.GridCells
import androidx.compose.foundation.lazy.grid.GridItemSpan
import androidx.compose.foundation.lazy.grid.LazyVerticalGrid
import androidx.compose.foundation.lazy.grid.items
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.rounded.Add
import androidx.compose.material.icons.rounded.DeleteOutline
import androidx.compose.material.icons.rounded.MenuBook
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import uniffi.baca_core.NativeBook
import kotlin.math.roundToInt

@Composable
fun ShelfScreen(
    books: List<NativeBook>,
    palette: BacaPalette,
    onImportClick: () -> Unit,
    onOpenBook: (NativeBook) -> Unit,
    onDeleteBook: (NativeBook) -> Unit,
) {
    var pendingDelete by remember { mutableStateOf<NativeBook?>(null) }
    val heroBook = remember(books) { books.firstOrNull { it.lastReadAt > 0L } }

    Scaffold(
        containerColor = palette.bg,
        floatingActionButton = {
            FloatingActionButton(
                onClick = onImportClick,
                containerColor = palette.textPrimary,
                contentColor = palette.bg,
                shape = CircleShape,
            ) {
                Icon(Icons.Rounded.Add, contentDescription = "Tambah buku")
            }
        },
    ) { padding ->
        if (books.isEmpty()) {
            Column(
                modifier = Modifier
                    .fillMaxSize()
                    .padding(padding)
                    .padding(32.dp),
                verticalArrangement = Arrangement.Center,
                horizontalAlignment = Alignment.CenterHorizontally,
            ) {
                Icon(
                    Icons.Rounded.MenuBook,
                    contentDescription = null,
                    tint = palette.textSecondary,
                    modifier = Modifier.size(48.dp),
                )
                Spacer(Modifier.height(16.dp))
                Text(
                    "Rak Buku Masih Kosong",
                    fontFamily = FontFamily.Serif,
                    fontSize = 22.sp,
                    color = palette.textPrimary,
                )
                Spacer(Modifier.height(8.dp))
                Text(
                    "Ketuk tombol + untuk mengimpor EPUB atau PDF dari perangkat Anda.",
                    fontSize = 14.sp,
                    color = palette.textSecondary,
                )
            }
        } else {
            LazyVerticalGrid(
                columns = GridCells.Adaptive(150.dp),
                contentPadding = PaddingValues(
                    start = 20.dp,
                    end = 20.dp,
                    top = padding.calculateTopPadding() + 16.dp,
                    bottom = padding.calculateBottomPadding() + 88.dp,
                ),
                verticalArrangement = Arrangement.spacedBy(16.dp),
                horizontalArrangement = Arrangement.spacedBy(14.dp),
                modifier = Modifier.fillMaxSize(),
            ) {
                item(span = { GridItemSpan(maxLineSpan) }) {
                    Text(
                        "Perpustakaan",
                        fontFamily = FontFamily.Serif,
                        fontSize = 30.sp,
                        fontWeight = FontWeight.Medium,
                        color = palette.textPrimary,
                    )
                }

                if (heroBook != null) {
                    item(span = { GridItemSpan(maxLineSpan) }) {
                        HeroBookCard(
                            book = heroBook,
                            palette = palette,
                            onClick = { onOpenBook(heroBook) },
                        )
                    }
                }

                items(books, key = { it.id }) { book ->
                    BookGridCard(
                        book = book,
                        palette = palette,
                        onClick = { onOpenBook(book) },
                        onDeleteClick = { pendingDelete = book },
                    )
                }
            }
        }
    }

    pendingDelete?.let { target ->
        AlertDialog(
            onDismissRequest = { pendingDelete = null },
            title = { Text("Hapus Buku") },
            text = { Text("Hapus \"${target.title}\" dari rak bacaan secara permanen?") },
            confirmButton = {
                TextButton(onClick = {
                    pendingDelete = null
                    onDeleteBook(target)
                }) {
                    Text("Hapus", color = MaterialTheme.colorScheme.error)
                }
            },
            dismissButton = {
                TextButton(onClick = { pendingDelete = null }) { Text("Batal") }
            },
        )
    }
}

@Composable
private fun HeroBookCard(book: NativeBook, palette: BacaPalette, onClick: () -> Unit) {
    val pct = (book.progressPercent * 100f).roundToInt().coerceIn(0, 100)
    Column(
        modifier = Modifier
            .fillMaxWidth()
            .clip(RoundedCornerShape(22.dp))
            .background(palette.surface)
            .border(1.dp, palette.border, RoundedCornerShape(22.dp))
            .clickable(onClick = onClick)
            .padding(18.dp),
    ) {
        Text(
            "LANJUTKAN MEMBACA",
            fontSize = 11.sp,
            fontWeight = FontWeight.SemiBold,
            color = palette.accent,
        )
        Spacer(Modifier.height(6.dp))
        Text(
            book.title,
            fontFamily = FontFamily.Serif,
            fontSize = 20.sp,
            color = palette.textPrimary,
            maxLines = 2,
            overflow = TextOverflow.Ellipsis,
        )
        Text(
            book.author,
            fontSize = 13.sp,
            color = palette.textSecondary,
            maxLines = 1,
            overflow = TextOverflow.Ellipsis,
        )
        Spacer(Modifier.height(12.dp))
        LinearProgressIndicator(
            progress = { book.progressPercent.coerceIn(0f, 1f) },
            modifier = Modifier.fillMaxWidth().height(5.dp).clip(CircleShape),
            color = palette.accent,
            trackColor = palette.border,
        )
        Spacer(Modifier.height(6.dp))
        Text("$pct% selesai • ${book.format.uppercase()}", fontSize = 11.sp, color = palette.textSecondary)
    }
}

@Composable
private fun BookGridCard(
    book: NativeBook,
    palette: BacaPalette,
    onClick: () -> Unit,
    onDeleteClick: () -> Unit,
) {
    val pct = (book.progressPercent * 100f).roundToInt().coerceIn(0, 100)
    Column(
        modifier = Modifier
            .fillMaxWidth()
            .clip(RoundedCornerShape(18.dp))
            .background(palette.surface)
            .border(1.dp, palette.border, RoundedCornerShape(18.dp))
            .clickable(onClick = onClick)
            .padding(14.dp),
    ) {
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Text(
                book.format.uppercase(),
                fontSize = 10.sp,
                fontWeight = FontWeight.Bold,
                color = palette.accent,
            )
            IconButton(onClick = onDeleteClick, modifier = Modifier.size(28.dp)) {
                Icon(
                    Icons.Rounded.DeleteOutline,
                    contentDescription = "Hapus",
                    tint = palette.textSecondary,
                    modifier = Modifier.size(18.dp),
                )
            }
        }
        Spacer(Modifier.height(10.dp))
        Text(
            book.title,
            fontFamily = FontFamily.Serif,
            fontSize = 16.sp,
            lineHeight = 21.sp,
            color = palette.textPrimary,
            maxLines = 3,
            overflow = TextOverflow.Ellipsis,
        )
        Spacer(Modifier.height(4.dp))
        Text(
            book.author,
            fontSize = 12.sp,
            color = palette.textSecondary,
            maxLines = 1,
            overflow = TextOverflow.Ellipsis,
        )
        Spacer(Modifier.height(10.dp))
        Text("$pct%", fontSize = 11.sp, color = palette.textSecondary)
    }
}
