//! Left tool panel (All tools + tool sub-panels, generated from the catalogue) and the
//! right-hand panels (Comments, Bookmarks, Pages, Fields, Layers, Attachments, Signatures,
//! Advanced Search).
//!
//! Port of `crates/ui-egui/src/panels.rs`, driven by the demo catalogue in `data.rs`.

use egui::{Align, Align2, Color32, CornerRadius, Layout, Rect, Sense, Stroke, vec2};

use crate::app::{App, LeftPanel, Panel};
use crate::data::{self, ToolGroup};
use crate::icons;
use crate::theme::{self, Tokens};
use crate::widgets;

const COLLAPSED_TOOLS: usize = 14;

fn hue(g: &ToolGroup) -> Color32 {
    Color32::from_rgb(g.hue.0, g.hue.1, g.hue.2)
}

pub fn left_panel(app: &mut App, ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    egui::Panel::left("tool_panel")
        .resizable(false)
        .exact_size(272.0)
        .frame(
            egui::Frame::NONE
                .fill(t.panel)
                .inner_margin(egui::Margin { left: 14, right: 10, top: 12, bottom: 10 })
                .stroke(Stroke::new(1.0, t.divider)),
        )
        .show(ui, |ui| match app.left {
            LeftPanel::AllTools => all_tools(app, ui, &t),
            LeftPanel::Tool(id) => match data::group(id) {
                Some(g) => tool_detail(app, ui, &t, g),
                None => app.left = LeftPanel::AllTools,
            },
        });
}

fn panel_header(ui: &mut egui::Ui, t: &Tokens, title: &str, count: Option<usize>, back: bool) -> (bool, bool) {
    let mut go_back = false;
    let mut close = false;
    ui.horizontal(|ui| {
        if back && icons::button(ui, "chevron-left", 26.0, false, "Back to all tools").clicked() {
            go_back = true;
        }
        ui.label(egui::RichText::new(title).font(theme::semibold(15.5)).color(t.text));
        if let Some(n) = count {
            ui.label(egui::RichText::new(n.to_string()).color(t.text_faint).font(theme::regular(12.0)));
        }
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if icons::button(ui, "x", 26.0, false, "Close panel").clicked() {
                close = true;
            }
        });
    });
    ui.add_space(6.0);
    (go_back, close)
}

fn all_tools(app: &mut App, ui: &mut egui::Ui, t: &Tokens) {
    let (_, close) = panel_header(ui, t, "All tools", None, false);
    if close {
        app.left_open = false;
    }
    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        let shown = if app.all_tools_expanded { data::TOOL_GROUPS.len() } else { COLLAPSED_TOOLS.min(data::TOOL_GROUPS.len()) };
        for g in &data::TOOL_GROUPS[..shown] {
            if tool_row(ui, t, g).clicked() {
                app.left = LeftPanel::Tool(g.id);
            }
        }
        ui.add_space(4.0);
        let more = if app.all_tools_expanded { "View less" } else { "View more" };
        if widgets::link(ui, more).clicked() {
            app.all_tools_expanded = !app.all_tools_expanded;
        }
    });
}

fn tool_row(ui: &mut egui::Ui, t: &Tokens, g: &ToolGroup) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 34.0), Sense::click());
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, g.label));
    if resp.hovered() {
        ui.painter().rect_filled(rect, CornerRadius::same(6), t.hover);
    }
    icons::paint(ui, Rect::from_min_size(rect.min + vec2(6.0, 7.0), vec2(20.0, 20.0)), g.icon, 19.0, hue(g));
    ui.painter().text(rect.left_center() + vec2(36.0, 0.0), Align2::LEFT_CENTER, g.label, theme::regular(13.5), t.text);
    match (g.badge, data::is_ready(g.availability)) {
        (Some(b), _) => {
            let font = theme::semibold(9.5);
            let w = ui.fonts_mut(|f| f.layout_no_wrap(b.to_string(), font.clone(), Color32::WHITE).size().x);
            let r = Rect::from_center_size(rect.right_center() - vec2(w / 2.0 + 10.0, 0.0), vec2(w + 10.0, 16.0));
            ui.painter().rect_filled(r, CornerRadius::same(4), t.badge_new);
            ui.painter().text(r.center(), Align2::CENTER_CENTER, b, font, Color32::WHITE);
        }
        // Planned tools: a quiet milestone hint instead of a chip, so the list stays calm.
        (None, false) if resp.hovered() => {
            ui.painter().text(
                rect.right_center() - vec2(10.0, 0.0),
                Align2::RIGHT_CENTER,
                format!("Planned · {}", g.availability),
                theme::medium(10.5),
                t.text_faint,
            );
        }
        _ => {}
    }
    resp.on_hover_text(if data::is_ready(g.availability) {
        "Available".to_string()
    } else if g.availability == "ai" {
        "Optional: needs an AI provider you configure".to_owned()
    } else {
        format!("Planned for milestone {} — open to see what it will include", g.availability)
    })
}

