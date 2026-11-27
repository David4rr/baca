package com.baca.nativeapp.ui

import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.animateContentSize
import androidx.compose.animation.expandVertically
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.animation.shrinkVertically
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
import androidx.compose.material.icons.automirrored.rounded.KeyboardArrowRight
import androidx.compose.material.icons.rounded.Add
import androidx.compose.material.icons.rounded.Close
import androidx.compose.material.icons.rounded.MenuBook
import androidx.compose.material.icons.rounded.MoreHoriz
import androidx.compose.material.icons.rounded.Search
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.graphics.Color
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
    var showSearch by remember { mutableStateOf(false) }
    var searchQuery by remember { mutableStateOf("") }
    var showAll by remember { mutableStateOf(false) }

    val heroBook = remember(books) {
        books.filter { it.lastReadAt > 0L }.maxByOrNull { it.lastReadAt } ?: books.firstOrNull()
    }
    val filteredBooks = remember(books, searchQuery, showAll, heroBook) {
        val base = if (searchQuery.isBlank()) books else books.filter {
            it.title.contains(searchQuery, ignoreCase = true) ||
                it.author.contains(searchQuery, ignoreCase = true)
        }
        if (showAll || searchQuery.isNotBlank()) base else base.take(9)
    }

    Box(
        modifier = Modifier
            .fillMaxSize()
            .background(palette.bg),
    ) {
        if (books.isEmpty()) {
            EmptyShelfState(palette = palette, onImportClick = onImportClick)
        } else {
            LazyVerticalGrid(
                columns = GridCells.Fixed(3),
                contentPadding = PaddingValues(
                    start = 20.dp,
                    end = 20.dp,
                    top = 12.dp,
                    bottom = 112.dp,
                ),
                verticalArrangement = Arrangement.spacedBy(18.dp),
                horizontalArrangement = Arrangement.spacedBy(12.dp),
                modifier = Modifier
                    .fillMaxSize()
                    .statusBarsPadding(),
            ) {
                item(span = { GridItemSpan(maxLineSpan) }) {
                    Column(modifier = Modifier.animateContentSize()) {
                        Row(
                            modifier = Modifier.fillMaxWidth(),
                            horizontalArrangement = Arrangement.SpaceBetween,
                            verticalAlignment = Alignment.CenterVertically,
                        ) {
                            Column(modifier = Modifier.weight(1f)) {
                                Text(
                                    text = "My Library",
                                    fontFamily = FontFamily.Serif,
                                    fontSize = 31.sp,
                                    fontWeight = FontWeight.Normal,
                                    color = palette.textPrimary,
                                )
                                Text(
                                    text = "Pick up where you left off",
                                    fontSize = 14.sp,
                                    color = palette.textSecondary,
                                )
                            }
                            Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                                CircleIconButton(
                                    icon = if (showSearch) Icons.Rounded.Close else Icons.Rounded.Search,
                                    desc = "Cari koleksi",
                                    palette = palette,
                                    onClick = {
                                        showSearch = !showSearch
                                        if (!showSearch) searchQuery = ""
                                    },
                                )
                                CircleIconButton(
                                    icon = Icons.Rounded.Add,
                                    desc = "Impor buku",
                                    palette = palette,
                                    onClick = onImportClick,
                                )
                            }
                        }

                        AnimatedVisibility(
                            visible = showSearch,
                            enter = fadeIn() + expandVertically(),
                            exit = fadeOut() + shrinkVertically(),
                        ) {
                            OutlinedTextField(
                                value = searchQuery,
                                onValueChange = { searchQuery = it },
                                placeholder = { Text("Cari judul atau penulis...", fontSize = 14.sp) },
                                singleLine = true,
                                shape = RoundedCornerShape(16.dp),
                                modifier = Modifier
                                    .fillMaxWidth()
                                    .padding(top = 12.dp),
                            )
                        }
                    }
                }

                if (heroBook != null && searchQuery.isBlank()) {
                    item(span = { GridItemSpan(maxLineSpan) }) {
                        HeroBookCard(
                            book = heroBook,
                            palette = palette,
                            onClick = { onOpenBook(heroBook) },
                            onDeleteClick = { pendingDelete = heroBook },
                        )
                    }
                }

                item(span = { GridItemSpan(maxLineSpan) }) {
                    Row(
                        modifier = Modifier
                            .fillMaxWidth()
                            .padding(top = 4.dp),
                        horizontalArrangement = Arrangement.SpaceBetween,
                        verticalAlignment = Alignment.CenterVertically,
                    ) {
                        Text(
                            text = if (searchQuery.isBlank()) "Recent" else "Search Results",
                            fontFamily = FontFamily.Serif,
                            fontSize = 19.sp,
                            fontWeight = FontWeight.Medium,
                            color = palette.textPrimary,
                        )
                        if (books.size > 9 && searchQuery.isBlank()) {
                            Text(
                                text = if (showAll) "Show less" else "View all",
                                fontSize = 13.sp,
                                color = palette.textSecondary,
                                modifier = Modifier
                                    .clip(RoundedCornerShape(8.dp))
                                    .clickable { showAll = !showAll }
                                    .padding(horizontal = 6.dp, vertical = 4.dp),
                            )
                        }
                    }
                }

                items(filteredBooks, key = { it.id }) { book ->
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
            title = { Text("Hapus Buku", fontFamily = FontFamily.Serif) },
            text = { Text("Hapus \"${target.title}\" dari perpustakaan?") },
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
private fun CircleIconButton(
    icon: androidx.compose.ui.graphics.vector.ImageVector,
    desc: String,
    palette: BacaPalette,
    onClick: () -> Unit,
) {
    Box(
        modifier = Modifier
            .size(42.dp)
            .shadow(3.dp, CircleShape, ambientColor = Color.Black.copy(alpha = 0.08f))
            .clip(CircleShape)
            .background(palette.surface)
            .border(0.5.dp, palette.border, CircleShape)
            .applePressScale()
            .clickable(onClick = onClick),
        contentAlignment = Alignment.Center,
    ) {
        Icon(icon, contentDescription = desc, tint = palette.textPrimary, modifier = Modifier.size(20.dp))
    }
}

@Composable
private fun EmptyShelfState(palette: BacaPalette, onImportClick: () -> Unit) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .statusBarsPadding()
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
            text = "My Library is Empty",
            fontFamily = FontFamily.Serif,
            fontSize = 24.sp,
            color = palette.textPrimary,
        )
        Spacer(Modifier.height(8.dp))
        Text(
            text = "Import an EPUB or PDF from your device, or explore free classics online.",
            fontSize = 14.sp,
            color = palette.textSecondary,
        )
        Spacer(Modifier.height(22.dp))
        Button(
            onClick = onImportClick,
            shape = CircleShape,
            colors = ButtonDefaults.buttonColors(
                containerColor = palette.textPrimary,
                contentColor = palette.bg,
            ),
            contentPadding = PaddingValues(horizontal = 24.dp, vertical = 14.dp),
        ) {
            Icon(Icons.Rounded.Add, contentDescription = null, modifier = Modifier.size(18.dp))
            Spacer(Modifier.width(8.dp))
            Text("Import Book", fontWeight = FontWeight.Medium)
        }
    }
}

