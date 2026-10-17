//! PrintCraft design demo — the desktop shell in Rust (egui + eframe).
//!
//! The window is created exactly as PrintCraft's `apps/printcraft/src/main.rs` does it
//! (same viewport options, same app id), minus the PDF engine.

use printcraft_design_demo::app::App;

const APP_ID: &str = "ai.storyteller.printcraft.designdemo";

fn main() -> eframe::Result<()> {
    let viewport = egui::ViewportBuilder::default()
        .with_title("PrintCraft — design demo")
        .with_inner_size([1440.0, 920.0])
        .with_min_inner_size([820.0, 520.0])
        .with_drag_and_drop(true)
        .with_app_id(APP_ID);
    let native = eframe::NativeOptions { viewport, ..Default::default() };
    eframe::run_native(
        "PrintCraft",
        native,
        Box::new(|cc| {
            let mut app = App::new();
            if let Some(json) = cc.storage.and_then(|s| s.get_string("printcraft-design-demo")) {
                app.restore(&json);
            }
            Ok(Box::new(app))
        }),
    )
}
