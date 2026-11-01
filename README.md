# kitka

The shared look of Albumka and its sibling apps, in Slint: `ui/theme.slint` (colors, fonts, text
sizes) and `ui/widgets.slint` (buttons, fields, chips…). A change here reaches every app on its
next build.

## Using it in an app

`Cargo.toml`:

```toml
[build-dependencies]
kitka = { path = "../kitka" }  # or { git = "https://github.com/vsvsokol/kitka" } once it's on GitHub
slint-build = "1.18"
```

`build.rs`:

```rust
let config = slint_build::CompilerConfiguration::new().with_library_paths(kitka::library_paths());
slint_build::compile_with_config("ui/app.slint", config).unwrap();
```

In `.slint` files:

```slint
import { Theme } from "@kitka/theme.slint";
import { Btn, Field } from "@kitka/widgets.slint";
```

The app also depends on kitka at run time (same commit) for `kitka::accent`, which makes a color
readable as `Theme.accent`, and `kitka::sigil`, which draws the ring mark `RingCover` shows. Its
window keeps the `Screen` global's size current, so menus near the bottom open upward and tips
near the right edge hang to the left:

```slint
changed width => { Screen.width = self.width; }
changed height => { Screen.height = self.height; }
```
