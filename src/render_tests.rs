use std::{cell::RefCell, rc::Rc};

use slint::{
    ComponentHandle, LogicalPosition, SharedPixelBuffer,
    platform::{Key, PointerEventButton, WindowEvent, set_platform},
};

use crate::testing::HeadlessPlatform;

slint::include_modules!();

/// Installs the headless platform on this test's thread and returns its
/// clipboard.
fn platform() -> Rc<RefCell<Option<String>>> {
    let platform = HeadlessPlatform::default();
    let clipboard = platform.clipboard();
    set_platform(Box::new(platform)).expect("one platform per test thread");
    clipboard
}

fn wheel(window: &slint::Window, notches: f32) {
    window.dispatch_event(WindowEvent::PointerScrolled {
        position: LogicalPosition::new(50., 50.),
        delta_x: 0.,
        delta_y: notches * 60.,
    });
}

#[test]
fn a_notch_scrolls_the_system_wheel_lines() {
    let _ = platform();
    let view = WheelHarness::new().unwrap();
    wheel(view.window(), -1.);
    assert_eq!(view.get_content_y(), -30.);
    // Half notches add up to a whole one.
    wheel(view.window(), -0.5);
    assert_eq!(view.get_content_y(), -30.);
    wheel(view.window(), -0.5);
    assert_eq!(view.get_content_y(), -60.);
    // Never past the start or the end.
    wheel(view.window(), 10.);
    assert_eq!(view.get_content_y(), 0.);
    wheel(view.window(), -1000.);
    assert_eq!(view.get_content_y(), -900.);
}

#[test]
fn a_negative_wheel_lines_scrolls_a_screen_less_a_line() {
    let _ = platform();
    let view = WheelHarness::new().unwrap();
    view.set_wheel_lines(-1);
    wheel(view.window(), -1.);
    assert_eq!(view.get_content_y(), -90.);
}

fn snapshot(view: &LogHarness) -> SharedPixelBuffer<slint::Rgba8Pixel> {
    view.show().unwrap();
    view.window().set_size(slint::LogicalSize::new(200., 120.));
    view.window().take_snapshot().unwrap()
}

#[test]
fn follow_end_keeps_the_newest_line_in_view() {
    let _ = platform();
    let lines = (0..50)
        .map(|i| format!("line {i}"))
        .collect::<Vec<_>>()
        .join("\n");
    let pinned = LogHarness::new().unwrap();
    pinned.set_follow_end(false);
    pinned.set_text(lines.clone().into());
    let following = LogHarness::new().unwrap();
    following.set_text(lines.into());
    assert_ne!(
        snapshot(&pinned).as_bytes(),
        snapshot(&following).as_bytes()
    );

    // Scrolled up, the text stays where the user put it as lines arrive; only
    // the scroll bar along the right edge changes.
    wheel(following.window(), 1.);
    let before = text_area(&snapshot(&following));
    following.set_text(format!("{}\nline 50", following.get_text()).into());
    assert!(before == text_area(&snapshot(&following)), "the text moved");
}

/// The pixels left of the scroll bar.
fn text_area(snapshot: &SharedPixelBuffer<slint::Rgba8Pixel>) -> Vec<slint::Rgba8Pixel> {
    let width = snapshot.width() as usize;
    snapshot
        .as_slice()
        .chunks(width)
        .flat_map(|row| &row[..width - 30])
        .copied()
        .collect()
}

fn drag(window: &slint::Window, from: (f32, f32), to: (f32, f32)) {
    let at = |(x, y)| LogicalPosition::new(x, y);
    let button = PointerEventButton::Left;
    window.dispatch_event(WindowEvent::PointerMoved { position: at(from) });
    window.dispatch_event(WindowEvent::PointerPressed {
        position: at(from),
        button,
    });
    window.dispatch_event(WindowEvent::PointerMoved { position: at(to) });
    window.dispatch_event(WindowEvent::PointerReleased {
        position: at(to),
        button,
    });
}

fn ctrl(window: &slint::Window, key: char) {
    let control = slint::SharedString::from(Key::Control);
    window.dispatch_event(WindowEvent::KeyPressed {
        text: control.clone(),
    });
    window.dispatch_event(WindowEvent::KeyPressed { text: key.into() });
    window.dispatch_event(WindowEvent::KeyReleased { text: key.into() });
    window.dispatch_event(WindowEvent::KeyReleased { text: control });
}

/// Lines that are markup to `@markdown`, and longer than the view is wide.
const MARKUP: &str = "*not emphasis* <u>not underlined</u> `not code` [not](a link)
    # not a heading
    - not a list item, and long enough to run well past the right edge of the view";

#[test]
fn selectable_text_copies_as_it_was_given() {
    let clipboard = platform();
    let view = LogHarness::new().unwrap();
    view.set_selectable(true);
    view.set_text(MARKUP.into());
    snapshot(&view);
    drag(view.window(), (20., 15.), (60., 15.));
    ctrl(view.window(), 'c');
    let part = clipboard.borrow().clone().expect("a drag selects");
    assert!(!part.is_empty() && MARKUP.contains(&part), "{part:?}");

    ctrl(view.window(), 'a');
    ctrl(view.window(), 'c');
    assert_eq!(clipboard.borrow().as_deref(), Some(MARKUP));
}

#[test]
fn text_that_isnt_selectable_takes_no_selection() {
    let clipboard = platform();
    let view = LogHarness::new().unwrap();
    view.set_text(MARKUP.into());
    snapshot(&view);
    drag(view.window(), (20., 15.), (60., 15.));
    ctrl(view.window(), 'a');
    ctrl(view.window(), 'c');
    assert_eq!(*clipboard.borrow(), None);
}

#[test]
fn selectable_text_scrolls_a_notch_at_a_time() {
    let _ = platform();
    let lines = (0..50)
        .map(|i| format!("line {i}"))
        .collect::<Vec<_>>()
        .join(
            "
",
        );
    let view = LogHarness::new().unwrap();
    view.set_selectable(true);
    view.set_follow_end(false);
    view.set_text(lines.into());
    let top = text_area(&snapshot(&view));
    wheel(view.window(), -1.);
    assert!(
        top != text_area(&snapshot(&view)),
        "the wheel didn't scroll"
    );
}

#[test]
fn unwrapped_lines_run_past_the_edge() {
    let _ = platform();
    for selectable in [false, true] {
        let wrapped = LogHarness::new().unwrap();
        wrapped.set_selectable(selectable);
        wrapped.set_follow_end(false);
        wrapped.set_text(MARKUP.into());
        let unwrapped = LogHarness::new().unwrap();
        unwrapped.set_selectable(selectable);
        unwrapped.set_follow_end(false);
        unwrapped.set_wrap(false);
        unwrapped.set_text(MARKUP.into());
        assert!(
            text_area(&snapshot(&wrapped)) != text_area(&snapshot(&unwrapped)),
            "selectable: {selectable}"
        );
    }
}
