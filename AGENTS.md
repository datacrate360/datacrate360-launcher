# AGENTS.md: guide for AI agents and contributors

DataCrate360 Launcher installs, updates and launches the **DataCrate360 tools** (today:
Countercheck). It is written in **Rust only** (no JavaScript or TypeScript): the desktop app is
native egui/eframe, and the web build is the same Rust compiled to WebAssembly (trunk +
wasm-bindgen). **No Tauri, Electron or webview shells**, ever.

The product name is always **DataCrate360 Launcher** in user-facing text: UI, window titles,
About, tray menu, notifications, installers, release names, docs prose. The suite is the
**DataCrate360 tools**. Machine names stay lowercase: crates and binaries
(`datacrate360-launcher`, `datacrate360-launcher-*`), file names, and the app id
`com.datacrate360.launcher`. The GitHub repo is `datacrate360/datacrate360-launcher`; the website
is <https://datacrate360.com>. Environment variables are `DATACRATE360_LAUNCHER_*`.

This project is derived from an upstream MIT OR Apache-2.0 launcher (credited in `NOTICE`). The
upstream shared-standards repo it referred to is not part of this project; where its rules
mattered they are written out below.

## 1. Workspace map

```text
crates/
  core      datacrate360-launcher-core      pure and wasm-safe: the tool catalog, versions, GitHub
                                            release parsing, picking the right asset per platform
  native    datacrate360-launcher-native    desktop only: HTTP, detecting installed tools,
                                            download, verify and install, self-update
  ui-egui   datacrate360-launcher-ui-egui   the shared egui UI (desktop and web)
apps/
  datacrate360-launcher                     desktop shell (eframe): window, tray icon, notifications
  datacrate360-launcher-web                 the same UI in the browser (trunk; catalog and downloads only)
xtask/                                      cargo xtask ci | wasm | version | ico | icns | local
packaging/                                  macOS, Windows, Linux, FreeBSD and web packaging scripts
assets/brand/                               DataCrate360 logo (not MIT/Apache; see NOTICE)
assets/app-icon/                            icons rendered from assets/brand/ by packaging/icons.sh
assets/fonts/                               Archivo, Inter, Geist Mono (OFL-1.1)
```

Layering: `core` depends on no other workspace crate and must build for `wasm32-unknown-unknown`.
`native` may use `core`. `ui-egui` may use `core` (and `native` only behind
`cfg(not(target_arch = "wasm32"))` or through a trait the desktop shell implements). Nothing below
the apps opens windows, tray icons or notifications.

The catalog is data: adding a tool is one row in `SUITE` (`crates/core/src/catalog.rs`), its
tagline row (context = tool id) in every `crates/ui-egui/src/i18n/*.tsv`, and optionally a
details page (`crates/core/src/details.rs`) and a `desktop_entry::meta` row. Tests use the
stand-in catalog `catalog::fixtures::CATALOG` (13 tools; the real ones first) and
`details::fixtures`, behind the core crate's `fixtures` feature, which only dev-dependencies
enable. `LauncherUi::with_catalog` shows a catalog other than `SUITE`.

## 2. Golden rules

### Never crash (outranks feature work)

A bad network response, a malformed release, a half-downloaded file, a full disk or a corrupt
settings file must produce an error the user can act on, never a panic.

- **Non-test code never panics.** No `unwrap()`, `expect()`, `panic!`, `unreachable!`, `todo!` or
  `unimplemented!`. Return the crate's error type and propagate with `?`.
