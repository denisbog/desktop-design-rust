//! Vector icons: Lucide SVGs embedded from `assets/icons`, recoloured to white, rasterized by
//! `egui_extras` at the exact on-screen pixel size and tinted per use — never a Unicode glyph.
//!
//! Port of `crates/ui-egui/src/icons.rs`.

use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

use egui::{Color32, Rect, Response, Sense, Vec2};

use crate::icon_data::ICONS;
use crate::theme::Tokens;

fn white_icons() -> &'static HashMap<&'static str, Arc<[u8]>> {
    static MAP: OnceLock<HashMap<&'static str, Arc<[u8]>>> = OnceLock::new();
    MAP.get_or_init(|| {
        ICONS
            .iter()
            .map(|(name, bytes)| {
                let svg = String::from_utf8_lossy(bytes).replace("currentColor", "#ffffff").replace("stroke-width=\"2\"", "stroke-width=\"1.75\"");
                (*name, Arc::from(svg.into_bytes().into_boxed_slice()))
            })
            .collect()
    })
}

pub fn exists(name: &str) -> bool {
    white_icons().contains_key(name)
}

pub fn image(name: &str, size: f32, tint: Color32) -> egui::Image<'static> {
    let (key, bytes) = match white_icons().get(name) {
        Some(b) => (name, b.clone()),
        None => ("square", white_icons()["square"].clone()),
    };
    egui::Image::from_bytes(format!("bytes://icons/{key}.svg"), egui::load::Bytes::Shared(bytes)).fit_to_exact_size(Vec2::splat(size)).tint(tint)
}

pub fn paint(ui: &egui::Ui, rect: Rect, name: &str, size: f32, tint: Color32) {
    image(name, size, tint).paint_at(ui, Rect::from_center_size(rect.center(), Vec2::splat(size)));
}

/// Square icon button: transparent until hovered; `selected` gets the accent wash.
pub fn button(ui: &mut egui::Ui, name: &str, box_size: f32, selected: bool, tooltip: &str) -> Response {
    let t = Tokens::get(ui.ctx());
    let (rect, resp) = ui.allocate_exact_size(Vec2::splat(box_size), Sense::click());
    let label = if tooltip.is_empty() { name } else { tooltip };
    resp.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Button, ui.is_enabled(), selected, label));
    if selected {
        ui.painter().rect_filled(rect, t.radius, t.accent_soft);
    } else if resp.is_pointer_button_down_on() {
        ui.painter().rect_filled(rect, t.radius, t.pressed);
    } else if resp.hovered() {
        ui.painter().rect_filled(rect, t.radius, t.hover);
    }
    let tint = if selected { t.accent_text } else { t.icon };
    paint(ui, rect, name, (box_size * 0.5).round(), tint);
    if tooltip.is_empty() { resp } else { resp.on_hover_text(tooltip) }
}
