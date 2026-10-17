//! Application state and layout orchestration.
//!
//! `App` owns the demo session (a specimen PDF, comments, panels, zoom, theme) and the
//! `eframe::App` implementation lays the shell out in the order PrintCraft uses:
//! tab strip → mode bar → right rail → right panel → left panel → document area → overlays.

use std::collections::BTreeSet;

use egui::Key;

use crate::components::{canvas, chrome, dialogs, home, palette, panels};
use crate::data;
use crate::theme::{self, ThemeKind, Tokens};
use crate::widgets;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    AllTools,
    Read,
    Edit,
    Convert,
    Sign,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Panel {
    Comments,
    Bookmarks,
    Pages,
    Fields,
    Layers,
    Attachments,
    Signatures,
    Search,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LeftPanel {
    AllTools,
    Tool(&'static str),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dialog {
    Shortcuts,
    About,
    Properties,
    Preferences,
}

pub struct PageState {
    pub label: String,
    pub nav: &'static str,
    pub rot: i32,
}

pub struct Reply {
    pub author: String,
    pub time: String,
    pub body: String,
    pub status: bool,
}

pub struct Comment {
    pub page: usize,
    pub author: &'static str,
    pub time: &'static str,
    pub body: &'static str,
    pub tool: &'static str,
    pub state: Option<&'static str>,
    pub replies: Vec<Reply>,
}

#[derive(Clone, Copy, Debug)]
pub struct Prefs {
    pub smooth_scroll: bool,
    pub hardware_accel: bool,
    pub remember_layout: bool,
}

impl Default for Prefs {
    fn default() -> Self {
        Self { smooth_scroll: true, hardware_accel: true, remember_layout: true }
    }
}

pub struct App {
    pub theme: ThemeKind,
    styled: bool,
    fonts_ready: bool,

    pub mode: Mode,
    pub left: LeftPanel,
    pub left_open: bool,
    pub all_tools_expanded: bool,
    pub tool_query: String,
    pub panel: Option<Panel>,

    /// `true` while the specimen tab is open; `false` shows the Home tab.
    pub active: bool,
    pub palette_open: bool,
    pub palette_query: String,
    /// Set for the frame the palette is opened, so the click that opened it is not also treated
    /// as a backdrop click (which would close it again immediately).
    pub palette_just_opened: bool,
    pub dialog: Option<Dialog>,
    pub toast: Option<(String, f64)>,

    pub find_open: bool,
    pub find_query: String,
    pub find_index: usize,

    pub pages: Vec<PageState>,
    pub page: usize,
    pub page_input: String,
    pub zoom: f32,
    pub fit_width: bool,
    pub organize: bool,
    pub organize_sel: BTreeSet<usize>,
    pub notice: bool,

    pub quick_tool: Option<&'static str>,
    pub quick_group: Option<usize>,

    pub comments: Vec<Comment>,
    pub selected_comment: Option<usize>,
    pub reply_draft: String,
    pub expanded: Vec<bool>,

    pub prefs: Prefs,
    pub recent: Vec<&'static (&'static str, &'static str, usize, &'static str)>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        let pages = data::PAGES.iter().map(|(label, nav)| PageState { label: (*label).to_owned(), nav, rot: 0 }).collect();
        let comments = data::COMMENTS
            .iter()
            .map(|c| Comment {
                page: c.page,
                author: c.author,
                time: c.time,
                body: c.body,
                tool: c.tool,
                state: c.state,
                replies: c
                    .replies
                    .iter()
                    .map(|r| Reply { author: r.author.to_owned(), time: r.time.to_owned(), body: r.body.to_owned(), status: r.status })
                    .collect(),
            })
            .collect();
        Self {
            theme: ThemeKind::Light,
            styled: false,
            fonts_ready: false,
            mode: Mode::AllTools,
            left: LeftPanel::Tool("organize"),
            left_open: true,
            all_tools_expanded: false,
            tool_query: String::new(),
            panel: Some(Panel::Comments),
            active: true,
            palette_open: false,
            palette_query: String::new(),
            palette_just_opened: false,
            dialog: None,
            toast: None,
            find_open: true,
            find_query: "type".to_owned(),
            find_index: 9,
            pages,
            page: 7,
            page_input: "5".to_owned(),
            zoom: 0.85,
            fit_width: false,
            organize: false,
            organize_sel: BTreeSet::new(),
            notice: true,
            quick_tool: None,
            quick_group: None,
            comments,
            selected_comment: Some(0),
            reply_draft: String::new(),
            expanded: vec![false, false, false, true, true, false, false, false, false, false, false, false, false],
            prefs: Prefs::default(),
            recent: data::RECENT.iter().collect(),
        }
    }

    pub fn doc_open(&self) -> bool {
        self.active
    }

    pub fn notify(&mut self, msg: impl Into<String>) {
        self.toast = Some((msg.into(), 0.0));
    }

    /// Test/preview hook: marks the one-frame font registration as done.
    #[doc(hidden)]
    pub fn ready(&mut self) {
        self.fonts_ready = true;
    }

    pub fn set_theme(&mut self, ctx: &egui::Context, kind: ThemeKind) {
        self.theme = kind;
        theme::apply(ctx, kind);
    }

    pub fn active_page(&self) -> usize {
        self.page.min(self.pages.len().saturating_sub(1))
    }

    pub fn page_label(&self, i: usize) -> &str {
        self.pages.get(i).map_or("", |p| p.label.as_str())
    }

    pub fn zoom_step(&mut self, up: bool) {
        let steps = [0.5, 0.75, 0.85, 1.0, 1.25, 1.5, 2.0, 3.0, 4.0];
        let cur = self.zoom;
        self.zoom = if up {
            *steps.iter().find(|s| **s > cur + 0.001).unwrap_or(&4.0)
        } else {
            *steps.iter().rev().find(|s| **s < cur - 0.001).unwrap_or(&0.25)
        };
        self.fit_width = false;
    }

    pub fn step_page(&mut self, forward: bool) {
        if forward {
            self.page = (self.page + 1).min(self.pages.len().saturating_sub(1));
        } else {
            self.page = self.page.saturating_sub(1);
        }
        self.page_input = self.page_label(self.page).to_owned();
    }

    pub fn open_file_dialog(&mut self) {
        let picked = rfd::FileDialog::new().add_filter("PDF", &["pdf"]).set_title("Open a PDF").pick_file();
        match picked {
            Some(p) => self.notify(format!("Opening {} — demo loads the specimen", p.display())),
            None => self.notify("Open cancelled"),
        }
        self.active = true;
    }

    /// Open the ⌘K palette, clearing the previous query and disarming backdrop dismissal for
    /// this frame (the click that opened it must not close it again).
    pub fn open_palette(&mut self) {
        self.palette_open = true;
        self.palette_query.clear();
        self.palette_just_opened = true;
    }

    /// The shortcut list in the Help ▸ Keyboard Shortcuts dialog.
    pub fn shortcut_rows() -> &'static [(&'static str, &'static str)] {
        data::SHORTCUTS
    }

    /// Every registered command and every catalogue item, as (label, icon) for the palette.
    pub fn command_label(id: &str) -> Option<&'static str> {
        data::COMMANDS
            .iter()
            .find(|c| c.id == id)
            .map(|c| c.label)
            .or_else(|| data::TOOL_GROUPS.iter().flat_map(|g| g.sections).flat_map(|s| s.items).find(|i| i.command == id).map(|i| i.label))
    }

    pub fn run_command(&mut self, id: &str) {
        let label = App::command_label(id);
        match id {
            "file.open" => {
                self.open_file_dialog();
                return;
            }
            "view.zoom_in" => self.zoom_step(true),
            "view.zoom_out" => self.zoom_step(false),
            "view.actual_size" => {
                self.zoom = 1.0;
                self.fit_width = false;
            }
            "view.fit_page" => {
                self.zoom = 0.85;
                self.fit_width = false;
            }
            "view.fit_width" => {
                self.zoom = 1.1;
                self.fit_width = true;
            }
            "view.rotate_cw" => {
                let p = self.active_page();
                self.pages[p].rot = (self.pages[p].rot + 90) % 360;
            }
            "view.read_mode" => self.mode = Mode::Read,
            "edit.find" => self.find_open = true,
            "edit.search" => {
                // Advanced Search… opens the dedicated side panel (⇧⌘F).
                self.panel = Some(Panel::Search);
                self.mode = Mode::AllTools;
                return;
            }
            "page.organize" => {
                self.organize = true;
                self.left = LeftPanel::Tool("organize");
                self.left_open = true;
            }
            "page.rotate" => self.rotate_selection(),
            "page.delete" => self.delete_selection(),
            "page.insert" => self.insert_blank(),
            "edit.preferences" => self.dialog = Some(Dialog::Preferences),
            "doc.properties" => self.dialog = Some(Dialog::Properties),
            "help.shortcuts" => self.dialog = Some(Dialog::Shortcuts),
            "help.about" => self.dialog = Some(Dialog::About),
            "help.discord" => {
                self.notify("Opening discord.gg/artcraft in your browser");
                return;
            }
            "file.save" | "file.save_as" => {
                self.notify(format!("Saved {} — demo only", data::DOC_NAME));
                return;
            }
            "comment.list" => {
                self.panel = Some(Panel::Comments);
                return;
            }
            "form.fields" => {
                self.panel = Some(Panel::Fields);
                return;
            }
            "optimize.audit" => {
                self.panel = Some(Panel::Search);
                return;
            }
            id if id.starts_with("panel.") => {
                self.panel = Some(match id {
                    "panel.comments" => Panel::Comments,
                    "panel.bookmarks" => Panel::Bookmarks,
                    "panel.pages" => Panel::Pages,
                    "panel.fields" => Panel::Fields,
                    "panel.layers" => Panel::Layers,
                    "panel.attachments" => Panel::Attachments,
                    "panel.signatures" => Panel::Signatures,
                    _ => Panel::Search,
                });
                return;
            }
            _ => {}
        }
        self.notify(match label {
            Some(l) => format!("{l} — demo stub ({id})"),
            None => id.to_owned(),
        });
    }

    // ── Organize-page edits (the commands that actually change the demo document) ──────────

    pub fn selection(&self) -> Vec<usize> {
        if self.organize_sel.is_empty() { vec![self.active_page()] } else { self.organize_sel.iter().copied().collect() }
    }

    pub fn rotate_selection(&mut self) {
        for i in self.selection() {
            if let Some(p) = self.pages.get_mut(i) {
                p.rot = (p.rot + 90) % 360;
            }
        }
        self.notify("Rotated pages — ⌘Z would undo this");
    }

    pub fn delete_selection(&mut self) {
        let sel = self.selection();
        if sel.len() >= self.pages.len() {
            self.notify("Cannot delete every page");
            return;
        }
        for i in sel.iter().rev() {
            self.pages.remove(*i);
        }
        self.organize_sel.clear();
        self.page = self.page.min(self.pages.len().saturating_sub(1));
        self.notify(format!("Deleted {} page(s)", sel.len()));
    }

    pub fn insert_blank(&mut self) {
        let at = self.active_page() + 1;
        let label = format!("blank-{}", self.pages.len());
        self.pages.insert(at, PageState { label, nav: "Blank page", rot: 0 });
        self.notify("Inserted a blank page");
    }

    fn shortcuts(&mut self, ctx: &egui::Context) {
        let (cmd, shift) = ctx.input(|i| (i.modifiers.command, i.modifiers.shift));
        let pressed = |k: Key| ctx.input(|i| i.key_pressed(k));
        if pressed(Key::Escape) {
            if self.palette_open {
                self.palette_open = false;
            } else if self.dialog.is_some() {
                self.dialog = None;
            } else if self.find_open {
                self.find_open = false;
            } else if !self.organize_sel.is_empty() {
                self.organize_sel.clear();
            }
        }
        if cmd && pressed(Key::K) {
            self.open_palette();
        }
        if cmd && pressed(Key::F) {
            // ⇧⌘F opens the Advanced Search panel; ⌘F the find bar on the page.
            if shift {
                self.panel = Some(Panel::Search);
                self.mode = Mode::AllTools;
            } else {
                self.find_open = true;
            }
        }
        if cmd && pressed(Key::D) {
            self.dialog = Some(Dialog::Properties);
        }
        if cmd && pressed(Key::Comma) {
            self.dialog = Some(Dialog::Preferences);
        }
        if cmd && pressed(Key::O) {
            self.open_file_dialog();
        }
        if cmd && pressed(Key::S) {
            self.notify(format!("Saved {} — demo only", data::DOC_NAME));
        }
        if cmd && pressed(Key::G) {
            let n = data::SEARCH_HITS.len();
            self.find_index = if shift { (self.find_index + n - 1) % n } else { (self.find_index + 1) % n };
            self.page = data::SEARCH_HITS[self.find_index].page;
        }
        if cmd {
            if pressed(Key::Plus) || pressed(Key::Equals) {
                if shift {
                    let p = self.active_page();
                    self.pages[p].rot = (self.pages[p].rot + 90) % 360;
                } else {
                    self.zoom_step(true);
                }
            }
            if pressed(Key::Minus) {
                self.zoom_step(false);
            }
            if pressed(Key::Num1) {
                self.zoom = 1.0;
            }
            if pressed(Key::Num0) {
                self.zoom = 0.85;
            }
            if pressed(Key::Num2) {
                self.zoom = 1.1;
                self.fit_width = true;
            }
        }
    }
}

