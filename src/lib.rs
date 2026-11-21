//! Slint components and platform helpers shared by Clash Nyanpasu and its
//! installer.
//!
//! The `.slint` library is in `ui/`. A dependent's build script maps it as a
//! library path, from the `DEP_NYANPASU_SLINT_PROJECT` this crate's build
//! script exports:
//!
//! ```ignore
//! let project = std::env::var_os("DEP_NYANPASU_SLINT_PROJECT").unwrap();
//! let config = slint_build::CompilerConfiguration::new()
//!     .with_library_paths([("nyanpasu-slint".into(), project.into())].into());
//! ```
//!
//! and its `.slint` files import `{ LogView } from "@nyanpasu-slint"`.

mod font;
#[cfg(feature = "testing")]
pub mod testing;
mod window;

pub use font::monospace_font;
pub use window::{centre, wheel_chars, wheel_lines};

#[cfg(all(test, feature = "testing"))]
mod render_tests;
