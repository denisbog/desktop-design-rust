//! Document area: notice bar, page stack, Organize-pages grid, the floating quick-action
//! bar and the floating find bar.
//!
//! Port of `crates/ui-egui/src/canvas.rs` (the document area half).

use egui::{Align2, Color32, CornerRadius, Pos2, Rect, Sense, Stroke, pos2, vec2};

use crate::app::App;
use crate::data::{self, Block};
use crate::icons;
use crate::theme::{self, Tokens};
use crate::widgets;

const PAGE_W: f32 = 816.0;
const PAGE_H: f32 = 1056.0;
const PAD_X: f32 = 92.0;
const PAD_TOP: f32 = 84.0;

pub fn document_area(app: &mut App, ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    let area = ui.max_rect();

    // Notice bar above the pages (canvas.rs::notices).
    if app.notice && app.active {
        egui::Frame::NONE.fill(t.accent_soft).inner_margin(egui::Margin::symmetric(14, 7)).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                icons::paint(ui, Rect::from_min_size(ui.cursor().min + vec2(0.0, 1.0), vec2(18.0, 18.0)), "info", 16.0, t.accent_text);
                ui.add_space(24.0);
                ui.label(
                    egui::RichText::new("Repaired 3 damaged objects while opening. The original file is untouched.")
                        .color(t.text)
                        .font(theme::regular(12.5)),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if icons::button(ui, "x", 24.0, false, "Dismiss").clicked() {
                        app.notice = false;
                    }
                    if widgets::link(ui, "Details").clicked() {
                        app.notify("Recovery report — demo stub");
                    }
                });
            });
        });
    }

    if app.organize {
        organize(app, ui, &t);
    } else {
        pages(app, ui, &t);
    }

    // The floating bars sit just below the notice bar when it is showing.
    let top = if app.notice { 42.0 } else { 14.0 };
    if !app.organize && app.mode != crate::app::Mode::Read {
        quick_bar(app, ui, &t, area, top);
    }
    if app.find_open {
        find_bar(app, ui, &t, area, if app.notice { 40.0 } else { 12.0 });
    }
}

// ── Page stack ────────────────────────────────────────────────────────────────────────────

fn pages(app: &mut App, ui: &mut egui::Ui, t: &Tokens) {
    let zoom = app.zoom;
    let w = PAGE_W * zoom;
    let h = PAGE_H * zoom;
    let count = app.pages.len();
    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        ui.add_space(28.0);
        let side = ((ui.available_width() - w) / 2.0).max(20.0);
        for i in 0..count {
            ui.horizontal(|ui| {
                ui.add_space(side);
                ui.vertical(|ui| {
                    let (rect, resp) = ui.allocate_exact_size(vec2(w, h), Sense::click());
                    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Image, true, &app.pages[i].label));
                    // A page is a white card on the pasteboard.
                    ui.painter().rect_filled(rect.translate(vec2(0.0, 2.0)), CornerRadius::same(2), t.page_shadow);
                    ui.painter().rect_filled(rect, CornerRadius::same(2), Color32::WHITE);
                    let rot = app.pages[i].rot;
                    let inner = if rot == 90 || rot == 270 { Rect::from_center_size(rect.center(), vec2(h, w)) } else { rect };
                    draw_page(ui, inner, i, zoom, rot, t);
                    if resp.clicked() {
                        app.page = i;
                    }
                    if resp.double_clicked() {
                        app.mode = crate::app::Mode::Read;
                    }
                    ui.add_space(14.0);
                });
            });
        }
        ui.add_space(60.0);
    });
}

