//! Window chrome: tab strip, mode bar (menus, workspace modes, document actions, command
//! search) and the right rail.
//!
//! Port of `crates/ui-egui/src/chrome.rs`.

use egui::{Align, Align2, Color32, CornerRadius, Layout, Rect, Sense, Stroke, vec2};

use crate::app::{App, Dialog, LeftPanel, Mode, Panel};
use crate::data;
use crate::icons;
use crate::theme::{self, ThemeKind, Tokens};
use crate::widgets;

pub fn tab_strip(app: &mut App, ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    egui::Panel::top("tab_strip")
        .exact_size(38.0)
        .frame(egui::Frame::NONE.fill(t.titlebar).inner_margin(egui::Margin { left: 8, right: 10, top: 0, bottom: 0 }))
        .show(ui, |ui| {
            ui.horizontal_centered(|ui| {
                ui.spacing_mut().item_spacing.x = 4.0;
                if icons::button(ui, "house", 28.0, !app.active, "Home").clicked() {
                    app.active = false;
                }
                if tab(ui, &t, data::DOC_NAME, true, app.active).clicked() {
                    app.active = true;
                }
                ui.add_space(4.0);
                if widgets::ghost_button(ui, "plus", "Open").on_hover_text("Open a PDF (⌘O)").clicked() {
                    app.run_command("file.open");
                }
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let (icon, next, tip) = match app.theme {
                        ThemeKind::Light => ("moon", ThemeKind::Dark, "Dark gray theme"),
                        ThemeKind::Dark => ("sun", ThemeKind::Light, "Light theme"),
                    };
                    if icons::button(ui, icon, 28.0, false, tip).clicked() {
                        let ctx = ui.ctx().clone();
                        app.set_theme(&ctx, next);
                    }
                    if icons::button(ui, "circle-help", 28.0, false, "Keyboard shortcuts").clicked() {
                        app.dialog = Some(Dialog::Shortcuts);
                    }
                    if widgets::ghost_button(ui, "messages-square", "Discord").on_hover_text("discord.gg/artcraft").clicked() {
                        app.run_command("help.discord");
                    }
                });
            });
        });
}

fn tab(ui: &mut egui::Ui, t: &Tokens, name: &str, dirty: bool, active: bool) -> egui::Response {
    let font = theme::regular(13.0);
    let label: String = if name.chars().count() > 28 { format!("{}…", name.chars().take(27).collect::<String>()) } else { name.to_string() };
    let text_w = ui.fonts_mut(|f| f.layout_no_wrap(label.clone(), font.clone(), t.text).size().x);
    let (rect, resp) = ui.allocate_exact_size(vec2(text_w + 64.0, 30.0), Sense::click());
    let a11y = if dirty { format!("{name} (edited)") } else { name.to_string() };
    resp.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Button, true, active, &a11y));
    let bg = if active {
        t.chrome
    } else if resp.hovered() {
        t.hover
    } else {
        Color32::TRANSPARENT
    };
    ui.painter().rect_filled(rect, CornerRadius { nw: 7, ne: 7, sw: 0, se: 0 }, bg);
    icons::paint(
        ui,
        Rect::from_min_size(rect.min + vec2(6.0, 7.0), vec2(16.0, 16.0)),
        "file-text",
        15.0,
        if active { t.accent } else { t.text_muted },
    );
    ui.painter().text(rect.min + vec2(28.0, rect.height() / 2.0), Align2::LEFT_CENTER, label, font, if active { t.text } else { t.text_muted });
    let x_rect = Rect::from_center_size(rect.right_center() - vec2(16.0, 0.0), vec2(20.0, 20.0));
    let x = ui.interact(x_rect, ui.id().with("tabclose"), Sense::click());
    if x.hovered() {
        ui.painter().rect_filled(x_rect, CornerRadius::same(4), t.pressed);
    }
    // Unsaved changes: a dot where the close button sits, until the tab is hovered.
    if dirty && !resp.hovered() && !x.hovered() {
        ui.painter().circle_filled(x_rect.center(), 4.0, if active { t.text } else { t.text_muted });
    } else if active || resp.hovered() || x.hovered() {
        icons::paint(ui, x_rect, "x", 13.0, t.text_muted);
    }
    resp.on_hover_text(format!("{name} — unsaved changes"))
}

