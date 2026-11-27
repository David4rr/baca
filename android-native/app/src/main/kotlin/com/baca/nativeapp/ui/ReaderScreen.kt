package com.baca.nativeapp.ui

import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.animation.slideInVertically
import androidx.compose.animation.slideOutVertically
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.itemsIndexed
import androidx.compose.foundation.lazy.rememberLazyListState
import androidx.compose.foundation.pager.HorizontalPager
import androidx.compose.foundation.pager.rememberPagerState
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.rounded.KeyboardArrowLeft
import androidx.compose.material.icons.automirrored.rounded.MenuBook
import androidx.compose.material.icons.rounded.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.SpanStyle
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.buildAnnotatedString
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontStyle
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.rememberTextMeasurer
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.Constraints
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import kotlinx.coroutines.flow.distinctUntilChanged
import kotlinx.coroutines.launch
import uniffi.baca_core.*
import kotlin.math.roundToInt

private sealed interface ScrollRow {
    data class ChapterHeader(val chapterIndex: Int, val title: String) : ScrollRow
    data class BlockRow(val chapterIndex: Int, val block: ReaderBlock) : ScrollRow
}

private data class PagedItem(val chapterIndex: Int, val blocks: List<ReaderBlock>)

private data class SearchHit(val chapterIndex: Int, val chapterTitle: String, val snippet: String)

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ReaderScreen(
    book: NativeBook,
    epub: NativeEpubBook,
    palette: BacaPalette,
    settings: NativeSettings,
    bookmarks: List<NativeBookmark>,
    onToggleBookmark: (Int, String, String) -> Unit,
    onDeleteBookmark: (String) -> Unit,
    onProgressCommit: (Int, Float) -> Unit,
    onUpdateSettings: (NativeSettings) -> Unit,
    onAskAi: suspend (String) -> Result<String>,
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

    var isPaged by remember { mutableStateOf(false) }
    var isFullscreen by remember { mutableStateOf(false) }
    var showToc by remember { mutableStateOf(false) }
    var showTypography by remember { mutableStateOf(false) }
    var showSearch by remember { mutableStateOf(false) }
    var searchQuery by remember { mutableStateOf("") }
    var showAiModal by remember { mutableStateOf(false) }

    val maxChapter = (epub.chapters.size - 1).coerceAtLeast(0)
    var currentChapter by remember(book.id) {
        mutableIntStateOf(book.currentChapter.toInt().coerceIn(0, maxChapter))
    }

    val fontFamily = remember(settings.fontFamily) { BacaPalettes.fontFamily(settings.fontFamily) }
    val textStyle = remember(settings, palette, fontFamily) {
        TextStyle(
            fontFamily = fontFamily,
            fontSize = settings.fontSize.toInt().sp,
            lineHeight = (settings.fontSize.toFloat() * settings.lineHeight).sp,
            color = palette.textPrimary,
        )
    }

    val scrollRows = remember(epub) {
        buildList {
            epub.chapters.forEachIndexed { chIdx, ch ->
                add(ScrollRow.ChapterHeader(chIdx, ch.title))
                ch.blocks.forEach { block -> add(ScrollRow.BlockRow(chIdx, block)) }
            }
        }
    }
    val chapterStartIndices = remember(scrollRows) {
        val map = mutableMapOf<Int, Int>()
        scrollRows.forEachIndexed { idx, row ->
            if (row is ScrollRow.ChapterHeader) map[row.chapterIndex] = idx
        }
        map
    }

    val scrollState = rememberLazyListState(
        initialFirstVisibleItemIndex = chapterStartIndices[currentChapter] ?: 0
    )
    val scope = rememberCoroutineScope()

    LaunchedEffect(scrollState) {
        snapshotFlow { scrollState.isScrollInProgress }
            .distinctUntilChanged()
            .collect { scrolling ->
                if (!scrolling && !isPaged && scrollRows.isNotEmpty()) {
                    val idx = scrollState.firstVisibleItemIndex.coerceIn(0, scrollRows.lastIndex)
                    val chIdx = when (val r = scrollRows[idx]) {
                        is ScrollRow.ChapterHeader -> r.chapterIndex
                        is ScrollRow.BlockRow -> r.chapterIndex
                    }
                    currentChapter = chIdx
                    val frac = ((idx + 1).toFloat() / scrollRows.size.coerceAtLeast(1).toFloat()).coerceIn(0f, 1f)
                    onProgressCommit(chIdx, frac)
                }
            }
    }

    val bookmarkedChapters = remember(bookmarks) { bookmarks.map { it.chapterIndex.toInt() }.toSet() }

    Box(
        modifier = Modifier
            .fillMaxSize()
            .background(palette.bg)
            .clickable(
                interactionSource = remember { MutableInteractionSource() },
                indication = null,
            ) { isFullscreen = !isFullscreen },
    ) {
        if (!isPaged) {
            LazyColumn(
                state = scrollState,
                contentPadding = PaddingValues(
                    start = settings.marginHorizontal.toInt().dp,
                    end = settings.marginHorizontal.toInt().dp,
                    top = 96.dp,
                    bottom = 172.dp,
                ),
                verticalArrangement = Arrangement.spacedBy(14.dp),
                modifier = Modifier.fillMaxSize(),
            ) {
                itemsIndexed(scrollRows) { _, row ->
                    when (row) {
                        is ScrollRow.ChapterHeader -> Column(
                            horizontalAlignment = Alignment.CenterHorizontally,
                            modifier = Modifier.fillMaxWidth().padding(top = 20.dp, bottom = 14.dp),
                        ) {
                            Text(
                                text = "CHAPTER ${row.chapterIndex + 1}",
                                fontSize = 11.sp,
                                letterSpacing = 1.6.sp,
                                color = palette.textSecondary,
                            )
                            Spacer(Modifier.height(8.dp))
                            Text(
                                text = row.title,
                                fontFamily = FontFamily.Serif,
                                fontSize = 27.sp,
                                lineHeight = 33.sp,
                                color = palette.textPrimary,
                                textAlign = TextAlign.Center,
                            )
                            Spacer(Modifier.height(14.dp))
                            Box(
                                modifier = Modifier
                                    .width(32.dp)
                                    .height(1.dp)
                                    .background(palette.border),
                            )
                        }
                        is ScrollRow.BlockRow -> RenderBlock(row.block, textStyle, palette, searchQuery)
                    }
                }
            }
        } else {
            PagedNativeContent(
                epub = epub,
                currentChapter = currentChapter,
                textStyle = textStyle,
                palette = palette,
                highlightQuery = searchQuery,
                marginDp = settings.marginHorizontal.toInt(),
                onPageSettled = { ch, frac ->
                    currentChapter = ch
                    onProgressCommit(ch, frac)
                },
            )
        }

        // Top Bar matching reference: circular < back on left; Aa, TOC, Bookmark, Search, Mode on right
        AnimatedVisibility(
            visible = !isFullscreen,
            enter = fadeIn() + slideInVertically { -it / 2 },
            exit = fadeOut() + slideOutVertically { -it / 2 },
            modifier = Modifier.align(Alignment.TopCenter),
        ) {
            Column(
                modifier = Modifier
                    .fillMaxWidth()
                    .background(palette.bg.copy(alpha = 0.94f))
                    .statusBarsPadding()
                    .padding(horizontal = 16.dp, vertical = 10.dp),
            ) {
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceBetween,
                    verticalAlignment = Alignment.CenterVertically,
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

                    Row(
                        horizontalArrangement = Arrangement.spacedBy(2.dp),
                        verticalAlignment = Alignment.CenterVertically,
                    ) {
                        IconButton(onClick = { showTypography = true }) {
                            Text(
                                "Aa",
                                fontFamily = FontFamily.Serif,
                                fontSize = 17.sp,
                                fontWeight = FontWeight.Medium,
                                color = palette.textPrimary,
                            )
                        }
                        IconButton(onClick = { showToc = true }) {
                            Icon(Icons.Rounded.FormatListBulleted, contentDescription = "Daftar isi", tint = palette.textPrimary)
                        }
                        val isBookmarked = currentChapter in bookmarkedChapters
                        IconButton(onClick = {
                            val ch = epub.chapters.getOrNull(currentChapter)
                            val title = ch?.title ?: "Chapter ${currentChapter + 1}"
                            val snippet = ch?.blocks?.firstOrNull()?.let { blockText(it).take(100) } ?: ""
                            onToggleBookmark(currentChapter, title, snippet)
                        }) {
                            Icon(
                                if (isBookmarked) Icons.Rounded.Bookmark else Icons.Rounded.BookmarkBorder,
                                contentDescription = "Markah",
                                tint = if (isBookmarked) palette.accent else palette.textPrimary,
                            )
                        }
                        IconButton(onClick = { showSearch = !showSearch }) {
                            Icon(
                                Icons.Rounded.Search,
                                contentDescription = "Cari dalam buku",
                                tint = if (showSearch) palette.accent else palette.textPrimary,
                            )
                        }
                        IconButton(onClick = { isPaged = !isPaged }) {
                            Icon(
                                if (isPaged) Icons.AutoMirrored.Rounded.MenuBook else Icons.Rounded.SwapVert,
                                contentDescription = "Mode baca",
                                tint = if (isPaged) palette.accent else palette.textPrimary,
                            )
                        }
                    }
                }

                AnimatedVisibility(visible = showSearch) {
                    InBookSearchBar(
                        query = searchQuery,
                        epub = epub,
                        palette = palette,
                        onQueryChange = { searchQuery = it },
                        onJumpChapter = { chIdx ->
                            currentChapter = chIdx
                            scope.launch {
                                chapterStartIndices[chIdx]?.let { scrollState.animateScrollToItem(it) }
                            }
                        },
                        onClose = {
                            showSearch = false
                            searchQuery = ""
                        },
                    )
                }
            }
        }

        // Bottom Scrubber + Floating AI Pill + Segmented 3-Icon Theme Switcher
        AnimatedVisibility(
            visible = !isFullscreen,
            enter = fadeIn() + slideInVertically { it / 2 },
            exit = fadeOut() + slideOutVertically { it / 2 },
            modifier = Modifier.align(Alignment.BottomCenter),
        ) {
            val totalCh = epub.chapters.size.coerceAtLeast(1)
            val progressFrac = ((currentChapter + 1).toFloat() / totalCh.toFloat()).coerceIn(0f, 1f)
            val pct = (progressFrac * 100f).roundToInt().coerceIn(0, 100)
            val minsLeft = (((1f - progressFrac) * 28f).roundToInt()).coerceAtLeast(1)

            Column(
                modifier = Modifier
                    .fillMaxWidth()
                    .background(palette.bg.copy(alpha = 0.94f))
                    .navigationBarsPadding()
                    .padding(horizontal = 20.dp, vertical = 12.dp),
                horizontalAlignment = Alignment.CenterHorizontally,
            ) {
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.End,
                ) {
                    Row(
                        modifier = Modifier
                            .shadow(4.dp, CircleShape, ambientColor = Color.Black.copy(alpha = 0.08f))
                            .clip(CircleShape)
                            .background(palette.surface)
                            .border(0.5.dp, palette.border, CircleShape)
                            .applePressScale()
                            .clickable { showAiModal = true }
                            .padding(horizontal = 14.dp, vertical = 7.dp),
                        verticalAlignment = Alignment.CenterVertically,
                        horizontalArrangement = Arrangement.spacedBy(5.dp),
                    ) {
                        Icon(
                            Icons.Rounded.AutoAwesome,
                            contentDescription = "AI Assistant",
                            tint = palette.accent,
                            modifier = Modifier.size(15.dp),
                        )
                        Text("AI Explain", fontSize = 12.sp, fontWeight = FontWeight.Medium, color = palette.textPrimary)
                    }
                }

                Spacer(Modifier.height(6.dp))

                Slider(
                    value = currentChapter.toFloat(),
                    onValueChange = { currentChapter = it.roundToInt().coerceIn(0, maxChapter) },
                    onValueChangeFinished = {
                        scope.launch {
                            chapterStartIndices[currentChapter]?.let { scrollState.scrollToItem(it) }
                        }
                        onProgressCommit(currentChapter, (currentChapter + 1).toFloat() / totalCh.toFloat())
                    },
                    valueRange = 0f..maxChapter.toFloat().coerceAtLeast(1f),
                    colors = SliderDefaults.colors(
                        thumbColor = palette.textPrimary,
                        activeTrackColor = palette.textPrimary,
                        inactiveTrackColor = palette.border,
                    ),
                    modifier = Modifier.fillMaxWidth().height(24.dp),
                )

                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceBetween,
                ) {
                    Text("$pct%", fontSize = 11.sp, color = palette.textSecondary)
                    Text("Chapter ${currentChapter + 1} of $totalCh", fontSize = 11.sp, color = palette.textSecondary)
                    Text(if (pct >= 100) "Done" else "$minsLeft min left", fontSize = 11.sp, color = palette.textSecondary)
                }

                Spacer(Modifier.height(10.dp))

                // Reference 3-icon segmented theme pill
                Row(
                    modifier = Modifier
                        .clip(CircleShape)
                        .background(palette.surfaceMuted)
                        .padding(4.dp),
                    horizontalArrangement = Arrangement.spacedBy(4.dp),
                    verticalAlignment = Alignment.CenterVertically,
                ) {
                    ThemeIconPill("light", Icons.Rounded.WbSunny, palette) {
                        onUpdateSettings(settings.copy(theme = "light"))
                    }
                    ThemeIconPill("warm", Icons.Rounded.DragHandle, palette) {
                        onUpdateSettings(settings.copy(theme = "warm"))
                    }
                    ThemeIconPill("amoled", Icons.Rounded.DarkMode, palette) {
                        onUpdateSettings(settings.copy(theme = "amoled"))
                    }
                }
            }
        }
    }

    if (showTypography) {
        ModalBottomSheet(
            onDismissRequest = { showTypography = false },
            containerColor = palette.surface,
        ) {
            TypographySheet(
                settings = settings,
                palette = palette,
                onUpdate = onUpdateSettings,
            )
        }
    }

    if (showToc) {
        ModalBottomSheet(
            onDismissRequest = { showToc = false },
            containerColor = palette.surface,
        ) {
            TocAndBookmarksSheet(
                epub = epub,
                currentChapter = currentChapter,
                bookmarks = bookmarks,
                palette = palette,
                onSelectChapter = { idx ->
                    showToc = false
                    currentChapter = idx
                    scope.launch {
                        chapterStartIndices[idx]?.let { scrollState.animateScrollToItem(it) }
                    }
                },
                onDeleteBookmark = onDeleteBookmark,
            )
        }
    }

    if (showAiModal) {
        val currentText = remember(currentChapter, epub) {
            epub.chapters.getOrNull(currentChapter)?.blocks?.take(4)?.joinToString("\n") { blockText(it) } ?: ""
        }
        AiExplanationSheet(
            chapterTitle = epub.chapters.getOrNull(currentChapter)?.title ?: "Chapter ${currentChapter + 1}",
            excerpt = currentText,
            palette = palette,
            onAskAi = onAskAi,
            onDismiss = { showAiModal = false },
        )
    }
}

