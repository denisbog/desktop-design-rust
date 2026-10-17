//! Demo data for the PrintCraft design demo.
//!
//! Mirrors the engine catalogue (`crates/engine/src/catalog.rs`) and the shape of a real
//! session (a 13-page specimen PDF with comments, bookmarks, form fields and a signature).
//! Nothing here talks to a backend: it exists so every UI component can be seen in a
//! realistic state. Port of the design's `js/data.js`.

// ── Tool catalogue ────────────────────────────────────────────────────────────────────────
// id, label, icon, hue, availability, sections. availability: "ready" | "M6".."M13" | "ai".

pub struct ToolItem {
    pub label: &'static str,
    pub icon: &'static str,
    pub command: &'static str,
    pub availability: &'static str,
}

pub struct Section {
    pub title: &'static str,
    pub items: &'static [ToolItem],
}

pub struct ToolGroup {
    pub id: &'static str,
    pub label: &'static str,
    pub icon: &'static str,
    pub hue: (u8, u8, u8),
    pub availability: &'static str,
    /// A right-aligned badge ("Optional" for the AI tool).
    pub badge: Option<&'static str>,
    pub sections: &'static [Section],
}

pub const RED: (u8, u8, u8) = (0xE0, 0x3E, 0x3E);
pub const PURPLE: (u8, u8, u8) = (0x8E, 0x4E, 0xE6);
pub const BLUE: (u8, u8, u8) = (0x3A, 0x6F, 0xE8);
pub const GREEN: (u8, u8, u8) = (0x2D, 0x9D, 0x5B);
pub const ORANGE: (u8, u8, u8) = (0xE8, 0x8A, 0x1A);
pub const TEAL: (u8, u8, u8) = (0x14, 0x9C, 0xA8);
pub const PINK: (u8, u8, u8) = (0xD6, 0x3B, 0x8F);

pub const fn it(label: &'static str, icon: &'static str, command: &'static str, a: &'static str) -> ToolItem {
    ToolItem { label, icon, command, availability: a }
}
pub const fn s(title: &'static str, items: &'static [ToolItem]) -> Section {
    Section { title, items }
}
pub const fn g(
    id: &'static str,
    label: &'static str,
    icon: &'static str,
    hue: (u8, u8, u8),
    availability: &'static str,
    sections: &'static [Section],
    badge: Option<&'static str>,
) -> ToolGroup {
    ToolGroup { id, label, icon, hue, availability, badge, sections }
}

pub fn is_ready(a: &str) -> bool {
    a == "ready"
}