pub fn mode_bar(app: &mut App, ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    egui::Panel::top("mode_bar")
        .exact_size(48.0)
        .frame(egui::Frame::NONE.fill(t.chrome).inner_margin(egui::Margin::symmetric(10, 0)).stroke(Stroke::new(1.0, t.divider)))
        .show(ui, |ui| {
            ui.horizontal_centered(|ui| {
                ui.spacing_mut().item_spacing.x = 2.0;
                main_menu(app, ui);
                ui.add_space(6.0);
                ui.painter().vline(ui.cursor().left(), ui.max_rect().y_range().shrink(12.0), Stroke::new(1.0, t.divider));
                ui.add_space(10.0);
                for (mode, label) in
                    [(Mode::AllTools, "All tools"), (Mode::Read, "Read"), (Mode::Edit, "Edit"), (Mode::Convert, "Convert"), (Mode::Sign, "E-Sign")]
                {
                    if widgets::mode_tab(ui, label, app.mode == mode).clicked() {
                        app.mode = mode;
                        app.left_open = true;
                        app.left = match mode {
                            Mode::Edit => LeftPanel::Tool("edit"),
                            Mode::Convert => LeftPanel::Tool("export"),
                            Mode::Sign => LeftPanel::Tool("fill_sign"),
                            _ => LeftPanel::AllTools,
                        };
                    }
                }
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.spacing_mut().item_spacing.x = 4.0;
                    ui.add_enabled_ui(app.active, |ui| {
                        if icons::button(ui, "printer", 32.0, false, "Print (⌘P)").clicked() {
                            app.run_command("file.print");
                        }
                        if icons::button(ui, "save", 32.0, false, "Save (⌘S)").clicked() {
                            app.run_command("file.save");
                        }
                        if icons::button(ui, "info", 32.0, false, "Document properties (⌘D)").clicked() {
                            app.dialog = Some(Dialog::Properties);
                        }
                    });
                    ui.add_space(8.0);
                    if widgets::search_box(ui, "Find tools and commands", 260.0).clicked() {
                        app.open_palette();
                    }
                });
            });
        });
}

fn main_menu(app: &mut App, ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    let resp = widgets::ghost_button(ui, "panel-left", "Menu");
    egui::Popup::menu(&resp).show(|ui| {
        ui.set_min_width(230.0);
        ui.menu_button("File", |ui| registry_menu(app, ui, "File"));
        ui.menu_button("Edit", |ui| registry_menu(app, ui, "Edit"));
        ui.menu_button("Pages", |ui| registry_menu(app, ui, "Pages"));
        ui.menu_button("View", |ui| {
            ui.label(egui::RichText::new("Zoom").color(t.text_faint).small());
            for (label, shortcut, zoom, width) in
                [("Actual size", "⌘1", 1.0, false), ("Zoom to page level", "⌘0", 0.85, false), ("Fit to width", "⌘2", 1.1, true)]
            {
                if widgets::menu_item(ui, label, shortcut).clicked() {
                    app.zoom = zoom;
                    app.fit_width = width;
                }
            }
            if widgets::menu_item(ui, "Zoom in", "⌘+").clicked() {
                app.zoom_step(true);
            }
            if widgets::menu_item(ui, "Zoom out", "⌘−").clicked() {
                app.zoom_step(false);
            }
            ui.separator();
            ui.label(egui::RichText::new("Display theme").color(t.text_faint).small());
            if ui.radio(app.theme == ThemeKind::Light, "Light gray").clicked() {
                let ctx = ui.ctx().clone();
                app.set_theme(&ctx, ThemeKind::Light);
            }
            if ui.radio(app.theme == ThemeKind::Dark, "Dark gray").clicked() {
                let ctx = ui.ctx().clone();
                app.set_theme(&ctx, ThemeKind::Dark);
            }
            ui.separator();
            ui.menu_button("Side panels", |ui| {
                for (p, label) in [
                    (Panel::Comments, "Comments"),
                    (Panel::Bookmarks, "Bookmarks"),
                    (Panel::Pages, "Page Thumbnails"),
                    (Panel::Fields, "Form Fields"),
                    (Panel::Layers, "Layers"),
                    (Panel::Attachments, "Attachments"),
                    (Panel::Signatures, "Signatures"),
                    (Panel::Search, "Advanced Search"),
                ] {
                    if ui.radio(app.panel == Some(p), label).clicked() {
                        app.panel = Some(p);
                    }
                }
            });
            ui.separator();
            if widgets::menu_item(ui, "Read Mode", "").clicked() {
                app.mode = Mode::Read;
            }
            if widgets::menu_item(ui, "Full Screen Mode", "⌘L").clicked() {
                app.run_command("view.full_screen");
            }
        });
        ui.menu_button("Help", |ui| registry_menu(app, ui, "Help"));
    });
}

/// Every registered command for one menu, with its shortcut (commands.rs::registry_menu).
fn registry_menu(app: &mut App, ui: &mut egui::Ui, menu: &str) {
    let mut run = None;
    let t = Tokens::get(ui.ctx());
    for c in data::COMMANDS.iter().filter(|c| c.menu == Some(menu)) {
        let enabled = match c.id {
            "file.save" | "file.save_as" | "file.print" | "file.revert" | "file.close" => app.active,
            _ => c.enabled,
        };
        ui.spacing_mut().button_padding = vec2(10.0, 3.0);
        let resp = ui.add_enabled(enabled, egui::Button::new(c.label).shortcut_text(c.shortcut.unwrap_or("")));
        if resp.clicked() {
            run = Some(c.id);
            ui.close();
        }
    }
    if let Some(id) = run {
        let _ = t;
        app.run_command(id);
    }
}