fn tool_detail(app: &mut App, ui: &mut egui::Ui, t: &Tokens, g: &'static ToolGroup) {
    let (back, close) = panel_header(ui, t, g.label, None, true);
    if back {
        app.left = LeftPanel::AllTools;
    }
    if close {
        app.left_open = false;
    }
    if !data::is_ready(g.availability) && g.availability != "ai" {
        egui::Frame::NONE.fill(t.accent_soft).corner_radius(CornerRadius::same(8)).inner_margin(egui::Margin::same(10)).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(
                egui::RichText::new(format!("Coming in milestone {}. Items marked Ready work today.", g.availability))
                    .color(t.text)
                    .font(theme::regular(12.0)),
            );
        });
    }
    let mut run = None;
    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        for s in g.sections {
            widgets::section_title(ui, s.title);
            for item in s.items {
                let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 34.0), Sense::click());
                resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, item.label));
                let ready = data::is_ready(item.availability);
                if resp.hovered() {
                    ui.painter().rect_filled(rect, CornerRadius::same(6), t.hover);
                }
                let fg = if ready { t.text } else { t.text_muted };
                icons::paint(
                    ui,
                    Rect::from_min_size(rect.min + vec2(6.0, 8.0), vec2(18.0, 18.0)),
                    item.icon,
                    17.0,
                    if ready { hue(g) } else { t.text_faint },
                );
                ui.painter().text(rect.left_center() + vec2(34.0, 0.0), Align2::LEFT_CENTER, item.label, theme::regular(13.0), fg);
                let (chip, fill, cfg) = if ready {
                    ("Ready", t.ready_bg, t.ready_fg)
                } else if item.availability == "ai" {
                    ("AI", t.pressed, t.text_muted)
                } else {
                    (item.availability, t.pressed, t.text_muted)
                };
                let font = theme::semibold(9.5);
                let w = ui.fonts_mut(|f| f.layout_no_wrap(chip.to_string(), font.clone(), cfg).size().x);
                let r = Rect::from_center_size(rect.right_center() - vec2(w / 2.0 + 10.0, 0.0), vec2(w + 10.0, 16.0));
                ui.painter().rect_filled(r, CornerRadius::same(4), fill);
                ui.painter().text(r.center(), Align2::CENTER_CENTER, chip, font, cfg);
                if resp.on_hover_text(item.command).clicked() {
                    run = Some(item.command);
                }
            }
        }
    });
    if let Some(cmd) = run {
        app.run_command(cmd);
    }
}

// ── Right side panel ──────────────────────────────────────────────────────────────────────