pub static TOOL_GROUPS: &[ToolGroup] = &[
    g(
        "export",
        "Export a PDF",
        "file-output",
        BLUE,
        "ready",
        &[s(
            "Export to",
            &[
                it("Microsoft Word (.docx)", "file-text", "export.docx", "ready"),
                it("Spreadsheet (.xlsx)", "grid-3x3", "export.xlsx", "M10"),
                it("Presentation (.pptx)", "presentation", "export.pptx", "M10"),
                it("Image (PNG)", "image", "export.image", "ready"),
                it("Export all images", "image", "export.all_images", "ready"),
                it("HTML web page", "file-symlink", "export.html", "ready"),
                it("Rich Text Format (.rtf)", "file-text", "export.rtf", "ready"),
                it("Text (plain)", "type", "export.text", "ready"),
                it("PostScript / EPS", "file-down", "export.ps", "M10"),
            ],
        )],
        None,
    ),
    g(
        "fill_sign",
        "Fill & Sign",
        "pen-line",
        PURPLE,
        "ready",
        &[s(
            "Fill",
            &[
                it("Add text", "type", "sign.fill.text", "ready"),
                it("Checkmark", "check", "sign.fill.check", "ready"),
                it("Cross", "x", "sign.fill.cross", "ready"),
                it("Dot", "circle-dot", "sign.fill.dot", "ready"),
                it("Line", "minus", "sign.fill.line", "ready"),
                it("Date", "clock-3", "sign.fill.date", "ready"),
                it("Signature", "signature", "sign.fill.signature", "ready"),
                it("Initials", "signature", "sign.fill.initials", "ready"),
            ],
        )],
        None,
    ),
    g(
        "edit",
        "Edit a PDF",
        "file-pen-line",
        PINK,
        "M7",
        &[
            s(
                "Modify page",
                &[
                    it("Rotate pages", "rotate-cw", "page.rotate", "ready"),
                    it("Insert pages", "file-plus-2", "page.insert", "ready"),
                    it("Delete pages", "trash-2", "page.delete", "ready"),
                    it("Extract pages", "file-output", "page.extract", "ready"),
                    it("Organize pages", "layout-grid", "page.organize", "ready"),
                ],
            ),
            s(
                "Add content",
                &[
                    it("Edit text & images", "text-cursor-input", "edit.edit_text", "ready"),
                    it("Text", "type", "edit.text", "ready"),
                    it("Image", "image-plus", "edit.image", "ready"),
                    it("Header and footer", "heading", "edit.header_footer", "ready"),
                    it("Watermark", "stamp", "edit.watermark", "ready"),
                    it("Link", "link-2", "edit.link", "ready"),
                    it("Bates numbering", "hash", "edit.bates", "ready"),
                    it("Background", "palette", "edit.background", "ready"),
                    it("Attach file", "paperclip", "edit.attach", "M12"),
                ],
            ),
        ],
        None,
    ),
    g(
        "create",
        "Create a PDF",
        "file-plus-2",
        RED,
        "ready",
        &[s(
            "Create from",
            &[
                it("Single file", "file-input", "create.file", "ready"),
                it("Multiple files", "files", "create.multiple", "M10"),
                it("Images", "image", "create.images", "ready"),
                it("Clipboard", "copy-plus", "create.clipboard", "ready"),
                it("Blank page", "file-plus-2", "create.blank", "ready"),
            ],
        )],
        None,
    ),
    g("combine", "Combine files", "files", BLUE, "ready", &[s("Combine", &[it("Add files to combine", "files", "page.combine", "ready")])], None),
    g(
        "organize",
        "Organize pages",
        "layout-grid",
        GREEN,
        "ready",
        &[
            s(
                "Page options",
                &[
                    it("Page grid", "layout-grid", "page.organize", "ready"),
                    it("Rotate", "rotate-cw", "page.rotate", "ready"),
                    it("Delete", "trash-2", "page.delete", "ready"),
                    it("Extract", "file-output", "page.extract", "ready"),
                    it("Insert", "file-plus-2", "page.insert", "ready"),
                    it("Replace", "replace", "page.replace", "ready"),
                    it("Split", "scissors", "page.split", "ready"),
                ],
            ),
            s(
                "More",
                &[
                    it("Set page boxes", "square-dashed-mouse-pointer", "page.boxes", "ready"),
                    it("Crop pages", "crop", "page.crop", "ready"),
                    it("Duplicate pages", "copy-plus", "page.duplicate", "ready"),
                    it("Page labels", "tag", "page.number", "ready"),
                    it("Page transitions", "presentation", "page.transitions", "M4"),
                ],
            ),
        ],
        None,
    ),
    g(
        "comment",
        "Add comments",
        "message-square-text",
        ORANGE,
        "ready",
        &[
            s(
                "Markup",
                &[
                    it("Sticky note", "sticky-note", "comment.note", "ready"),
                    it("Highlight text", "highlighter", "comment.highlight", "ready"),
                    it("Underline", "underline", "comment.underline", "ready"),
                    it("Strikethrough", "strikethrough", "comment.strikeout", "ready"),
                    it("Squiggly underline", "spline", "comment.squiggly", "ready"),
                    it("Text box", "type", "comment.freetext", "ready"),
                ],
            ),
            s(
                "Drawing",
                &[
                    it("Draw freehand", "pencil", "comment.ink", "ready"),
                    it("Rectangle", "square", "comment.square", "ready"),
                    it("Oval", "circle", "comment.circle", "ready"),
                    it("Line", "minus", "comment.line", "ready"),
                    it("Arrow", "move-right", "comment.arrow", "ready"),
                    it("Cloud", "cloud", "comment.cloud", "M5"),
                    it("Stamp", "stamp", "comment.stamp", "M5"),
                ],
            ),
            s(
                "Review",
                &[it("Comment list", "message-square-text", "comment.list", "ready"), it("Flatten comments", "layers", "comment.flatten", "ready")],
            ),
        ],
        None,
    ),
    g(
        "scan",
        "Scan & OCR",
        "scan-text",
        GREEN,
        "ready",
        &[s(
            "Recognize text",
            &[
                it("In this file", "scan-text", "ocr.recognize", "ready"),
                it("In multiple files", "files", "ocr.recognize_batch", "ready"),
                it("Enhance scanned file", "sparkles", "ocr.enhance", "M10"),
                it("Correct recognized text", "text-select", "ocr.correct", "M10"),
            ],
        )],
        None,
    ),
    g(
        "protect",
        "Protect a PDF",
        "shield-check",
        BLUE,
        "M8",
        &[
            s(
                "Protect",
                &[
                    it("Protect with password", "lock", "protect.password", "ready"),
                    it("Remove hidden information", "eye-off", "protect.remove_hidden", "ready"),
                ],
            ),
            s(
                "Advanced options",
                &[
                    it("Encrypt with certificate", "file-lock-2", "protect.certificate", "M8"),
                    it("Security properties", "shield-check", "protect.properties", "ready"),
                    it("Remove security", "lock-open", "protect.remove", "ready"),
                ],
            ),
        ],
        None,
    ),
    g(
        "redact",
        "Redact a PDF",
        "rectangle-horizontal",
        RED,
        "ready",
        &[s(
            "Redact",
            &[
                it("Redact text and images", "rectangle-horizontal", "redact.mark", "ready"),
                it("Redact pages", "file-x", "redact.pages", "ready"),
                it("Find text and redact", "file-search", "redact.search", "ready"),
                it("Set properties", "settings-2", "redact.properties", "ready"),
                it("Sanitize document", "sparkles", "redact.sanitize", "ready"),
            ],
        )],
        None,
    ),
    g(
        "compress",
        "Compress a PDF",
        "file-down",
        RED,
        "ready",
        &[s(
            "Optimize",
            &[
                it("Reduce file size", "file-down", "optimize.reduce", "ready"),
                it("Advanced optimization", "settings-2", "optimize.advanced", "ready"),
                it("Audit space usage", "gauge", "optimize.audit", "M11"),
            ],
        )],
        None,
    ),
    g(
        "form",
        "Prepare a form",
        "text-cursor-input",
        PURPLE,
        "ready",
        &[
            s(
                "Fields",
                &[
                    it("Field list", "list", "form.fields", "ready"),
                    it("Detect form fields", "scan", "form.detect", "ready"),
                    it("Clear form", "eraser", "form.clear", "ready"),
                    it("Field properties", "settings-2", "form.field.properties", "ready"),
                    it("Tab order by rows", "rows-3", "form.tab_order.row", "ready"),
                    it("Tab order by columns", "columns-3", "form.tab_order.column", "ready"),
                    it("Flatten form fields", "layers", "form.flatten", "ready"),
                    it("Import data", "file-input", "form.import_data", "ready"),
                    it("Export data", "file-output", "form.export_data", "ready"),
                ],
            ),
            s(
                "Add form components",
                &[
                    it("Text field", "text-cursor-input", "form.add.text", "ready"),
                    it("Checkbox", "check-circle-2", "form.add.checkbox", "ready"),
                    it("Radio button", "circle", "form.add.radio", "ready"),
                    it("Drop-down list", "chevron-down", "form.add.combo", "ready"),
                    it("List box", "list", "form.add.list", "ready"),
                    it("Button", "square", "form.add.button", "ready"),
                    it("Image field", "image", "form.add.image", "ready"),
                    it("Date field", "clock-3", "form.add.date", "ready"),
                    it("Digital signature", "signature", "form.add.signature", "ready"),
                ],
            ),
        ],
        None,
    ),
    g("stamp", "Add a stamp", "stamp", PURPLE, "ready", &[s("Stamps", &[it("Stamp palette", "stamp", "comment.stamp", "ready")])], None),
    g(
        "certificate",
        "Use a certificate",
        "badge-check",
        TEAL,
        "ready",
        &[s(
            "Certificates",
            &[
                it("Digitally sign", "signature", "sign.digital", "ready"),
                it("Timestamp", "clock-3", "sign.timestamp", "M9"),
                it("Validate all signatures", "badge-check", "sign.validate", "ready"),
                it("Certify (visible signature)", "badge-check", "sign.certify", "ready"),
                it("Certify (invisible signature)", "badge-check", "sign.certify_invisible", "ready"),
            ],
        )],
        None,
    ),
    g(
        "print_production",
        "Use print production",
        "printer",
        PURPLE,
        "M11",
        &[s(
            "Print production",
            &[
                it("Output preview", "layers", "prepress.output_preview", "M11"),
                it("Preflight", "file-check", "preflight.run", "M11"),
                it("Convert colors", "palette", "prepress.convert_colors", "M11"),
                it("Set page boxes", "square-dashed-mouse-pointer", "page.boxes", "ready"),
                it("Add printer marks", "ruler", "prepress.marks", "M11"),
                it("Fix hairlines", "minus", "prepress.hairlines", "M11"),
            ],
        )],
        None,
    ),
    g(
        "measure",
        "Measure objects",
        "ruler",
        PINK,
        "M12",
        &[s("Measure", &[it("Measuring tool", "ruler", "measure.distance", "M12"), it("Geospatial location", "compass", "measure.geo", "M12")])],
        None,
    ),
    g(
        "compare",
        "Compare files",
        "git-compare",
        PINK,
        "ready",
        &[s("Compare", &[it("Select files to compare", "git-compare", "doc.compare", "ready")])],
        None,
    ),
    g(
        "actions",
        "Use guided actions",
        "list-checks",
        PURPLE,
        "ready",
        &[s(
            "Actions list",
            &[
                it("Action Wizard", "list-checks", "actions.wizard", "ready"),
                it("Make accessible", "accessibility", "actions.make_accessible", "M13"),
                it("Prepare for distribution", "send", "actions.distribution", "ready"),
                it("Optimize scanned documents", "scan-text", "actions.optimize_scans", "ready"),
                it("Archive documents", "file-check", "actions.archive", "M13"),
            ],
        )],
        None,
    ),
    g(
        "accessibility",
        "Prepare for accessibility",
        "accessibility",
        PURPLE,
        "ready",
        &[s(
            "Accessibility",
            &[
                it("Automatically tag PDF", "tag", "a11y.autotag", "M12"),
                it("Change reading options", "book-open", "a11y.reading_options", "ready"),
                it("Check for accessibility", "accessibility", "a11y.check", "ready"),
                it("Open accessibility report", "file-text", "a11y.report", "ready"),
                it("Add alternate text", "image", "a11y.alt_text", "ready"),
                it("Fix reading order", "list-ordered", "a11y.reading_order", "M12"),
            ],
        )],
        None,
    ),
    g(
        "standards",
        "Apply PDF standards",
        "file-check",
        RED,
        "ready",
        &[s(
            "Standards",
            &[
                it("Save as PDF/A", "file-check", "standards.pdfa", "ready"),
                it("Save as PDF/X", "file-check", "standards.pdfx", "M11"),
                it("Save as PDF/UA", "accessibility", "standards.pdfua", "M11"),
                it("Preflight", "file-check", "preflight.run", "M11"),
            ],
        )],
        None,
    ),
    g(
        "search_index",
        "Add search index",
        "file-search",
        GREEN,
        "M12",
        &[s("Index", &[it("Manage embedded index", "file-search", "search.index", "M12")])],
        None,
    ),
    g(
        "javascript",
        "Use JavaScript",
        "braces",
        BLUE,
        "M6",
        &[s(
            "JavaScript",
            &[
                it("Console", "braces", "js.console", "M6"),
                it("Document JavaScripts", "file-text", "js.document", "M6"),
                it("Document actions", "list-checks", "js.actions", "M6"),
            ],
        )],
        None,
    ),
    g(
        "ai",
        "AI assistant",
        "sparkles",
        PINK,
        "ai",
        &[s(
            "Bring your own model (off by default)",
            &[
                it("Summarize", "sparkles", "ai.summary", "ai"),
                it("Ask about this document", "message-circle-reply", "ai.ask", "ai"),
                it("Translate", "languages", "ai.translate", "ai"),
            ],
        )],
        Some("Optional"),
    ),
];

