package com.baca.nativeapp.ui

import androidx.compose.animation.animateContentSize
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.rounded.Check
import androidx.compose.material.icons.rounded.Close
import androidx.compose.material.icons.rounded.Download
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
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import uniffi.baca_core.NativeBook
import uniffi.baca_core.NativeSearchBook

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ExploreScreen(
    palette: BacaPalette,
    libraryBooks: List<NativeBook>,
    onSearch: suspend (String) -> Result<List<NativeSearchBook>>,
    onDownload: suspend (NativeSearchBook, String) -> Result<NativeBook>,
    onOpenDownloaded: (NativeBook) -> Unit,
) {
    var query by remember { mutableStateOf("") }
    var activeCategory by remember { mutableStateOf("Classic") }
    var results by remember { mutableStateOf<List<NativeSearchBook>>(emptyList()) }
    var loading by remember { mutableStateOf(false) }
    var errorMsg by remember { mutableStateOf<String?>(null) }
    var selectedBook by remember { mutableStateOf<NativeSearchBook?>(null) }
    var downloadingFormat by remember { mutableStateOf<String?>(null) }

    val categories = listOf("Classic", "Philosophy", "Science", "History", "Fiction")

    LaunchedEffect(query, activeCategory) {
        val term = if (query.isNotBlank()) query.trim() else activeCategory
        delay(350)
        loading = true
        errorMsg = null
        onSearch(term)
            .onSuccess { results = it }
            .onFailure { errorMsg = it.message ?: "Gagal memuat pencarian" }
        loading = false
    }

    LazyColumn(
        contentPadding = PaddingValues(
            start = 20.dp,
            end = 20.dp,
            top = 12.dp,
            bottom = 112.dp,
        ),
        verticalArrangement = Arrangement.spacedBy(14.dp),
        modifier = Modifier
            .fillMaxSize()
            .background(palette.bg)
            .statusBarsPadding(),
    ) {
        item {
            Column {
                Text(
                    text = "Explore",
                    fontFamily = FontFamily.Serif,
                    fontSize = 31.sp,
                    color = palette.textPrimary,
                )
                Text(
                    text = "Free classics from Archive.org & Gutenberg",
                    fontSize = 14.sp,
                    color = palette.textSecondary,
                )
                Spacer(Modifier.height(14.dp))
                OutlinedTextField(
                    value = query,
                    onValueChange = { query = it },
                    placeholder = { Text("Search title or author...", fontSize = 14.sp) },
                    leadingIcon = { Icon(Icons.Rounded.Search, contentDescription = null) },
                    trailingIcon = {
                        if (query.isNotEmpty()) {
                            IconButton(onClick = { query = "" }) {
                                Icon(Icons.Rounded.Close, contentDescription = "Hapus")
                            }
                        }
                    },
                    singleLine = true,
                    shape = RoundedCornerShape(18.dp),
                    modifier = Modifier.fillMaxWidth(),
                )
                Spacer(Modifier.height(12.dp))
                Row(
                    modifier = Modifier.horizontalScroll(rememberScrollState()),
                    horizontalArrangement = Arrangement.spacedBy(8.dp),
                ) {
                    categories.forEach { cat ->
                        val selected = activeCategory == cat && query.isBlank()
                        Box(
                            modifier = Modifier
                                .clip(CircleShape)
                                .background(if (selected) palette.textPrimary else palette.surface)
                                .border(0.5.dp, palette.border, CircleShape)
                                .applePressScale()
                                .clickable {
                                    query = ""
                                    activeCategory = cat
                                }
                                .padding(horizontal = 16.dp, vertical = 8.dp),
                        ) {
                            Text(
                                text = cat,
                                fontSize = 13.sp,
                                fontWeight = if (selected) FontWeight.SemiBold else FontWeight.Normal,
                                color = if (selected) palette.bg else palette.textPrimary,
                            )
                        }
                    }
                }
            }
        }

        if (loading) {
            item {
                Box(
                    modifier = Modifier.fillMaxWidth().padding(32.dp),
                    contentAlignment = Alignment.Center,
                ) {
                    CircularProgressIndicator(color = palette.accent)
                }
            }
        } else if (errorMsg != null) {
            item {
                Text(
                    text = errorMsg!!,
                    color = MaterialTheme.colorScheme.error,
                    fontSize = 14.sp,
                    modifier = Modifier.padding(vertical = 16.dp),
                )
            }
        } else if (results.isEmpty()) {
            item {
                Text(
                    text = "Tidak ada buku ditemukan untuk \"$query\".",
                    color = palette.textSecondary,
                    fontSize = 14.sp,
                    modifier = Modifier.padding(vertical = 24.dp),
                )
            }
        } else {
            items(results, key = { it.identifier }) { item ->
                val saved = remember(libraryBooks, item.title) {
                    libraryBooks.firstOrNull { it.title.equals(item.title, ignoreCase = true) }
                }
                ExploreBookRow(
                    item = item,
                    savedBook = saved,
                    palette = palette,
                    onClick = { selectedBook = item },
                    onOpenSaved = { if (saved != null) onOpenDownloaded(saved) },
                )
            }
        }
    }

    selectedBook?.let { target ->
        val saved = libraryBooks.firstOrNull { it.title.equals(target.title, ignoreCase = true) }
        val scope = rememberCoroutineScope()
        var detailError by remember { mutableStateOf<String?>(null) }

        ModalBottomSheet(
            onDismissRequest = { if (downloadingFormat == null) selectedBook = null },
            containerColor = palette.surface,
        ) {
            Column(
                modifier = Modifier
                    .fillMaxWidth()
                    .navigationBarsPadding()
                    .padding(horizontal = 24.dp, vertical = 12.dp)
                    .animateContentSize(),
            ) {
                Row(horizontalArrangement = Arrangement.spacedBy(16.dp)) {
                    BookCoverArt(
                        coverDataUri = null,
                        title = target.title,
                        author = target.creator,
                        palette = palette,
                        modifier = Modifier.width(86.dp).aspectRatio(2f / 3f),
                    )
                    Column(modifier = Modifier.weight(1f)) {
                        Text(
                            text = target.title,
                            fontFamily = FontFamily.Serif,
                            fontSize = 20.sp,
                            color = palette.textPrimary,
                            maxLines = 3,
                            overflow = TextOverflow.Ellipsis,
                        )
                        Spacer(Modifier.height(4.dp))
                        Text(target.creator, fontSize = 13.sp, color = palette.textSecondary)
                    }
                }

                if (target.description.isNotBlank()) {
                    Spacer(Modifier.height(14.dp))
                    Text(
                        text = target.description,
                        fontSize = 13.sp,
                        lineHeight = 19.sp,
                        color = palette.textSecondary,
                        maxLines = 6,
                        overflow = TextOverflow.Ellipsis,
                    )
                }

                detailError?.let {
                    Spacer(Modifier.height(10.dp))
                    Text(it, color = MaterialTheme.colorScheme.error, fontSize = 12.sp)
                }

                Spacer(Modifier.height(18.dp))

                if (saved != null) {
                    Button(
                        onClick = {
                            selectedBook = null
                            onOpenDownloaded(saved)
                        },
                        shape = CircleShape,
                        colors = ButtonDefaults.buttonColors(
                            containerColor = palette.textPrimary,
                            contentColor = palette.bg,
                        ),
                        modifier = Modifier.fillMaxWidth().height(50.dp),
                    ) {
                        Icon(Icons.Rounded.Check, contentDescription = null, modifier = Modifier.size(18.dp))
                        Spacer(Modifier.width(8.dp))
                        Text("Baca Sekarang")
                    }
                } else {
                    Row(
                        modifier = Modifier.fillMaxWidth(),
                        horizontalArrangement = Arrangement.spacedBy(10.dp),
                    ) {
                        DownloadFormatButton(
                            label = if (downloadingFormat == "epub") "Mengunduh..." else "Unduh EPUB",
                            enabled = downloadingFormat == null,
                            primary = true,
                            palette = palette,
                            modifier = Modifier.weight(1f),
                            onClick = {
                                detailError = null
                                downloadingFormat = "epub"
                                scope.launch {
                                    onDownload(target, "epub")
                                        .onSuccess {
                                            downloadingFormat = null
                                            selectedBook = null
                                        }
                                        .onFailure {
                                            downloadingFormat = null
                                            detailError = it.message ?: "Gagal mengunduh EPUB"
                                        }
                                }
                            },
                        )
                        DownloadFormatButton(
                            label = if (downloadingFormat == "pdf") "Mengunduh..." else "Unduh PDF",
                            enabled = downloadingFormat == null,
                            primary = false,
                            palette = palette,
                            modifier = Modifier.weight(1f),
                            onClick = {
                                detailError = null
                                downloadingFormat = "pdf"
                                scope.launch {
                                    onDownload(target, "pdf")
                                        .onSuccess {
                                            downloadingFormat = null
                                            selectedBook = null
                                        }
                                        .onFailure {
                                            downloadingFormat = null
                                            detailError = it.message ?: "Gagal mengunduh PDF"
                                        }
                                }
                            },
                        )
                    }
                }
                Spacer(Modifier.height(16.dp))
            }
        }
    }
}