pub fn right_panel(app: &mut App, ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    let Some(panel) = app.panel else { return };
    let title = match panel {
        Panel::Comments => "Comments",
        Panel::Bookmarks => "Bookmarks",
        Panel::Pages => "Page Thumbnails",
        Panel::Fields => "Form Fields",
        Panel::Layers => "Layers",
        Panel::Attachments => "Attachments",
        Panel::Signatures => "Signatures",
        Panel::Search => "Advanced Search",
    };
    // The comments panel shows its thread count next to the title, as PrintCraft does.
    let count = (panel == Panel::Comments).then(|| app.comments.iter().map(|c| 1 + c.replies.len()).sum());
    egui::Panel::right("side_panel")
        .resizable(false)
        .exact_size(330.0)
        .frame(
            egui::Frame::NONE
                .fill(t.panel)
                .inner_margin(egui::Margin { left: 14, right: 12, top: 12, bottom: 8 })
                .stroke(Stroke::new(1.0, t.divider)),
        )
        .show(ui, |ui| {
            let (_, close) = panel_header(ui, &t, title, count, false);
            if close {
                app.panel = None;
                return;
            }
            match panel {
                Panel::Comments => comments(app, ui, &t),
                Panel::Bookmarks => bookmarks(app, ui, &t),
                Panel::Pages => thumbnails(app, ui, &t),
                Panel::Fields => fields(app, ui, &t),
                Panel::Layers => layers(app, ui, &t),
                Panel::Attachments => attachments(app, ui, &t),
                Panel::Signatures => signatures(app, ui, &t),
                Panel::Search => search_results(app, ui, &t),
            }
        });
}

fn comments(app: &mut App, ui: &mut egui::Ui, t: &Tokens) {
    let mut select = None;
    let mut resolve = None;
    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        let mut last_page = usize::MAX;
        for (i, c) in app.comments.iter().enumerate() {
            if c.page != last_page {
                last_page = c.page;
                let on_page = app.comments.iter().filter(|x| x.page == c.page).count();
                ui.add_space(2.0);
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(format!("Page {} · {}", app.page_label(c.page), app.pages.get(c.page).map_or("", |p| p.nav)))
                            .color(t.text_faint)
                            .font(theme::regular(11.0)),
                    );
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(egui::RichText::new(on_page.to_string()).color(t.text_faint).font(theme::regular(11.0)));
                    });
                });
            }
            let selected = app.selected_comment == Some(i);
            // The card is laid out from a child UI so its height follows the content.
            let card = egui::Frame::new()
                .fill(t.card)
                .stroke(Stroke::new(1.0, if selected { t.accent } else { t.border }))
                .corner_radius(CornerRadius::same(8))
                .inner_margin(egui::Margin::same(9))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal(|ui| {
                        avatar(ui, t, c.author);
                        ui.label(egui::RichText::new(c.author).font(theme::medium(12.5)).color(t.text));
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.label(egui::RichText::new(c.time).color(t.text_faint).font(theme::regular(11.0)));
                        });
                    });
                    ui.add_space(4.0);
                    ui.label(egui::RichText::new(c.body).color(t.text).font(theme::regular(12.5)));
                    ui.horizontal(|ui| {
                        icons::paint(
                            ui,
                            Rect::from_min_size(ui.cursor().min + vec2(0.0, 3.0), vec2(12.0, 12.0)),
                            "message-square-quote",
                            11.0,
                            t.text_faint,
                        );
                        ui.add_space(12.0);
                        ui.label(egui::RichText::new(c.tool).color(t.text_faint).font(theme::regular(11.0)));
                        if let Some(state) = c.state {
                            ui.add_space(4.0);
                            widgets::chip(ui, state, t.accent_soft, t.accent_text);
                        }
                    });
                    // Replies hang off the parent on a thread line (comments_panel.rs::card).
                    for r in &c.replies {
                        ui.add_space(6.0);
                        let row = ui.horizontal(|ui| {
                            ui.add_space(24.0);
                            avatar(ui, t, &r.author);
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new(&r.author).font(theme::medium(11.5)).color(t.text));
                                    ui.label(egui::RichText::new(&r.time).color(t.text_faint).font(theme::regular(10.5)));
                                    if r.status {
                                        widgets::chip(ui, "Marked", t.ready_bg, t.ready_fg);
                                    }
                                });
                                ui.label(egui::RichText::new(&r.body).color(t.text_muted).font(theme::regular(11.5)));
                            });
                        });
                        let x = row.response.rect.left() + 36.0;
                        ui.painter().vline(x, (row.response.rect.top() - 12.0)..=(row.response.rect.top() - 2.0), Stroke::new(1.5, t.border));
                    }
                })
                .response;
            let card_rect = card.rect;
            // The selected card carries a 3 px accent bar flush with its left edge — drawn from the
            // frame's own rect, never from the inner UI (whose max_rect is the whole panel).
            if selected {
                ui.painter().rect_filled(
                    Rect::from_min_size(card_rect.min, vec2(3.0, card_rect.height())),
                    CornerRadius { nw: 8, sw: 8, ne: 0, se: 0 },
                    Color32::from_rgb(0x01, 0x65, 0xDD),
                );
            }
            let hit = ui.interact(card.rect.expand(4.0), ui.id().with(("comment", i)), Sense::click());
            if hit.clicked() {
                select = Some(i);
            }
            if hit.double_clicked() {
                resolve = Some(i);
            }
            ui.add_space(6.0);
        }
    });
    if let Some(i) = select {
        app.selected_comment = Some(i);
        app.page = app.comments[i].page;
    }
    if let Some(i) = resolve {
        let resolved = app.comments[i].state == Some("Resolved");
        app.comments[i].state = if resolved { None } else { Some("Resolved") };
        app.notify(if resolved { "Comment reopened" } else { "Comment marked resolved" });
    }
    // Reply box for the selected thread (kept outside the card list to avoid nested borrows).
    if let Some(i) = app.selected_comment {
        ui.add_space(4.0);
        ui.separator();
        ui.label(egui::RichText::new(format!("Reply to {}", app.comments[i].author)).color(t.text_faint).font(theme::regular(11.0)));
        let mut draft = std::mem::take(&mut app.reply_draft);
        ui.horizontal(|ui| {
            let r = widgets::text_field(ui, egui::Id::new("reply"), "Reply…", &mut draft, ui.available_width() - 78.0);
            let send = (r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) || widgets::pill_button(ui, "Reply", true).clicked();
            if send && !draft.trim().is_empty() {
                let body = draft.trim().to_owned();
                app.comments[i].replies.push(crate::app::Reply { author: "You".to_owned(), time: "just now".to_owned(), body, status: false });
                draft.clear();
                app.notify("Reply added to the thread");
            }
        });
        app.reply_draft = draft;
    }
}

