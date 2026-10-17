# PrintCraft design demo — UI components

Every component in this crate, what it looks like and what it does when you use it.
The demo is a faithful port of the PrintCraft egui shell (`crates/ui-egui` in the upstream
repository) driven by the design mock-up in `../desktop-design` (HTML/CSS/JS). All visual
values come from the design tokens in [`src/theme.rs`](../src/theme.rs); no widget ever
invents a colour.

- **Source of truth for looks:** `theme.rs` (tokens, port of `crates/ui-egui/src/theme.rs`).
- **Source of truth for content:** `src/data.rs` (port of the mock-up's `js/data.js`).
- **Shell order (top to bottom, right to left):** tab strip → mode bar → right rail →
  right panel → left panel → document area → overlays (palette, dialogs, toast).

```
┌──────────────────────────────────────────────────────────────────────────────┐
│ 1 tab strip        Home │ ● PrintCraft-Showcase.pdf ✕ │ + Open   …theme ⓘ 💬  │
├──────────────────────────────────────────────────────────────────────────────┤
│ 2 mode bar   ☰ Menu │ All tools Read Edit Convert E-Sign │ …print save ⓘ 🔍⌘K │
├────────────┬────────────────────────────────────────────────┬──────────┬─────┤
│ 3 left     │ 4 document area                                │ 6 right  │ 5   │
│   tool     │   ┌ notice bar ──────────────────────────────┐  │   side   │ rail│
│   panel    │   │ quick bar ▮   page card (specimen)  find │  │   panel  │     │
│            │   └──────────────────────────────────────────┘  │          │     │
├────────────┴────────────────────────────────────────────────┴──────────┴─────┤
│ 8 ⌘K palette · 9 dialogs (shortcuts/about/properties/preferences) · 10 toast │
└──────────────────────────────────────────────────────────────────────────────┘
```

---

## Design tokens — `src/theme.rs`

| Token group | Members | Notes |
| --- | --- | --- |
| Surfaces | `titlebar`, `chrome`, `panel`, `pasteboard`, `card`, `field` | Light gray and Dark gray themes; pages stay white in both. |
| Lines | `border`, `divider` | 1 px separators between bands and panels. |
| Text | `text`, `text_muted`, `text_faint` | Contrast-checked at ≥ 4.5:1 against every surface. |
| Interaction | `hover`, `pressed`, `selected`, `icon`, `accent`, `accent_text`, `accent_soft` | One blue accent; `accent_soft` is the wash for selection and callouts. |
| Status | `ready_bg`/`ready_fg`, `badge_new`, `page_shadow`, `toast_bg`/`toast_fg`, `scrim` | Availability chips, new-tool badge, page shadow, toasts and modal backdrop. |
| Geometry | `radius` (6) | Panels use 10–12 pt for cards, 14–16 pt for pills. |
| Type | `Inter`, `Inter-Medium`, `Inter-SemiBold`, `JetBrains Mono`, `Dancing Script` | Named font families `medium`, `semibold`, `display`, `monospace`. |

`theme::apply(ctx, kind)` installs the tokens, an egui `Visuals`, and the global style
(item spacing, scroll bar width, tooltip delay 0.35 s). `Tokens::get(ctx)` is how widgets read
them. `ThemeKind` is persisted, so the app reopens in the theme you left it in.

---

## 1. Tab strip — `components/chrome.rs::tab_strip`

The window's top band (38 pt, `titlebar` fill).

| Element | Function in the demo |
| --- | --- |
| **Home button** (`house`) | Switches to the Home tab (no document). Selected state = accent wash. |
| **Document tab** | Shows the open file (`PrintCraft-Showcase.pdf`) with a PDF glyph, an unsaved-changes dot that becomes a close ✕ on hover, and a tooltip. Clicking activates the viewer. Closing is visual only. |
| **Open** (`plus`) | Ghost button; opens the native file dialog (`rfd`) and toasts the chosen path. |
| **Theme toggle** (`moon`/`sun`) | Switches Light gray ⇄ Dark gray and persists the choice. |
| **Keyboard shortcuts** (`circle-help`) | Opens the Keyboard Shortcuts dialog. |
| **Discord** | Community link button (toast in the demo). |

## 2. Mode bar — `components/chrome.rs::mode_bar`

The second band (48 pt, `chrome` fill) holds four groups.

| Element | Function in the demo |
| --- | --- |
| **Menu** (`☰`) | Opens the main popup: **File**, **Edit**, **Pages**, **View**, **Help** as submenus. |
| **Registered commands** — `chrome.rs::registry_menu` | Lists every command of one menu from `data::COMMANDS` with its shortcut (`⌘S`, `⇧⌘Z`, …). Document commands are disabled while no document is open. Clicking runs the command and closes the menu. |
| **View menu extras** | Live zoom presets, a *Display theme* radio pair, a *Side panels* submenu (radio list of all eight panels) and Read / Full Screen. |
| **Workspace modes** — `widgets::mode_tab` | `All tools · Read · Edit · Convert · E-Sign`. The active tab is semibold with a 3 px underline. Each mode preselects the matching tool panel (Edit → *Edit a PDF*, Convert → *Export a PDF*, E-Sign → *Fill & Sign*); **Read** hides the left panel, right panel and floating tools for distraction-free reading. |
| **Document actions** (`printer`, `save`, `info`) | Print (toast), Save (toast), Document Properties dialog. Disabled without a document. |
| **Command search** — `widgets::search_box` | A field-shaped button labelled *Find tools and commands* with the `⌘K` hint. Clicking (or `⌘K`) opens the command palette. |

## 3. Left tool panel — `components/panels.rs::left_panel`

272 pt wide, `panel` fill, with its own header and scrollable body.

| Element | Function in the demo |
| --- | --- |
| **Panel header** — `panels::panel_header` | Title (semibold 15.5), an optional trailing count (the Comments panel shows *Comments 22*), optional **back** (`chevron-left`, tool detail only) and **close** (`x`) buttons. |
| **Tool list** — `panels::tool_row` | One row per catalogue group: Lucide icon tinted with the group's hue, label, and either an *Optional* badge (AI assistant) or a hover-only `Planned · M<n>` hint. Clicking opens the group. |
| **View more / View less** | Expands the list from the first 14 groups to all 26. |
| **Tool detail** — `panels::tool_detail` | The group's sections (uppercase `widgets::section_title`) with one row per item: hue-tinted icon, label, and a chip — **Ready** (green), the milestone (`M10`, grey) or **AI**. Hovering shows the internal command id. Clicking runs the command. |
| **Milestone callout** | For planned groups, an `accent_soft` card explains that items marked Ready work today. |

The catalogue has 26 groups and ~150 items, mirroring `crates/engine/src/catalog.rs`
(export, fill & sign, edit, create, combine, organize, comment, scan & OCR, protect, redact,
compress, form, stamp, certificate, print production, measure, compare, guided actions,
accessibility, standards, search index, JavaScript, AI assistant, …).

## 4. Document area — `components/canvas.rs`

The central panel (`pasteboard` fill). It owns four independent pieces.

### 4a. Notice bar — `canvas::document_area`
`accent_soft` strip above the pages: info icon, message (“Repaired 3 damaged objects while
opening…”), a **Details** link (toast) and a dismiss ✕ that hides the bar for the session.

### 4b. Page stack — `canvas::pages` + `canvas::draw_page`
Each page is a white card (816×1056 pt scaled by the zoom) with a soft shadow. The specimen
content is painted from a block list (`data::page_blocks`), one renderer arm per block:

| Block | Rendering |
| --- | --- |
| `Kicker`, `H1`, `H2`, `Lede`, `Body` | Wrapped text with the page's own palette (ink `#1A1A1E`, muted `#6B6B73`). |
| `Quote` | 19 pt line with a 3 px accent rule. |
| `Specimen(text, display)` | 30 pt, or 44 pt Dancing Script when `display` is set. |
| `Code` | Monospaced listing in a rounded `#F4F4F6` panel. |
| `Swatches` | Row of solid/gradient colour chips. |
| `Figure` | Gradient artwork (approximated with a 24-step horizontal gradient) plus caption. |
| `Columns` | Two balanced text columns. |
| `Footer` | Document name left, page label right. |

Clicking a page makes it current; double-clicking enters Read mode. Rotated pages show a
rotated placeholder card.

### 4c. Quick-action bar — `canvas::quick_bar`
A floating card at the top-left of the document area.

* **Groups** — Comment, Highlight, Draw, and Fill & Sign. Clicking a group selects it (accent
  wash) and opens a flyout to the right listing its tools; clicking a tool selects it and
  toasts the command. Clicking the active group again closes the flyout.
* **Single tools** — crop, redact, edit text, add text, link, digitally sign, marquee zoom,
  take a snapshot, each an icon button with a tooltip.
* Hidden in Read mode and in the Organize grid.

### 4d. Find bar — `canvas::find_bar`
A floating card at the top-right: search icon, text field (pre-filled with “type”), a
`10 of 16`-style counter, previous/next chevrons and a close ✕. It is the visible face of
`data::SEARCH_HITS`; stepping moves the current match and jumps the document to its page.
`⌘F` opens it, `Esc` closes it.

### 4e. Organize pages grid — `canvas::organize`
Replaces the page stack when `page.organize` runs.

* **Toolbar** — page count, rotate / delete / insert blank / extract / split buttons, the
  selection summary, **Select all** and **Done**.
* **Grid** — auto-fitting cells with a white paper card, a wireframe preview
  (`canvas::mini_page`), the document's own page label and its bookmark name. Click selects,
  `⌘`-click toggles, `⇧`-click extends a range; a double-click opens the page in the viewer.
* Real edits: rotate and delete change the in-memory page list, insert adds a blank page, and
  the selection count updates live.

## 5. Right rail — `components/chrome.rs::right_rail`

48 pt wide, `chrome` fill.

| Element | Function |
| --- | --- |
| **Panel buttons** | Comments, Bookmarks, Page thumbnails, Form fields, Layers, Attachments, Signatures, Advanced Search. An accent dot marks a panel that has content; clicking selects (accent wash) or toggles the panel off. |
| **Zoom cluster** | Zoom out, zoom in, rotate view, toggle fit-page/fit-width, and the current zoom percentage. |
| **Page navigation** | Previous/next page chevrons, the page count, and a page field that accepts either a number or the document's own label (`i`, `ii`, `Cover`) and jumps on `Enter`. |

## 6. Right side panel — `components/panels.rs::right_panel`

330 pt wide; the content follows the rail selection. All eight panels share the header
component from §3.

| Panel | What it shows / does |
| --- | --- |
| **Comments** — `panels::comments` | Grouped by page (*Page 1 · Setting Text* with the page's own thread count on the right); each comment is a card with an initials avatar, author, relative time, body, tool tag and status chip (`Resolved`, `Marked`). The selected card gets the accent border plus a 3 px `#0165DD` bar flush with its left edge and exactly its own height (drawn from the frame rect, so it can never bleed between cards). Replies hang off the parent on a hairline thread line. Double-clicking a card toggles Resolved; the reply box at the bottom appends a reply (“You · just now”) and toasts. Clicking a card also jumps the document to that page. |
| **Bookmarks** — `panels::bookmarks` | The nested outline (13 top-level entries with children) with twist buttons, page labels on the right, and click-to-jump. |
| **Page Thumbnails** — `panels::thumbnails` | Two-column grid of page cards with the same wireframe preview as the Organize grid, the page label, and the bookmark name; the current page is outlined in the accent colour. |
| **Form Fields** — `panels::fields` | One row per field: type icon, label, `kind · value`, and the field name in monospace. A *Tab order* section documents the row order, with **Preview** and **Flatten** actions. Clicking jumps to the field's page. |
| **Layers** — `panels::layers` | Optional-content rows with visibility (`eye`/`eye-off`) and lock (`lock`/`lock-open`) glyphs; hidden layers are dimmed. |
| **Attachments** — `panels::attachments` | File rows with name, description, size and a save action. |
| **Signatures** — `panels::signatures` | A card per signature: signer, timestamp, a green **Valid** chip, reason and issuer, plus **Validate all signatures**. |
| **Advanced Search** — `panels::search_results` | A search field plus all 16 hits for “type”, grouped by page, with the matching word highlighted in yellow. Clicking a hit selects it and jumps to its page (shares state with the find bar). |

## 7. Home tab — `components/home.rs::show`

Shown when no document tab is active (the `house` button).

| Element | Function |
| --- | --- |
| **Welcome block** | Product name and one-line description. |
| **Community card** | ArtCraft mark, blurb and pill links (Discord, website, documentation). |
| **Recommended tools** | Five tiles (Organize, Comment, Form, Edit, Protect) with the group's hue icon, the first three item names, and “Use now”; clicking opens the matching tool panel and the document tab. |
| **Open file tile** | Runs the native open dialog. |
| **More tools** | One pill per catalogue group (all 26) that jumps straight to that tool panel. |
| **Recent** | Four files with path, page count and size; clicking reopens the specimen. |
| **Privacy note** | Static text: offline, no account, no telemetry. |

## 8. Command palette — `components/palette.rs::show`

Centred card (`⌘K`, the search box, or *Edit ▸ …*): a focused search field over a ranked list
of every registered command (with its shortcut) and every catalogue tool, filtered by
substring then subsequence match, capped at 12 hits. `Enter` runs the top hit, clicking runs
that hit, `Esc` or a click outside closes. Choosing a tool also opens its left panel.

## 9. Dialogs — `components/dialogs.rs::show`

All four use `egui::Modal` (centred, `scrim` backdrop, dismiss on backdrop click or `Esc`).

| Dialog | Contents |
| --- | --- |
| **Keyboard Shortcuts** | Two-column grid of all 28 shortcuts from `data::SHORTCUTS`. |
| **About PrintCraft** | Mark, version, description, community pills, licence line. |
| **Document Properties** | Five tabs (Description, Initial View, Security, Fonts, Advanced) rendered with `mode_tab` and a key/value grid; the selected tab is remembered in egui memory. |
| **Preferences** | Theme radios (live), Rendering checkboxes (smooth scrolling, hardware acceleration, follow system theme) bound to `App::prefs`, and the offline note. |

## 10. Toast — `widgets::toast`

Transient message at the bottom centre (dark pill in the light theme, light pill in the dark
theme) that auto-dismisses after 3.5 s. Every stubbed action reports here, so the demo is
never silent about what was clicked.

## 11. Tooltips

Every icon button and most rows carry a tooltip. egui's delay is set from the design tokens
(0.35 s) in `theme::apply`, matching the mock-up's `.tooltip`.

## 12. Icons — `src/icons.rs`, `src/icon_data.rs`

183 Lucide SVGs (ISC licence, `assets/icons/`) are embedded with `include_bytes!`, recoloured
to white at startup (`currentColor` → `#ffffff`, stroke width 1.75), rasterized by
`egui_extras`' SVG loader at the exact on-screen size and tinted per use. `icons::button` is
the standard square icon button (transparent → hover → pressed, accent wash when selected);
`icons::paint` draws an icon into an existing rect. Named families never use Unicode glyphs.

---

## Shared widgets — `src/widgets.rs`

| Widget | Used by |
| --- | --- |
| `mode_tab` | Mode bar, Document Properties tabs. |
| `pill_button` (primary/secondary) | Organize toolbar, forms, signatures, dialog footers, Home pills. |
| `icon_pill` | Home community card, About dialog. |
| `ghost_button` (icon + label) | Tab strip (Open, Discord), mode bar (Menu). |
| `search_box` | Mode bar command search. |
| `text_field` | Reply box, panel search, dialog and panel inputs. |
| `menu_item` | Registered-command rows. |
| `section_title` | Left tool panel, Home, Forms. |
| `chip` | Availability, status and “Marked” chips. |
| `link` | “View more”, “Details”. |
| `card_frame` | Home cards, signature cards. |
| `toast` | Global notifications. |

## Interactions and shortcuts

| Input | Effect |
| --- | --- |
| `⌘K` | Command palette (also the mode-bar *Find tools and commands* box) |
| `⌘F` | Find bar |
| `⇧⌘F` | Advanced Search panel (also **Edit ▸ Advanced Search…**) |
| `⌘G` / `⇧⌘G` | Next / previous match |
| `⌘O` | Open file dialog |
| `⌘S` | Save (toast) |
| `⌘D` | Document Properties |
| `⌘,` | Preferences |
| `⌘+` / `⌘−` / `⌘0` / `⌘1` / `⌘2` | Zoom in / out / page level / 100 % / fit width |
| `⇧⌘+` | Rotate the current page |
| `Esc` | Close palette → dialog → find bar → clear Organize selection, in that order |
| Click / `⌘`-click / `⇧`-click (Organize) | Select a page / toggle / extend a range |
| Double-click a comment | Toggle Resolved |
| Double-click a page or an Organize cell | Open it in the viewer |

## What is real, and what is a stub

**Real in the demo:** theme switching and persistence; workspace modes; tool catalogue
browsing and search; command palette and menus; panel switching; comment selection, replies
and resolution; bookmark and search-result navigation; page rotation, deletion and insertion
in Organize; zoom and page navigation; find match stepping; dialogs.

**Stubbed (toast only):** anything that needs the PDF engine or the network — opening a real
file, saving, exporting, printing, OCR, redaction marks, signatures, form filling, layer
visibility, attachments and the community links.

## Module map

```
src/
  main.rs            window creation (viewport, app id) — mirrors apps/printcraft/src/main.rs
  lib.rs             module list + headless smoke tests
  app.rs             App state, Mode/Panel/LeftPanel/Dialog, commands, shortcuts, persistence
  theme.rs           Tokens, fonts, egui style
  icons.rs           embedded Lucide icons, tinting, icon button
  icon_data.rs       generated include_bytes! table
  data.rs            tool catalogue, commands, specimen pages/comments/fields/…, quick tools
  widgets.rs         the shared controls listed above
  components/
    chrome.rs        tab strip, mode bar (menus, modes, actions, search), right rail
    panels.rs        left tool panel + all eight right panels
    canvas.rs        notice bar, page stack, quick bar, find bar, Organize grid
    home.rs          Home tab
    palette.rs       ⌘K command palette
    dialogs.rs       Keyboard Shortcuts, About, Properties, Preferences
```
