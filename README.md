# WinSafe examples

This repo contains several examples of native Win32 applications written in [Rust](https://www.rust-lang.org) with [WinSafe](https://crates.io/crates/winsafe). All examples follow the same program structure, which is the recommended way to build a WinSafe application.

Each directory is a full application, with is own `Cargo.toml`. Note that WinSafe dependency is set to a sibling local directory for testing purposes, but you can change it to the normal [crates.io](https://crates.io/crates/winsafe) if you want to.

The directories are numbered starting from the easiest example. Note that this order can change at any time, as new examples can be added.

## Resources

Each example has a `.res` file with its application resources (manifests, icons, dialogs and so on). You can edit the `.res` file with any resource editor, or even generate your own `.res` by compiling a `.rc` script.

The `.res` file is linked into the final `.exe` by the `build.rs`, which is a [Cargo build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html).

## Examples list

1. [Button click](01_button_click/)

![Example 01](01_button_click/screen.gif)

2. [Combo and radios](02_combo_and_radios/)

![Example 02](02_combo_and_radios/screen.gif)

## License

Licensed under [MIT license](https://opensource.org/licenses/MIT), see [LICENSE.md](LICENSE.md) for details.