/// Draw one page's specimen content with the painter, scaled by `zoom`.
fn draw_page(ui: &mut egui::Ui, rect: Rect, index: usize, zoom: f32, rot: i32, t: &Tokens) {
    let p = ui.painter().clone();
    let clip = rect.intersect(ui.clip_rect());
    let p = p.with_clip_rect(clip);
    let left = rect.left() + PAD_X * zoom;
    let right = rect.right() - PAD_X * zoom;
    let content_w = (right - left).max(40.0);
    let mut y = rect.top() + PAD_TOP * zoom;
    let ink = Color32::from_rgb(0x1A, 0x1A, 0x1E);
    let muted = Color32::from_rgb(0x6B, 0x6B, 0x73);
    let mono_gray = theme::mono(12.5 * zoom);
    let body_font = theme::regular(15.0 * zoom);

    if rot != 0 {
        // Rotated pages are shown as a labelled placeholder, as the demo's CSS `rotate` does.
        p.text(rect.center(), Align2::CENTER_CENTER, format!("Page {} · rotated {rot}°", index + 1), theme::regular(16.0 * zoom), t.text_faint);
        return;
    }

    for block in data::page_blocks(index) {
        match block {
            Block::Kicker(text) => {
                y += wrapped(&p, ui, pos2(left, y), text, theme::semibold(11.0 * zoom), muted, content_w);
                y += 12.0 * zoom;
            }
            Block::H1(text) => {
                for line in text.split('\n') {
                    y += wrapped(&p, ui, pos2(left, y), line, theme::semibold(40.0 * zoom), ink, content_w);
                }
                y += 10.0 * zoom;
            }
            Block::H2(text) => {
                y += wrapped(&p, ui, pos2(left, y), text, theme::semibold(24.0 * zoom), ink, content_w);
                y += 14.0 * zoom;
            }
            Block::Lede(text) => {
                y += wrapped(&p, ui, pos2(left, y), text, theme::regular(15.0 * zoom), ink, content_w);
                y += 14.0 * zoom;
            }
            Block::Body(text) => {
                for para in text.split('\n') {
                    y += wrapped(&p, ui, pos2(left, y), para, body_font.clone(), ink, content_w);
                    y += 4.0 * zoom;
                }
                y += 12.0 * zoom;
            }
            Block::Quote(text) => {
                let h = wrapped(&p, ui, pos2(left + 16.0 * zoom, y + 4.0 * zoom), text, theme::regular(19.0 * zoom), ink, content_w - 20.0 * zoom);
                p.rect_filled(Rect::from_min_size(pos2(left, y), vec2(3.0, h + 8.0 * zoom)), CornerRadius::ZERO, Color32::from_rgb(0x1B, 0x63, 0xE0));
                y += h + 26.0 * zoom;
            }
            Block::Specimen(text, display) => {
                let font = if display { theme::display(44.0 * zoom) } else { theme::regular(30.0 * zoom) };
                for line in text.split('\n') {
                    y += wrapped(&p, ui, pos2(left, y), line, font.clone(), ink, content_w);
                }
                y += 20.0 * zoom;
            }
            Block::Code(text) => {
                let font = mono_gray.clone();
                let h = text.lines().count() as f32 * 19.0 * zoom + 28.0 * zoom;
                p.rect_filled(Rect::from_min_size(pos2(left, y), vec2(content_w, h)), CornerRadius::same(6), Color32::from_rgb(0xF4, 0xF4, 0xF6));
                p.rect_stroke(
                    Rect::from_min_size(pos2(left, y), vec2(content_w, h)),
                    CornerRadius::same(6),
                    Stroke::new(1.0, Color32::from_rgb(0xE3, 0xE3, 0xE7)),
                    egui::StrokeKind::Inside,
                );
                let mut cy = y + 14.0 * zoom;
                for line in text.lines() {
                    p.text(pos2(left + 14.0 * zoom, cy), Align2::LEFT_TOP, line, font.clone(), ink);
                    cy += 19.0 * zoom;
                }
                y += h + 18.0 * zoom;
            }
            Block::Swatches(list) => {
                let n = list.len() as f32;
                let gap = 8.0 * zoom;
                let cw = (content_w - gap * (n - 1.0)) / n;
                for (i, (a, b)) in list.iter().enumerate() {
                    let r = Rect::from_min_size(pos2(left + i as f32 * (cw + gap), y), vec2(cw, 54.0 * zoom));
                    gradient(&p, r, parse_hex(a), parse_hex(b));
                }
                y += 74.0 * zoom;
            }
            Block::Figure(color, caption) => {
                let r = Rect::from_min_size(pos2(left, y), vec2(content_w, 180.0 * zoom));
                gradient(&p, r, parse_hex(color), Color32::from_rgb(0x19, 0x19, 0x1C));
                y += 186.0 * zoom;
                y += wrapped(&p, ui, pos2(left, y), caption, theme::regular(11.5 * zoom), muted, content_w);
                y += 18.0 * zoom;
            }
            Block::Columns(a, b) => {
                let gapp = 28.0 * zoom;
                let cw = (content_w - gapp) / 2.0;
                let ha = wrapped(&p, ui, pos2(left, y), a, body_font.clone(), ink, cw);
                let hb = wrapped(&p, ui, pos2(left + cw + gapp, y), b, body_font.clone(), ink, cw);
                y += ha.max(hb) + 18.0 * zoom;
            }
            Block::Footer(name, label) => {
                let fy = rect.bottom() - 48.0 * zoom;
                p.text(pos2(left, fy), Align2::LEFT_TOP, name, theme::regular(11.0 * zoom), Color32::from_rgb(0x8A, 0x8A, 0x92));
                p.text(pos2(right, fy), Align2::RIGHT_TOP, label, theme::regular(11.0 * zoom), Color32::from_rgb(0x8A, 0x8A, 0x92));
            }
        }
        if y > rect.bottom() {
            break;
        }
    }
}

