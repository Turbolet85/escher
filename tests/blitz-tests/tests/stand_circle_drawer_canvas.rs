//! Circle Drawer's drawing area is a canvas painted from Rust: an `<object>` carrying a widget
//! that paints the app's circles and each circle's diameter as text shaped from the document's
//! own fonts, the bundled DejaVu Sans on the stand. The canvas states `role="group"` named
//! «Drawing area», every control reads its author key and the lint reads nothing at boot; a
//! click on the canvas paints a circle and its diameter, undo and redo repaint it, and two
//! clicks at one point open the resize dialog, whose slider resizes the circle. The circles
//! leave the DOM and cross as shot pixels alone. A driver `click` lands at the canvas's centre,
//! so every circle stands there. Booted in process through `stand::boot_circle_drawer` and read
//! through the driver, in both layout modes. A failure names a layout mode and an index only.

use accesskit::Role;
use escher_driver::Session;
use seven_guis::stand;

mod session_common;
use session_common::{act, advance, click, lint, linted, screenshot_of, shot};

const CANVAS: &str = "circle-canvas";

/// A session on Circle Drawer, booted in the stand's shell.
fn circle_drawer(incremental: bool) -> Session {
    Session::start("circle-drawer", move || {
        stand::boot_circle_drawer(stand::options(incremental))
    })
    .expect("circle-drawer is a label")
}

/// The circles the canvas carries, as its `data-circles` attribute reads.
fn circles(session: &Session) -> Option<String> {
    session
        .harness()
        .attr(&format!("#{CANVAS}"), "data-circles")
}

/// The canvas's element shot: its PNG bytes.
fn canvas_shot(session: &mut Session) -> Vec<u8> {
    shot(session, &screenshot_of(CANVAS))
        .expect("the canvas shot is accepted")
        .png
}

fn decode(png_bytes: &[u8]) -> (u32, u32, Vec<u8>) {
    let decoder = png::Decoder::new(std::io::Cursor::new(png_bytes));
    let mut reader = decoder.read_info().expect("the shot is a PNG");
    let mut buf = vec![0; reader.output_buffer_size().expect("a bounded image")];
    let info = reader.next_frame(&mut buf).expect("the PNG holds a frame");
    assert!(info.color_type == png::ColorType::Rgba && info.bit_depth == png::BitDepth::Eight);
    buf.truncate(info.buffer_size());
    (info.width, info.height, buf)
}

fn pixel(rgba: &[u8], width: u32, x: u32, y: u32) -> [u8; 4] {
    let at = ((y * width + x) * 4) as usize;
    [rgba[at], rgba[at + 1], rgba[at + 2], rgba[at + 3]]
}

/// The pixels inside `(left, top, right, bottom)` whose every colour channel is at most 128.
fn dark_pixels(rgba: &[u8], width: u32, (left, top, right, bottom): (u32, u32, u32, u32)) -> usize {
    (top..bottom)
        .flat_map(|y| (left..right).map(move |x| (x, y)))
        .filter(|&(x, y)| pixel(rgba, width, x, y)[..3].iter().all(|&c| c <= 128))
        .count()
}

/// The first circle's centre in the canvas shot: its border box's origin plus the canvas's
/// border, plus the centre the app records.
const CENTRE: (u32, u32) = (350, 200);

/// The box the first circle's diameter label is drawn in, around [`CENTRE`].
const LABEL_BOX: (u32, u32, u32, u32) = (CENTRE.0 - 12, CENTRE.1 - 9, CENTRE.0 + 12, CENTRE.1 + 9);

#[test]
fn circle_drawer_boots_with_a_named_keyed_canvas_and_no_lint_finding() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");
        let mut session = circle_drawer(incremental);
        let screen = session.harness().doc.snapshot();
        let Some(canvas) = screen.get(CANVAS) else {
            panic!("{at}: the canvas is on the screen");
        };
        assert!(canvas.role == Role::Group, "{at}: the canvas is a group");
        assert!(canvas.name == "Drawing area", "{at}: the canvas is named");
        assert!(canvas.state.clickable, "{at}: the canvas takes a click");
        for (index, id) in ["circle-undo", "circle-redo"].into_iter().enumerate() {
            assert!(
                screen.get(id).map(|node| node.state.enabled) == Some(Some(false)),
                "{at}: control {index} reads not enabled"
            );
        }
        assert!(
            linted(&mut session, &lint()).is_some_and(|findings| findings.is_empty()),
            "{at}: the lint reads no finding"
        );
        assert!(
            session.harness().doc.unkeyed_actionable().is_empty(),
            "{at}: every control reads its author key"
        );
    }
}

