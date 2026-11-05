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

#[cfg_attr(not(test), allow(dead_code))] // For the pages' pictures, once they are published.
const fn shot(key: &'static str, caption: &'static str) -> Shot {
    Shot { key, caption }
}

/// The editorial half of an app's page.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AppDetails {
    /// Matches [`crate::CraftApp::id`].
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

const fn feature(title: &'static str, text: &'static str) -> Feature {
    Feature { title, text }
}

/// Where the pictures live: one 1200 px wide, 8:5 JPEG per [`Shot`], at `<app>/<key>.jpg`.
///
/// Pinned to a tag (`SCREENSHOTS_REV`) so that what a launcher fetches never changes under it, and
/// so that a saved copy never goes stale. To change a picture, publish a new tag and bump the
/// revision: older launchers keep working from the tag they know.
pub const SCREENSHOTS_BASE: &str = "https://raw.githubusercontent.com/storytold/craft-launcher-assets";

/// The tag of the pictures repository the launcher reads. Also part of the saved copies' path.
pub const SCREENSHOTS_REV: &str = "v1";

/// The largest picture the launcher accepts. The real ones are 60 to 150 KB.
pub const MAX_SCREENSHOT_BYTES: usize = 2 << 20;

/// True when `bytes` start like a JPEG file. A captive portal's login page, a proxy's error page or
/// a truncated answer is refused before it is saved or decoded.
pub fn looks_like_jpeg(bytes: &[u8]) -> bool {
    bytes.starts_with(&[0xFF, 0xD8, 0xFF])
}

/// Where the Uni apps are shipped: Windows installers.
const PLATFORMS: &str = "Windows";

pub const DETAILS: [AppDetails; 3] = [
    AppDetails {
        id: "unitexto",
        headline: "The familiar word processor, native and offline.",
        about: "A word processor with the ribbon, shortcuts and behavior you already know, written in Rust. \
                Open source, and built from public specifications.",
        features: &[
            feature(
                "Familiar",
                "Ribbon tabs and groups, a live Styles gallery and the editing behavior you expect, from list continuation to Format Painter.",
            ),
            feature(
                "Your files",
                "Opens and saves .docx, and also .odt, .rtf, .html, .md, .tex and .txt. Opens older .doc files and exports PDF with real, selectable text.",
            ),
            feature("Fast", "Paragraph layout is cached, so typing stays responsive even in long documents."),
            feature("Private", "Spelling, grammar and everything else work offline."),
        ],
        shots: &[],
        platforms: PLATFORMS,
    },
    AppDetails {
        id: "uniplanner",
        headline: "Spreadsheets you already know how to use.",
        about: "A spreadsheet with the ribbon, formula bar, sheet tabs and shortcuts you know, on a calculation engine \
                written in Rust. No subscription, no license server and no telemetry.",
        features: &[
            feature(
                "Familiar",
                "The ribbon, formula bar, Name Box, fill handle, point-mode formulas, Format Cells, Paste Special, conditional formatting, tables and charts.",
            ),
            feature(
                "Compatible",
                "XLSX is the native format, with formulas, tables, conditional formats, charts, pivot tables and print settings round-tripping. CSV and TSV too.",
            ),
            feature(
                "Capable",
                "A dependency-graph calculation engine with dynamic arrays, LET and LAMBDA, structured table references, sort, AutoFilter and Flash Fill.",
            ),
            feature("Fast", "Copy-on-write workbooks, incremental recalculation and a grid that only lays out what is on screen."),
        ],
        shots: &[],
        platforms: PLATFORMS,
    },
    AppDetails {
        id: "unipdf",
        headline: "The PDF workbench, native and offline.",
        about: "A PDF reader and editor written in Rust. Read, organize, combine, split and secure PDFs in a fast, \
                native app, with no account and no telemetry.",
        features: &[
            feature("Read", "Fast, accurate rendering, with search as you type and text you can select and copy."),
            feature("Organize", "Reorder, rotate, insert and delete pages, and combine or split documents."),
            feature("Secure", "Protect documents with a password and choose what readers may do with them."),
            feature("Convert", "Export to Word and Excel, and recognise the text of scanned pages with Windows OCR."),
        ],
        shots: &[],
        platforms: PLATFORMS,
    },
];

impl AppDetails {
    /// Where the picture `key` of this app is published, or `None` if the page has no such picture.
    /// The only way to turn a request into an address: callers never build one from their own text.
    pub fn screenshot_url(&self, key: &str) -> Option<String> {
        self.shots.iter().find(|s| s.key == key).map(|s| format!("{SCREENSHOTS_BASE}/{SCREENSHOTS_REV}/{}/{}.jpg", self.id, s.key))
    }

    pub fn by_id(id: &str) -> Option<&'static AppDetails> {
        DETAILS.iter().find(|d| d.id == id)
    }

    pub fn paragraphs(&self) -> impl Iterator<Item = &'static str> {
        self.about.split("\n\n").map(str::trim).filter(|p| !p.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CraftApp, SUITE};

    #[test]
    fn every_page_belongs_to_a_suite_app_once() {
        let mut ids: Vec<_> = DETAILS.iter().map(|d| d.id).collect();
        for id in &ids {
            assert!(CraftApp::by_id(id).is_some(), "{id}");
        }
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), DETAILS.len());
    }

    #[test]
    fn every_app_has_a_page() {
        for a in &SUITE {
            assert!(AppDetails::by_id(a.id).is_some(), "{}", a.id);
        }
    }

    #[test]
    fn picture_addresses_come_only_from_the_pages_own_pictures() {
        // The Uni pages have no pictures yet, so check the rule on a page of our own.
        const SHOTS: &[Shot] = &[shot("demo", "A demo")];
        let page = AppDetails { shots: SHOTS, ..*AppDetails::by_id("unitexto").unwrap() };
        assert_eq!(
            page.screenshot_url("demo").as_deref(),
            Some("https://raw.githubusercontent.com/storytold/craft-launcher-assets/v1/unitexto/demo.jpg")
        );
        for key in ["", "nope", "../uniplanner/neon", "demo/../masks", "demo.jpg", "/etc/passwd", "demo?x=1", "demo#"] {
            assert_eq!(page.screenshot_url(key), None, "{key:?}");
        }
        // Every picture of every page has an address of the one shape, with nothing to escape.
        for d in &DETAILS {
            for s in d.shots {
                let url = d.screenshot_url(s.key).unwrap();
                let path = url.strip_prefix(&format!("{SCREENSHOTS_BASE}/{SCREENSHOTS_REV}/")).unwrap();
                assert_eq!(path, format!("{}/{}.jpg", d.id, s.key));
                assert!(path.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '/' | '-' | '.')), "{path}");
            }
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
        for d in &DETAILS {
            assert!(!d.headline.is_empty() && !d.features.is_empty() && d.paragraphs().next().is_some(), "{}", d.id);
        }
    }
}