@Composable
private fun DownloadFormatButton(
    label: String,
    enabled: Boolean,
    primary: Boolean,
    palette: BacaPalette,
    modifier: Modifier = Modifier,
    onClick: () -> Unit,
) {
    Button(
        onClick = onClick,
        enabled = enabled,
        shape = CircleShape,
        colors = ButtonDefaults.buttonColors(
            containerColor = if (primary) palette.textPrimary else palette.surfaceMuted,
            contentColor = if (primary) palette.bg else palette.textPrimary,
        ),
        modifier = modifier.height(48.dp),
    ) {
        Icon(Icons.Rounded.Download, contentDescription = null, modifier = Modifier.size(16.dp))
        Spacer(Modifier.width(6.dp))
        Text(label, fontSize = 13.sp, fontWeight = FontWeight.Medium)
    }
}

@Composable
private fun ExploreBookRow(
    item: NativeSearchBook,
    savedBook: NativeBook?,
    palette: BacaPalette,
    onClick: () -> Unit,
    onOpenSaved: () -> Unit,
) {
    val shape = RoundedCornerShape(18.dp)
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .shadow(4.dp, shape, ambientColor = Color.Black.copy(alpha = 0.06f))
            .clip(shape)
            .background(palette.surface)
            .border(0.5.dp, palette.border, shape)
            .applePressScale()
            .clickable(onClick = if (savedBook != null) onOpenSaved else onClick)
            .padding(14.dp),
        horizontalArrangement = Arrangement.spacedBy(14.dp),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        BookCoverArt(
            coverDataUri = null,
            title = item.title,
            author = item.creator,
            palette = palette,
            modifier = Modifier.width(60.dp).aspectRatio(2f / 3f),
        )
        Column(modifier = Modifier.weight(1f)) {
            Text(
                text = item.title,
                fontFamily = FontFamily.Serif,
                fontSize = 16.sp,
                lineHeight = 20.sp,
                color = palette.textPrimary,
                maxLines = 2,
                overflow = TextOverflow.Ellipsis,
            )
            Spacer(Modifier.height(2.dp))
            Text(
                text = item.creator,
                fontSize = 12.sp,
                color = palette.textSecondary,
                maxLines = 1,
                overflow = TextOverflow.Ellipsis,
            )
            if (item.description.isNotBlank()) {
                Spacer(Modifier.height(4.dp))
                Text(
                    text = item.description,
                    fontSize = 11.sp,
                    lineHeight = 15.sp,
                    color = palette.textSecondary,
                    maxLines = 2,
                    overflow = TextOverflow.Ellipsis,
                )
            }
        }
        Box(
            modifier = Modifier
                .clip(CircleShape)
                .background(if (savedBook != null) palette.accent.copy(alpha = 0.14f) else palette.surfaceMuted)
                .padding(horizontal = 10.dp, vertical = 6.dp),
        ) {
            Text(
                text = if (savedBook != null) "Tersimpan" else "Unduh",
                fontSize = 11.sp,
                fontWeight = FontWeight.SemiBold,
                color = if (savedBook != null) palette.accent else palette.textPrimary,
            )
        }
    }
}
