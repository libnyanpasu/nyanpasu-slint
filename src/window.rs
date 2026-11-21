//! Window placement and system input settings.

use i_slint_backend_winit::winit::event_loop::ActiveEventLoop;
use slint::{LogicalSize, PhysicalPosition};

/// Places a window of `size` in the middle of the primary monitor. Call it
/// from the event loop's `resumed`, before winit creates the window: Slint
/// keeps the position for the creation, so the window opens there instead of
/// where the system puts new windows.
pub fn centre(window: &slint::Window, size: LogicalSize, event_loop: &ActiveEventLoop) {
    let Some(monitor) = event_loop
        .primary_monitor()
        .or_else(|| event_loop.available_monitors().next())
    else {
        return;
    };
    let size = size.to_physical(monitor.scale_factor() as f32);
    let (origin, area) = (monitor.position(), monitor.size());
    window.set_position(PhysicalPosition::new(
        origin.x + (area.width as i32 - size.width as i32) / 2,
        origin.y + (area.height as i32 - size.height as i32) / 2,
    ));
}

/// Lines a wheel notch scrolls (Windows' mouse settings), or -1 for a screen
/// at a time. 3 elsewhere. `LogView` and `WheelSteps` take it as `wheel-lines`.
#[cfg(windows)]
pub fn wheel_lines() -> i32 {
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        SPI_GETWHEELSCROLLLINES, SystemParametersInfoW,
    };
    let mut lines = 3u32;
    let read = unsafe {
        SystemParametersInfoW(
            SPI_GETWHEELSCROLLLINES,
            0,
            std::ptr::from_mut(&mut lines).cast(),
            0,
        )
    };
    match (read, lines) {
        (0, _) => 3,
        // WHEEL_PAGESCROLL
        (_, u32::MAX) => -1,
        (_, lines) => i32::try_from(lines).unwrap_or(3),
    }
}

#[cfg(not(windows))]
pub fn wheel_lines() -> i32 {
    3
}

/// Characters a sideways wheel notch scrolls (Windows' mouse settings). 3
/// elsewhere. `LogView` and `WheelSteps` take it as `wheel-chars`.
#[cfg(windows)]
pub fn wheel_chars() -> i32 {
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        SPI_GETWHEELSCROLLCHARS, SystemParametersInfoW,
    };
    let mut chars = 3u32;
    let read = unsafe {
        SystemParametersInfoW(
            SPI_GETWHEELSCROLLCHARS,
            0,
            std::ptr::from_mut(&mut chars).cast(),
            0,
        )
    };
    match read {
        0 => 3,
        _ => i32::try_from(chars).unwrap_or(3),
    }
}

#[cfg(not(windows))]
pub fn wheel_chars() -> i32 {
    3
}