pub fn group(id: &str) -> Option<&'static ToolGroup> {
    TOOL_GROUPS.iter().find(|g| g.id == id)
}

/// The five tools the Home tab recommends, in PrintCraft's order.
pub const RECOMMENDED: [&str; 5] = ["organize", "comment", "form", "edit", "protect"];

// ── Registered commands (⌘K palette, menus, shortcuts dialog) ──────────────────────────────

pub struct Command {
    pub id: &'static str,
    pub label: &'static str,
    pub menu: Option<&'static str>,
    pub icon: &'static str,
    pub shortcut: Option<&'static str>,
    pub enabled: bool,
}

const fn c(id: &'static str, label: &'static str, menu: &'static str, icon: &'static str, shortcut: &'static str, enabled: bool) -> Command {
    Command { id, label, menu: Some(menu), icon, shortcut: if shortcut.is_empty() { None } else { Some(shortcut) }, enabled }
}
const fn cp(id: &'static str, label: &'static str, icon: &'static str) -> Command {
    Command { id, label, menu: None, icon, shortcut: None, enabled: true }
}

pub static COMMANDS: &[Command] = &[
    c("file.open", "Open…", "File", "folder-open", "⌘O", true),
    c("file.save", "Save", "File", "save", "⌘S", true),
    c("file.save_as", "Save As…", "File", "save", "⇧⌘S", true),
    c("file.export", "Export a PDF…", "File", "file-output", "", true),
    c("file.print", "Print…", "File", "printer", "⌘P", true),
    c("file.revert", "Revert", "File", "rotate-ccw", "", true),
    c("file.close", "Close", "File", "x", "⌘W", true),
    c("edit.undo", "Undo", "Edit", "undo-2", "⌘Z", true),
    c("edit.redo", "Redo", "Edit", "redo-2", "⇧⌘Z", true),
    c("edit.copy", "Copy", "Edit", "copy", "⌘C", true),
    c("edit.select_all", "Select All", "Edit", "square-dashed", "⌘A", true),
    c("edit.find", "Find…", "Edit", "search", "⌘F", true),
    c("edit.search", "Advanced Search…", "Edit", "search", "⇧⌘F", true),
    c("edit.snapshot", "Take a Snapshot", "Edit", "camera", "", true),
    c("edit.preferences", "Preferences…", "Edit", "settings", "⌘,", true),
    c("page.organize", "Organize Pages", "Pages", "layout-grid", "", true),
    c("page.rotate", "Rotate Pages…", "Pages", "rotate-cw", "", true),
    c("page.delete", "Delete Pages", "Pages", "trash-2", "", true),
    c("page.extract", "Extract Pages…", "Pages", "file-output", "", true),
    c("page.insert", "Insert Pages…", "Pages", "file-plus-2", "", true),
    c("page.split", "Split Document…", "Pages", "scissors", "", true),
    c("page.number", "Number Pages…", "Pages", "tag", "", true),
    c("view.zoom_in", "Zoom In", "View", "zoom-in", "⌘+", true),
    c("view.zoom_out", "Zoom Out", "View", "zoom-out", "⌘−", true),
    c("view.actual_size", "Actual Size", "View", "maximize-2", "⌘1", true),
    c("view.fit_page", "Zoom to Page Level", "View", "columns-2", "⌘0", true),
    c("view.fit_width", "Fit to Width", "View", "maximize", "⌘2", true),
    c("view.rotate_cw", "Rotate View Clockwise", "View", "rotate-cw", "⇧⌘+", true),
    c("view.full_screen", "Full Screen Mode", "View", "app-window", "⌘L", true),
    c("view.read_mode", "Read Mode", "View", "book-open", "", true),
    c("doc.properties", "Document Properties…", "Help", "info", "⌘D", true),
    c("help.shortcuts", "Keyboard Shortcuts", "Help", "circle-help", "", true),
    c("help.about", "About PrintCraft", "Help", "info", "", true),
    c("help.discord", "Join our Discord", "Help", "messages-square", "", true),
    // Sub-panels, mirroring the rail.
    cp("panel.comments", "Comments", "message-square-text"),
    cp("panel.bookmarks", "Bookmarks", "bookmark"),
    cp("panel.pages", "Page Thumbnails", "files"),
    cp("panel.fields", "Form Fields", "text-cursor-input"),
    cp("panel.layers", "Layers", "layers"),
    cp("panel.attachments", "Attachments", "paperclip"),
    cp("panel.signatures", "Signatures", "signature"),
    cp("panel.search", "Advanced Search", "search"),
];