impl eframe::App for App {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.styled {
            egui_extras::install_image_loaders(ctx);
            theme::install_fonts(ctx);
            theme::apply(ctx, self.theme);
            self.styled = true;
        } else {
            self.fonts_ready = true;
        }
        if let Some(sys) = ctx.system_theme() {
            let want = if sys == egui::Theme::Dark { ThemeKind::Dark } else { ThemeKind::Light };
            if want != self.theme && self.prefs.remember_layout {
                self.set_theme(ctx, want);
            }
        }
        self.shortcuts(ctx);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        // Fonts registered via set_fonts only take effect next frame; named families would panic now.
        if !self.fonts_ready {
            ctx.request_repaint();
            return;
        }
        let title = if self.active { format!("{} — PrintCraft", data::DOC_NAME) } else { "PrintCraft".to_owned() };
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(title));
        self.show(ui);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        storage.set_string("printcraft-design-demo", self.persist());
    }
}

impl App {
    /// The whole shell, laid out into the root `Ui` in PrintCraft's panel order.
    /// Split out from the `eframe::App` implementation so the smoke test can drive it.
    pub fn show(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        chrome::tab_strip(self, ui);
        chrome::mode_bar(self, ui);
        if self.active {
            chrome::right_rail(self, ui);
            if self.panel.is_some() && self.mode != Mode::Read {
                panels::right_panel(self, ui);
            }
        }
        if self.left_open && self.mode != Mode::Read && self.active {
            panels::left_panel(self, ui);
        }
        let t = Tokens::get(&ctx);
        egui::CentralPanel::default().frame(egui::Frame::NONE.fill(t.pasteboard)).show(ui, |ui| {
            if self.active {
                canvas::document_area(self, ui);
            } else {
                home::show(self, ui);
            }
        });
        palette::show(self, &ctx);
        dialogs::show(self, &ctx);
        widgets::toast(self, &ctx);
    }
}

