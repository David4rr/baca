package com.baca.nativeapp.ui

import android.graphics.Bitmap
import android.graphics.BitmapFactory
import android.util.Base64
import androidx.compose.animation.core.Spring
import androidx.compose.animation.core.animateFloatAsState
import androidx.compose.animation.core.spring
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.composed
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.graphics.asImageBitmap
import androidx.compose.ui.graphics.graphicsLayer
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext

data class BacaPalette(
    val id: String,
    val bg: Color,
    val surface: Color,
    val surfaceMuted: Color,
    val textPrimary: Color,
    val textSecondary: Color,
    val border: Color,
    val accent: Color,
    val isDark: Boolean,
)

object BacaPalettes {
    val Warm = BacaPalette(
        id = "warm",
        bg = Color(0xFFF7F3EC),
        surface = Color(0xFFFFFFFF),
        surfaceMuted = Color(0xFFEDE7DC),
        textPrimary = Color(0xFF1C1917),
        textSecondary = Color(0xFF6B645C),
        border = Color(0xFFE6DFD3),
        accent = Color(0xFFC25E2E),
        isDark = false,
    )

    val Light = BacaPalette(
        id = "light",
        bg = Color(0xFFFFFFFF),
        surface = Color(0xFFF8F8F7),
        surfaceMuted = Color(0xFFEEEEEC),
        textPrimary = Color(0xFF09090B),
        textSecondary = Color(0xFF52525B),
        border = Color(0xFFE4E4E7),
        accent = Color(0xFF2563EB),
        isDark = false,
    )

    val Amoled = BacaPalette(
        id = "amoled",
        bg = Color(0xFF000000),
        surface = Color(0xFF121212),
        surfaceMuted = Color(0xFF1E1E1E),
        textPrimary = Color(0xFFF4F4F5),
        textSecondary = Color(0xFFA1A1AA),
        border = Color(0xFF27272A),
        accent = Color(0xFFF59E0B),
        isDark = true,
    )

    val Slate = BacaPalette(
        id = "slate",
        bg = Color(0xFF0F172A),
        surface = Color(0xFF1E293B),
        surfaceMuted = Color(0xFF283548),
        textPrimary = Color(0xFFF1F5F9),
        textSecondary = Color(0xFF94A3B8),
        border = Color(0xFF334155),
        accent = Color(0xFF38BDF8),
        isDark = true,
    )

    fun fromName(name: String): BacaPalette = when (name.lowercase()) {
        "light" -> Light
        "amoled" -> Amoled
        "slate" -> Slate
        else -> Warm
    }

    fun fontFamily(name: String): FontFamily = when (name.lowercase()) {
        "sans" -> FontFamily.SansSerif
        "mono" -> FontFamily.Monospace
        else -> FontFamily.Serif
    }
}

fun Modifier.applePressScale(): Modifier = composed {
    var pressed by remember { mutableStateOf(false) }
    val scale by animateFloatAsState(
        targetValue = if (pressed) 0.96f else 1f,
        animationSpec = spring(
            dampingRatio = Spring.DampingRatioMediumBouncy,
            stiffness = Spring.StiffnessMedium,
        ),
        label = "pressScale",
    )
    this
        .graphicsLayer {
            scaleX = scale
            scaleY = scale
        }
        .pointerInput(Unit) {
            detectTapGestures(
                onPress = {
                    pressed = true
                    tryAwaitRelease()
                    pressed = false
                }
            )
        }
}

@Composable
fun BookCoverArt(
    coverDataUri: String?,
    title: String,
    author: String,
    palette: BacaPalette,
    modifier: Modifier = Modifier,
) {
    var bitmap by remember(coverDataUri) { mutableStateOf<ImageBitmap?>(null) }

    LaunchedEffect(coverDataUri) {
        if (coverDataUri.isNullOrBlank()) {
            bitmap = null
            return@LaunchedEffect
        }
        bitmap = withContext(Dispatchers.IO) {
            decodeDataUriBitmap(coverDataUri)?.asImageBitmap()
        }
    }

    val shape = RoundedCornerShape(10.dp)
    Box(
        modifier = modifier
            .shadow(6.dp, shape, ambientColor = Color.Black.copy(alpha = 0.16f))
            .clip(shape)
            .background(palette.surfaceMuted)
            .border(0.5.dp, palette.border, shape),
    ) {
        if (bitmap != null) {
            Image(
                bitmap = bitmap!!,
                contentDescription = title,
                contentScale = ContentScale.Crop,
                modifier = Modifier.fillMaxSize(),
            )
        } else {
            val spineBg = remember(title) {
                val hues = listOf(
                    Color(0xFFE7DFD1),
                    Color(0xFF1F2937),
                    Color(0xFF3E4C3D),
                    Color(0xFF4A3728),
                    Color(0xFF2B3A4A),
                )
                hues[(title.hashCode().ushr(1)) % hues.size]
            }
            val isDarkCover = spineBg != Color(0xFFE7DFD1)
            val fg = if (isDarkCover) Color(0xFFF7F3EC) else Color(0xFF1C1917)
            Box(
                modifier = Modifier
                    .fillMaxSize()
                    .background(spineBg)
                    .padding(10.dp),
            ) {
                Column(
                    modifier = Modifier.fillMaxSize(),
                    verticalArrangement = Arrangement.SpaceBetween,
                    horizontalAlignment = Alignment.CenterHorizontally,
                ) {
                    Text(
                        text = title,
                        fontFamily = FontFamily.Serif,
                        fontSize = 12.sp,
                        lineHeight = 15.sp,
                        fontWeight = FontWeight.SemiBold,
                        color = fg,
                        maxLines = 4,
                        overflow = TextOverflow.Ellipsis,
                        textAlign = TextAlign.Center,
                    )
                    Text(
                        text = author,
                        fontSize = 9.sp,
                        color = fg.copy(alpha = 0.72f),
                        maxLines = 1,
                        overflow = TextOverflow.Ellipsis,
                        textAlign = TextAlign.Center,
                    )
                }
            }
        }
        // Subtle left 3D book spine ridge matching reference
        Box(
            modifier = Modifier
                .fillMaxHeight()
                .width(6.dp)
                .background(
                    Brush.horizontalGradient(
                        listOf(
                            Color.Black.copy(alpha = 0.16f),
                            Color.White.copy(alpha = 0.12f),
                            Color.Transparent,
                        )
                    )
                ),
        )
    }
}

private fun decodeDataUriBitmap(uri: String): Bitmap? {
    return try {
        val base64Part = uri.substringAfter("base64,", "")
        if (base64Part.isEmpty()) return null
        val bytes = Base64.decode(base64Part, Base64.DEFAULT)
        BitmapFactory.decodeByteArray(bytes, 0, bytes.size)
    } catch (_: Exception) {
        null
    }
}