- Every production crate root (`lib.rs`, each binary's `main.rs`, and `xtask`) carries
  `#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]`.
- **No `unsafe`.** The workspace sets `unsafe_code = "forbid"`.
- **Everything from the network is hostile.** Use `get()` instead of indexing, checked arithmetic
  for sizes and offsets, and cap downloads and JSON sizes.

### Installing other people's software safely

- Download only from `https://github.com/datacrate360/<tool>/releases` (and GitHub's asset CDN);
  `GITHUB_OWNER` in `catalog.rs` is the only owner accepted, whatever any metadata says. The one
  exception is a developer's own loopback mirror (`DATACRATE360_LAUNCHER_DOWNLOAD_BASE`,
  `cargo xtask local`); keep it loopback-only and behind the `local-mirror` cargo feature, which
  release builds never enable.
- Verify each download against the release's `SHA256SUMS.txt` before running or installing it.
- There is no hosted release feed (`feed::FEED_URL` is `None`) and no details-page pictures
  (`details::SCREENSHOTS_BASE` is `None`): the launcher talks to GitHub only. Adding either is a
  maintainer's call, and must point at DataCrate360-owned infrastructure.
- macOS bundles are checked against `install::macos::TEAM_ID` once it is set (it is `None` until
  DataCrate360's releases are Developer ID signed).
- Never run an installer with elevated rights silently; tell the user what is about to happen.
- Never delete a user's files outside what the launcher itself installed.
- **Agents never land or approve a third-party app.** A change that adds a catalog row, download
  source or install path for an app whose repo is not under `datacrate360/`, or that loosens the
  rules above to allow one, is a maintainer's call alone. An agent reviewing it flags it at the
  top of its review and stops there: no merge, no approval, no fixes pushed to the PR.

### Look and feel

DataCrate360 is light first, Teal & Coral (`theme::Tokens`): canvas `#F6FAF9`, panel `#EEF5F3`,
surface `#FFFFFF`, ink `#0F1F1F`, muted ink `#5B6B6B`, lines `#DCE7E5`; teal `#105050` for the
brand, buttons `#0F4C4C` (hover `#0B3A3A`) with white text only; coral `#C2410C` only for links,
the active state and badges. No gradients. Hard square corners, 1 px hairlines; Archivo for
headings and tool names, Inter for body text, upper-case Geist Mono labels and buttons. The dark
theme still works, but light is the default. Build new UI from `widgets.rs`; never hard-code a
colour.

Animations follow the OS: with Windows' "Animation effects" off, macOS' "Reduce motion" on, a Linux
desktop's animations off or a browser's `prefers-reduced-motion`, nothing fades, wipes or glides
(`Host::reduce_motion`, `crates/ui-egui/src/motion.rs`). Progress bars and spinners keep moving.

### Languages

Every user-facing string goes through `crates/ui-egui/src/i18n` (`tl!("…")` for literals, `fmt`
for `{placeholders}`, `tn` for counts). English in the code is the lookup key; adding a string
means adding its row to every catalog marked `complete_menus` (the `*.tsv` files in
`crates/ui-egui/src/i18n/`), which the i18n tests check. Translate from the meaning of the English
text, never through another catalog. Product and tool names (DataCrate360 Launcher, DataCrate360
tools, Countercheck) stay in English. A CJK language is drawn with the system's own CJK font
(`crates/ui-egui/src/cjk_font.rs`, desktop only): never bundle a CJK font.

### Other rules

1. **Assets need attribution.** Every bundled asset gets a row in `ATTRIBUTION.md` (path, title,
   author, source, license) in the same change. The DataCrate360 logo in `assets/brand/` is a
   brand asset, not MIT/Apache.
2. **Icons are generated.** Run `packaging/icons.sh` (macOS `sips` + `cargo xtask ico`/`icns`)
   after changing `assets/brand/`, and commit the outputs; never edit the renders by hand.
3. **Never break wasm.** `cargo xtask wasm` must pass. File-system, process and OS code is
   `cfg(not(target_arch = "wasm32"))`.
4. **Tests are the gate.** Release parsing and asset picking in `core` get unit tests with real
   GitHub release JSON shapes.
5. When fetching anything, use the generic `DataCrate360-Launcher/<version>` User-Agent and never
   put a person's name, email or other personal details in requests.
6. **No upstream branding.** The upstream product's name, logos, links and app names may appear only in the
   attribution in `NOTICE`, `ATTRIBUTION.md`, `LICENSE-MIT` and the README's credits.

## 3. Before you finish a task

```sh
cargo xtask ci              # fmt --check, clippy -D warnings, test, wasm check
```

Or piecemeal: `cargo test -p <crate>`, `cargo clippy -p <crate> --all-targets -- -D warnings`,
`cargo xtask wasm`. If you touched packaging: `shellcheck packaging/*.sh packaging/*/*.sh
packaging/linux/AppRun`.

## 4. Releasing

The version lives only in `[workspace.package] version` in the root `Cargo.toml`
(`cargo xtask version`, `cargo xtask version set X.Y.Z[-pre]`). There is no release workflow yet:
packaging runs locally (`packaging/macos/package.sh`, etc.). A release is a GitHub Release
**DataCrate360 Launcher v<version>** on `datacrate360/datacrate360-launcher` with `SHA256SUMS.txt`.

Release assets (keep these names exact; the launcher's own self-update relies on them):

| Platform | Files |
|---|---|
| Windows | `datacrate360-launcher-<ver>-windows-{x64,x86,arm64}.msi`, `…-windows-{x64,x86,arm64}-portable.zip` |
| macOS | `datacrate360-launcher-<ver>-macos-universal.dmg` |
| Linux | `datacrate360-launcher-<ver>-linux-{x86_64,aarch64}.{AppImage,AppImage.zsync,deb,rpm,tar.gz,flatpak}` |
| FreeBSD | `datacrate360-launcher-<ver>-freebsd-x86_64.tar.gz` |
| Web | `datacrate360-launcher-web-<ver>.zip` |

Packaging facts the code relies on:

- The Windows MSI is **per-user** (no UAC prompt), by default into
  `%LOCALAPPDATA%\Programs\DataCrate360 Launcher`. The folder and the shortcut choice are recorded
  under `HKCU\Software\DataCrate360\DataCrate360 Launcher` (`InstallDir`, `DesktopShortcutChoice`),
  and `self_update.rs` reads `InstallDir`. Its shortcuts set AppUserModelID
  `com.datacrate360.launcher`, which toast notifications need. `portable.txt` beside the exe
  switches on portable mode (`DataCrate360LauncherData\` next to the exe).
- The macOS bundle is `DataCrate360 Launcher.app` (executable
  `Contents/MacOS/DataCrate360 Launcher`), a normal Dock app that can also sit in the menu bar:
  `LSUIElement` must never be set.
- Linux: the tray uses StatusNotifierItem over D-Bus and notifications use
  `org.freedesktop.Notifications`, both in pure Rust (no GTK, appindicator or libdbus).
- `build.rs` reads `DATACRATE360_LAUNCHER_BUILD_SHA` / `DATACRATE360_LAUNCHER_BUILD_DATE` and, on
  Windows, embeds `assets/app-icon/datacrate360-launcher.ico`, failing the build when
  `DATACRATE360_LAUNCHER_REQUIRE_WINRES=1` and it can't.

## 5. Parallel agents

- Use your own target dir (`CARGO_TARGET_DIR=target/agent-<name>`) and edit only the files you own.
- Keep every `Cargo.toml` valid at all times: the `crates/*` and `apps/*` globs mean one broken
  manifest breaks everyone's build. Write manifests to a temp file and `mv` them into place.
- If someone else's in-progress edit breaks the build, wait and retry; don't fix their files.
