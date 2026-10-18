//! The DataCrate360 tools: the apps the launcher knows about.
//!
//! The list is data, so adding a tool is one row in [`SUITE`] (and, for its tagline, one row in
//! each UI catalog); a later version could fetch it from an API.
//!
//! Text here is English, the source the UI translates from: each `tagline` has a catalog row
//! keyed by the app id (`tr_ctx(app.id, app.tagline)` in `ui-egui`).

use serde::{Deserialize, Serialize};

/// Which group of the suite an app belongs to: one section of the catalog.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppGroup {
    /// AI agents and the tooling around them.
    AiTools,
}

impl AppGroup {
    pub fn label(self) -> &'static str {
        match self {
            Self::AiTools => "AI tools",
        }
    }
}

/// One app in the suite.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SuiteApp {
    /// Machine name: repo, binary, package and asset stem (`countercheck`).
    pub id: &'static str,
    /// Display name (`countercheck`).
    pub name: &'static str,
    /// What it's for, in a few words (English; the UI translates it by app id).
    pub tagline: &'static str,
    pub group: AppGroup,
    /// Brand colour (sRGB), for surfaces. White text must stay readable on it.
    pub color: [u8; 3],
    /// The brand colour for text on a light background.
    pub ink_light: [u8; 3],
    /// The brand colour for text on a dark background.
    pub ink_dark: [u8; 3],
    /// GitHub `owner/repo`.
    pub repo: &'static str,
    /// Older names its release assets may carry.
    pub former_ids: &'static [&'static str],
    /// Older display names its installers may have registered.
    pub former_names: &'static [&'static str],
}

impl SuiteApp {
    /// The macOS bundle id, Flatpak id and Linux desktop-file id: `com.datacrate360.<id>`, and
    /// `com.datacrate360.launcher` for the launcher itself.
    pub fn bundle_id(&self) -> String {
        if self.id == LAUNCHER_ID { LAUNCHER_BUNDLE_ID.to_string() } else { format!("com.datacrate360.{}", self.id) }
    }

    pub fn releases_url(&self) -> String {
        format!("https://github.com/{}/releases", self.repo)
    }

    pub fn repo_url(&self) -> String {
        format!("https://github.com/{}", self.repo)
    }

    /// The app's web page: DataCrate360's website.
    pub fn website(&self) -> String {
        WEBSITE_URL.to_string()
    }

    /// Asset stems this app's releases may use, current first.
    pub fn stems(&self) -> impl Iterator<Item = &'static str> + '_ {
        std::iter::once(self.id).chain(self.former_ids.iter().copied())
    }

    /// Display names this app's installers may have registered, current first.
    pub fn names(&self) -> impl Iterator<Item = &'static str> + '_ {
        std::iter::once(self.name).chain(self.former_names.iter().copied())
    }

    pub fn by_id(id: &str) -> Option<&'static SuiteApp> {
        SUITE.iter().find(|a| a.id == id)
    }
}

const fn app(
    id: &'static str,
    name: &'static str,
    tagline: &'static str,
    group: AppGroup,
    [color, ink_light, ink_dark]: [[u8; 3]; 3],
    repo: &'static str,
) -> SuiteApp {
    SuiteApp { id, name, tagline, group, color, ink_light, ink_dark, repo, former_ids: &[], former_names: &[] }
}

/// DataCrate360's teal: surface, ink on light, ink on dark.
const TEAL: [[u8; 3]; 3] = [[0x10, 0x50, 0x50], [0x0f, 0x4c, 0x4c], [0x6f, 0xc4, 0xb8]];

/// The DataCrate360 tools, in display order. Adding a tool is one more row.
pub const SUITE: [SuiteApp; 1] =
    [app("countercheck", "Countercheck", "Proof that AI work really happened", AppGroup::AiTools, TEAL, "datacrate360/countercheck")];

/// DataCrate360's website.
pub const WEBSITE_URL: &str = "https://datacrate360.com";
/// The GitHub account every tool's repository lives under: the only one the launcher installs
/// from, whatever any release metadata says.
pub const GITHUB_OWNER: &str = "datacrate360";
/// DataCrate360 on GitHub: every tool's source.
pub const GITHUB_URL: &str = "https://github.com/datacrate360";
/// Social accounts: (service, URL), shown as small icons. None for now.
pub const SOCIAL: [(&str, &str); 0] = [];

/// DataCrate360 Launcher itself, described like a tool so it can update itself with the same
/// machinery (asset names `datacrate360-launcher-<ver>-…`, bundle id `com.datacrate360.launcher`,
/// `DataCrate360 Launcher.app`). Not part of [`SUITE`].
pub const LAUNCHER: SuiteApp = app(
    "datacrate360-launcher",
    "DataCrate360 Launcher",
    "Installs and updates the DataCrate360 tools",
    AppGroup::AiTools,
    TEAL,
    "datacrate360/datacrate360-launcher",
);

