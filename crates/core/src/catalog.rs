//! The Uniphar Indústria Farmacêutica: the thirteen apps the launcher knows about.
//!
//! The list is data so a later version can fetch it from an API; [`SUITE`] is the built-in copy.
//!
//! Text here is English, the source the UI translates from: each `tagline` has a catalog row
//! keyed by the app id (`tr_ctx(app.id, app.tagline)` in `ui-egui`).

/// One app in the suite.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CraftApp {
    /// Machine name: repo, binary, package and asset stem (`photocraft`).
    pub id: &'static str,
    /// Display name (`PhotoCraft`).
    pub name: &'static str,
    /// What it's for, in a few words (English; the UI translates it by app id).
    pub tagline: &'static str,
    /// Brand colour (sRGB), for surfaces: getartcraft.com's `--app`.
    pub color: [u8; 3],
    /// The brand colour for text on a light background (the site's light `--app-ink`).
    pub ink_light: [u8; 3],
    /// The brand colour for text on a dark background (the site's dark `--app-ink`).
    pub ink_dark: [u8; 3],
    /// GitHub `owner/repo`.
    pub repo: &'static str,
    /// Older names its release assets may carry (PdfCraft shipped as PrintCraft until 0.2.1).
    pub former_ids: &'static [&'static str],
    /// Older display names its installers registered (PdfCraft's MSI said PrintCraft until 0.2.1).
    pub former_names: &'static [&'static str],
}

impl CraftApp {
    /// `ai.storyteller.<id>`: macOS bundle id, Flatpak id and Linux desktop-file id.
    pub fn bundle_id(&self) -> String {
        format!("ai.storyteller.{}", self.id)
    }

    pub fn releases_url(&self) -> String {
        format!("https://github.com/{}/releases", self.repo)
    }

    pub fn repo_url(&self) -> String {
        format!("https://github.com/{}", self.repo)
    }

    /// Where to read about the app: Uniphar's site, which has no per-app pages.
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

    pub fn by_id(id: &str) -> Option<&'static CraftApp> {
        SUITE.iter().find(|a| a.id == id)
    }
}

const fn app(
    id: &'static str,
    name: &'static str,
    tagline: &'static str,
    [color, ink_light, ink_dark]: [[u8; 3]; 3],
    repo: &'static str,
) -> CraftApp {
    CraftApp { id, name, tagline, color, ink_light, ink_dark, repo, former_ids: &[], former_names: &[] }
}

/// The Uniphar Indústria Farmacêutica, in display order matching getartcraft.com:
pub const SUITE: [CraftApp; 2] = [
    app("unitexto", "UniTexto", "Writing and document design", [[0x39, 0x58, 0x86], [0x39, 0x58, 0x86], [0xb1, 0xc9, 0xef]], "uaidenys/unitexto"),
    app("uniplanner", "UniPlanner", "Spreadsheets and data", [[0x37, 0x55, 0x34], [0x37, 0x55, 0x34], [0xae, 0xc3, 0xb0]], "uaidenys/uniplanner"),
];

/// Uni Launcher's website.
pub const WEBSITE_URL: &str = "https://uniphar.com.br/";
/// The GitHub account every Craft App's repository lives under: the only one the launcher
/// installs from, whatever the release feed says.
pub const GITHUB_OWNER: &str = "uaidenys";
/// Uni Launcher on GitHub: every Craft App's source.
pub const GITHUB_URL: &str = "https://github.com/uaidenys";
/// Uni Launcher's social accounts: (service, URL). Shown as small icons, less prominent than Discord.
pub const SOCIAL: [(&str, &str); 3] = [
    ("Instagram", "https://www.instagram.com/uniphar.ind/"),
    ("Facebook", "https://www.facebook.com/profile.php?id=61577659983789"),
    ("LinkedIn", "https://www.linkedin.com/company/unipharfarma/?viewAsMember=true"),
];

/// Uniphar customer service (SAC) phone.
pub const SAC_PHONE: &str = "(62) 3316-3131";

/// The Uni Launcher community on Discord.
pub const DISCORD_URL: &str = "https://discord.gg/artcraft";
/// Deep link into the Discord application for the Uni Launcher server invite.
pub const DISCORD_APP_URL: &str = "discord://-/invite/artcraft";

/// Uni Launcher itself, described like a Craft App so it can update itself with the same
/// machinery (asset names `artcraft-launcher-<ver>-…`, bundle id `ai.storyteller.artcraft-launcher`,
/// `Uni Launcher.app`). Not part of [`SUITE`].
pub const LAUNCHER: CraftApp = app(
    "artcraft-launcher",
    "Uni Launcher",
    "Uniphar Indústria Farmacêutica · Denys Rodrigues de Oliveira",
    [[0x0b, 0x0b, 0x0c], [0x0b, 0x0b, 0x0c], [0xf2, 0xf1, 0xee]],
    "uaidenys/uni-launcher",
);

/// The launcher's own repo, checked so it can say when a newer launcher is out.
pub const LAUNCHER_REPO: &str = "uaidenys/uni-launcher";
/// The launcher's own asset stem.
pub const LAUNCHER_ID: &str = "artcraft-launcher";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suite_has_the_uni_apps_with_unique_ids() {
        let ids: Vec<_> = SUITE.iter().map(|a| a.id).collect();
        assert_eq!(ids, ["unitexto", "uniplanner"]);
        for a in &SUITE {
            assert_eq!(a.repo, format!("{GITHUB_OWNER}/{}", a.id));
            assert_eq!(a.name.to_lowercase(), a.id);
        }
    }

    #[test]
    fn apps_are_found_by_id_and_link_to_uniphar() {
        let texto = CraftApp::by_id("unitexto").unwrap();
        assert_eq!(texto.stems().collect::<Vec<_>>(), ["unitexto"]);
        assert_eq!(texto.names().collect::<Vec<_>>(), ["UniTexto"]);
        assert_eq!(texto.website(), "https://uniphar.com.br/");
        assert_eq!(texto.repo_url(), "https://github.com/uaidenys/unitexto");
        assert!(CraftApp::by_id("wordcraft").is_none());
    }
}
