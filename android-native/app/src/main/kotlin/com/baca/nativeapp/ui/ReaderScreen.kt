package com.baca.nativeapp.ui

import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
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
import androidx.compose.material.icons.automirrored.rounded.ArrowBack
import androidx.compose.material.icons.automirrored.rounded.MenuBook
import androidx.compose.material.icons.rounded.Bookmark
import androidx.compose.material.icons.rounded.BookmarkBorder
import androidx.compose.material.icons.rounded.FormatListBulleted
import androidx.compose.material.icons.rounded.SwapVert
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.FontStyle
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.rememberTextMeasurer
import androidx.compose.ui.text.style.TextAlign
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

@Composable
fun ReaderScreen(
    book: NativeBook,
    epub: NativeEpubBook,
    palette: BacaPalette,
    settings: NativeSettings,
    bookmarkedChapters: Set<Int>,
    onToggleBookmark: (Int, String) -> Unit,
    onProgressCommit: (Int, Float) -> Unit,
    onChangeTheme: (String) -> Unit,
    onBack: () -> Unit,
) {
    var isPaged by remember { mutableStateOf(false) }
    var isFullscreen by remember { mutableStateOf(false) }
    var showToc by remember { mutableStateOf(false) }
    var currentChapter by remember(book.id) { mutableIntStateOf(book.currentChapter.toInt().coerceIn(0, (epub.chapters.size - 1).coerceAtLeast(0))) }
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
                    val frac = idx.toFloat() / scrollRows.size.coerceAtLeast(1).toFloat()
                    onProgressCommit(chIdx, frac)
                }
            }
    }

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
                    bottom = 140.dp,
                ),
                verticalArrangement = Arrangement.spacedBy(14.dp),
                modifier = Modifier.fillMaxSize(),
            ) {
                itemsIndexed(scrollRows) { _, row ->
                    when (row) {
                        is ScrollRow.ChapterHeader -> Text(
                            row.title,
                            fontFamily = fontFamily,
                            fontSize = 26.sp,
                            lineHeight = 32.sp,
                            color = palette.textPrimary,
                            textAlign = TextAlign.Center,
                            modifier = Modifier.fillMaxWidth().padding(top = 24.dp, bottom = 12.dp),
                        )
                        is ScrollRow.BlockRow -> RenderBlock(row.block, textStyle, palette)
                    }
                }
            }
        } else {
            PagedNativeContent(
                epub = epub,
                currentChapter = currentChapter,
                textStyle = textStyle,
                palette = palette,
                marginDp = settings.marginHorizontal.toInt(),
                onPageSettled = { ch, frac ->
                    currentChapter = ch
                    onProgressCommit(ch, frac)
                },
            )
        }

        AnimatedVisibility(
            visible = !isFullscreen,
            enter = fadeIn(),
            exit = fadeOut(),
            modifier = Modifier.align(Alignment.TopCenter),
        ) {
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .background(palette.bg.copy(alpha = 0.92f))
                    .statusBarsPadding()
                    .padding(horizontal = 14.dp, vertical = 10.dp),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically,
            ) {
                IconButton(onClick = onBack) {
                    Icon(Icons.AutoMirrored.Rounded.ArrowBack, contentDescription = "Kembali", tint = palette.textPrimary)
                }
                Row(horizontalArrangement = Arrangement.spacedBy(4.dp)) {
                    IconButton(onClick = { isPaged = !isPaged }) {
                        Icon(
                            if (isPaged) Icons.AutoMirrored.Rounded.MenuBook else Icons.Rounded.SwapVert,
                            contentDescription = "Mode baca",
                            tint = if (isPaged) palette.accent else palette.textPrimary,
                        )
                    }
                    val isBookmarked = currentChapter in bookmarkedChapters
                    IconButton(onClick = {
                        val title = epub.chapters.getOrNull(currentChapter)?.title ?: "Bab ${currentChapter + 1}"
                        onToggleBookmark(currentChapter, title)
                    }) {
                        Icon(
                            if (isBookmarked) Icons.Rounded.Bookmark else Icons.Rounded.BookmarkBorder,
                            contentDescription = "Markah",
                            tint = if (isBookmarked) palette.accent else palette.textPrimary,
                        )
                    }
                    IconButton(onClick = { showToc = true }) {
                        Icon(Icons.Rounded.FormatListBulleted, contentDescription = "Daftar isi", tint = palette.textPrimary)
                    }
                }
            }
        }

        AnimatedVisibility(
            visible = !isFullscreen,
            enter = fadeIn(),
            exit = fadeOut(),
            modifier = Modifier.align(Alignment.BottomCenter),
        ) {
            val totalCh = epub.chapters.size.coerceAtLeast(1)
            val pct = (((currentChapter + 1).toFloat() / totalCh) * 100f).roundToInt().coerceIn(0, 100)
            Column(
                modifier = Modifier
                    .fillMaxWidth()
                    .navigationBarsPadding()
                    .padding(horizontal = 20.dp, vertical = 14.dp),
                horizontalAlignment = Alignment.CenterHorizontally,
            ) {
                Row(
                    modifier = Modifier
                        .clip(CircleShape)
                        .background(palette.surface.copy(alpha = 0.95f))
                        .border(1.dp, palette.border, CircleShape)
                        .padding(horizontal = 16.dp, vertical = 8.dp),
                    horizontalArrangement = Arrangement.spacedBy(12.dp),
                    verticalAlignment = Alignment.CenterVertically,
                ) {
                    Text("$pct%", fontSize = 12.sp, fontWeight = FontWeight.SemiBold, color = palette.textPrimary)
                    listOf("light" to "Terang", "warm" to "Hangat", "amoled" to "AMOLED", "slate" to "Slate").forEach { (id, label) ->
                        Text(
                            label,
                            fontSize = 11.sp,
                            fontWeight = if (palette.id == id) FontWeight.Bold else FontWeight.Normal,
                            color = if (palette.id == id) palette.accent else palette.textSecondary,
                            modifier = Modifier.clickable { onChangeTheme(id) },
                        )
                    }
                }
            }
        }
    }

    if (showToc) {
        AlertDialog(
            onDismissRequest = { showToc = false },
            title = { Text("Daftar Isi") },
            text = {
                LazyColumn(modifier = Modifier.heightIn(max = 360.dp)) {
                    itemsIndexed(epub.chapters) { idx, ch ->
                        Text(
                            ch.title,
                            fontWeight = if (idx == currentChapter) FontWeight.Bold else FontWeight.Normal,
                            color = if (idx == currentChapter) palette.accent else palette.textPrimary,
                            modifier = Modifier
                                .fillMaxWidth()
                                .clickable {
                                    showToc = false
                                    currentChapter = idx
                                    scope.launch {
                                        chapterStartIndices[idx]?.let { scrollState.scrollToItem(it) }
                                    }
                                }
                                .padding(vertical = 10.dp),
                        )
                    }
                }
            },
            confirmButton = { TextButton(onClick = { showToc = false }) { Text("Tutup") } },
        )
    }
}

