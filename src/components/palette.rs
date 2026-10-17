//! ⌘K command palette: every registered command (with its shortcut) and every tool in the
//! catalogue, with a substring (then subsequence) match.
//!
//! Port of `crates/ui-egui/src/palette.rs`.

use egui::{Align2, CornerRadius, Rect, Sense, vec2};

use crate::app::{App, LeftPanel};
use crate::data;
use crate::icons;
use crate::theme::{self, Tokens};

struct Hit {
    group: Option<&'static str>,
    label: String,
    detail: String,
    icon: &'static str,
    command: Option<&'static str>,
    ready: bool,
}

fn score(hay: &str, needle: &str) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }
    let h = hay.to_lowercase();
    if let Some(p) = h.find(needle) {
        return Some(p);
    }
    // Subsequence match as a fallback.
    let mut it = h.chars();
    needle.chars().all(|c| it.any(|x| x == c)).then_some(100)
}

pub fn show(app: &mut App, ctx: &egui::Context) {
    if !app.palette_open {
        return;
    }
    // Consume the "just opened" flag: the click that opened the palette is in this frame's
    // input, and must not be mistaken for a backdrop click.
    let just_opened = std::mem::take(&mut app.palette_just_opened);
    let t = Tokens::get(ctx);
    if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
        app.palette_open = false;
        return;
    }
    let q = app.palette_query.trim().to_lowercase();
    let mut hits: Vec<(usize, Hit)> = Vec::new();
    for c in data::COMMANDS {
        if let Some(s) = score(c.label, &q).or_else(|| score(c.id, &q).map(|s| s + 50)) {
            hits.push((
                s,
                Hit {
                    group: None,
                    label: c.label.to_owned(),
                    detail: c.shortcut.unwrap_or(c.menu.unwrap_or("Command")).to_owned(),
                    icon: c.icon,
                    command: Some(c.id),
                    ready: c.enabled,
                },
            ));
        }
    }
    for g in data::TOOL_GROUPS {
        if let Some(s) = score(g.label, &q) {
            hits.push((
                s + 1,
                Hit {
                    group: Some(g.id),
                    label: g.label.to_owned(),
                    detail: "Tool".into(),
                    icon: g.icon,
                    command: None,
                    ready: data::is_ready(g.availability),
                },
            ));
        }
        for sec in g.sections {
            for i in sec.items {
                if data::COMMANDS.iter().any(|c| c.id == i.command) {
                    continue; // listed above as a command
                }
                if let Some(s) = score(i.label, &q).or_else(|| score(i.command, &q).map(|s| s + 50)) {
                    hits.push((
                        s + 2,
                        Hit {
                            group: Some(g.id),
                            label: i.label.to_owned(),
                            detail: g.label.into(),
                            icon: i.icon,
                            command: Some(i.command),
                            ready: data::is_ready(i.availability),
                        },
                    ));
                }
            }
        }
    }
    hits.sort_by_key(|(s, h)| (*s, !h.ready));
    hits.truncate(12);

    let screen = ctx.content_rect();
    let mut chosen: Option<(Option<&'static str>, Option<&'static str>)> = None;
    let palette_rect = egui::Area::new(egui::Id::new("palette"))
        .order(egui::Order::Foreground)
        .pivot(Align2::CENTER_TOP)
        .fixed_pos(egui::pos2(screen.center().x, screen.top() + 96.0))
        .show(ctx, |ui| {
            egui::Frame::popup(ui.style()).corner_radius(CornerRadius::same(12)).inner_margin(egui::Margin::same(10)).show(ui, |ui| {
                ui.set_width(560.0);
                ui.horizontal(|ui| {
                    ui.add(icons::image("search", 18.0, t.text_muted));
                    let r = ui.add(
                        egui::TextEdit::singleline(&mut app.palette_query)
                            .hint_text("Search tools and commands…")
                            .frame(egui::Frame::NONE)
                            .font(theme::regular(15.0))
                            .desired_width(f32::INFINITY),
                    );
                    if (r.has_focus() || r.lost_focus())
                        && ui.input(|i| i.key_pressed(egui::Key::Enter))
                        && let Some((_, h)) = hits.first()
                    {
                        chosen = Some((h.command, h.group));
                    }
                    r.request_focus();
                });
                ui.separator();
                for (_, h) in &hits {
                    let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 36.0), Sense::click());
                    if resp.hovered() {
                        ui.painter().rect_filled(rect, CornerRadius::same(6), t.hover);
                    }
                    icons::paint(ui, Rect::from_min_size(rect.min + vec2(8.0, 9.0), vec2(18.0, 18.0)), h.icon, 17.0, t.icon);
                    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, h.ready, &h.label));
                    let fg = if h.ready { t.text } else { t.text_faint };
                    ui.painter().text(rect.left_center() + vec2(36.0, 0.0), Align2::LEFT_CENTER, &h.label, theme::regular(13.5), fg);
                    ui.painter().text(rect.right_center() - vec2(10.0, 0.0), Align2::RIGHT_CENTER, &h.detail, theme::regular(12.0), t.text_faint);
                    if resp.clicked() {
                        chosen = Some((h.command, h.group));
                    }
                }
                if hits.is_empty() {
                    ui.label(egui::RichText::new("No matching tools").color(t.text_muted));
                }
                ui.add_space(2.0);
            });
        })
        .response
        .rect;
    // A click outside the palette closes it (the backdrop), unless this is the frame the
    // palette was opened on — that click belongs to the search box or the ⌘K shortcut.
    if !just_opened
        && chosen.is_none()
        && ctx.input(|i| i.pointer.any_click())
        && let Some(p) = ctx.input(|i| i.pointer.interact_pos())
        && !palette_rect.contains(p)
    {
        app.palette_open = false;
    }
    if let Some((command, group)) = chosen {
        app.palette_open = false;
        app.palette_query.clear();
        if let Some(g) = group {
            app.left_open = true;
            app.left = LeftPanel::Tool(g);
        }
        if let Some(c) = command {
            app.run_command(c);
        }
    }
}