// ── The open document (the PrintCraft Showcase specimen) ───────────────────────────────────
// Page labels are the document's own (Cover, i, ii, 1…), as in the README.

pub static PAGES: &[(&str, &str)] = &[
    ("Cover", "Cover"),
    ("i", "Foreword"),
    ("ii", "Contents"),
    ("1", "Setting Text"),
    ("2", "Expressive Type"),
    ("3", "Scripts of the World"),
    ("4", "Code and Images"),
    ("5", "Color and Transparency"),
    ("6", "Forms and Layers"),
    ("7", "Accessibility"),
    ("8", "Signatures"),
    ("9", "Security"),
    ("10", "Colophon"),
];

pub struct Bookmark {
    pub label: &'static str,
    pub page: usize,
    pub children: &'static [Bookmark],
}
const fn b(label: &'static str, page: usize, children: &'static [Bookmark]) -> Bookmark {
    Bookmark { label, page, children }
}

pub static BOOKMARKS: &[Bookmark] = &[
    b("Cover", 0, &[]),
    b("Foreword", 1, &[]),
    b("Contents", 2, &[]),
    b("Setting Text", 3, &[b("Drop caps and pull quotes", 3, &[]), b("Kerning and ligatures", 3, &[])]),
    b("Expressive Type", 4, &[b("Display faces", 4, &[])]),
    b("Scripts of the World", 5, &[]),
    b("Code and Images", 6, &[]),
    b("Color and Transparency", 7, &[]),
    b("Forms and Layers", 8, &[]),
    b("Accessibility", 9, &[]),
    b("Signatures", 10, &[]),
    b("Security", 11, &[]),
    b("Colophon", 12, &[]),
];