fn avatar(ui: &mut egui::Ui, t: &Tokens, name: &str) {
    let initials: String = name.split_whitespace().filter_map(|w| w.chars().next()).take(2).collect();
    let (rect, _) = ui.allocate_exact_size(vec2(24.0, 24.0), Sense::hover());
    ui.painter().circle_filled(rect.center(), 12.0, t.accent_soft);
    ui.painter().text(rect.center(), Align2::CENTER_CENTER, initials, theme::semibold(10.5), t.accent_text);
}

fn bookmarks(app: &mut App, ui: &mut egui::Ui, t: &Tokens) {
    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        for (i, b) in data::BOOKMARKS.iter().enumerate() {
            if bookmark_row(app, ui, t, b, i, 0) {
                app.expanded[i] = !app.expanded[i];
            }
        }
    });
}

fn bookmark_row(app: &mut App, ui: &mut egui::Ui, t: &Tokens, b: &data::Bookmark, index: usize, depth: usize) -> bool {
    let mut toggled = false;
    let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 28.0), Sense::click());
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, b.label));
    if resp.hovered() {
        ui.painter().rect_filled(rect, CornerRadius::same(5), t.hover);
    }
    let x = rect.left() + 4.0 + depth as f32 * 14.0;
    if b.children.is_empty() {
        icons::paint(ui, Rect::from_min_size(egui::pos2(x, rect.top() + 7.0), vec2(14.0, 14.0)), "file-text", 13.0, t.text_faint);
    } else {
        let icon = if app.expanded[index] { "chevron-down" } else { "chevron-right" };
        icons::paint(ui, Rect::from_min_size(egui::pos2(x, rect.top() + 7.0), vec2(14.0, 14.0)), icon, 13.0, t.text_muted);
    }
    ui.painter().text(rect.left_center() + vec2(24.0 + depth as f32 * 14.0, 0.0), Align2::LEFT_CENTER, b.label, theme::regular(13.0), t.text);
    ui.painter().text(rect.right_center() - vec2(6.0, 0.0), Align2::RIGHT_CENTER, app.page_label(b.page), theme::regular(11.0), t.text_faint);
    if resp.clicked() {
        app.page = b.page;
        if !b.children.is_empty() {
            toggled = true;
            app.notify(format!("{} — page {}", b.label, app.page_label(b.page)));
        }
    }
    if app.expanded[index] {
        for (j, child) in b.children.iter().enumerate() {
            bookmark_row(app, ui, t, child, index + j + 1, depth + 1);
        }
    }
    toggled
}