#[test]
fn a_click_paints_a_circle_and_its_diameter_on_the_canvas() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");
        let mut session = circle_drawer(incremental);
        let boot = canvas_shot(&mut session);
        let (boot_width, _, boot_rgba) = decode(&boot);
        assert!(
            dark_pixels(&boot_rgba, boot_width, LABEL_BOX) == 0,
            "{at}: the canvas holds no text at boot"
        );

        assert!(
            act(&mut session, &click(CANVAS)).is_some(),
            "{at}: the click is accepted"
        );
        assert!(
            circles(&session).as_deref() == Some("348,198,40"),
            "{at}: the canvas carries one circle of diameter 40 at the centre"
        );
        let after = canvas_shot(&mut session);
        assert!(after != boot, "{at}: the canvas repaints");
        let (width, height, rgba) = decode(&after);
        assert!(
            (width, height) == (700, 400),
            "{at}: the shot is the canvas's border box"
        );
        assert!(
            pixel(&rgba, width, CENTRE.0, CENTRE.1 - 19) == [58, 92, 229, 255],
            "{at}: the ring's top is the stroke colour"
        );
        assert!(
            pixel(&rgba, width, CENTRE.0 + 19, CENTRE.1) == [70, 102, 231, 255],
            "{at}: the ring's right side is the measured stroke"
        );
        assert!(
            pixel(&rgba, width, CENTRE.0 + 14, CENTRE.1) == [226, 231, 251, 255],
            "{at}: the disc is the measured fill"
        );
        assert!(
            dark_pixels(&rgba, width, LABEL_BOX) == 40,
            "{at}: the diameter's glyphs are drawn at the centre"
        );
    }
}

#[test]
fn undo_and_redo_repaint_the_canvas() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");
        let mut session = circle_drawer(incremental);
        let boot = canvas_shot(&mut session);
        assert!(
            act(&mut session, &click(CANVAS)).is_some(),
            "{at}: the click is accepted"
        );
        let drawn = canvas_shot(&mut session);
        assert!(drawn != boot, "{at}: the click repaints the canvas");
        assert!(
            act(&mut session, &advance(600)).is_some(),
            "{at}: the advance is accepted"
        );

        assert!(
            act(&mut session, &click("circle-undo")).is_some(),
            "{at}: undo is accepted"
        );
        assert!(
            canvas_shot(&mut session) == boot,
            "{at}: undo repaints the canvas as it booted"
        );
        assert!(
            act(&mut session, &click("circle-redo")).is_some(),
            "{at}: redo is accepted"
        );
        assert!(
            canvas_shot(&mut session) == drawn,
            "{at}: redo repaints the circle"
        );
    }
}

#[test]
fn two_centre_clicks_open_the_dialog_and_its_slider_resizes_the_circle() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");
        let mut session = circle_drawer(incremental);
        for index in 0..2 {
            assert!(
                act(&mut session, &click(CANVAS)).is_some(),
                "{at}: click {index} is accepted"
            );
        }
        let screen = session.harness().doc.snapshot();
        assert!(
            screen.get("circle-diameter").is_some() && screen.get("circle-close").is_some(),
            "{at}: a double click opens the dialog"
        );

        let before = (circles(&session), canvas_shot(&mut session));
        assert!(
            act(&mut session, &click("circle-diameter")).is_some(),
            "{at}: the slider's click is accepted"
        );
        let after = (circles(&session), canvas_shot(&mut session));
        assert!(
            before.0.as_deref() == Some("348,198,40;348,198,40"),
            "{at}: both clicks added a circle at the centre"
        );
        assert!(
            after.0.as_deref() == Some("348,198,53;348,198,40"),
            "{at}: the slider resizes the circle the dialog names"
        );
        assert!(after.1 != before.1, "{at}: the canvas repaints the circle");

        assert!(
            act(&mut session, &click("circle-close")).is_some(),
            "{at}: Close is accepted"
        );
        assert!(
            session
                .harness()
                .doc
                .snapshot()
                .get("circle-diameter")
                .is_none(),
            "{at}: Close removes the dialog"
        );
    }
}