pub struct Comment {
    pub page: usize,
    pub author: &'static str,
    pub time: &'static str,
    pub body: &'static str,
    pub tool: &'static str,
    pub selected: bool,
    pub state: Option<&'static str>,
    pub replies: &'static [Reply],
}
pub struct Reply {
    pub author: &'static str,
    pub time: &'static str,
    pub body: &'static str,
    pub status: bool,
}
const fn r(author: &'static str, time: &'static str, body: &'static str, status: bool) -> Reply {
    Reply { author, time, body, status }
}
#[allow(clippy::too_many_arguments)]
const fn cm(
    page: usize,
    author: &'static str,
    time: &'static str,
    body: &'static str,
    tool: &'static str,
    selected: bool,
    state: Option<&'static str>,
    replies: &'static [Reply],
) -> Comment {
    Comment { page, author, time, body, tool, selected, state, replies }
}

/// 20 threaded comments across the specimen (counts include replies).
pub static COMMENTS: &[Comment] = &[
    cm(
        3,
        "Marta Oliveira",
        "2 min ago",
        "The drop cap sits a hair low against the first line. Nudge the baseline by 1 pt?",
        "Note",
        true,
        None,
        &[r("Sam Whitfield", "1 min ago", "Agreed — I'll re-align it in the master.", false), r("Marta Oliveira", "just now", "Marked", true)],
    ),
    cm(3, "Sam Whitfield", "12 min ago", "Kerning between “Ty” and “pe” reads tight at 125%.", "Highlight", false, Some("Resolved"), &[]),
    cm(3, "Priya Nair", "26 min ago", "Can we set the pull quote in the display face?", "Text box", false, None, &[]),
    cm(4, "Marta Oliveira", "38 min ago", "This specimen line is a good candidate for the cover.", "Highlight", false, None, &[]),
    cm(4, "Luis Ferreira", "41 min ago", "Add a note about the optical size axis.", "Note", false, None, &[]),
    cm(
        5,
        "Priya Nair",
        "1 h ago",
        "Vertical Japanese is rendered on its own layer — please check the reading order.",
        "Highlight",
        false,
        Some("Resolved"),
        &[],
    ),
    cm(5, "Sam Whitfield", "1 h ago", "Devanagari shaping looks correct now.", "Underline", false, None, &[]),
    cm(6, "Luis Ferreira", "2 h ago", "The listing needs a 4-space tab stop.", "Note", false, None, &[]),
    cm(6, "Marta Oliveira", "2 h ago", "Fractal image: export at 600 dpi for print.", "Rectangle", false, None, &[]),
    cm(7, "Priya Nair", "3 h ago", "Transparency group flattens correctly against the gradient.", "Oval", false, None, &[]),
    cm(8, "Sam Whitfield", "3 h ago", "Tab order runs left-to-right, top-to-bottom. Good.", "Highlight", false, Some("Resolved"), &[]),
    cm(8, "Luis Ferreira", "4 h ago", "Add a required marker to the email field.", "Note", false, None, &[]),
    cm(9, "Marta Oliveira", "4 h ago", "Alt text for the figure is missing.", "Highlight", false, None, &[]),
    cm(9, "Priya Nair", "5 h ago", "Tags read in the right order; the table needs headers.", "Underline", false, None, &[]),
    cm(10, "Sam Whitfield", "5 h ago", "Signature is valid and the certificate chain is complete.", "Note", false, Some("Resolved"), &[]),
    cm(10, "Luis Ferreira", "6 h ago", "Add a visible signature line on the colophon.", "Text box", false, None, &[]),
    cm(11, "Priya Nair", "6 h ago", "Permissions: printing allowed, assembly restricted.", "Note", false, None, &[]),
    cm(12, "Marta Oliveira", "1 d ago", "Colophon: credit Lucide and the OFL fonts.", "Highlight", false, None, &[]),
    cm(12, "Sam Whitfield", "1 d ago", "Version line should read 0.2.1.", "Note", false, Some("Resolved"), &[]),
    cm(12, "Priya Nair", "1 d ago", "Ship it.", "Note", false, None, &[]),
];

pub struct Field {
    pub name: &'static str,
    pub label: &'static str,
    pub kind: &'static str,
    pub page: usize,
    pub value: &'static str,
    pub required: bool,
}
const fn f(name: &'static str, label: &'static str, kind: &'static str, page: usize, value: &'static str, required: bool) -> Field {
    Field { name, label, kind, page, value, required }
}