@Composable
private fun ThemeIconPill(
    id: String,
    icon: androidx.compose.ui.graphics.vector.ImageVector,
    palette: BacaPalette,
    onClick: () -> Unit,
) {
    val active = palette.id == id || (id == "amoled" && palette.id == "slate")
    Box(
        modifier = Modifier
            .clip(CircleShape)
            .background(if (active) palette.surface else Color.Transparent)
            .clickable(onClick = onClick)
            .padding(horizontal = 18.dp, vertical = 7.dp),
        contentAlignment = Alignment.Center,
    ) {
        Icon(
            icon,
            contentDescription = id,
            tint = if (active) palette.textPrimary else palette.textSecondary,
            modifier = Modifier.size(18.dp),
        )
    }
}

@Composable
private fun TypographySheet(
    settings: NativeSettings,
    palette: BacaPalette,
    onUpdate: (NativeSettings) -> Unit,
) {
    Column(
        modifier = Modifier
            .fillMaxWidth()
            .navigationBarsPadding()
            .padding(horizontal = 24.dp, vertical = 12.dp),
        verticalArrangement = Arrangement.spacedBy(14.dp),
    ) {
        Text("Typography", fontFamily = FontFamily.Serif, fontSize = 20.sp, color = palette.textPrimary)

        Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
            listOf("serif" to "Serif", "sans" to "Sans", "mono" to "Mono").forEach { (id, label) ->
                val active = settings.fontFamily == id
                Box(
                    modifier = Modifier
                        .weight(1f)
                        .clip(RoundedCornerShape(12.dp))
                        .background(if (active) palette.textPrimary else palette.surfaceMuted)
                        .clickable { onUpdate(settings.copy(fontFamily = id)) }
                        .padding(vertical = 10.dp),
                    contentAlignment = Alignment.Center,
                ) {
                    Text(label, fontSize = 13.sp, color = if (active) palette.bg else palette.textPrimary)
                }
            }
        }

        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Text("Font Size (${settings.fontSize}sp)", fontSize = 14.sp, color = palette.textPrimary)
            Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                OutlinedButton(
                    onClick = {
                        val next = (settings.fontSize.toInt() - 1).coerceAtLeast(13).toUInt()
                        onUpdate(settings.copy(fontSize = next))
                    },
                    shape = CircleShape,
                ) { Text("A-") }
                OutlinedButton(
                    onClick = {
                        val next = (settings.fontSize.toInt() + 1).coerceAtMost(30).toUInt()
                        onUpdate(settings.copy(fontSize = next))
                    },
                    shape = CircleShape,
                ) { Text("A+") }
            }
        }

        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Text("Line Spacing (${String.format("%.1f", settings.lineHeight)})", fontSize = 14.sp, color = palette.textPrimary)
            Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                OutlinedButton(
                    onClick = {
                        val next = (settings.lineHeight - 0.1f).coerceAtLeast(1.2f)
                        onUpdate(settings.copy(lineHeight = next))
                    },
                    shape = CircleShape,
                ) { Text("-") }
                OutlinedButton(
                    onClick = {
                        val next = (settings.lineHeight + 0.1f).coerceAtMost(2.2f)
                        onUpdate(settings.copy(lineHeight = next))
                    },
                    shape = CircleShape,
                ) { Text("+") }
            }
        }

        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Text("Margins (${settings.marginHorizontal}dp)", fontSize = 14.sp, color = palette.textPrimary)
            Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                OutlinedButton(
                    onClick = {
                        val next = (settings.marginHorizontal.toInt() - 4).coerceAtLeast(12).toUInt()
                        onUpdate(settings.copy(marginHorizontal = next))
                    },
                    shape = CircleShape,
                ) { Text("-") }
                OutlinedButton(
                    onClick = {
                        val next = (settings.marginHorizontal.toInt() + 4).coerceAtMost(48).toUInt()
                        onUpdate(settings.copy(marginHorizontal = next))
                    },
                    shape = CircleShape,
                ) { Text("+") }
            }
        }
        Spacer(Modifier.height(12.dp))
    }
}