@Composable
private fun HeroBookCard(
    book: NativeBook,
    palette: BacaPalette,
    onClick: () -> Unit,
    onDeleteClick: () -> Unit,
) {
    var menuExpanded by remember { mutableStateOf(false) }
    val pct = (book.progressPercent * 100f).roundToInt().coerceIn(0, 100)
    val minsLeft = (((1f - book.progressPercent.coerceIn(0f, 1f)) * 40f).roundToInt()).coerceAtLeast(1)
    val shape = RoundedCornerShape(24.dp)

    Column(
        modifier = Modifier
            .fillMaxWidth()
            .shadow(10.dp, shape, ambientColor = Color.Black.copy(alpha = 0.08f))
            .clip(shape)
            .background(palette.surface)
            .border(0.5.dp, palette.border, shape)
            .padding(18.dp),
    ) {
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(16.dp),
        ) {
            BookCoverArt(
                coverDataUri = book.coverImage,
                title = book.title,
                author = book.author,
                palette = palette,
                modifier = Modifier
                    .width(96.dp)
                    .aspectRatio(2f / 3f)
                    .clickable(onClick = onClick),
            )

            Column(
                modifier = Modifier
                    .weight(1f)
                    .heightIn(min = 144.dp),
                verticalArrangement = Arrangement.SpaceBetween,
            ) {
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceBetween,
                    verticalAlignment = Alignment.Top,
                ) {
                    Column(modifier = Modifier.weight(1f).padding(end = 6.dp)) {
                        Text(
                            text = book.title,
                            fontFamily = FontFamily.Serif,
                            fontSize = 18.sp,
                            lineHeight = 23.sp,
                            fontWeight = FontWeight.Medium,
                            color = palette.textPrimary,
                            maxLines = 2,
                            overflow = TextOverflow.Ellipsis,
                        )
                        Spacer(Modifier.height(4.dp))
                        Text(
                            text = book.author,
                            fontSize = 13.sp,
                            color = palette.textSecondary,
                            maxLines = 1,
                            overflow = TextOverflow.Ellipsis,
                        )
                    }

                    Box {
                        Box(
                            modifier = Modifier
                                .size(30.dp)
                                .clip(CircleShape)
                                .background(palette.surfaceMuted)
                                .clickable { menuExpanded = true },
                            contentAlignment = Alignment.Center,
                        ) {
                            Icon(
                                Icons.Rounded.MoreHoriz,
                                contentDescription = "Opsi",
                                tint = palette.textSecondary,
                                modifier = Modifier.size(18.dp),
                            )
                        }
                        DropdownMenu(expanded = menuExpanded, onDismissRequest = { menuExpanded = false }) {
                            DropdownMenuItem(
                                text = { Text("Baca Buku") },
                                onClick = {
                                    menuExpanded = false
                                    onClick()
                                },
                            )
                            DropdownMenuItem(
                                text = { Text("Hapus Buku", color = MaterialTheme.colorScheme.error) },
                                onClick = {
                                    menuExpanded = false
                                    onDeleteClick()
                                },
                            )
                        }
                    }
                }

                Column(modifier = Modifier.fillMaxWidth().padding(top = 14.dp)) {
                    Row(
                        modifier = Modifier.fillMaxWidth(),
                        horizontalArrangement = Arrangement.End,
                    ) {
                        Text("$pct%", fontSize = 12.sp, fontWeight = FontWeight.Medium, color = palette.textSecondary)
                    }
                    Spacer(Modifier.height(4.dp))
                    LinearProgressIndicator(
                        progress = { book.progressPercent.coerceIn(0f, 1f) },
                        modifier = Modifier
                            .fillMaxWidth()
                            .height(4.dp)
                            .clip(CircleShape),
                        color = palette.textPrimary,
                        trackColor = palette.surfaceMuted,
                    )
                    Spacer(Modifier.height(6.dp))
                    Text(
                        text = if (pct >= 100) "Finished" else "$minsLeft min left in chapter",
                        fontSize = 12.sp,
                        color = palette.textSecondary,
                    )
                }
            }
        }

        Spacer(Modifier.height(16.dp))

        Row(
            modifier = Modifier
                .fillMaxWidth()
                .clip(CircleShape)
                .background(palette.textPrimary)
                .applePressScale()
                .clickable(onClick = onClick)
                .padding(horizontal = 20.dp, vertical = 14.dp),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Text(
                text = "Continue Reading",
                fontSize = 15.sp,
                fontWeight = FontWeight.Medium,
                color = palette.bg,
            )
            Icon(
                Icons.AutoMirrored.Rounded.KeyboardArrowRight,
                contentDescription = null,
                tint = palette.bg,
                modifier = Modifier.size(20.dp),
            )
        }
    }
}

