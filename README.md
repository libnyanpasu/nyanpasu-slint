# nyanpasu-slint

Slint components and platform helpers shared by
[Clash Nyanpasu](https://github.com/libnyanpasu/clash-nyanpasu) (its crash reporter and
startup-failure windows) and its Windows installer. Everything here renders well with Slint's
software renderer, which all of them use on Windows and macOS.

Slint's internal crates only stay compatible within a release, so this crate and its dependents pin
`slint`, `i-slint-backend-winit` and `slint-build` to the same exact version, currently `=1.19.0`.
Move them together.

## Contents

`.slint` library (`ui/`, Slint edition 1.19):

| Item | What it is |
| --- | --- |
| `Fluent` | WinUI's layer colours (base, content layer, card, stroke, subtle fills) for light and dark. |
| `ContentLayer` | The translucent layer content is drawn on, edged like WinUI's. |
| `SubtleButton` | WinUI's subtle button: an outlined 16×16 path icon and a label, filled only on hover. |
| `LogView` | Read-only text on a card: optionally selectable, wrapped or scrolling sideways, following its end. |
| `WheelSteps` | Wheel scrolling by the system's wheel lines (sideways, wheel characters), one frame a notch instead of an animation. |

Rust (`src/`):

| Item | What it is |
| --- | --- |
| `monospace_font()` | The first installed family of Clash Nyanpasu's monospace stack. |
| `centre()` | Opens a window in the middle of the primary monitor. |
| `wheel_lines()` | The lines a wheel notch scrolls (Windows' mouse settings), for `wheel-lines`. |
| `wheel_chars()` | The characters a sideways wheel notch scrolls (Windows' mouse settings), for `wheel-chars`. |
| `testing::HeadlessPlatform` | Feature `testing`: a software-rendered headless platform with a test clock and clipboard. |

## Use

```toml
[dependencies]
nyanpasu-slint = { git = 'https://github.com/libnyanpasu/nyanpasu-slint', rev = '…' }
```

The crate's build script exports the library's project file. Map it as a library path in the
dependent's `build.rs`:

```rust
let project = std::env::var_os("DEP_NYANPASU_SLINT_PROJECT").unwrap();
let config = slint_build::CompilerConfiguration::new()
    .with_library_paths([("nyanpasu-slint".into(), project.into())].into());
```

```slint
import { LogView } from "@nyanpasu-slint";

LogView {
    text: root.log;
    font-family: root.monospace-font; // nyanpasu_slint::monospace_font()
    wheel-lines: root.wheel-lines;    // nyanpasu_slint::wheel_lines()
    wheel-chars: root.wheel-chars;    // nyanpasu_slint::wheel_chars()
    selectable: true;
    wrap: false;
}
```

## LogView

Slint's `TextEdit` and `ScrollView` animate every wheel notch for about 28 frames, and a read-only
`TextInput` lays out all of its text on each of them. `LogView` draws `Text` (or `StyledText`) and
scrolls through `WheelSteps`, one frame a notch, as Notepad++ does. Scrolling 2,000 or 20,000 lines
in a real 200% window with the software renderer, 100 notches 250 ms apart:

| View | Frames a notch | Main-thread CPU a notch |
| --- | --- | --- |
| Slint `TextEdit` (read-only) | 28 | 153–158 ms |
| Slint `StyledText` in a `ScrollView` | 28 | 145–152 ms |
| `LogView` | 1 | 6.1–6.9 ms |
| `LogView`, `selectable` | 1 | 6.6–7.5 ms |

- `selectable` swaps the `Text` for a `StyledText` (Slint 1.19) whose parent is `WheelSteps`, so the
  text takes the presses that select and only the wheel goes up. Off, the view holds the plain
  `Text` and nothing of the selection: `StyledText` takes presses even when it can't select.
- The text is interpolated into `@markdown`, so it stays plain: copying gives back exactly what
  was set, markup characters included.
- Dragging a selection past the view doesn't scroll it; Ctrl+A selects everything.
- The `follow-end` view scrolls to new lines only while the user hasn't scrolled up.

## Develop

```sh
cargo test --features testing
cargo clippy --all-targets --features testing -- -D warnings
```

## License

GPL-3.0, as Clash Nyanpasu.
