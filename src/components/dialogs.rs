//! Modal dialogs: Document Properties, Keyboard Shortcuts, About and Preferences.
//!
//! Port of the dialog half of `crates/ui-egui/src/dialogs.rs`, using egui's `Modal` (centred
//! window + backdrop) so the scrim and dismissal come for free.

use egui::{Grid, RichText};

use crate::app::{App, Dialog};
use crate::data;
use crate::theme::{self, ThemeKind, Tokens};
use crate::widgets;

const PROP_TABS: [&str; 5] = ["Description", "Initial View", "Security", "Fonts", "Advanced"];

pub fn show(app: &mut App, ctx: &egui::Context) {
    let Some(dialog) = app.dialog else { return };
    let t = Tokens::get(ctx);
    let width = match dialog {
        Dialog::Properties => 640.0,
        Dialog::Shortcuts => 560.0,
        _ => 520.0,
    };
    let resp = egui::Modal::new(egui::Id::new("dialog")).backdrop_color(t.scrim).show(ctx, |ui| {
        ui.set_width(width);
        match dialog {
            Dialog::Shortcuts => shortcuts(ui, &t),
            Dialog::About => about(app, ui, &t),
            Dialog::Properties => properties(app, ui, &t),
            Dialog::Preferences => preferences(app, ui, &t),
        }
    });
    if resp.should_close() {
        app.dialog = None;
    }
}

fn title(ui: &mut egui::Ui, text: &str, t: &Tokens) {
    ui.label(RichText::new(text).font(theme::semibold(18.0)).color(t.text));
    ui.add_space(10.0);
}

fn shortcuts(ui: &mut egui::Ui, t: &Tokens) {
    title(ui, "Keyboard Shortcuts", t);
    egui::ScrollArea::vertical().max_height(420.0).show(ui, |ui| {
        Grid::new("shortcuts").num_columns(2).spacing([24.0, 6.0]).show(ui, |ui| {
            for (key, label) in data::SHORTCUTS {
                ui.label(RichText::new(*key).font(theme::mono(12.5)).color(t.text));
                ui.label(RichText::new(*label).color(t.text_muted));
                ui.end_row();
            }
        });
    });
    ui.add_space(14.0);
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        if widgets::pill_button(ui, "Close", true).clicked() {
            ui.close();
        }
    });
}

fn about(_app: &mut App, ui: &mut egui::Ui, t: &Tokens) {
    ui.horizontal(|ui| {
        let (rect, _) = ui.allocate_exact_size(egui::vec2(40.0, 40.0), egui::Sense::hover());
        ui.painter().rect_filled(rect, egui::CornerRadius::same(10), t.accent);
        crate::icons::paint(ui, rect, "file-text", 22.0, egui::Color32::WHITE);
        ui.vertical(|ui| {
            ui.label(RichText::new("PrintCraft").font(theme::semibold(20.0)).color(t.text));
            ui.label(RichText::new(format!("Design demo · version {}", data::DOC_VERSION)).color(t.text_muted));
        });
    });
    ui.add_space(12.0);
    ui.label(RichText::new("The PDF workbench; an open-source, clean-room reimplementation of Adobe Acrobat, rebuilt in pure Rust.").color(t.text));
    ui.add_space(10.0);
    ui.label(
        RichText::new("This build is a UI design demo: every surface is live, but the PDF engine is not linked.")
            .color(t.text_muted)
            .font(theme::regular(12.0)),
    );
    ui.add_space(14.0);
    ui.horizontal(|ui| {
        if widgets::icon_pill(ui, "messages-square", "Join our Discord", true).clicked() {
            ui.close();
        }
        if widgets::icon_pill(ui, "globe", "getartcraft.com", false).clicked() {
            ui.close();
        }
    });
    ui.add_space(14.0);
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        if widgets::pill_button(ui, "Close", true).clicked() {
            ui.close();
        }
        ui.label(RichText::new("MIT OR Apache-2.0").color(t.text_faint).font(theme::regular(11.5)));
    });
}

fn properties(app: &mut App, ui: &mut egui::Ui, t: &Tokens) {
    title(ui, "Document Properties", t);
    let mut tab = ui.data_mut(|d| d.get_temp::<usize>(egui::Id::new("props-tab")).unwrap_or(0));
    ui.horizontal(|ui| {
        for (i, label) in PROP_TABS.iter().enumerate() {
            if widgets::mode_tab(ui, label, tab == i).clicked() {
                tab = i;
            }
        }
    });
    ui.separator();
    ui.add_space(6.0);
    ui.data_mut(|d| d.insert_temp(egui::Id::new("props-tab"), tab));
    let rows: &[(&str, &str)] = match tab {
        1 => &[("Page layout", "Continuous"), ("Magnification", "Fit to page"), ("Open on page", "1"), ("Show bookmarks", "Yes")],
        2 => &[
            ("Security method", "Password (revision 6, AES-256)"),
            ("Printing", "Allowed"),
            ("Changing the document", "Not allowed"),
            ("Commenting", "Allowed"),
        ],
        3 => &[
            ("Inter", "Embedded subset"),
            ("Inter-Medium / SemiBold", "Embedded subset"),
            ("JetBrains Mono", "Embedded subset"),
            ("Dancing Script", "Embedded subset"),
        ],
        4 => &[("PDF version", "1.7 (ISO 32000-1)"), ("Page count", "13"), ("Linearized", "Yes"), ("Tagged", "Yes")],
        _ => &[
            ("Title", "PrintCraft Showcase"),
            ("Author", data::DOC_AUTHOR),
            ("Subject", "A specimen for the PDF workbench"),
            ("Keywords", "printcraft, pdf, specimen, type"),
        ],
    };
    Grid::new("props").num_columns(2).spacing([18.0, 8.0]).min_col_width(120.0).show(ui, |ui| {
        for (k, v) in rows {
            ui.label(RichText::new(*k).color(t.text_muted));
            ui.label(RichText::new(*v).font(theme::regular(12.5)).color(t.text));
            ui.end_row();
        }
    });
    ui.add_space(14.0);
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        if widgets::pill_button(ui, "OK", true).clicked() {
            ui.close();
        }
        if widgets::pill_button(ui, "Cancel", false).clicked() {
            ui.close();
        }
        let _ = app;
    });
}

fn preferences(app: &mut App, ui: &mut egui::Ui, t: &Tokens) {
    title(ui, "Preferences", t);
    ui.label(RichText::new("Display theme").color(t.text_muted));
    ui.horizontal(|ui| {
        if ui.radio(app.theme == ThemeKind::Light, "Light gray").clicked() {
            let ctx = ui.ctx().clone();
            app.set_theme(&ctx, ThemeKind::Light);
        }
        if ui.radio(app.theme == ThemeKind::Dark, "Dark gray").clicked() {
            let ctx = ui.ctx().clone();
            app.set_theme(&ctx, ThemeKind::Dark);
        }
    });
    ui.add_space(10.0);
    ui.separator();
    ui.add_space(6.0);
    ui.label(RichText::new("Rendering").color(t.text_muted));
    ui.checkbox(&mut app.prefs.smooth_scroll, "Smooth scrolling");
    ui.checkbox(&mut app.prefs.hardware_accel, "Hardware acceleration (wgpu)");
    ui.checkbox(&mut app.prefs.remember_layout, "Follow the system theme");
    ui.add_space(10.0);
    ui.label(RichText::new("This demo stores nothing: no account, no telemetry, no cloud.").color(t.text_faint).font(theme::regular(11.5)));
    ui.add_space(14.0);
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        if widgets::pill_button(ui, "Done", true).clicked() {
            ui.close();
        }
    });
}
