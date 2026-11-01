//! The shared look: `ui/theme.slint` and `ui/widgets.slint`. An app's `build.rs` passes
//! [`library_paths`] to `slint_build`, and its screens import `@kitka/theme.slint` and
//! `@kitka/widgets.slint`. At run time, [`accent`] turns an app's color into one the theme can
//! draw text in.

use std::collections::HashMap;
use std::path::PathBuf;

/// The `@kitka` import path for `slint_build::CompilerConfiguration::with_library_paths`.
pub fn library_paths() -> HashMap<String, PathBuf> {
    HashMap::from([("kitka".into(), PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/ui")))])
}

// `Theme.bg` in theme.slint, for each theme.
const GROUND_LIGHT: [u8; 3] = [0xf4, 0xf4, 0xf0];
const GROUND_DARK: [u8; 3] = [0x05, 0x05, 0x08];

/// A color (an album's, say) as `Theme.accent`: pulled toward black on the light theme and toward
/// white on the dark one, only as far as it takes for text in it to reach 4.5:1 on the ground.
/// Colors that already read are returned as they are.
pub fn accent(color: [u8; 3], light: bool) -> [u8; 3] {
    let (ground, toward) = if light { (GROUND_LIGHT, 0.0) } else { (GROUND_DARK, 255.0) };
    (0..=20)
        .map(|step| color.map(|c| (c as f64 + (toward - c as f64) * step as f64 / 20.0).round() as u8))
        .find(|&c| contrast(c, ground) >= 4.5)
        .unwrap_or([toward as u8; 3])
}

/// The WCAG contrast ratio of two colors, from 1 to 21.
pub fn contrast(a: [u8; 3], b: [u8; 3]) -> f64 {
    let (a, b) = (luminance(a), luminance(b));
    (a.max(b) + 0.05) / (a.min(b) + 0.05)
}

fn luminance(c: [u8; 3]) -> f64 {
    let lin = |v: u8| {
        let v = v as f64 / 255.0;
        if v <= 0.04045 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) }
    };
    0.2126 * lin(c[0]) + 0.7152 * lin(c[1]) + 0.0722 * lin(c[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_defaults_pass_unchanged() {
        assert_eq!(accent([0x1f, 0x5f, 0xbf], true), [0x1f, 0x5f, 0xbf]);
        assert_eq!(accent([0x6a, 0xaf, 0xff], false), [0x6a, 0xaf, 0xff]);
    }

    #[test]
    fn dark_colors_lighten_on_the_dark_theme() {
        // The blue the screen tests' album picks from its cover: 1.83:1 as it is.
        let c = accent([0x14, 0x28, 0xa6], false);
        assert!(contrast(c, GROUND_DARK) >= 4.5);
        assert!(c[2] > c[0], "still blue: {c:?}");
    }

    #[test]
    fn pale_colors_darken_on_the_light_theme() {
        for c in [[0xff, 0xff, 0xff], [0xf0, 0xe4, 0x42], [0x88, 0xcc, 0xee]] {
            assert!(contrast(accent(c, true), GROUND_LIGHT) >= 4.5, "{c:?}");
        }
    }

    #[test]
    fn contrast_spans_one_to_twenty_one() {
        assert!((contrast([0; 3], [255; 3]) - 21.0).abs() < 1e-9);
        assert_eq!(contrast([9, 9, 9], [9, 9, 9]), 1.0);
    }
}
