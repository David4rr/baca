package com.baca.nativeapp.ui

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import kotlinx.coroutines.launch
import uniffi.baca_core.NativeReleaseInfo
import uniffi.baca_core.NativeSettings

@Composable
fun SettingsScreen(
    palette: BacaPalette,
    settings: NativeSettings,
    appVersion: String,
    onSaveSettings: (NativeSettings) -> Unit,
    onExportBackup: () -> Unit,
    onImportBackup: () -> Unit,
    onCheckUpdate: suspend () -> Result<NativeReleaseInfo?>,
    onOpenUrl: (String) -> Unit,
) {
    val scope = rememberCoroutineScope()
    var apiKey by remember(settings.geminiApiKey) { mutableStateOf(settings.geminiApiKey ?: "") }
    var model by remember(settings.geminiModel) { mutableStateOf(settings.geminiModel ?: "gemini-2.5-flash") }
    var checkingUpdate by remember { mutableStateOf(false) }
    var releaseInfo by remember { mutableStateOf<NativeReleaseInfo?>(null) }
    var updateStatus by remember { mutableStateOf<String?>(null) }

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
                    text = "Settings",
                    fontFamily = FontFamily.Serif,
                    fontSize = 31.sp,
                    color = palette.textPrimary,
                )
                Text(
                    text = "Reading theme, Gemini AI, backup & updates",
                    fontSize = 14.sp,
                    color = palette.textSecondary,
                )
            }
        }

        item {
            SettingsCard(title = "Reading Theme", palette = palette) {
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.spacedBy(8.dp),
                ) {
                    listOf("warm" to "Warm", "light" to "Light", "amoled" to "AMOLED", "slate" to "Slate").forEach { (id, label) ->
                        val active = settings.theme == id
                        Box(
                            modifier = Modifier
                                .weight(1f)
                                .clip(RoundedCornerShape(12.dp))
                                .background(if (active) palette.textPrimary else palette.surfaceMuted)
                                .applePressScale()
                                .clickable { onSaveSettings(settings.copy(theme = id)) }
                                .padding(vertical = 10.dp),
                            contentAlignment = Alignment.Center,
                        ) {
                            Text(
                                text = label,
                                fontSize = 12.sp,
                                fontWeight = if (active) FontWeight.SemiBold else FontWeight.Normal,
                                color = if (active) palette.bg else palette.textPrimary,
                            )
                        }
                    }
                }
            }
        }

        item {
            SettingsCard(title = "Gemini AI Assistant", palette = palette) {
                OutlinedTextField(
                    value = apiKey,
                    onValueChange = { apiKey = it },
                    label = { Text("Gemini API Key") },
                    visualTransformation = PasswordVisualTransformation(),
                    singleLine = true,
                    shape = RoundedCornerShape(14.dp),
                    modifier = Modifier.fillMaxWidth(),
                )
                Spacer(Modifier.height(10.dp))
                Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                    listOf("gemini-2.5-flash", "gemini-2.5-pro").forEach { m ->
                        val active = model == m
                        Box(
                            modifier = Modifier
                                .clip(CircleShape)
                                .background(if (active) palette.textPrimary else palette.surfaceMuted)
                                .clickable { model = m }
                                .padding(horizontal = 14.dp, vertical = 7.dp),
                        ) {
                            Text(
                                text = m,
                                fontSize = 12.sp,
                                color = if (active) palette.bg else palette.textPrimary,
                            )
                        }
                    }
                }
                Spacer(Modifier.height(12.dp))
                Button(
                    onClick = {
                        onSaveSettings(
                            settings.copy(
                                geminiApiKey = apiKey.trim().ifEmpty { null },
                                geminiModel = model,
                            )
                        )
                    },
                    shape = CircleShape,
                    colors = ButtonDefaults.buttonColors(
                        containerColor = palette.textPrimary,
                        contentColor = palette.bg,
                    ),
                ) {
                    Text("Simpan Pengaturan AI", fontSize = 13.sp)
                }
            }
        }

        item {
            SettingsCard(title = "Library Backup (JSON)", palette = palette) {
                Text(
                    text = "Export or restore your library metadata, reading progress, and bookmarks.",
                    fontSize = 13.sp,
                    color = palette.textSecondary,
                )
                Spacer(Modifier.height(12.dp))
                Row(horizontalArrangement = Arrangement.spacedBy(10.dp)) {
                    Button(
                        onClick = onExportBackup,
                        shape = CircleShape,
                        colors = ButtonDefaults.buttonColors(
                            containerColor = palette.textPrimary,
                            contentColor = palette.bg,
                        ),
                        modifier = Modifier.weight(1f),
                    ) {
                        Text("Export JSON", fontSize = 13.sp)
                    }
                    OutlinedButton(
                        onClick = onImportBackup,
                        shape = CircleShape,
                        modifier = Modifier.weight(1f),
                    ) {
                        Text("Restore JSON", fontSize = 13.sp, color = palette.textPrimary)
                    }
                }
            }
        }

        item {
            SettingsCard(title = "App Version & Updates", palette = palette) {
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceBetween,
                    verticalAlignment = Alignment.CenterVertically,
                ) {
                    Column {
                        Text("Baca Native v$appVersion", fontWeight = FontWeight.SemiBold, color = palette.textPrimary)
                        updateStatus?.let {
                            Text(it, fontSize = 12.sp, color = palette.textSecondary)
                        }
                    }
                    Button(
                        enabled = !checkingUpdate,
                        onClick = {
                            checkingUpdate = true
                            updateStatus = null
                            scope.launch {
                                onCheckUpdate()
                                    .onSuccess { rel ->
                                        releaseInfo = rel
                                        updateStatus = if (rel != null) {
                                            "Tersedia versi ${rel.tagName}"
                                        } else {
                                            "Sudah versi terbaru"
                                        }
                                    }
                                    .onFailure {
                                        updateStatus = it.message ?: "Gagal memeriksa pembaruan"
                                    }
                                checkingUpdate = false
                            }
                        },
                        shape = CircleShape,
                        colors = ButtonDefaults.buttonColors(
                            containerColor = palette.surfaceMuted,
                            contentColor = palette.textPrimary,
                        ),
                    ) {
                        Text(if (checkingUpdate) "Memeriksa..." else "Cek Update", fontSize = 12.sp)
                    }
                }

                releaseInfo?.let { rel ->
                    Spacer(Modifier.height(12.dp))
                    Text(rel.name, fontWeight = FontWeight.Medium, color = palette.accent, fontSize = 13.sp)
                    if (rel.body.isNotBlank()) {
                        Spacer(Modifier.height(4.dp))
                        Text(rel.body.take(240), fontSize = 12.sp, color = palette.textSecondary)
                    }
                    Spacer(Modifier.height(10.dp))
                    Button(
                        onClick = { onOpenUrl(rel.apkUrl ?: rel.htmlUrl) },
                        shape = CircleShape,
                        colors = ButtonDefaults.buttonColors(
                            containerColor = palette.accent,
                            contentColor = Color.White,
                        ),
                    ) {
                        Text("Unduh ${rel.tagName}", fontSize = 13.sp)
                    }
                }
            }
        }
    }
}

@Composable
private fun SettingsCard(
    title: String,
    palette: BacaPalette,
    content: @Composable ColumnScope.() -> Unit,
) {
    val shape = RoundedCornerShape(20.dp)
    Column(
        modifier = Modifier
            .fillMaxWidth()
            .shadow(4.dp, shape, ambientColor = Color.Black.copy(alpha = 0.05f))
            .clip(shape)
            .background(palette.surface)
            .border(0.5.dp, palette.border, shape)
            .padding(18.dp),
    ) {
        Text(
            text = title,
            fontFamily = FontFamily.Serif,
            fontSize = 17.sp,
            color = palette.textPrimary,
        )
        Spacer(Modifier.height(12.dp))
        content()
    }
}