pub fn right_rail(app: &mut App, ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    if !app.active {
        return;
    }
    let has_comments = !app.comments.is_empty();
    let has_outline = !data::BOOKMARKS.is_empty();
    let has_fields = !data::FIELDS.is_empty();
    let has_layers = !data::LAYERS.is_empty();
    let has_files = !data::ATTACHMENTS.is_empty();
    let has_signatures = !data::SIGNATURES.is_empty();
    let page_count = app.pages.len();
    egui::Panel::right("rail")
        .resizable(false)
        .exact_size(48.0)
        .frame(egui::Frame::NONE.fill(t.chrome).inner_margin(egui::Margin::symmetric(6, 8)).stroke(Stroke::new(1.0, t.divider)))
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 4.0;
            let mut rail_button = |ui: &mut egui::Ui, panel: Panel, icon: &str, tip: &str, has: bool| {
                let selected = app.panel == Some(panel);
                let r = icons::button(ui, icon, 34.0, selected, tip);
                if has && !selected {
                    let c = r.rect.right_top() + vec2(-8.0, 8.0);
                    ui.painter().circle_filled(c, 3.0, t.accent);
                }
                if r.clicked() {
                    app.panel = if selected { None } else { Some(panel) };
                }
            };
            rail_button(ui, Panel::Comments, "message-square-text", "Comments", has_comments);
            rail_button(ui, Panel::Bookmarks, "bookmark", "Bookmarks", has_outline);
            rail_button(ui, Panel::Pages, "files", "Page thumbnails", false);
            rail_button(ui, Panel::Fields, "text-cursor-input", "Form fields", has_fields);
            rail_button(ui, Panel::Layers, "layers", "Layers", has_layers);
            rail_button(ui, Panel::Attachments, "paperclip", "Attachments", has_files);
            rail_button(ui, Panel::Signatures, "signature", "Signatures", has_signatures);
            rail_button(ui, Panel::Search, "search", "Advanced Search", true);

            // Page navigation cluster at the bottom (as in Acrobat's rail).
            ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                if icons::button(ui, "zoom-out", 32.0, false, "Zoom out (⌘−)").clicked() {
                    app.zoom_step(false);
                }
                if icons::button(ui, "zoom-in", 32.0, false, "Zoom in (⌘+)").clicked() {
                    app.zoom_step(true);
                }
                if icons::button(ui, "rotate-cw", 32.0, false, "Rotate view clockwise (⇧⌘+)").clicked() {
                    app.run_command("view.rotate_cw");
                }
                let fit_icon = if app.fit_width { "maximize" } else { "columns-2" };
                if icons::button(ui, fit_icon, 32.0, false, "Toggle fit page / fit width").clicked() {
                    app.fit_width = !app.fit_width;
                    app.zoom = if app.fit_width { 1.1 } else { 0.85 };
                }
                ui.label(egui::RichText::new(format!("{:.0}%", app.zoom * 100.0)).font(theme::regular(10.5)).color(t.text_faint));
                ui.add_space(6.0);
                if icons::button(ui, "chevron-down", 30.0, false, "Next page").clicked() {
                    app.step_page(true);
                }
                if icons::button(ui, "chevron-up", 30.0, false, "Previous page").clicked() {
                    app.step_page(false);
                }
                ui.label(egui::RichText::new(page_count.to_string()).font(theme::regular(11.0)).color(t.text_muted));
                let mut input = app.page_input.clone();
                let edit = egui::TextEdit::singleline(&mut input)
                    .id(egui::Id::new("page-input"))
                    .desired_width(34.0)
                    .horizontal_align(Align::Center)
                    .font(theme::medium(12.0))
                    .margin(vec2(2.0, 4.0));
                let r = egui::Frame::NONE
                    .fill(t.field)
                    .stroke(Stroke::new(1.0, t.border))
                    .corner_radius(CornerRadius::same(5))
                    .show(ui, |ui| ui.add(edit))
                    .inner
                    .on_hover_text("Current page — type a page number or label and press Enter");
                if r.changed() {
                    app.page_input = input.clone();
                }
                if r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    match app.pages.iter().position(|p| p.label.eq_ignore_ascii_case(input.trim())) {
                        Some(i) => app.page = i,
                        None => {
                            if let Ok(n) = input.trim().parse::<usize>() {
                                app.page = n.saturating_sub(1).min(page_count.saturating_sub(1));
                            } else {
                                app.page_input = app.page_label(app.page).to_owned();
                            }
                        }
                    }
                }
            });
        });
}