@Composable
private fun TocAndBookmarksSheet(
    epub: NativeEpubBook,
    currentChapter: Int,
    bookmarks: List<NativeBookmark>,
    palette: BacaPalette,
    onSelectChapter: (Int) -> Unit,
    onDeleteBookmark: (String) -> Unit,
) {
    var tab by remember { mutableIntStateOf(0) }
    Column(
        modifier = Modifier
            .fillMaxWidth()
            .navigationBarsPadding()
            .padding(horizontal = 24.dp, vertical = 12.dp),
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .clip(CircleShape)
                .background(palette.surfaceMuted)
                .padding(4.dp),
        ) {
            listOf("Chapters (${epub.chapters.size})", "Bookmarks (${bookmarks.size})").forEachIndexed { idx, label ->
                val active = tab == idx
                Box(
                    modifier = Modifier
                        .weight(1f)
                        .clip(CircleShape)
                        .background(if (active) palette.surface else Color.Transparent)
                        .clickable { tab = idx }
                        .padding(vertical = 8.dp),
                    contentAlignment = Alignment.Center,
                ) {
                    Text(
                        label,
                        fontSize = 13.sp,
                        fontWeight = if (active) FontWeight.SemiBold else FontWeight.Normal,
                        color = palette.textPrimary,
                    )
                }
            }
        }

        Spacer(Modifier.height(12.dp))

        LazyColumn(modifier = Modifier.heightIn(max = 360.dp)) {
            if (tab == 0) {
                itemsIndexed(epub.chapters) { idx, ch ->
                    Row(
                        modifier = Modifier
                            .fillMaxWidth()
                            .clickable { onSelectChapter(idx) }
                            .padding(vertical = 12.dp),
                        verticalAlignment = Alignment.CenterVertically,
                    ) {
                        Text(
                            "${idx + 1}. ${ch.title}",
                            fontWeight = if (idx == currentChapter) FontWeight.Bold else FontWeight.Normal,
                            color = if (idx == currentChapter) palette.accent else palette.textPrimary,
                            modifier = Modifier.weight(1f),
                        )
                    }
                }
            } else if (bookmarks.isEmpty()) {
                item {
                    Text(
                        "Belum ada markah untuk buku ini.",
                        color = palette.textSecondary,
                        fontSize = 13.sp,
                        modifier = Modifier.padding(vertical = 24.dp),
                    )
                }
            } else {
                itemsIndexed(bookmarks) { _, bm ->
                    Row(
                        modifier = Modifier
                            .fillMaxWidth()
                            .clickable { onSelectChapter(bm.chapterIndex.toInt()) }
                            .padding(vertical = 10.dp),
                        verticalAlignment = Alignment.CenterVertically,
                    ) {
                        Column(modifier = Modifier.weight(1f)) {
                            Text(bm.chapterTitle, fontWeight = FontWeight.Medium, color = palette.textPrimary)
                            if (bm.textSnippet.isNotBlank()) {
                                Text(
                                    bm.textSnippet,
                                    fontSize = 12.sp,
                                    color = palette.textSecondary,
                                    maxLines = 2,
                                    overflow = TextOverflow.Ellipsis,
                                )
                            }
                        }
                        IconButton(onClick = { onDeleteBookmark(bm.id) }) {
                            Icon(Icons.Rounded.DeleteOutline, contentDescription = "Hapus markah", tint = palette.textSecondary)
                        }
                    }
                }
            }
        }
    }
}