fn thumbnails(app: &mut App, ui: &mut egui::Ui, t: &Tokens) {
    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        let cols = 2;
        let gap = 10.0;
        let w = (ui.available_width() - gap * (cols as f32 - 1.0)) / cols as f32;
        let h = w * 11.0 / 8.5;
        let count = app.pages.len();
        for row in 0..count.div_ceil(cols) {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = gap;
                for col in 0..cols {
                    let i = row * cols + col;
                    if i >= count {
                        break;
                    }
                    let selected = app.page == i;
                    ui.vertical(|ui| {
                        let (rect, resp) = ui.allocate_exact_size(vec2(w, h), Sense::click());
                        resp.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Button, true, selected, &app.pages[i].label));
                        let shadow = rect.translate(vec2(0.0, 1.0));
                        ui.painter().rect_filled(shadow, CornerRadius::same(3), t.page_shadow);
                        ui.painter().rect(
                            rect,
                            CornerRadius::same(3),
                            Color32::WHITE,
                            Stroke::new(1.0, if selected { t.accent } else { t.border }),
                            egui::StrokeKind::Inside,
                        );
                        let rot = app.pages[i].rot;
                        mini_page(ui, rect.shrink(4.0), t, i, rot);
                        if resp.clicked() {
                            app.page = i;
                        }
                        ui.label(
                            egui::RichText::new(format!("{} · {}", app.pages[i].label, app.pages[i].nav))
                                .color(t.text_muted)
                                .font(theme::regular(11.0)),
                        );
                    });
                }
            });
        }
    });
}

/// A tiny wireframe of a page's content, so thumbnails read like the real thing.
fn mini_page(ui: &egui::Ui, rect: Rect, t: &Tokens, index: usize, rot: i32) {
    let p = ui.painter();
    let ink = Color32::from_gray(0x9A);
    let mut y = rect.top() + 6.0;
    let line = |y: f32, w: f32| Rect::from_min_size(egui::pos2(rect.left() + 6.0, y), vec2(rect.width() - 12.0 * (1.0 - w), 2.0));
    for block in data::page_blocks(index).iter().take(6) {
        match block {
            data::Block::Kicker(_) => {
                p.rect_filled(line(y, 0.4), CornerRadius::same(1), ink.linear_multiply(0.6));
                y += 6.0;
            }
            data::Block::H1(_) | data::Block::H2(_) => {
                p.rect_filled(line(y, 0.9), CornerRadius::same(1), Color32::from_gray(0x44));
                y += 7.0;
            }
            data::Block::Swatches(sw) => {
                let n = sw.len() as f32;
                let cw = (rect.width() - 12.0 - (n - 1.0) * 2.0) / n;
                for (i, (a, _)) in sw.iter().enumerate() {
                    let c = parse_hex(a);
                    let r = Rect::from_min_size(egui::pos2(rect.left() + 6.0 + i as f32 * (cw + 2.0), y), vec2(cw, 10.0));
                    p.rect_filled(r, CornerRadius::same(1), c);
                }
                y += 14.0;
            }
            data::Block::Code(_) => {
                p.rect_filled(Rect::from_min_size(egui::pos2(rect.left() + 6.0, y), vec2(rect.width() - 12.0, 16.0)), CornerRadius::same(2), t.hover);
                y += 20.0;
            }
            data::Block::Figure(a, _) => {
                p.rect_filled(
                    Rect::from_min_size(egui::pos2(rect.left() + 6.0, y), vec2(rect.width() - 12.0, 14.0)),
                    CornerRadius::same(2),
                    parse_hex(a),
                );
                y += 18.0;
            }
            _ => {
                for _ in 0..3 {
                    p.rect_filled(line(y, 1.0), CornerRadius::same(1), ink.linear_multiply(0.45));
                    y += 5.0;
                }
                y += 2.0;
            }
        }
        if y > rect.bottom() - 8.0 {
            break;
        }
    }
    if rot != 0 {
        p.text(rect.right_top() + vec2(-4.0, 6.0), Align2::RIGHT_TOP, format!("{rot}°"), theme::regular(9.0), t.accent_text);
    }
}