pub static FIELDS: &[Field] = &[
    f("full_name", "Full name", "Text field", 8, "Marta Oliveira", true),
    f("email", "Email", "Text field", 8, "marta@example.org", false),
    f("subscribe", "Subscribe", "Checkbox", 8, "Off", false),
    f("plan", "Plan", "Drop-down list", 8, "Studio", false),
    f("signature", "Signature", "Digital signature", 10, "—", false),
];

pub static LAYERS: &[(&str, bool, bool)] = &[
    ("Body text", true, false),
    ("Captions", true, false),
    ("Artwork", true, true),
    ("Japanese vertical", true, false),
    ("Printer marks", false, false),
];

pub static ATTACHMENTS: &[(&str, &str, &str)] = &[
    ("showcase-data.csv", "18 KB", "The tables in this specimen"),
    ("Inter-OFL.txt", "4 KB", "Font licence"),
    ("brand-kit.pdf", "1.2 MB", "Logos and colour chips"),
];

pub static SIGNATURES: &[(&str, &str, &str, &str)] = &[("Sam Whitfield", "Today, 09:14", "I am the author of this document", "ArtCraft Test CA")];

pub struct Hit {
    pub page: usize,
    pub before: &'static str,
    pub after: &'static str,
    /// The current match (10 of 16, as the README's screenshot shows).
    pub current: bool,
}
const fn h(page: usize, before: &'static str, after: &'static str, current: bool) -> Hit {
    Hit { page, before, after, current }
}

/// 16 hits for “type”, current match 10.
pub static SEARCH_HITS: &[Hit] = &[
    h(3, "…setting ", " is a matter of rhythm…", false),
    h(3, "…the ", " sits on a 12 pt baseline grid…", false),
    h(3, "…optical sizes let the ", " breathe at display sizes…", false),
    h(4, "…expressive ", " needs a display face…", false),
    h(4, "…contrast in ", " design…", false),
    h(4, "…a ", " face with a large x-height…", false),
    h(5, "…every writing system has its own ", " traditions…", false),
    h(6, "…set code in a monospaced ", " face…", false),
    h(6, "…the ", " of a variable font…", false),
    h(7, "…colour and ", " interact…", true),
    h(7, "…registration of ", " over images…", false),
    h(8, "…form fields use the interface ", " face…", false),
    h(9, "…readable ", " for screen readers…", false),
    h(10, "…the certificate names the ", " of signature…", false),
    h(11, "…permissions depend on the ", " of protection…", false),
    h(12, "…this ", " specimen was set in…", false),
];

pub static RECENT: &[(&str, &str, usize, &str)] = &[
    ("PrintCraft-Showcase.pdf", "~/Documents/printcraft/showcase.pdf", 13, "2.4 MB"),
    ("Expressive-Type-Specimen.pdf", "~/Documents/printcraft/specimen.pdf", 24, "8.1 MB"),
    ("Form-W9-filled.pdf", "~/Downloads/form-w9-filled.pdf", 2, "184 KB"),
    ("Annual-Report-2026.pdf", "~/Work/annual-report-2026.pdf", 96, "31 MB"),
];

/// The Help ▸ Keyboard Shortcuts dialog.
pub static SHORTCUTS: &[(&str, &str)] = &[
    ("⌘O", "Open…"),
    ("⌘S", "Save"),
    ("⇧⌘S", "Save As…"),
    ("⌘P", "Print…"),
    ("⌘Z", "Undo"),
    ("⇧⌘Z", "Redo"),
    ("⌘C", "Copy selected text"),
    ("⌘A", "Select all pages (Organize)"),
    ("⌘F", "Find"),
    ("⇧⌘F", "Advanced Search"),
    ("⌘G / ⇧⌘G", "Next / previous match"),
    ("⌘K", "Command palette"),
    ("⌘D", "Document properties"),
    ("⌘,", "Preferences"),
    ("⌘1", "Actual size"),
    ("⌘0", "Zoom to page level"),
    ("⌘2", "Fit to width"),
    ("⌘3", "Fit visible"),
    ("⌘+ / ⌘−", "Zoom in / out (also pinch or ⌘-scroll)"),
    ("⇧⌘+ / ⇧⌘−", "Rotate view"),
    ("Home / End", "First / last page"),
    ("⌘← / ⌘→", "Previous / next page"),
    ("V", "Select tool"),
    ("H", "Hand tool"),
    ("Delete", "Delete selected pages (Organize)"),
    ("Esc", "Clear selection / close find"),
    ("⌘L", "Full screen"),
    ("Double-click", "Select a word"),
];

pub const DOC_NAME: &str = "PrintCraft-Showcase.pdf";
pub const DOC_VERSION: &str = "0.2.1";
pub const DOC_AUTHOR: &str = "Marta Oliveira";

// ── Page bodies (the specimen's own content) ───────────────────────────────────────────────