/// The launcher's own repo, checked so it can say when a newer launcher is out.
pub const LAUNCHER_REPO: &str = "datacrate360/datacrate360-launcher";
/// The launcher's own asset stem.
pub const LAUNCHER_ID: &str = "datacrate360-launcher";
/// The launcher's app id (macOS bundle id, Flatpak id, AppUserModelID).
pub const LAUNCHER_BUNDLE_ID: &str = "com.datacrate360.launcher";

/// Stand-in tools for tests: the real catalog's tools followed by twelve made-up ones, so that
/// lists, sorting, filters and multi-app flows stay exercised while the real catalog is small.
/// Built only for tests (the `fixtures` feature, which only dev-dependencies enable).
#[cfg(any(test, feature = "fixtures"))]
pub mod fixtures {
    use super::{AppGroup, SUITE, SuiteApp, TEAL, app};

    const fn tool(id: &'static str, name: &'static str, tagline: &'static str, repo: &'static str) -> SuiteApp {
        app(id, name, tagline, AppGroup::AiTools, TEAL, repo)
    }

    pub const CATALOG: [SuiteApp; 13] = [
        SUITE[0],
        tool("beta", "Beta", "Vector illustration", "datacrate360/beta"),
        tool("gamma", "Gamma", "Video editing, color and sound", "datacrate360/gamma"),
        tool("delta", "Delta", "Photo library and raw development", "datacrate360/delta"),
        SuiteApp {
            former_ids: &["epsilonold"],
            former_names: &["EpsilonOld"],
            ..tool("epsilon", "Epsilon", "Reading, organizing and protecting PDFs", "datacrate360/epsilon")
        },
        tool("zeta", "Zeta", "Motion graphics and visual effects", "datacrate360/zeta"),
        tool("kappa", "Kappa", "Page layout and publishing", "datacrate360/kappa"),
        tool("theta", "Theta", "Writing and document design", "datacrate360/theta"),
        tool("iota", "Iota", "Presentations and slide shows", "datacrate360/iota"),
        tool("lambda", "Lambda", "Spreadsheets and data", "datacrate360/lambda"),
        tool("sigma", "Sigma", "Computer-aided design and drafting", "datacrate360/sigma"),
        tool("omega", "Omega", "Recording, editing and mixing audio", "datacrate360/omega"),
        tool("tau", "Tau", "Interface design and prototyping", "datacrate360/tau"),
    ];

    /// A tool of [`CATALOG`] by id.
    pub fn by_id(id: &str) -> Option<&'static SuiteApp> {
        CATALOG.iter().find(|a| a.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suite_has_unique_ids_under_the_owner() {
        assert!(!SUITE.is_empty());
        let mut ids: Vec<_> = SUITE.iter().map(|a| a.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), SUITE.len());
        for a in &SUITE {
            assert_eq!(a.repo, format!("{GITHUB_OWNER}/{}", a.id));
            assert!(a.id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'), "{}", a.id);
            assert_ne!(a.id, LAUNCHER_ID);
        }
        assert_eq!(LAUNCHER.repo, LAUNCHER_REPO);
        assert_eq!(LAUNCHER.bundle_id(), LAUNCHER_BUNDLE_ID);
    }

    #[test]
    fn countercheck_is_in_the_catalog() {
        let omni = SuiteApp::by_id("countercheck").unwrap();
        assert_eq!(omni.name, "Countercheck");
        assert_eq!(omni.tagline, "Proof that AI work really happened");
        assert_eq!(omni.group.label(), "AI tools");
        assert_eq!(omni.stems().collect::<Vec<_>>(), ["countercheck"]);
        assert_eq!(omni.releases_url(), "https://github.com/datacrate360/countercheck/releases");
        assert_eq!(omni.bundle_id(), "com.datacrate360.countercheck");
        assert_eq!(omni.website(), "https://datacrate360.com");
        assert_eq!(SuiteApp::by_id("beta"), None);
    }

    #[test]
    fn fixtures_start_with_the_real_catalog_and_have_unique_ids() {
        assert_eq!(fixtures::CATALOG[..SUITE.len()], SUITE);
        let mut ids: Vec<_> = fixtures::CATALOG.iter().map(|a| a.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), fixtures::CATALOG.len());
        assert_eq!(fixtures::by_id("epsilon").unwrap().stems().collect::<Vec<_>>(), ["epsilon", "epsilonold"]);
    }
}