@Composable
private fun PagedNativeContent(
    epub: NativeEpubBook,
    currentChapter: Int,
    textStyle: TextStyle,
    palette: BacaPalette,
    marginDp: Int,
    onPageSettled: (Int, Float) -> Unit,
) {
    val measurer = rememberTextMeasurer()
    BoxWithConstraints(
        modifier = Modifier
            .fillMaxSize()
            .padding(horizontal = marginDp.dp, vertical = 88.dp),
    ) {
        val maxWidthPx = constraints.maxWidth.coerceAtLeast(1)
        val maxHeightPx = constraints.maxHeight.coerceAtLeast(1)

        val pages = remember(epub, textStyle, maxWidthPx, maxHeightPx) {
            val out = mutableListOf<PagedItem>()
            epub.chapters.forEachIndexed { chIdx, ch ->
                val currentBlocks = mutableListOf<ReaderBlock>()
                var usedHeight = 0
                ch.blocks.forEach { block ->
                    val text = when (block) {
                        is ReaderBlock.Heading -> block.text
                        is ReaderBlock.Paragraph -> block.text
                        is ReaderBlock.Blockquote -> block.text
                        ReaderBlock.Divider -> "—"
                    }
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
                page.blocks.forEach { block -> RenderBlock(block, textStyle, palette) }
            }
        }
    }
}

@Composable
private fun RenderBlock(block: ReaderBlock, textStyle: TextStyle, palette: BacaPalette) {
    when (block) {
        is ReaderBlock.Heading -> Text(
            block.text,
            style = textStyle.copy(
                fontSize = (textStyle.fontSize.value + 6f).sp,
                fontWeight = FontWeight.SemiBold,
            ),
            modifier = Modifier.padding(top = 8.dp),
        )
        is ReaderBlock.Paragraph -> Text(block.text, style = textStyle)
        is ReaderBlock.Blockquote -> Text(
            block.text,
            style = textStyle.copy(fontStyle = FontStyle.Italic, color = palette.textSecondary),
            modifier = Modifier
                .padding(start = 12.dp)
                .border(2.dp, palette.accent, RoundedCornerShape(2.dp))
                .padding(start = 12.dp),
        )
        ReaderBlock.Divider -> HorizontalDivider(color = palette.border, modifier = Modifier.padding(vertical = 8.dp))
    }
}
