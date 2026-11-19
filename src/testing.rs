//! A headless platform for render tests: windows render with the software
//! renderer into a new buffer each frame, time only moves when the test moves
//! it, and the clipboard is the platform's own.

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    time::Duration,
};

use slint::{
    PlatformError,
    platform::{
        Clipboard, Platform, WindowAdapter,
        software_renderer::{MinimalSoftwareWindow, RepaintBufferType},
    },
};

/// Install with `slint::platform::set_platform(Box::new(platform))`, once per
/// test thread.
#[derive(Default)]
pub struct HeadlessPlatform {
    clock: Rc<Cell<Duration>>,
    clipboard: Rc<RefCell<Option<String>>>,
}

impl HeadlessPlatform {
    /// The platform's clock, for advancing animations and timers.
    pub fn clock(&self) -> Rc<Cell<Duration>> {
        self.clock.clone()
    }

    /// What windows copied to the clipboard.
    pub fn clipboard(&self) -> Rc<RefCell<Option<String>>> {
        self.clipboard.clone()
    }
}

impl Platform for HeadlessPlatform {
    fn duration_since_start(&self) -> Duration {
        self.clock.get()
    }

    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        Ok(MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer))
    }

    fn set_clipboard_text(&self, text: &str, clipboard: Clipboard) {
        if clipboard == Clipboard::DefaultClipboard {
            *self.clipboard.borrow_mut() = Some(text.to_owned());
        }
    }

    fn clipboard_text(&self, clipboard: Clipboard) -> Option<String> {
        (clipboard == Clipboard::DefaultClipboard)
            .then(|| self.clipboard.borrow().clone())
            .flatten()
    }
}
