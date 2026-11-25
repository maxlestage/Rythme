//! The color themes of the site. Their colors are custom properties of `site.css`
//! (`:root[data-theme="…"]`); this module knows their names, the color of the browser's
//! bar for each (`theme-color`: the paper of the theme) and writes the inline script of
//! the head that applies the visitor's choice before the first paint. Paquets allows
//! that script in the Content-Security-Policy by its hash, like any inline script.
//!
//! It is the one piece of the site's behavior written in JavaScript, and it has to be:
//! the browser side is Rust compiled to WebAssembly (`client/`), and WebAssembly only
//! runs once downloaded and compiled, after the page is painted; a theme applied then
//! would flash. A cookie read by the server would not do either: the static export
//! (GitHub Pages) has no server, and the service worker saves the pages once for every
//! theme. The rest of the picker (its button, the choice, the other tabs) is Rust
//! (`client/src/layout.rs`).

use crate::i18n::Texts;

/// The key of `localStorage` that keeps the chosen theme (read by the browser side too).
pub const STORAGE_KEY: &str = "rythme-theme";

/// A theme: its id (`data-theme`, the value kept), and the color of the browser's bar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Theme {
    pub id: &'static str,
    pub color: &'static str,
}

/// Auto follows the system: no `data-theme`, light or dark by `prefers-color-scheme`.
pub const AUTO: &str = "auto";

/// In the order of the picker. Auto's color is the light one (the dark one comes with
/// its `media` query).
pub const THEMES: [Theme; 6] = [
    Theme { id: AUTO, color: LIGHT },
    Theme { id: "light", color: LIGHT },
    Theme { id: "dark", color: DARK },
    Theme { id: "contrast", color: "#000000" },
    Theme { id: "sepia", color: "#f3ead6" },
    Theme { id: "tempo", color: "#160f2e" },
];

/// The paper of the light and dark themes (`site.css`): the bar of the browser in Auto,
/// the splash screen of the installed app.
pub const LIGHT: &str = "#eef1f6";
pub const DARK: &str = "#0e1118";

impl Theme {
    /// Its name in the language of `t`.
    pub fn name(self, t: &Texts) -> &'static str {
        let at = THEMES.iter().position(|th| th.id == self.id).unwrap_or(0);
        t.theme_names[at]
    }
}

/// The inline script of the head, run before the page is painted: `js` on <html> when
/// the browser runs WebAssembly (the menu folds behind the burger only when the browser
/// side can open it), then the theme kept in `localStorage`, if it is one, as
/// `data-theme`, and its color for the browser's bar. Storage may be blocked: Auto then.
pub fn head_script() -> String {
    let colors: Vec<String> = THEMES.iter().filter(|t| t.id != AUTO).map(|t| format!("{}:\"{}\"", t.id, t.color)).collect();
    format!(
        concat!(
            "(function(d){{if(typeof WebAssembly==\"object\")d.classList.add(\"js\");try{{var t=localStorage.getItem(\"{key}\"),c={{{colors}}};",
            "if(t&&Object.prototype.hasOwnProperty.call(c,t)){{d.setAttribute(\"data-theme\",t);",
            "var m=document.querySelectorAll('meta[name=\"theme-color\"]');for(var i=0;i<m.length;i++)m[i].setAttribute(\"content\",c[t])}}",
            "}}catch(e){{}}}})(document.documentElement)",
        ),
        key = STORAGE_KEY,
        colors = colors.join(","),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_script_knows_every_theme_but_auto() {
        let script = head_script();
        for theme in THEMES.iter().skip(1) {
            assert!(script.contains(&format!("{}:\"{}\"", theme.id, theme.color)), "{script}");
        }
        assert!(!script.contains("auto:"));
        assert!(!script.contains("</"), "the script cannot close itself");
        assert!(script.contains(STORAGE_KEY));
    }
}