@Composable
private fun InBookSearchBar(
    query: String,
    epub: NativeEpubBook,
    palette: BacaPalette,
    onQueryChange: (String) -> Unit,
    onJumpChapter: (Int) -> Unit,
    onClose: () -> Unit,
) {
    val hits = remember(query, epub) {
        if (query.trim().length < 2) emptyList()
        else {
            val q = query.trim()
            buildList {
                epub.chapters.forEachIndexed { idx, ch ->
                    for (b in ch.blocks) {
                        val txt = blockText(b)
                        if (txt.contains(q, ignoreCase = true)) {
                            add(SearchHit(idx, ch.title, txt.take(90)))
                            break
                        }
                    }
                }
            }
        }
    }

    Column(modifier = Modifier.fillMaxWidth().padding(top = 8.dp)) {
        OutlinedTextField(
            value = query,
            onValueChange = onQueryChange,
            placeholder = { Text("Search in book...", fontSize = 13.sp) },
            trailingIcon = {
                IconButton(onClick = onClose) {
                    Icon(Icons.Rounded.Close, contentDescription = "Tutup")
                }
            },
            singleLine = true,
            shape = RoundedCornerShape(14.dp),
            modifier = Modifier.fillMaxWidth(),
        )
        if (hits.isNotEmpty()) {
            LazyColumn(
                modifier = Modifier
                    .fillMaxWidth()
                    .heightIn(max = 160.dp)
                    .padding(top = 6.dp)
                    .clip(RoundedCornerShape(12.dp))
                    .background(palette.surface)
                    .border(0.5.dp, palette.border, RoundedCornerShape(12.dp)),
            ) {
                itemsIndexed(hits) { _, hit ->
                    Column(
                        modifier = Modifier
                            .fillMaxWidth()
                            .clickable { onJumpChapter(hit.chapterIndex) }
                            .padding(horizontal = 12.dp, vertical = 8.dp),
                    ) {
                        Text(hit.chapterTitle, fontSize = 12.sp, fontWeight = FontWeight.SemiBold, color = palette.accent)
                        Text(hit.snippet, fontSize = 12.sp, color = palette.textSecondary, maxLines = 1, overflow = TextOverflow.Ellipsis)
                    }
                }
            }
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun AiExplanationSheet(
    chapterTitle: String,
    excerpt: String,
    palette: BacaPalette,
    onAskAi: suspend (String) -> Result<String>,
    onDismiss: () -> Unit,
) {
    var question by remember { mutableStateOf("Ringkas dan jelaskan makna bagian ini: $chapterTitle") }
    var answer by remember { mutableStateOf<String?>(null) }
    var loading by remember { mutableStateOf(false) }
    val scope = rememberCoroutineScope()

    ModalBottomSheet(onDismissRequest = onDismiss, containerColor = palette.surface) {
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .navigationBarsPadding()
                .padding(horizontal = 24.dp, vertical = 12.dp),
            verticalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            Text("Gemini AI Assistant", fontFamily = FontFamily.Serif, fontSize = 20.sp, color = palette.textPrimary)
            OutlinedTextField(
                value = question,
                onValueChange = { question = it },
                label = { Text("Pertanyaan / Topik") },
                maxLines = 3,
                shape = RoundedCornerShape(14.dp),
                modifier = Modifier.fillMaxWidth(),
            )
            Button(
                enabled = !loading && question.isNotBlank(),
                onClick = {
                    loading = true
                    answer = null
                    val prompt = "${question.trim()}\n\nKonteks ($chapterTitle):\n${excerpt.take(1200)}"
                    scope.launch {
                        onAskAi(prompt)
                            .onSuccess { answer = it }
                            .onFailure { answer = "Error: ${it.message}" }
                        loading = false
                    }
                },
                shape = CircleShape,
                colors = ButtonDefaults.buttonColors(containerColor = palette.textPrimary, contentColor = palette.bg),
            ) {
                Text(if (loading) "Berpikir..." else "Tanya Gemini")
            }

            answer?.let {
                Text(it, fontSize = 13.sp, lineHeight = 20.sp, color = palette.textPrimary)
            }
            Spacer(Modifier.height(12.dp))
        }
    }
}

@Composable
private fun PagedNativeContent(
    epub: NativeEpubBook,
    currentChapter: Int,
    textStyle: TextStyle,
    palette: BacaPalette,
    highlightQuery: String,
    marginDp: Int,
    onPageSettled: (Int, Float) -> Unit,
) {
    val measurer = rememberTextMeasurer()
    BoxWithConstraints(
        modifier = Modifier
            .fillMaxSize()
            .padding(horizontal = marginDp.dp, vertical = 104.dp),
    ) {
        val maxWidthPx = constraints.maxWidth.coerceAtLeast(1)
        val maxHeightPx = constraints.maxHeight.coerceAtLeast(1)

        val pages = remember(epub, textStyle, maxWidthPx, maxHeightPx) {
            val out = mutableListOf<PagedItem>()
            epub.chapters.forEachIndexed { chIdx, ch ->
                val currentBlocks = mutableListOf<ReaderBlock>()
                var usedHeight = 0
                ch.blocks.forEach { block ->
                    val text = blockText(block)
                    val measured = measurer.measure(
                        text = text,
                        style = textStyle,
                        constraints = Constraints(maxWidth = maxWidthPx),
                    )
                    val blockH = measured.size.height + 28
                    if (usedHeight + blockH > maxHeightPx && currentBlocks.isNotEmpty()) {
                        out.add(PagedItem(chIdx, currentBlocks.toList()))
                        currentBlocks.clear()
                        usedHeight = 0
                    }
                    currentBlocks.add(block)
                    usedHeight += blockH
                }
                if (currentBlocks.isNotEmpty()) {
                    out.add(PagedItem(chIdx, currentBlocks.toList()))
                }
            }
            if (out.isEmpty()) listOf(PagedItem(0, emptyList())) else out
        }

        val initialPage = remember(currentChapter, pages) {
            pages.indexOfFirst { it.chapterIndex == currentChapter }.coerceAtLeast(0)
        }
        val pagerState = rememberPagerState(initialPage = initialPage, pageCount = { pages.size })

        LaunchedEffect(pagerState.settledPage, pages) {
            val item = pages.getOrNull(pagerState.settledPage) ?: return@LaunchedEffect
            val frac = (pagerState.settledPage + 1).toFloat() / pages.size.coerceAtLeast(1).toFloat()
            onPageSettled(item.chapterIndex, frac)
        }

        HorizontalPager(state = pagerState, modifier = Modifier.fillMaxSize()) { pageIdx ->
            val page = pages[pageIdx]
            Column(
                verticalArrangement = Arrangement.spacedBy(14.dp),
                modifier = Modifier.fillMaxSize(),
            ) {
                page.blocks.forEach { block -> RenderBlock(block, textStyle, palette, highlightQuery) }
            }
        }
    }
}

private fun blockText(block: ReaderBlock): String = when (block) {
    is ReaderBlock.Heading -> block.text
    is ReaderBlock.Paragraph -> block.text
    is ReaderBlock.Blockquote -> block.text
    ReaderBlock.Divider -> "—"
}

@Composable
private fun RenderBlock(
    block: ReaderBlock,
    textStyle: TextStyle,
    palette: BacaPalette,
    highlightQuery: String = "",
) {
    val q = highlightQuery.trim()
    fun highlighted(raw: String) = buildAnnotatedString {
        append(raw)
        if (q.length >= 2) {
            var start = 0
            while (true) {
                val idx = raw.indexOf(q, startIndex = start, ignoreCase = true)
                if (idx < 0) break
                addStyle(
                    SpanStyle(
                        background = palette.accent.copy(alpha = 0.28f),
                        fontWeight = FontWeight.SemiBold,
                    ),
                    idx,
                    idx + q.length,
                )
                start = idx + q.length
            }
        }
    }

    when (block) {
        is ReaderBlock.Heading -> Text(
            text = highlighted(block.text),
            style = textStyle.copy(
                fontSize = (textStyle.fontSize.value + 6f).sp,
                fontWeight = FontWeight.SemiBold,
            ),
            modifier = Modifier.padding(top = 8.dp),
        )
        is ReaderBlock.Paragraph -> Text(text = highlighted(block.text), style = textStyle)
        is ReaderBlock.Blockquote -> Text(
            text = highlighted(block.text),
            style = textStyle.copy(fontStyle = FontStyle.Italic, color = palette.textSecondary),
            modifier = Modifier
                .padding(start = 12.dp)
                .border(2.dp, palette.accent, RoundedCornerShape(2.dp))
                .padding(start = 12.dp),
        )
        ReaderBlock.Divider -> HorizontalDivider(color = palette.border, modifier = Modifier.padding(vertical = 8.dp))
    }
}