/// Draw wrapped text and return its height.
fn wrapped(p: &egui::Painter, ui: &egui::Ui, pos: Pos2, text: &str, font: egui::FontId, color: Color32, wrap: f32) -> f32 {
    let galley = ui.fonts_mut(|f| f.layout(text.to_owned(), font, color, wrap));
    let h = galley.size().y;
    p.galley(pos, galley, color);
    h
}

fn gradient(p: &egui::Painter, rect: Rect, a: Color32, b: Color32) {
    let steps = 24;
    for i in 0..steps {
        let t0 = i as f32 / steps as f32;
        let t1 = (i + 1) as f32 / steps as f32;
        let c = lerp_color(a, b, (t0 + t1) / 2.0);
        let r = Rect::from_min_max(pos2(rect.left() + rect.width() * t0, rect.top()), pos2(rect.left() + rect.width() * t1 + 0.5, rect.bottom()));
        p.rect_filled(r, CornerRadius::ZERO, c);
    }
}

fn lerp_color(a: Color32, b: Color32, t: f32) -> Color32 {
    let l = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t).round() as u8;
    Color32::from_rgb(l(a.r(), b.r()), l(a.g(), b.g()), l(a.b(), b.b()))
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

// ── Organize pages grid (canvas.rs::organize_grid) ─────────────────────────────────────────