@Composable
private fun BookGridCard(
    book: NativeBook,
    palette: BacaPalette,
    onClick: () -> Unit,
    onDeleteClick: () -> Unit,
) {
    var menuExpanded by remember { mutableStateOf(false) }
    val pct = (book.progressPercent * 100f).roundToInt().coerceIn(0, 100)

    Column(
        modifier = Modifier
            .fillMaxWidth()
            .applePressScale()
            .clickable(onClick = onClick),
    ) {
        Box {
            BookCoverArt(
                coverDataUri = book.coverImage,
                title = book.title,
                author = book.author,
                palette = palette,
                modifier = Modifier
                    .fillMaxWidth()
                    .aspectRatio(2f / 3f),
            )
            Box(
                modifier = Modifier
                    .align(Alignment.TopEnd)
                    .padding(6.dp)
                    .size(24.dp)
                    .clip(CircleShape)
                    .background(Color.Black.copy(alpha = 0.45f))
                    .clickable { menuExpanded = true },
                contentAlignment = Alignment.Center,
            ) {
                Icon(
                    Icons.Rounded.MoreHoriz,
                    contentDescription = "Opsi",
                    tint = Color.White,
                    modifier = Modifier.size(14.dp),
                )
                DropdownMenu(expanded = menuExpanded, onDismissRequest = { menuExpanded = false }) {
                    DropdownMenuItem(
                        text = { Text("Hapus Buku", color = MaterialTheme.colorScheme.error) },
                        onClick = {
                            menuExpanded = false
                            onDeleteClick()
                        },
                    )
                }
            }
        }
        Spacer(Modifier.height(8.dp))
        Text(
            text = book.title,
            fontSize = 13.sp,
            lineHeight = 17.sp,
            fontWeight = FontWeight.SemiBold,
            color = palette.textPrimary,
            maxLines = 2,
            overflow = TextOverflow.Ellipsis,
        )
        Spacer(Modifier.height(2.dp))
        Text(
            text = book.author,
            fontSize = 11.sp,
            color = palette.textSecondary,
            maxLines = 1,
            overflow = TextOverflow.Ellipsis,
        )
        Spacer(Modifier.height(2.dp))
        Text(
            text = if (pct >= 100) "Finished" else "$pct%",
            fontSize = 11.sp,
            color = palette.textSecondary,
        )
    }
}