pub enum Block {
    Kicker(&'static str),
    H1(&'static str),
    H2(&'static str),
    Lede(&'static str),
    Body(&'static str),
    Quote(&'static str),
    /// A specimen line; `true` sets it in the display face.
    Specimen(&'static str, bool),
    Code(&'static str),
    /// Gradient or solid swatches: (start, end).
    Swatches(&'static [(&'static str, &'static str)]),
    Figure(&'static str, &'static str),
    Columns(&'static str, &'static str),
    Footer(&'static str, &'static str),
}

const COVER_SWATCHES: &[(&str, &str)] =
    &[("#1b63e0", "#1b63e0"), ("#149ca8", "#149ca8"), ("#2d9d5b", "#2d9d5b"), ("#e88a1a", "#e88a1a"), ("#e03e3e", "#e03e3e")];
const COLOR_SWATCHES: &[(&str, &str)] = &[("#1b63e0", "#149ca8"), ("#e88a1a", "#e03e3e"), ("#2d9d5b", "#149ca8"), ("#8e4ee6", "#d63b8f")];

pub fn page_blocks(i: usize) -> Vec<Block> {
    use Block::*;
    match i {
        0 => vec![
            Kicker("An ArtCraft specimen"),
            H1("PrintCraft\nShowcase"),
            Lede(
                "Thirteen pages that exercise the parts of PDF that make a page feel right: world scripts, kerning, ligatures, gradients, transparency, forms and signatures.",
            ),
            Quote("“Every save appends your changes and leaves the original bytes untouched.”"),
            Swatches(COVER_SWATCHES),
            Footer("PrintCraft Showcase", "Cover"),
        ],
        1 => vec![
            Kicker("Foreword"),
            H2("Foreword"),
            Lede(
                "PrintCraft is a PDF workbench: it reads, organizes, combines, splits and secures PDFs in a fast, native app — with no account, no telemetry and no cloud.",
            ),
            Body(
                "This specimen exists so every surface of the application can be exercised against a document that behaves like the real thing: nested bookmarks, threaded comments, form fields, optional content, attachments and a digital signature.",
            ),
            Columns(
                "Deep zoom stays sharp because large pages render in tiles, so text stays crisp at any magnification. Pages render in isolation, and damaged documents are repaired rather than rejected.",
                "Layouts cover every task: continuous, single page, two-up, view rotation, full screen and a distraction-free Read mode. Both light and dark themes are designed for long sessions.",
            ),
            Footer("PrintCraft Showcase", "i"),
        ],
        2 => vec![
            Kicker("Contents"),
            H2("Contents"),
            Body("Cover · Foreword · Contents"),
            Body(
                "1 Setting Text — drop caps, kerning and ligatures\n2 Expressive Type — display faces and optical sizes\n3 Scripts of the World — twelve writing systems\n4 Code and Images — listings and rasters\n5 Color and Transparency — shadings and soft masks\n6 Forms and Layers — optional content\n7 Accessibility — tags and reading order\n8 Signatures — certificates and validation\n9 Security — permissions and encryption\n10 Colophon",
            ),
            Footer("PrintCraft Showcase", "ii"),
        ],
        3 => vec![
            Kicker("Chapter one"),
            H2("Setting Text"),
            Lede(
                "Type sits on a baseline grid; the page is a rhythm before it is a picture. Set the measure first, then the leading, then the size — in that order, and the page holds together at any zoom.",
            ),
            Body(
                "Kerning and ligatures are applied from the font's own tables. Where a run is right-to-left, or set vertically, the reading order follows the script rather than the page.",
            ),
            Quote("Set the measure, then the leading, then the size."),
            Specimen("Hamburgefonstiv — fi fl ffi ffl · Ty, To, Wa, AV", false),
            Footer("PrintCraft Showcase", "1"),
        ],
        4 => vec![
            Kicker("Chapter two"),
            H2("Expressive Type"),
            Lede(
                "A display face earns its place by what it does to a short line. At 125% the counters close, the stems carry weight, and the line becomes a shape.",
            ),
            Specimen("Expressive Type", true),
            Body("Optical size axes let one family do the work of three: caption, text and display, each with the contrast its size demands."),
            Footer("PrintCraft Showcase", "2"),
        ],
        5 => vec![
            Kicker("Chapter three"),
            H2("Scripts of the World"),
            Lede("Twelve writing systems on one page, plus vertical Japanese in the right margin."),
            Specimen("العربية · עברית · देवनागरी · ไทย · Ελληνικά · Кириллица\n中文 · 한국어 · IPA · Հայերեն · ქართული · தமிழ்", false),
            Body("Vertical CJK, colour emoji and complex script shaping all render the way the author intended, at any magnification."),
            Footer("PrintCraft Showcase", "3"),
        ],
        6 => vec![
            Kicker("Chapter four"),
            H2("Code and Images"),
            Body(
                "Set code in a monospaced type face; set prose in a text face. The listing below is syntax-coloured and survives copy and paste in the right reading order.",
            ),
            Code(
                "fn main() -> Result<(), Error> {\n    let doc = Document::open(\"showcase.pdf\")?;\n    for page in doc.pages() {\n        println!(\"{} — {} pt\", page.label(), page.width());\n    }\n    Ok(())\n}",
            ),
            Figure("#1b63e0", "Figure 1 — a raster draws through a shader; the surrounding text keeps its own colour."),
            Footer("PrintCraft Showcase", "4"),
        ],
        7 => vec![
            Kicker("Chapter five"),
            H2("Color and Transparency"),
            Lede("Shadings, blend modes and soft masks compose in the order the file declares them, not the order a viewer happens to draw."),
            Swatches(COLOR_SWATCHES),
            Body("A transparency group flattens only where it must; a soft mask keeps its own alpha channel."),
            Footer("PrintCraft Showcase", "5"),
        ],
        8 => vec![
            Kicker("Chapter six"),
            H2("Forms and Layers"),
            Lede(
                "Prepare a form once: name the fields, order the tabs, flatten when the data is in. Optional content groups let one file carry several printed versions.",
            ),
            Body("Fields in this document\nFull name · Email · Subscribe · Plan · Signature"),
            Body("Layers\nBody text · Captions · Artwork · Japanese vertical · Printer marks"),
            Footer("PrintCraft Showcase", "6"),
        ],
        9 => vec![
            Kicker("Chapter seven"),
            H2("Accessibility"),
            Lede("A tagged PDF reads in the order the author intends, on screen readers and in the copy buffer alike."),
            Body("Headings nest, figures carry alternate text and tables name their headers. Run the accessibility checker to see what remains."),
            Footer("PrintCraft Showcase", "7"),
        ],
        10 => vec![
            Kicker("Chapter eight"),
            H2("Signatures"),
            Lede("A signature is a byte range plus a certificate. Validation is a verification of both, and of the chain behind it."),
            Body("Signed by Sam Whitfield · Today, 09:14\nReason I am the author of this document\nIssuer ArtCraft Test CA"),
            Body("DocMDP permissions recorded with the signature: form filling and commenting allowed, assembly restricted."),
            Footer("PrintCraft Showcase", "8"),
        ],
        11 => vec![
            Kicker("Chapter nine"),
            H2("Security"),
            Lede("Open a protected document and respect its rules: printing may be allowed while assembly is not."),
            Body("Encryption is standard security handler revision 6, AES-256. The original bytes are never rewritten in place."),
            Footer("PrintCraft Showcase", "9"),
        ],
        _ => vec![
            Kicker("Colophon"),
            H2("Colophon"),
            Body("PrintCraft Showcase — set in Inter and JetBrains Mono, with Dancing Script for the display page."),
            Body("Rendering: hayro · UI: egui · Icons: Lucide (ISC) · Fonts: Inter, JetBrains Mono, Dancing Script (OFL)."),
            Body("Version 0.2.1 · MIT OR Apache-2.0"),
            Footer("PrintCraft Showcase", "10"),
        ],
    }
}

// ── The quick-action bar ───────────────────────────────────────────────────────────────────
// Vertical floating strip; a group opens a flyout on click.

pub struct QuickTool {
    pub id: &'static str,
    pub label: &'static str,
    pub icon: &'static str,
    pub command: &'static str,
}
const fn q(id: &'static str, label: &'static str, icon: &'static str, command: &'static str) -> QuickTool {
    QuickTool { id, label, icon, command }
}
pub struct QuickGroup {
    pub name: &'static str,
    pub icon: &'static str,
    pub tools: &'static [QuickTool],
}

pub static COMMENT_GROUPS: &[QuickGroup] = &[
    QuickGroup {
        name: "Comment",
        icon: "message-square-text",
        tools: &[
            q("note", "Sticky note", "sticky-note", "comment.note"),
            q("textbox", "Text box", "type", "comment.freetext"),
            q("callout", "Callout", "message-square-quote", "comment.callout"),
            q("attach", "Attach file", "paperclip", "comment.attach"),
        ],
    },
    QuickGroup {
        name: "Highlight",
        icon: "highlighter",
        tools: &[
            q("highlight", "Highlight text", "highlighter", "comment.highlight"),
            q("underline", "Underline", "underline", "comment.underline"),
            q("strikeout", "Strikethrough", "strikethrough", "comment.strikeout"),
            q("squiggly", "Squiggly underline", "spline", "comment.squiggly"),
            q("caret", "Insert text", "text-select", "comment.caret"),
            q("replace", "Replace text", "replace", "comment.replace"),
        ],
    },
    QuickGroup {
        name: "Draw",
        icon: "pencil",
        tools: &[
            q("ink", "Draw freehand", "pencil", "comment.ink"),
            q("line", "Line", "minus", "comment.line"),
            q("arrow", "Arrow", "move-right", "comment.arrow"),
            q("rect", "Rectangle", "square", "comment.square"),
            q("oval", "Oval", "circle", "comment.circle"),
            q("polyline", "Polyline", "pen-tool", "comment.polyline"),
            q("polygon", "Polygon", "pentagon", "comment.polygon"),
            q("cloud", "Cloud", "cloud", "comment.cloud"),
            q("eraser", "Eraser", "eraser", "comment.eraser"),
        ],
    },
];

pub static FILL_TOOLS: &[QuickTool] = &[
    q("text", "Add text", "type", "sign.fill.text"),
    q("check", "Checkmark", "check", "sign.fill.check"),
    q("cross", "Cross", "x", "sign.fill.cross"),
    q("dot", "Dot", "circle-dot", "sign.fill.dot"),
    q("line", "Line", "minus", "sign.fill.line"),
    q("date", "Date", "clock-3", "sign.fill.date"),
    q("signature", "Signature", "signature", "sign.fill.signature"),
    q("initials", "Initials", "signature", "sign.fill.initials"),
];

/// Single-tool buttons after the Fill & Sign group (canvas.rs::quick_bar).
pub static QUICK_TOOLS: &[QuickTool] = &[
    q("crop", "Crop pages", "crop", "page.crop"),
    q("redact", "Redact text and images", "rectangle-horizontal", "redact.mark"),
    q("editText", "Edit text & images", "text-cursor-input", "edit.edit_text"),
    q("addText", "Add text", "type", "edit.text"),
    q("link", "Link", "link-2", "edit.link"),
    q("sign", "Digitally sign", "signature", "sign.digital"),
    q("marquee", "Marquee zoom", "maximize-2", "view.marquee_zoom"),
    q("snapshot", "Take a snapshot", "camera", "edit.snapshot"),
];