fn organize(app: &mut App, ui: &mut egui::Ui, t: &Tokens) {
    egui::Frame::NONE.fill(t.panel).inner_margin(egui::Margin::symmetric(12, 8)).stroke(Stroke::new(1.0, t.divider)).show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(format!("{} pages", app.pages.len())).font(theme::semibold(13.5)).color(t.text));
            ui.add_space(6.0);
            ui.separator();
            for (icon, tip) in [
                ("rotate-cw", "Rotate selected pages"),
                ("trash-2", "Delete selected pages"),
                ("file-plus-2", "Insert a blank page"),
                ("file-output", "Extract selected pages"),
                ("scissors", "Split document"),
            ] {
                if icons::button(ui, icon, 30.0, false, tip).clicked() {
                    match icon {
                        "rotate-cw" => app.rotate_selection(),
                        "trash-2" => app.delete_selection(),
                        "file-plus-2" => app.insert_blank(),
                        _ => app.notify(format!("{tip} — demo stub")),
                    }
                }
            }
            ui.add_space(6.0);
            let n = app.organize_sel.len();
            let text =
                if n == 0 { "No selection — click pages, ⌘-click to add, ⇧-click for a range".to_owned() } else { format!("{n} selected") };
            ui.label(egui::RichText::new(text).color(t.text_muted).font(theme::regular(11.5)));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if widgets::pill_button(ui, "Done", true).clicked() {
                    app.organize = false;
                    app.organize_sel.clear();
                }
                if widgets::pill_button(ui, "Select all", false).clicked() {
                    app.organize_sel = (0..app.pages.len()).collect();
                }
            });
        });
    });

    let mut open = None;
    let mut new_sel = app.organize_sel.clone();
    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        let avail = ui.available_width();
        let cols = ((avail - 52.0) / 168.0).floor().max(1.0) as usize;
        let gap = 18.0;
        let w = (avail - 52.0 - gap * (cols as f32 - 1.0)) / cols as f32;
        let h = w * 11.0 / 8.5;
        let count = app.pages.len();
        ui.add_space(18.0);
        for row in 0..count.div_ceil(cols) {
            ui.horizontal(|ui| {
                ui.add_space(26.0);
                ui.spacing_mut().item_spacing.x = gap;
                for col in 0..cols {
                    let i = row * cols + col;
                    if i >= count {
                        break;
                    }
                    let selected = app.organize_sel.contains(&i);
                    ui.vertical(|ui| {
                        let (rect, resp) = ui.allocate_exact_size(vec2(w, h), Sense::click());
                        resp.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Button, true, selected, &app.pages[i].label));
                        ui.painter().rect_filled(rect.translate(vec2(0.0, 1.0)), CornerRadius::same(3), t.page_shadow);
                        ui.painter().rect(
                            rect,
                            CornerRadius::same(3),
                            Color32::WHITE,
                            Stroke::new(1.0, if selected { t.accent } else { t.border }),
                            egui::StrokeKind::Inside,
                        );
                        if selected {
                            ui.painter().rect_stroke(rect.expand(2.0), CornerRadius::same(4), Stroke::new(2.0, t.accent), egui::StrokeKind::Outside);
                        }
                        let rot = app.pages[i].rot;
                        mini_page(ui, rect.shrink(4.0), t, i, rot);
                        ui.painter().text(
                            rect.left_bottom() + vec2(6.0, -6.0),
                            Align2::LEFT_BOTTOM,
                            app.pages[i].label.clone(),
                            theme::regular(11.0),
                            Color32::from_rgb(0x8A, 0x8A, 0x92),
                        );
                        ui.label(egui::RichText::new(app.pages[i].nav).color(t.text_muted).font(theme::regular(11.5)));
                        if resp.clicked() {
                            let mods = ui.input(|inp| inp.modifiers);
                            if mods.command {
                                if !new_sel.insert(i) {
                                    new_sel.remove(&i);
                                }
                            } else if mods.shift && !new_sel.is_empty() {
                                let lo = *new_sel.iter().next().unwrap_or(&i).min(&i);
                                let hi = *new_sel.iter().next_back().unwrap_or(&i).max(&i);
                                for k in lo..=hi {
                                    new_sel.insert(k);
                                }
                            } else {
                                new_sel.clear();
                                new_sel.insert(i);
                            }
                        }
                        if resp.double_clicked() {
                            open = Some(i);
                        }
                    });
                }
            });
            ui.add_space(14.0);
        }
        ui.add_space(40.0);
    });
    app.organize_sel = new_sel;
    if let Some(i) = open {
        app.page = i;
        app.organize = false;
    }
}

