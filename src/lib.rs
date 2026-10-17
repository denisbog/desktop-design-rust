//! A minimal, runnable Rust port of the PrintCraft desktop design demo.
//!
//! The crate is laid out exactly like PrintCraft's egui shell:
//!
//! * [`theme`] — the design tokens (light/dark) and egui style,
//! * [`icons`] — the embedded Lucide SVG set, tinted per use,
//! * [`data`] — the demo catalogue and specimen document,
//! * [`widgets`] — the small custom controls built on the tokens,
//! * [`components`] — the shell regions (chrome, panels, canvas, home, palette, dialogs),
//! * [`app`] — the session state and `eframe::App` implementation.

pub mod app;
pub mod components;
pub mod data;
pub mod icon_data;
pub mod icons;
pub mod theme;
pub mod widgets;

pub use app::App;

#[cfg(test)]
mod tests {
    use super::app::{App, Dialog, Mode, Panel};
    use super::theme::{self, ThemeKind};

    fn frame(ctx: &egui::Context, app: &mut App) {
        let input =
            egui::RawInput { screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1440.0, 920.0))), ..Default::default() };
        for _ in 0..2 {
            let mut output = ctx.run_ui(input.clone(), |ui| app.show(ui));
            output.textures_delta.clear();
        }
    }
    /// One frame with a synthetic primary click at `pos`.
    fn click_frame(ctx: &egui::Context, app: &mut App, pos: egui::Pos2) {
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1440.0, 920.0))),
            events: vec![
                egui::Event::PointerMoved(pos),
                egui::Event::PointerButton { pos, button: egui::PointerButton::Primary, pressed: true, modifiers: Default::default() },
                egui::Event::PointerButton { pos, button: egui::PointerButton::Primary, pressed: false, modifiers: Default::default() },
            ],
            ..Default::default()
        };
        let mut output = ctx.run_ui(input, |ui| app.show(ui));
        output.textures_delta.clear();
    }

    /// The "Find tools and commands" box really opens the palette on click, and the click that
    /// opened it is not mistaken for a backdrop click on the same frame.
    #[test]
    fn find_tools_and_commands_opens_the_palette() {
        let ctx = egui::Context::default();
        egui_extras::install_image_loaders(&ctx);
        theme::install_fonts(&ctx);
        theme::apply(&ctx, ThemeKind::Light);
        let mut app = App::new();
        app.ready();
        // Warm-up frame so the mode bar (and its search box) exists before the click.
        frame(&ctx, &mut app);
        assert!(!app.palette_open);
        // The search box is 260 pt wide, 10 pt from the right edge, centred in the 48 pt mode bar.
        click_frame(&ctx, &mut app, egui::pos2(1300.0, 62.0));
        assert!(app.palette_open, "clicking Find tools and commands must open the palette");
        // A later click away from the palette is a backdrop click and closes it.
        click_frame(&ctx, &mut app, egui::pos2(600.0, 820.0));
        assert!(!app.palette_open, "a backdrop click should close the palette");
    }

    /// Drive every mode, panel, overlay and dialog in both themes: the shell must not panic.
    #[test]
    fn every_mode_panel_and_dialog_renders() {
        let ctx = egui::Context::default();
        egui_extras::install_image_loaders(&ctx);
        theme::install_fonts(&ctx);
        theme::apply(&ctx, ThemeKind::Light);
        let mut app = App::new();
        app.ready();
        for dark in [false, true] {
            theme::apply(&ctx, if dark { ThemeKind::Dark } else { ThemeKind::Light });
            for mode in [Mode::AllTools, Mode::Read, Mode::Edit, Mode::Convert, Mode::Sign] {
                app.mode = mode;
                for panel in [
                    Panel::Comments,
                    Panel::Bookmarks,
                    Panel::Pages,
                    Panel::Fields,
                    Panel::Layers,
                    Panel::Attachments,
                    Panel::Signatures,
                    Panel::Search,
                ] {
                    app.panel = Some(panel);
                    app.active = true;
                    frame(&ctx, &mut app);
                }
            }
            app.active = false;
            frame(&ctx, &mut app); // Home tab
            app.active = true;
            app.organize = true;
            frame(&ctx, &mut app); // Organize pages
            app.organize = false;
            app.palette_open = true;
            frame(&ctx, &mut app); // ⌘K palette
            app.palette_open = false;
            app.find_open = true;
            app.notice = true;
            frame(&ctx, &mut app);
            for dialog in [Dialog::Shortcuts, Dialog::About, Dialog::Properties, Dialog::Preferences] {
                app.dialog = Some(dialog);
                frame(&ctx, &mut app);
            }
            app.dialog = None;
        }
    }

    /// The demo's catalogue, specimen and persistence round-trip.
    #[test]
    fn data_and_persistence_are_consistent() {
        for g in super::data::TOOL_GROUPS {
            assert!(super::icons::exists(g.icon), "missing group icon {}", g.icon);
            for s in g.sections {
                for i in s.items {
                    assert!(super::icons::exists(i.icon), "missing item icon {} ({})", i.icon, i.command);
                }
            }
        }
        for c in super::data::COMMANDS {
            assert!(super::icons::exists(c.icon), "missing command icon {}", c.icon);
        }
        assert_eq!(super::data::PAGES.len(), 13);
        assert_eq!(super::data::SEARCH_HITS.len(), 16);
        assert!(super::data::SEARCH_HITS.iter().any(|h| h.current));

        let mut app = App::new();
        app.mode = Mode::Edit;
        app.panel = Some(Panel::Layers);
        app.zoom = 1.25;
        app.theme = ThemeKind::Dark;
        let json = app.persist();
        let mut restored = App::new();
        restored.restore(&json);
        assert_eq!(restored.mode, Mode::Edit);
        assert_eq!(restored.panel, Some(Panel::Layers));
        assert_eq!(restored.zoom, 1.25);
        assert_eq!(restored.theme, ThemeKind::Dark);
    }
}
