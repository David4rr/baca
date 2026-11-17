use baca::app::App;
use dioxus::prelude::*;

fn main() {
    std::panic::set_hook(Box::new(|info| {
        eprintln!("[Baca Panic] {:?}", info);
        #[cfg(target_os = "android")]
        {
            use std::io::Write;
            let log_path = baca::db::Database::get_app_dir()
                .map(|d| d.join("baca_crash.log"))
                .unwrap_or_else(|_| std::env::temp_dir().join("baca_crash.log"));
            if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(log_path) {
                let _ = writeln!(f, "[Crash Log] {:?}", info);
            }
        }
    }));

    #[cfg(not(target_os = "android"))]
    {
        use dioxus::desktop::{Config, WindowBuilder};

        let window = WindowBuilder::new()
            .with_title("Baca. — Modern E-Book Reader")
            .with_inner_size(dioxus::desktop::tao::dpi::LogicalSize::new(1200.0, 800.0))
            .with_min_inner_size(dioxus::desktop::tao::dpi::LogicalSize::new(760.0, 540.0));

        let custom_head = r#"
            <meta name="viewport" content="width=device-width, initial-scale=1.0, viewport-fit=cover">
            <meta http-equiv="Content-Security-Policy" content="script-src 'self' 'unsafe-inline' 'unsafe-eval' blob:; object-src 'none'; base-uri 'none';">
        "#.to_string();

        let cfg = Config::new()
            .with_window(window)
            .with_custom_head(custom_head);

        LaunchBuilder::desktop()
            .with_cfg(cfg)
            .launch(App);
    }

    #[cfg(target_os = "android")]
    {
        LaunchBuilder::mobile()
            .launch(App);
    }
}