// ── Persistence (eframe's "persistence" feature) ──────────────────────────────────────────
// The demo remembers the theme, workspace mode and open panels between runs, written as JSON
// through serde/serde_json — the same crates PrintCraft uses for its session state.

impl App {
    pub fn persist(&self) -> String {
        let v = serde_json::json!({
            "theme": self.theme,
            "mode": mode_id(self.mode),
            "panel": self.panel.map(panel_id),
            "left_open": self.left_open,
            "left": match self.left { LeftPanel::AllTools => "", LeftPanel::Tool(id) => id },
            "zoom": self.zoom,
            "active": self.active,
        });
        v.to_string()
    }

    pub fn restore(&mut self, json: &str) {
        let Ok(v) = serde_json::from_str::<serde_json::Value>(json) else { return };
        if let Ok(theme) = serde_json::from_value::<ThemeKind>(v["theme"].clone()) {
            self.theme = theme;
        }
        if let Some(m) = v["mode"].as_u64() {
            self.mode = match m {
                1 => Mode::Read,
                2 => Mode::Edit,
                3 => Mode::Convert,
                4 => Mode::Sign,
                _ => Mode::AllTools,
            };
        }
        if let Some(p) = v["panel"].as_u64() {
            self.panel = Some(match p {
                1 => Panel::Bookmarks,
                2 => Panel::Pages,
                3 => Panel::Fields,
                4 => Panel::Layers,
                5 => Panel::Attachments,
                6 => Panel::Signatures,
                7 => Panel::Search,
                _ => Panel::Comments,
            });
        }
        self.left_open = v["left_open"].as_bool().unwrap_or(self.left_open);
        if let Some(id) = v["left"].as_str()
            && let Some(g) = data::group(id)
        {
            self.left = LeftPanel::Tool(g.id);
        }
        if let Some(z) = v["zoom"].as_f64() {
            self.zoom = z as f32;
        }
        self.active = v["active"].as_bool().unwrap_or(true);
    }
}

fn mode_id(mode: Mode) -> u64 {
    match mode {
        Mode::AllTools => 0,
        Mode::Read => 1,
        Mode::Edit => 2,
        Mode::Convert => 3,
        Mode::Sign => 4,
    }
}

fn panel_id(panel: Panel) -> u64 {
    match panel {
        Panel::Comments => 0,
        Panel::Bookmarks => 1,
        Panel::Pages => 2,
        Panel::Fields => 3,
        Panel::Layers => 4,
        Panel::Attachments => 5,
        Panel::Signatures => 6,
        Panel::Search => 7,
    }
}
