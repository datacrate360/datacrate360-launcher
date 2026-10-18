# Attribution

Every non-code asset in this repository, with its author, source and license. DataCrate360
Launcher's own code is MIT OR Apache-2.0 (see [`LICENSE-MIT`](LICENSE-MIT),
[`LICENSE-APACHE`](LICENSE-APACHE) and [`NOTICE`](NOTICE)). When you add an asset, add a row here
in the same change; third-party assets must be permissively licensed and keep their license file
next to them.

## Bundled in the app

| Path | Title | Author | Source | License |
|---|---|---|---|---|
| `assets/brand/logo.png`, `assets/brand/logo-mark.png` | DataCrate360 logo (full-bleed, 1024 px) and mark (transparent, 192 px) | DataCrate360 | datacrate360.com (`frontend/public/logo.png`, `logo-mark.png`), copied unmodified | DataCrate360 brand asset, **not** covered by MIT or Apache-2.0 (see [`NOTICE`](NOTICE)) |
| `assets/app-icon/` (all files) | App icon (PNG sizes, `.icns`, `.ico`), tray icons, header mark | DataCrate360 | Rendered from `assets/brand/` by [`packaging/icons.sh`](packaging/icons.sh) (`sips` + `cargo xtask ico`/`icns`) | As `assets/brand/` |
| `assets/fonts/Inter-Variable.ttf` | Inter 4.001, variable (body text) | The Inter Project Authors (Rasmus Andersson) | Google Fonts, <https://github.com/google/fonts/tree/main/ofl/inter> (`Inter[opsz,wght].ttf`, renamed) | SIL OFL 1.1, [`assets/fonts/OFL-Inter.txt`](assets/fonts/OFL-Inter.txt) |
| `assets/fonts/Archivo-Variable.ttf` | Archivo 2.001, variable (headings and app names) | The Archivo Project Authors (Omnibus-Type) | Google Fonts, <https://github.com/google/fonts/tree/main/ofl/archivo> (`Archivo[wdth,wght].ttf`, renamed) | SIL OFL 1.1, [`assets/fonts/OFL-Archivo.txt`](assets/fonts/OFL-Archivo.txt) |
| `assets/fonts/GeistMono-Variable.ttf` | Geist Mono 1.701, variable (labels and buttons) | The Geist Project Authors (Vercel) | Google Fonts, <https://github.com/google/fonts/tree/main/ofl/geistmono> (`GeistMono[wght].ttf`, renamed) | SIL OFL 1.1, [`assets/fonts/OFL-GeistMono.txt`](assets/fonts/OFL-GeistMono.txt) |

## Documentation

| Path | Title | Author | Source | License |
|---|---|---|---|---|
| `docs/screenshots/launcher.png` | DataCrate360 Launcher on macOS (grid view, light theme) | DataCrate360 Launcher contributors | Saved by the running app itself (`DATACRATE360_LAUNCHER_SELF_SCREENSHOT`) | MIT OR Apache-2.0; the logo in it stays a DataCrate360 brand asset |

## Upstream

The code, the fonts above and the i18n catalogs come from ArtCraft Launcher
(<https://github.com/storytold/craft-launcher>, MIT OR Apache-2.0, Copyright (c) 2026 ArtCraft
Team and the ArtCraft Launcher contributors). Its brand files, app icons, social icons, DMG
artwork and screenshots were removed and are not part of this repository.