fn fields(app: &mut App, ui: &mut egui::Ui, t: &Tokens) {
    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        for f in data::FIELDS {
            let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 34.0), Sense::click());
            resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, f.label));
            if resp.hovered() {
                ui.painter().rect_filled(rect, CornerRadius::same(5), t.hover);
            }
            icons::paint(ui, Rect::from_min_size(rect.min + vec2(6.0, 9.0), vec2(16.0, 16.0)), "text-cursor-input", 15.0, t.accent_text);
            ui.painter().text(rect.left_center() + vec2(30.0, -6.0), Align2::LEFT_CENTER, f.label, theme::regular(13.0), t.text);
            ui.painter().text(
                rect.left_center() + vec2(30.0, 8.0),
                Align2::LEFT_CENTER,
                format!("{} · {}", f.kind, f.value),
                theme::regular(10.5),
                t.text_faint,
            );
            ui.painter().text(rect.right_center() - vec2(8.0, 0.0), Align2::RIGHT_CENTER, f.name, theme::mono(10.5), t.text_faint);
            if resp.clicked() {
                app.page = f.page;
                app.notify(format!("Field “{}” on page {}", f.name, app.page_label(f.page)));
            }
        }
        widgets::section_title(ui, "Tab order");
        ui.label(egui::RichText::new("Rows · left to right, top to bottom").color(t.text_muted).font(theme::regular(12.0)));
        ui.horizontal(|ui| {
            if widgets::pill_button(ui, "Preview", false).clicked() {
                app.notify("Form preview — demo stub");
            }
            if widgets::pill_button(ui, "Flatten", false).clicked() {
                app.notify("Flattened form fields — demo stub");
            }
        });
    });
}

fn layers(app: &mut App, ui: &mut egui::Ui, t: &Tokens) {
    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        for (name, visible, locked) in data::LAYERS {
            let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 30.0), Sense::click());
            resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, *name));
            if resp.hovered() {
                ui.painter().rect_filled(rect, CornerRadius::same(5), t.hover);
            }
            icons::paint(
                ui,
                Rect::from_min_size(rect.min + vec2(6.0, 8.0), vec2(14.0, 14.0)),
                if *visible { "eye" } else { "eye-off" },
                13.0,
                if *visible { t.icon } else { t.text_faint },
            );
            icons::paint(
                ui,
                Rect::from_min_size(rect.min + vec2(26.0, 8.0), vec2(14.0, 14.0)),
                if *locked { "lock" } else { "lock-open" },
                13.0,
                t.text_faint,
            );
            let fg = if *visible { t.text } else { t.text_faint };
            ui.painter().text(rect.left_center() + vec2(48.0, 0.0), Align2::LEFT_CENTER, *name, theme::regular(13.0), fg);
            if resp.clicked() {
                app.notify(format!("{name} — layer visibility is a demo stub"));
            }
        }
    });
}

fn attachments(app: &mut App, ui: &mut egui::Ui, t: &Tokens) {
    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        for (name, size, desc) in data::ATTACHMENTS {
            let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 40.0), Sense::click());
            resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, *name));
            if resp.hovered() {
                ui.painter().rect_filled(rect, CornerRadius::same(5), t.hover);
            }
            icons::paint(ui, Rect::from_min_size(rect.min + vec2(6.0, 11.0), vec2(18.0, 18.0)), "paperclip", 17.0, t.accent_text);
            ui.painter().text(rect.left_center() + vec2(32.0, -7.0), Align2::LEFT_CENTER, *name, theme::medium(12.5), t.text);
            ui.painter().text(rect.left_center() + vec2(32.0, 8.0), Align2::LEFT_CENTER, *desc, theme::regular(11.0), t.text_faint);
            ui.painter().text(rect.right_center() - vec2(8.0, 0.0), Align2::RIGHT_CENTER, *size, theme::regular(11.0), t.text_muted);
            if resp.clicked() {
                app.notify(format!("Saved {name} — demo stub"));
            }
        }
    });
}

