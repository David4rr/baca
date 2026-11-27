package com.baca.nativeapp.ui

import androidx.compose.animation.core.animateFloatAsState
import androidx.compose.animation.core.spring
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import uniffi.baca_core.NativeBook
import uniffi.baca_core.NativeDailyStat

@Composable
fun StatsScreen(
    palette: BacaPalette,
    books: List<NativeBook>,
    stats: List<NativeDailyStat>,
    totalMinutes: Int,
) {
    val finishedCount = remember(books) { books.count { it.progressPercent >= 0.99f } }
    val activeDays = remember(stats) { stats.count { it.minutes > 0u } }
    val maxMinutes = remember(stats) { (stats.maxOfOrNull { it.minutes.toInt() } ?: 1).coerceAtLeast(1) }

    LazyColumn(
        contentPadding = PaddingValues(
            start = 20.dp,
            end = 20.dp,
            top = 12.dp,
            bottom = 112.dp,
        ),
        verticalArrangement = Arrangement.spacedBy(16.dp),
        modifier = Modifier
            .fillMaxSize()
            .background(palette.bg)
            .statusBarsPadding(),
    ) {
        item {
            Column {
                Text(
                    text = "Reading Stats",
                    fontFamily = FontFamily.Serif,
                    fontSize = 31.sp,
                    color = palette.textPrimary,
                )
                Text(
                    text = "Your 7-day reading rhythm",
                    fontSize = 14.sp,
                    color = palette.textSecondary,
                )
            }
        }

        item {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(12.dp),
            ) {
                StatMetricCard(
                    label = "Total Time",
                    value = "${totalMinutes}m",
                    palette = palette,
                    modifier = Modifier.weight(1f),
                )
                StatMetricCard(
                    label = "Books",
                    value = "${books.size}",
                    palette = palette,
                    modifier = Modifier.weight(1f),
                )
                StatMetricCard(
                    label = "Finished",
                    value = "$finishedCount",
                    palette = palette,
                    modifier = Modifier.weight(1f),
                )
            }
        }

        item {
            val shape = RoundedCornerShape(22.dp)
            Column(
                modifier = Modifier
                    .fillMaxWidth()
                    .shadow(6.dp, shape, ambientColor = Color.Black.copy(alpha = 0.06f))
                    .clip(shape)
                    .background(palette.surface)
                    .border(0.5.dp, palette.border, shape)
                    .padding(20.dp),
            ) {
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceBetween,
                    verticalAlignment = Alignment.CenterVertically,
                ) {
                    Text(
                        text = "Last 7 Days",
                        fontFamily = FontFamily.Serif,
                        fontSize = 18.sp,
                        color = palette.textPrimary,
                    )
                    Text(
                        text = "$activeDays active days",
                        fontSize = 12.sp,
                        color = palette.textSecondary,
                    )
                }
                Spacer(Modifier.height(20.dp))

                Row(
                    modifier = Modifier
                        .fillMaxWidth()
                        .height(150.dp),
                    horizontalArrangement = Arrangement.SpaceEvenly,
                    verticalAlignment = Alignment.Bottom,
                ) {
                    stats.forEach { day ->
                        val ratio = (day.minutes.toFloat() / maxMinutes.toFloat()).coerceIn(0.06f, 1f)
                        val animatedRatio by animateFloatAsState(
                            targetValue = ratio,
                            animationSpec = spring(dampingRatio = 0.72f, stiffness = 280f),
                            label = "bar_${day.dateStr}",
                        )
                        Column(
                            horizontalAlignment = Alignment.CenterHorizontally,
                            verticalArrangement = Arrangement.Bottom,
                            modifier = Modifier.weight(1f).fillMaxHeight(),
                        ) {
                            Text(
                                text = "${day.minutes}m",
                                fontSize = 10.sp,
                                color = palette.textSecondary,
                            )
                            Spacer(Modifier.height(6.dp))
                            Box(
                                modifier = Modifier
                                    .width(22.dp)
                                    .weight(1f, fill = false)
                                    .fillMaxHeight(animatedRatio * 0.78f)
                                    .clip(CircleShape)
                                    .background(if (day.minutes > 0u) palette.accent else palette.surfaceMuted),
                            )
                            Spacer(Modifier.height(8.dp))
                            Text(
                                text = day.dayLabel.take(3),
                                fontSize = 11.sp,
                                fontWeight = FontWeight.Medium,
                                color = palette.textSecondary,
                            )
                        }
                    }
                }
            }
        }
    }
}

@Composable
private fun StatMetricCard(
    label: String,
    value: String,
    palette: BacaPalette,
    modifier: Modifier = Modifier,
) {
    val shape = RoundedCornerShape(18.dp)
    Column(
        modifier = modifier
            .shadow(4.dp, shape, ambientColor = Color.Black.copy(alpha = 0.05f))
            .clip(shape)
            .background(palette.surface)
            .border(0.5.dp, palette.border, shape)
            .padding(14.dp),
    ) {
        Text(label, fontSize = 11.sp, color = palette.textSecondary)
        Spacer(Modifier.height(4.dp))
        Text(
            text = value,
            fontFamily = FontFamily.Serif,
            fontSize = 24.sp,
            fontWeight = FontWeight.Medium,
            color = palette.textPrimary,
        )
    }
}
