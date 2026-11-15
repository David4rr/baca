use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    Warm,   // Master default: warm physical paper cream
    Light,  // Pure pearl daytime light
    Amoled, // OLED pitch black
    Slate,  // Midnight navy
}

impl ThemeMode {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "light" => Self::Light,
            "amoled" => Self::Amoled,
            "slate" => Self::Slate,
            _ => Self::Warm,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Warm => "warm",
            Self::Light => "light",
            Self::Amoled => "amoled",
            Self::Slate => "slate",
        }
    }

    pub fn bg_color(&self) -> &'static str {
        match self {
            Self::Warm => "#F9F6F0",    // Warm paper linen
            Self::Light => "#F8FAFC",   // Pearl light
            Self::Amoled => "#0A0A0A",  // Deepest charcoal
            Self::Slate => "#0F172A",   // Navy slate
        }
    }

    pub fn surface_color(&self) -> &'static str {
        match self {
            Self::Warm => "#FFFFFF",    // Crisp white sheet
            Self::Light => "#FFFFFF",   // Crisp white sheet
            Self::Amoled => "#161616",  // Smoky black glass
            Self::Slate => "#1E293B",   // Soft navy slate
        }
    }

    pub fn surface_subtle(&self) -> &'static str {
        match self {
            Self::Warm => "#F3EFE6",
            Self::Light => "#F1F5F9",
            Self::Amoled => "#222222",
            Self::Slate => "#334155",
        }
    }

    pub fn text_primary(&self) -> &'static str {
        match self {
            Self::Warm => "#1C1917",    // Deep espresso black
            Self::Light => "#0F172A",   // Obsidian
            Self::Amoled => "#F4F4F5",  // Crisp bone white
            Self::Slate => "#F8FAFC",   // Clean white
        }
    }

    pub fn text_secondary(&self) -> &'static str {
        match self {
            Self::Warm => "#78716C",    // Stone warm gray
            Self::Light => "#64748B",   // Slate gray
            Self::Amoled => "#A1A1AA",  // Muted zinc
            Self::Slate => "#94A3B8",   // Soft navy gray
        }
    }

    pub fn text_muted(&self) -> &'static str {
        match self {
            Self::Warm => "#A8A29E",
            Self::Light => "#94A3B8",
            Self::Amoled => "#71717A",
            Self::Slate => "#64748B",
        }
    }

    pub fn accent_color(&self) -> &'static str {
        match self {
            Self::Warm => "#1C1917",    // Pitch espresso button
            Self::Light => "#0F172A",   // Obsidian button
            Self::Amoled => "#F4F4F5",  // Bone white button
            Self::Slate => "#38BDF8",   // Sky blue accent
        }
    }

    pub fn accent_contrast_text(&self) -> &'static str {
        match self {
            Self::Warm => "#FFFFFF",
            Self::Light => "#FFFFFF",
            Self::Amoled => "#0A0A0A",
            Self::Slate => "#0F172A",
        }
    }

    pub fn border_color(&self) -> &'static str {
        match self {
            Self::Warm => "rgba(28, 25, 23, 0.06)",
            Self::Light => "rgba(15, 23, 42, 0.08)",
            Self::Amoled => "rgba(255, 255, 255, 0.08)",
            Self::Slate => "rgba(255, 255, 255, 0.1)",
        }
    }

    pub fn card_shadow(&self) -> &'static str {
        match self {
            Self::Warm => "0 12px 36px -4px rgba(28, 25, 23, 0.04), 0 4px 12px -2px rgba(28, 25, 23, 0.02)",
            Self::Light => "0 12px 36px -4px rgba(0, 0, 0, 0.03), 0 4px 12px -2px rgba(0, 0, 0, 0.02)",
            Self::Amoled => "0 12px 36px rgba(0, 0, 0, 0.6)",
            Self::Slate => "0 12px 36px rgba(0, 0, 0, 0.4)",
        }
    }

    pub fn highlight_color(&self) -> &'static str {
        match self {
            Self::Warm => "#FEF08A",
            Self::Light => "#FEF08A",
            Self::Amoled => "#FDE047",
            Self::Slate => "#38BDF8",
        }
    }

    pub fn is_dark(&self) -> bool {
        matches!(self, Self::Amoled | Self::Slate)
    }
}