fn signatures(app: &mut App, ui: &mut egui::Ui, t: &Tokens) {
    for (signer, when, reason, issuer) in data::SIGNATURES {
        widgets::card_frame(t).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                icons::paint(ui, Rect::from_min_size(ui.cursor().min + vec2(0.0, 2.0), vec2(20.0, 20.0)), "badge-check", 18.0, t.ready_fg);
                ui.add_space(26.0);
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new(*signer).font(theme::semibold(13.5)).color(t.text));
                    ui.label(egui::RichText::new(*when).color(t.text_faint).font(theme::regular(11.0)));
                });
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    widgets::chip(ui, "Valid", t.ready_bg, t.ready_fg);
                });
            });
            ui.add_space(8.0);
            ui.label(egui::RichText::new(format!("Reason · {reason}")).color(t.text_muted).font(theme::regular(12.0)));
            ui.label(egui::RichText::new(format!("Issuer · {issuer}")).color(t.text_muted).font(theme::regular(12.0)));
            ui.add_space(6.0);
            if widgets::pill_button(ui, "Validate all signatures", false).clicked() {
                app.notify("All signatures valid — demo stub");
            }
        });
    }
}

fn search_results(app: &mut App, ui: &mut egui::Ui, t: &Tokens) {
    let mut query = app.find_query.clone();
    let r = widgets::text_field(ui, egui::Id::new("panel-search"), "Search the document", &mut query, ui.available_width());
    if r.changed() {
        app.find_query = query.clone();
        app.find_index = 0;
    }
    ui.add_space(6.0);
    ui.label(egui::RichText::new(format!("{} results for “type”", data::SEARCH_HITS.len())).color(t.text_muted).font(theme::regular(11.5)));
    let mut go = None;
    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        let mut last_page = usize::MAX;
        for (i, hit) in data::SEARCH_HITS.iter().enumerate() {
            if hit.page != last_page {
                last_page = hit.page;
                ui.label(
                    egui::RichText::new(format!("Page {} · {}", app.page_label(hit.page), app.pages.get(hit.page).map_or("", |p| p.nav)))
                        .color(t.text_faint)
                        .font(theme::regular(11.0)),
                );
                ui.add_space(2.0);
            }
            let selected = app.find_index == i;
            let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 26.0), Sense::click());
            if selected || resp.hovered() {
                ui.painter().rect_filled(rect, CornerRadius::same(5), if selected { t.accent_soft } else { t.hover });
            }
            let font = theme::regular(12.5);
            let before = ui.fonts_mut(|f| f.layout_no_wrap(hit.before.to_owned(), font.clone(), t.text_muted));
            ui.painter().galley(rect.left_center() + vec2(6.0, -before.size().y / 2.0), before.clone(), t.text_muted);
            let x = rect.left() + 6.0 + before.size().x;
            let hitg = ui.fonts_mut(|f| f.layout_no_wrap("type".to_owned(), font.clone(), t.text));
            ui.painter().rect_filled(
                Rect::from_min_size(egui::pos2(x, rect.center().y - hitg.size().y / 2.0), hitg.size()),
                CornerRadius::same(2),
                Color32::from_rgba_unmultiplied(0xFF, 0xD6, 0x00, 90),
            );
            ui.painter().galley(egui::pos2(x, rect.center().y - hitg.size().y / 2.0), hitg.clone(), t.text);
            let after = ui.fonts_mut(|f| f.layout_no_wrap(hit.after.to_owned(), font.clone(), t.text_muted));
            ui.painter().galley(egui::pos2(x + hitg.size().x, rect.center().y - after.size().y / 2.0), after, t.text_muted);
            if resp.clicked() {
                go = Some(i);
            }
        }
    });
    if let Some(i) = go {
        app.find_index = i;
        app.page = data::SEARCH_HITS[i].page;
    }
}

fn parse_hex(s: &str) -> Color32 {
    let s = s.trim_start_matches('#');
    if s.len() >= 6 {
        let v = u32::from_str_radix(&s[0..6], 16).unwrap_or(0);
        Color32::from_rgb(((v >> 16) & 0xFF) as u8, ((v >> 8) & 0xFF) as u8, (v & 0xFF) as u8)
    } else {
        Color32::GRAY
    }
}
