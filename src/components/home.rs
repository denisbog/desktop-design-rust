//! Home tab: recommended tools, the open card, the community card and recent files.
//!
//! Port of `crates/ui-egui/src/home.rs`, with the design demo's own copy.

use egui::{Align2, Color32, CornerRadius, Rect, Sense, Stroke, vec2};

use crate::app::{App, LeftPanel};
use crate::data;
use crate::icons;
use crate::theme::{self, Tokens};
use crate::widgets;

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        egui::Frame::NONE.inner_margin(egui::Margin { left: 36, right: 36, top: 28, bottom: 28 }).show(ui, |ui| {
            ui.label(egui::RichText::new("Welcome to PrintCraft").font(theme::semibold(24.0)));
            ui.label(
                egui::RichText::new("An open-source PDF workbench — local, private, and scriptable.").color(t.text_muted).font(theme::regular(14.0)),
            );
            ui.add_space(14.0);

            // Community card.
            widgets::card_frame(&t).show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    let (rect, _) = ui.allocate_exact_size(vec2(28.0, 28.0), Sense::hover());
                    ui.painter().rect_filled(rect, CornerRadius::same(8), t.accent);
                    icons::paint(ui, rect, "sparkles", 16.0, Color32::WHITE);
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new("Join the ArtCraft community").font(theme::semibold(15.0)));
                        ui.label(egui::RichText::new("Get help, share feedback and follow development on Discord.").color(t.text_muted));
                    });
                });
                ui.add_space(8.0);
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = vec2(8.0, 8.0);
                    for (icon, label) in [("messages-square", "Join our Discord"), ("globe", "Website"), ("book-open", "Documentation")] {
                        if widgets::icon_pill(ui, icon, label, label.starts_with("Join")).clicked() {
                            app.notify(format!("{label} — demo stub"));
                        }
                    }
                });
            });

            ui.add_space(22.0);

            // Recommended tools.
            widgets::card_frame(&t).show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.label(egui::RichText::new("Recommended tools").font(theme::semibold(15.0)));
                ui.add_space(10.0);
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = vec2(14.0, 14.0);
                    for id in data::RECOMMENDED {
                        let Some(g) = data::group(id) else { continue };
                        if tile(ui, &t, g.icon, Color32::from_rgb(g.hue.0, g.hue.1, g.hue.2), g.label, &blurb(g)).clicked() {
                            app.left = LeftPanel::Tool(g.id);
                            app.left_open = true;
                            app.active = true;
                        }
                    }
                    if open_tile(ui, &t).clicked() {
                        app.run_command("file.open");
                    }
                });
            });

            ui.add_space(22.0);

            // More tools.
            widgets::card_frame(&t).show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.label(egui::RichText::new("More tools").font(theme::semibold(15.0)));
                ui.add_space(10.0);
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = vec2(8.0, 8.0);
                    for g in data::TOOL_GROUPS {
                        if widgets::pill_button(ui, g.label, false).on_hover_text(g.id).clicked() {
                            app.left = LeftPanel::Tool(g.id);
                            app.left_open = true;
                            app.active = true;
                        }
                    }
                });
            });

            ui.add_space(26.0);
            ui.label(egui::RichText::new("Recent").font(theme::semibold(17.0)));
            ui.add_space(8.0);
            let mut open = None;
            for (name, path, pages, size) in &app.recent {
                let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 46.0), Sense::click());
                resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, *name));
                if resp.hovered() {
                    ui.painter().rect_filled(rect, CornerRadius::same(8), t.hover);
                }
                icons::paint(
                    ui,
                    Rect::from_min_size(rect.min + vec2(10.0, 11.0), vec2(24.0, 24.0)),
                    "file-text",
                    22.0,
                    Color32::from_rgb(0xE0, 0x3E, 0x3E),
                );
                ui.painter().text(rect.min + vec2(46.0, 15.0), Align2::LEFT_CENTER, *name, theme::medium(13.5), t.text);
                ui.painter().text(rect.min + vec2(46.0, 32.0), Align2::LEFT_CENTER, *path, theme::regular(11.0), t.text_faint);
                ui.painter().text(
                    rect.right_center() - vec2(12.0, 0.0),
                    Align2::RIGHT_CENTER,
                    format!("{pages} pages  ·  {size}"),
                    theme::regular(12.0),
                    t.text_muted,
                );
                if resp.clicked() {
                    open = Some(*name);
                }
            }
            if let Some(name) = open {
                app.active = true;
                app.notify(format!("Opening {name} — the demo loads the specimen"));
            }

            ui.add_space(20.0);
            widgets::section_title(ui, "Privacy");
            ui.label(
                egui::RichText::new("PrintCraft works offline. No telemetry, no account, and no cloud processing unless you add a provider.")
                    .color(t.text_muted),
            );
        });
    });
}

fn blurb(g: &data::ToolGroup) -> String {
    g.sections.first().map(|s| s.items.iter().take(3).map(|i| i.label).collect::<Vec<_>>().join(" · ")).unwrap_or_default()
}

fn tile(ui: &mut egui::Ui, t: &Tokens, icon: &str, color: Color32, label: &str, blurb: &str) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(vec2(190.0, 104.0), Sense::click());
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, label));
    let fill = if resp.hovered() { t.hover } else { t.card };
    ui.painter().rect(rect, CornerRadius::same(10), fill, Stroke::new(1.0, t.divider), egui::StrokeKind::Inside);
    icons::paint(ui, Rect::from_min_size(rect.min + vec2(14.0, 14.0), vec2(22.0, 22.0)), icon, 21.0, color);
    ui.painter().text(rect.min + vec2(44.0, 25.0), Align2::LEFT_CENTER, label, theme::semibold(13.5), t.text);
    let galley = ui.fonts_mut(|f| f.layout(blurb.to_owned(), theme::regular(11.5), t.text_muted, rect.width() - 28.0));
    ui.painter().galley(rect.min + vec2(14.0, 46.0), galley, t.text_muted);
    ui.painter().text(rect.left_bottom() + vec2(14.0, -14.0), Align2::LEFT_CENTER, "Use now", theme::medium(12.0), t.accent_text);
    resp
}

fn open_tile(ui: &mut egui::Ui, t: &Tokens) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(vec2(170.0, 104.0), Sense::click());
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, "Open file"));
    ui.painter().rect(
        rect,
        CornerRadius::same(10),
        if resp.hovered() { t.hover } else { t.pasteboard },
        Stroke::new(1.0, t.divider),
        egui::StrokeKind::Inside,
    );
    icons::paint(ui, Rect::from_center_size(rect.center() - vec2(0.0, 16.0), vec2(28.0, 28.0)), "folder-open", 26.0, t.icon);
    ui.painter().text(rect.center() + vec2(0.0, 22.0), Align2::CENTER_CENTER, "Open file", theme::semibold(13.0), t.text);
    resp
}
