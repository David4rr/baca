use baca::app::App;
use dioxus::prelude::*;

fn main() {
    std::panic::set_hook(Box::new(|info| {
        eprintln!("[Baca Panic] {:?}", info);
        #[cfg(target_os = "android")]
        {
            use std::io::Write;
            if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open("/sdcard/Download/baca_crash.log") {
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
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <link rel="stylesheet" href="assets/styles/main.css">
            <script src="https://cdn.tailwindcss.com"></script>
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
