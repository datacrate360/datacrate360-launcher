//! Hand-written product pages: what each app is, in the launcher's own words.
//!
//! The text is compiled in, so it works offline and on the web, and nothing from the network is
//! ever shown as page copy. What changes with every release (version, date, size, notes) comes
//! from the GitHub release instead. An app without an entry here simply has no details page.
//!
//! The pictures are not compiled in: they are fetched when a page is opened (see
//! [`AppDetails::screenshot_url`]), so the repository and the binary stay small.

/// One feature: a short name and a sentence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Feature {
    pub title: &'static str,
    pub text: &'static str,
}

/// A screenshot: `key` names the file to fetch (`<app>/<key>.jpg` in the pictures repository).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Shot {
    pub key: &'static str,
    pub caption: &'static str,
}

pub const fn shot(key: &'static str, caption: &'static str) -> Shot {
    Shot { key, caption }
}

/// The editorial half of an app's page.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AppDetails {
    /// Matches [`crate::SuiteApp::id`].
    pub id: &'static str,
    /// One line under the name.
    pub headline: &'static str,
    /// One or two paragraphs, separated by a blank line.
    pub about: &'static str,
    pub features: &'static [Feature],
    /// The first is the lead picture. All are 8:5.
    pub shots: &'static [Shot],
    /// Where it runs, for display (`"macOS, Windows, Linux, web"`).
    pub platforms: &'static str,
}

pub const fn feature(title: &'static str, text: &'static str) -> Feature {
    Feature { title, text }
}

/// Where the pictures live: one 1200 px wide, 8:5 JPEG per [`Shot`], at `<app>/<key>.jpg`.
///
/// `None`: DataCrate360 publishes no pictures yet, so no picture is ever fetched. When it does,
/// point this at a repository under `datacrate360/`.
///
/// Pinned to a tag (`SCREENSHOTS_REV`) so that what a launcher fetches never changes under it, and
/// so that a saved copy never goes stale. To change a picture, publish a new tag and bump the
/// revision: older launchers keep working from the tag they know.
pub const SCREENSHOTS_BASE: Option<&str> = None;

/// The tag of the pictures repository the launcher reads. Also part of the saved copies' path.
pub const SCREENSHOTS_REV: &str = "v1";

/// The largest picture the launcher accepts. The real ones are 60 to 150 KB.
pub const MAX_SCREENSHOT_BYTES: usize = 2 << 20;

/// True when `bytes` start like a JPEG file. A captive portal's login page, a proxy's error page or
/// a truncated answer is refused before it is saved or decoded.
pub fn looks_like_jpeg(bytes: &[u8]) -> bool {
    bytes.starts_with(&[0xFF, 0xD8, 0xFF])
}

/// The tools' product pages. Empty for now: a tool without an entry simply has no details page.
/// Adding one is one more row (and, with pictures, a [`SCREENSHOTS_BASE`]).
pub const DETAILS: [AppDetails; 0] = [];

impl AppDetails {
    /// Where the picture `key` of this app is published, or `None` if the page has no such picture.
    /// The only way to turn a request into an address: callers never build one from their own text.
    pub fn screenshot_url(&self, key: &str) -> Option<String> {
        let base = SCREENSHOTS_BASE?;
        self.shots.iter().find(|s| s.key == key).map(|s| format!("{base}/{SCREENSHOTS_REV}/{}/{}.jpg", self.id, s.key))
    }

    pub fn by_id(id: &str) -> Option<&'static AppDetails> {
        #[cfg(feature = "fixtures")]
        if let Some(d) = fixtures::DETAILS.iter().find(|d| d.id == id) {
            return Some(d);
        }
        DETAILS.iter().find(|d| d.id == id)
    }

    pub fn paragraphs(&self) -> impl Iterator<Item = &'static str> {
        self.about.split("\n\n").map(str::trim).filter(|p| !p.is_empty())
    }
}

/// Stand-in pages for the stand-in tools of [`crate::catalog::fixtures`], so the details drawer,
/// its gallery and its keyboard handling stay tested while the real catalog has no pages. With the
/// `fixtures` feature (tests only), [`AppDetails::by_id`] finds these first.
#[cfg(any(test, feature = "fixtures"))]
pub mod fixtures {
    use super::{AppDetails, feature, shot};

    const PLATFORMS: &str = "macOS, Windows, Linux";

    pub const DETAILS: [AppDetails; 4] = [
        AppDetails {
            id: "omni-agent",
            headline: "A stand-in page with four pictures.",
            about: "The first paragraph of a stand-in page.\n\nThe second paragraph of a stand-in page.",
            features: &[
                feature("First feature", "What the first feature does."),
                feature("Second feature", "What the second feature does."),
                feature("Third feature", "What the third feature does."),
            ],
            shots: &[
                shot("demo", "The lead picture"),
                shot("second", "The second picture"),
                shot("third", "The third picture"),
                shot("fourth", "The fourth picture"),
            ],
            platforms: PLATFORMS,
        },
        AppDetails {
            id: "beta",
            headline: "A stand-in page with two pictures.",
            about: "One paragraph.",
            features: &[feature("Only feature", "What it does.")],
            shots: &[shot("demo", "The lead picture"), shot("second", "The second picture")],
            platforms: PLATFORMS,
        },
        AppDetails {
            id: "epsilon",
            headline: "A stand-in page without pictures.",
            about: "One paragraph.",
            features: &[feature("Only feature", "What it does.")],
            shots: &[],
            platforms: PLATFORMS,
        },
        AppDetails {
            id: "theta",
            headline: "A stand-in page with one picture.",
            about: "One paragraph.",
            features: &[feature("Only feature", "What it does.")],
            shots: &[shot("demo", "The lead picture")],
            platforms: PLATFORMS,
        },
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAGE: AppDetails = AppDetails {
        id: "omni-agent",
        headline: "Headline.",
        about: "One.\n\nTwo.",
        features: &[feature("A", "B")],
        shots: &[shot("demo", "Demo")],
        platforms: "macOS",
    };

    #[test]
    fn every_page_belongs_to_a_suite_app_once() {
        let mut ids: Vec<_> = DETAILS.iter().map(|d| d.id).collect();
        for id in &ids {
            assert!(crate::SuiteApp::by_id(id).is_some(), "{id}");
        }
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), DETAILS.len());
    }

    #[test]
    fn no_pictures_are_fetched_without_a_pictures_repository() {
        assert_eq!(SCREENSHOTS_BASE, None);
        assert_eq!(PAGE.screenshot_url("demo"), None);
        for key in ["", "nope", "../x/neon", "demo/../masks", "demo.jpg", "/etc/passwd", "demo?x=1", "demo#"] {
            assert_eq!(PAGE.screenshot_url(key), None, "{key:?}");
        }
    }

    #[test]
    fn only_jpeg_files_are_accepted() {
        assert!(looks_like_jpeg(&[0xFF, 0xD8, 0xFF, 0xE0, 0, 0x10]));
        assert!(!looks_like_jpeg(b""));
        assert!(!looks_like_jpeg(&[0xFF, 0xD8]));
        assert!(!looks_like_jpeg(b"<!DOCTYPE html>"));
        // A PNG's signature.
        assert!(!looks_like_jpeg(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]));
    }

    #[test]
    fn pages_are_filled_in() {
        assert_eq!(PAGE.paragraphs().collect::<Vec<_>>(), ["One.", "Two."]);
        for d in &DETAILS {
            assert!(!d.headline.is_empty() && !d.features.is_empty() && d.paragraphs().next().is_some(), "{}", d.id);
        }
    }
}
