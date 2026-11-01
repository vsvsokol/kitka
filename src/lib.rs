//! The shared look: `ui/theme.slint` and `ui/widgets.slint`. An app's `build.rs` passes
//! [`library_paths`] to `slint_build`, and its screens import `@kitka/theme.slint` and
//! `@kitka/widgets.slint`. At run time, [`accent`] turns an app's color into one the theme can
//! draw text in, and [`sigil`] draws the ring mark `RingCover` shows.

use std::collections::HashMap;
use std::f64::consts::PI;
use std::fmt::Write;
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

/// A ring mark as `Path` commands in a 100 x 100 box, for `RingCover`'s `Sigil`: the rings to
/// stroke, then the hits and the rests to fill.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Sigil {
    pub rings: String,
    pub hits: String,
    pub rests: String,
}

/// One euclidean rhythm ring per name (up to 12), each with as many steps as there are names. A
/// name picks how many hits its ring gets; its position rotates the ring.
pub fn sigil<'a>(names: impl ExactSizeIterator<Item = &'a str>) -> Sigil {
    let names: Vec<&str> = names.collect();
    let steps = names.len();
    let mut out = Sigil::default();
    if steps == 0 {
        return out;
    }
    let rings = steps.min(12);
    let outer = 43.0;
    let gap = outer * 0.8 / rings as f64;
    for (r, name) in names.iter().take(rings).enumerate() {
        let radius = outer - r as f64 * gap;
        // Keep the whole hit inside the box, including sparse one/two-name rings.
        let dot = (gap * 0.32).min(radius * PI / steps as f64 * 0.4).min(48.0 - radius);
        let hits = 1 + (fnv1a(name) % (steps / 2).max(1) as u32) as usize;
        circle(&mut out.rings, 50.0, 50.0, radius);
        for i in 0..steps {
            let angle = 2.0 * PI * (i + r) as f64 / steps as f64 - PI / 2.0;
            let (x, y) = (50.0 + radius * angle.cos(), 50.0 + radius * angle.sin());
            if (i * hits) % steps < hits {
                circle(&mut out.hits, x, y, dot);
            } else {
                circle(&mut out.rests, x, y, dot * 0.4);
            }
        }
    }
    out
}

fn circle(path: &mut String, x: f64, y: f64, r: f64) {
    let _ = write!(
        path,
        "M {:.2} {y:.2} A {r:.2} {r:.2} 0 1 0 {:.2} {y:.2} A {r:.2} {r:.2} 0 1 0 {:.2} {y:.2} Z ",
        x - r,
        x + r,
        x - r
    );
}

/// FNV-1a over UTF-16 units. Changing it would change every ring mark already drawn.
fn fnv1a(s: &str) -> u32 {
    s.encode_utf16().fold(2166136261u32, |h, c| (h ^ c as u32).wrapping_mul(16777619))
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

    #[test]
    fn sigil_has_one_ring_per_name_up_to_twelve() {
        let rings = |n: usize| {
            let names: Vec<String> = (0..n).map(|i| i.to_string()).collect();
            sigil(names.iter().map(String::as_str)).rings.matches('M').count()
        };
        assert_eq!((rings(0), rings(3), rings(20)), (0, 3, 12));
    }

    #[test]
    fn sigil_steps_are_hits_or_rests() {
        let s = sigil(["a", "b", "c"].into_iter());
        assert_eq!(s.hits.matches('M').count() + s.rests.matches('M').count(), 9);
    }
}
