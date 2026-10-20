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
