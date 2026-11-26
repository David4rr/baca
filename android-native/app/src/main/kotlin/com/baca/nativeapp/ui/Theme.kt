package com.baca.nativeapp.ui

import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontFamily

data class BacaPalette(
    val id: String,
    val bg: Color,
    val surface: Color,
    val textPrimary: Color,
    val textSecondary: Color,
    val border: Color,
    val accent: Color,
    val isDark: Boolean,
)

object BacaPalettes {
    val Warm = BacaPalette(
        id = "warm",
        bg = Color(0xFFF9F6F0),
        surface = Color(0xFFFFFFFF),
        textPrimary = Color(0xFF1C1917),
        textSecondary = Color(0xFF57534E),
        border = Color(0xFFE7E2D8),
        accent = Color(0xFFD97706),
        isDark = false,
    )

    val Light = BacaPalette(
        id = "light",
        bg = Color(0xFFFFFFFF),
        surface = Color(0xFFF5F5F4),
        textPrimary = Color(0xFF09090B),
        textSecondary = Color(0xFF52525B),
        border = Color(0xFFE4E4E7),
        accent = Color(0xFFD97706),
        isDark = false,
    )

    val Amoled = BacaPalette(
        id = "amoled",
        bg = Color(0xFF000000),
        surface = Color(0xFF121212),
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