/// A tiny wireframe of a page's content, so thumbnails read like the real thing.
fn mini_page(ui: &egui::Ui, rect: Rect, t: &Tokens, index: usize, rot: i32) {
    if rot != 0 {
        ui.painter().text(rect.center(), Align2::CENTER_CENTER, format!("{rot}°"), theme::medium(14.0), t.accent_text);
        return;
    }
    let p = ui.painter();
    let ink = Color32::from_gray(0x9A);
    let mut y = rect.top() + 6.0;
    let line = |y: f32, w: f32| Rect::from_min_size(pos2(rect.left() + 6.0, y), vec2((rect.width() - 12.0) * w, 2.0));
    for block in data::page_blocks(index).iter().take(6) {
        match block {
            Block::Kicker(_) => {
                p.rect_filled(line(y, 0.4), CornerRadius::same(1), ink.linear_multiply(0.6));
                y += 6.0;
            }
            Block::H1(_) | Block::H2(_) => {
                p.rect_filled(line(y, 0.9), CornerRadius::same(1), Color32::from_gray(0x44));
                y += 7.0;
            }
            Block::Swatches(sw) => {
                let n = sw.len() as f32;
                let cw = (rect.width() - 12.0 - (n - 1.0) * 2.0) / n;
                for (i, (a, _)) in sw.iter().enumerate() {
                    let r = Rect::from_min_size(pos2(rect.left() + 6.0 + i as f32 * (cw + 2.0), y), vec2(cw, 10.0));
                    p.rect_filled(r, CornerRadius::same(1), parse_hex(a));
                }
                y += 14.0;
            }
            Block::Code(_) => {
                p.rect_filled(Rect::from_min_size(pos2(rect.left() + 6.0, y), vec2(rect.width() - 12.0, 16.0)), CornerRadius::same(2), t.hover);
                y += 20.0;
            }
            Block::Figure(a, _) => {
                p.rect_filled(Rect::from_min_size(pos2(rect.left() + 6.0, y), vec2(rect.width() - 12.0, 14.0)), CornerRadius::same(2), parse_hex(a));
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
}

// ── Floating quick-action bar (canvas.rs::quick_bar) ──────────────────────────────────────

fn quick_bar(app: &mut App, ui: &mut egui::Ui, t: &Tokens, area: Rect, top: f32) {
    let ctx = ui.ctx().clone();
    let pos = area.left_top() + vec2(14.0, top);
    let mut chosen: Option<(&'static str, &'static str)> = None;
    let mut group_click: Option<usize> = None;
    egui::Area::new(egui::Id::new("quickbar")).order(egui::Order::Foreground).fixed_pos(pos).show(&ctx, |ui| {
        egui::Frame::new()
            .fill(t.card)
            .stroke(Stroke::new(1.0, t.border))
            .corner_radius(CornerRadius::same(10))
            .shadow(egui::epaint::Shadow { offset: [0, 2], blur: 10, spread: 0, color: Color32::from_black_alpha(24) })
            .inner_margin(egui::Margin::same(4))
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing = vec2(2.0, 2.0);
                ui.vertical(|ui| {
                    for (gi, g) in data::COMMENT_GROUPS.iter().enumerate() {
                        let active = app.quick_group == Some(gi);
                        if icons::button(ui, g.icon, 32.0, active, g.name).clicked() {
                            group_click = Some(gi);
                        }
                    }
                    // Fill & Sign group.
                    let fill = data::COMMENT_GROUPS.len();
                    let fill_active = app.quick_group == Some(fill);
                    if icons::button(ui, "signature", 32.0, fill_active, "Fill & Sign").clicked() {
                        group_click = Some(fill);
                    }
                    ui.add_space(4.0);
                    ui.painter().hline(ui.max_rect().x_range().shrink(4.0), ui.cursor().top(), Stroke::new(1.0, t.divider));
                    ui.add_space(4.0);
                    for q in data::QUICK_TOOLS {
                        let active = app.quick_tool == Some(q.id);
                        if icons::button(ui, q.icon, 32.0, active, q.label).clicked() {
                            chosen = Some((q.id, q.command));
                        }
                    }
                });
            });
    });
    if let Some(gi) = group_click {
        app.quick_group = if app.quick_group == Some(gi) { None } else { Some(gi) };
    }
    if let Some((id, command)) = chosen {
        app.quick_tool = Some(id);
        app.notify(format!("{} — tool selected", App::command_label(command).unwrap_or(id)));
    }
    // The flyout for the open group, to the right of the strip.
    if let Some(gi) = app.quick_group {
        let tools = if gi < data::COMMENT_GROUPS.len() { data::COMMENT_GROUPS[gi].tools } else { data::FILL_TOOLS };
        egui::Area::new(egui::Id::new("quickbar-flyout")).order(egui::Order::Foreground).fixed_pos(pos + vec2(46.0, gi as f32 * 34.0)).show(
            &ctx,
            |ui| {
                egui::Frame::popup(ui.style()).show(ui, |ui| {
                    ui.set_min_width(210.0);
                    for q in tools {
                        let resp = ui.add(egui::Button::new(q.label).min_size(vec2(200.0, 26.0)));
                        if resp.clicked() {
                            app.quick_tool = Some(q.id);
                            app.quick_group = None;
                            app.notify(format!("{} — tool selected", q.label));
                        }
                    }
                });
            },
        );
    }
}

// ── Floating find bar (canvas.rs::find_bar) ───────────────────────────────────────────────

fn find_bar(app: &mut App, ui: &mut egui::Ui, t: &Tokens, area: Rect, top: f32) {
    let ctx = ui.ctx().clone();
    let total = data::SEARCH_HITS.len();
    let current = app.find_index.min(total.saturating_sub(1));
    let mut action = 0i8;
    let mut close = false;
    egui::Area::new(egui::Id::new("findbar"))
        .order(egui::Order::Foreground)
        .fixed_pos(area.right_top() + vec2(-18.0, top))
        .pivot(Align2::RIGHT_TOP)
        .show(&ctx, |ui| {
            egui::Frame::new()
                .fill(t.card)
                .stroke(Stroke::new(1.0, t.border))
                .corner_radius(CornerRadius::same(10))
                .shadow(egui::epaint::Shadow { offset: [0, 4], blur: 16, spread: 0, color: Color32::from_black_alpha(30) })
                .inner_margin(egui::Margin::symmetric(10, 6))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        icons::paint(ui, Rect::from_min_size(ui.cursor().min + vec2(0.0, 8.0), vec2(16.0, 16.0)), "search", 15.0, t.text_muted);
                        ui.add_space(22.0);
                        let mut q = std::mem::take(&mut app.find_query);
                        let resp = ui.add(
                            egui::TextEdit::singleline(&mut q)
                                .frame(egui::Frame::NONE)
                                .desired_width(200.0)
                                .hint_text("Find")
                                .font(theme::regular(13.0)),
                        );
                        app.find_query = q;
                        if resp.changed() {
                            app.find_index = 0;
                        }
                        ui.label(egui::RichText::new(format!("{} of {}", current + 1, total)).color(t.text_muted).font(theme::regular(12.0)));
                        if icons::button(ui, "chevron-up", 26.0, false, "Previous match (⇧⌘G)").clicked() {
                            action = -1;
                        }
                        if icons::button(ui, "chevron-down", 26.0, false, "Next match (⌘G)").clicked() {
                            action = 1;
                        }
                        if icons::button(ui, "x", 26.0, false, "Close find (Esc)").clicked() {
                            close = true;
                        }
                    });
                });
        });
    if action != 0 {
        app.find_index = ((current as i32 + action as i32).rem_euclid(total as i32)) as usize;
        app.page = data::SEARCH_HITS[app.find_index].page;
    }
    if close {
        app.find_open = false;
    }
}
