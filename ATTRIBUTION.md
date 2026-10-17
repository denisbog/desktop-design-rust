# Attribution

This crate is a UI design demo of [PrintCraft](https://github.com/storytold/printcraft) by
Storytold / the ArtCraft Team. It is a clean-room port of the application's egui shell
(`crates/ui-egui`) and of the companion design mock-up (`desktop-design`, HTML/CSS/JS), used
here for study and as a runnable reference. No engine code, PDF renderer or brand asset is
included.

## Third-party assets

| Asset | Source | Licence |
| --- | --- | --- |
| `assets/icons/*.svg` (183 Lucide icons) | [Lucide](https://lucide.dev) | ISC — see `assets/icons/LICENSE-lucide.txt` |
| `assets/fonts/Inter-*.ttf` | [Inter](https://rsms.me/inter/) | SIL Open Font License 1.1 — see `assets/fonts/OFL-Inter.txt` |
| `assets/fonts/JetBrainsMono-Regular.ttf` | [JetBrains Mono](https://www.jetbrains.com/lp/mono/) | SIL Open Font License 1.1 — see `assets/fonts/OFL-JetBrainsMono.txt` |
| `assets/fonts/DancingScript.ttf` | [Dancing Script](https://github.com/impallari/DancingScript) | SIL Open Font License 1.1 — see `assets/fonts/OFL-DancingScript.txt` |

## Code

| Crate | Licence |
| --- | --- |
| `egui`, `eframe`, `egui_extras` | MIT OR Apache-2.0 |
| `serde`, `serde_json` | MIT OR Apache-2.0 |
| `rfd` | MIT OR Apache-2.0 |
| `log` | MIT OR Apache-2.0 |

## Trademarks

The ArtCraft and PrintCraft names and logos are trademarks of the ArtCraft Team. This demo
does not ship their brand assets; it uses the application's own icon set, copy and specimen
content to reproduce the interface for design study. This crate is licensed MIT OR
Apache-2.0, matching the upstream source it is ported from.
