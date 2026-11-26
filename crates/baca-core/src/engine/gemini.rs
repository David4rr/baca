use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
}

#[derive(Serialize)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}

#[derive(Serialize)]
struct GeminiPart {
    text: String,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Option<Vec<GeminiCandidate>>,
    error: Option<GeminiError>,
}

#[derive(Deserialize)]
struct GeminiCandidate {
    content: Option<GeminiContentResponse>,
}

#[derive(Deserialize)]
struct GeminiContentResponse {
    parts: Option<Vec<GeminiPartResponse>>,
}

#[derive(Deserialize)]
struct GeminiPartResponse {
    text: Option<String>,
}

#[derive(Deserialize)]
struct GeminiError {
    message: String,
}

pub async fn ask_gemini(api_key: &str, model: &str, term: &str) -> Result<String, String> {
    if api_key.trim().is_empty() {
        return Err("API key is not configured. Please set your Gemini API key in Settings.".to_string());
    }

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(20))
        .build()
        .map_err(|e| e.to_string())?;

    let model_name = if model.trim().is_empty() {
        "gemini-2.5-flash"
    } else {
        model.trim()
    };

    if !model_name.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '.' | '_')) {
        return Err("Nama model Gemini tidak valid".to_string());
    }
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
        model_name
    );

    let prompt = format!(
        "Jelaskan arti, makna, konteks penggunaan, dan contoh kalimat dari istilah atau kutipan berikut. Tulis dalam paragraf rapi dan jelas tanpa pembuka basa-basi:\n\n\"{}\"",
        term.trim()
    );

    let body = GeminiRequest {
        contents: vec![GeminiContent {
            parts: vec![GeminiPart { text: prompt }],
        }],
    };

    let resp = client
        .post(&url)
        .header("x-goog-api-key", api_key.trim())
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Network request failed: {}", e))?;

    let gemini_resp: GeminiResponse = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse Gemini response: {}", e))?;

    if let Some(err) = gemini_resp.error {
        return Err(format!("Gemini API Error: {}", err.message));
    }

    if let Some(candidates) = gemini_resp.candidates {
        if let Some(candidate) = candidates.first() {
            if let Some(content) = &candidate.content {
                if let Some(parts) = &content.parts {
                    if let Some(first_part) = parts.first() {
                        if let Some(text) = &first_part.text {
                            return Ok(text.clone());
                        }
                    }
                }
            }
        }
    }

    Err("No explanation was returned by the AI model.".to_string())
}
