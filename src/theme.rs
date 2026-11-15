use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    Amoled,
    Warm,
    Slate,
    Light,
}

impl ThemeMode {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "warm" => Self::Warm,
            "slate" => Self::Slate,
            "light" => Self::Light,
            _ => Self::Amoled,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Amoled => "amoled",
            Self::Warm => "warm",
            Self::Slate => "slate",
            Self::Light => "light",
        }
    }

    pub fn bg_color(&self) -> &'static str {
        match self {
            Self::Amoled => "#000000",
            Self::Warm => "#FBF0D9",
            Self::Slate => "#0D1117",
            Self::Light => "#FAFAFA",
        }
    }

    pub fn surface_color(&self) -> &'static str {
        match self {
            Self::Amoled => "#121212",
            Self::Warm => "#F4E7CB",
            Self::Slate => "#161B22",
            Self::Light => "#FFFFFF",
        }
    }

    pub fn text_primary(&self) -> &'static str {
        match self {
            Self::Amoled => "#E5E7EB",
            Self::Warm => "#4A3B2C",
            Self::Slate => "#C9D1D9",
            Self::Light => "#1A1A1A",
        }
    }

    pub fn text_secondary(&self) -> &'static str {
        match self {
            Self::Amoled => "#9CA3AF",
            Self::Warm => "#7D6853",
            Self::Slate => "#8B949E",
            Self::Light => "#666666",
        }
    }

    pub fn accent_color(&self) -> &'static str {
        match self {
            Self::Amoled => "#2DD4BF", // Teal
            Self::Warm => "#C27838",   // Warm amber
            Self::Slate => "#58A6FF",  // Ocean blue
            Self::Light => "#0F766E",  // Deep teal
        }
    }

    pub fn border_color(&self) -> &'static str {
        match self {
            Self::Amoled => "#27272A",
            Self::Warm => "#DFCBB2",
            Self::Slate => "#30363D",
            Self::Light => "#E5E7EB",
        }
    }
}
