//! Theme persistence in localStorage. The theme class itself lives on `<main>`
//! and is driven by the Dioxus signal (see main.rs / nav_section.rs).
//!
//! `save_theme` is a no-op on the native SSG build; `resolve_theme` is
//! client-only, since its only caller is `wasm32`-gated.

use crate::Theme;

/// The theme to apply after hydration: the saved choice if any, else the OS
/// `prefers-color-scheme`. `None` means keep the default.
#[cfg(target_arch = "wasm32")]
pub fn resolve_theme() -> Option<Theme> {
    let window = web_sys::window()?;

    let saved = window
        .local_storage()
        .ok()
        .flatten()
        .and_then(|storage| storage.get_item(Theme::STORAGE_KEY).ok().flatten())
        .and_then(|value| Theme::from_storage_value(&value));

    // A saved choice wins; otherwise fall back to the OS preference.
    saved.or_else(|| {
        let prefers_dark = window
            .match_media("(prefers-color-scheme: dark)")
            .ok()
            .flatten()?;
        Some(if prefers_dark.matches() {
            Theme::Dark
        } else {
            Theme::Light
        })
    })
}

/// Persist the user's chosen theme to localStorage. No-op on the server build.
pub fn save_theme(theme: Theme) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item(Theme::STORAGE_KEY, theme.storage_value());
            }
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = theme;
    }
}
