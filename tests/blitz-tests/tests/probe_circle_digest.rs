//! PROBE ONLY (branch `build/probe-circle-digest`, never merged): measures why Circle Drawer's
//! canvas element shot after one centre click reads another digest on macOS than on linux.
//! It shoots the canvas four ways (the circle and its label, the circle alone, the label alone,
//! neither), in both layout modes, and writes on each host one line per shot straight to the
//! process's stderr (past libtest's capture, so CI's log holds it on a green run): the answer
//! digest, and the pixels that differ from the linux bytes committed beside it — their count,
//! their bounding box and the largest channel difference, and the first few of them.

use std::io::Write as _;

use escher_driver::{Outcome, Session, answer_digest};
use seven_guis::stand;
use seven_guis::tasks::circle_drawer::PROBE_PAINT;

mod session_common;
use session_common::{act, click, screenshot_of};

const VARIANTS: [(&str, u8); 6] = [
    ("both", 0),
    ("circle", 1),
    ("text", 2),
    ("none", 3),
    ("text-snapped", 4),
    ("both-snapped", 5),
];

fn reference_path(variant: &str, incremental: bool) -> String {
    format!(
        "{}/tests/probe_data/linux-{variant}-{}.png",
        env!("CARGO_MANIFEST_DIR"),
        if incremental { "incremental" } else { "full" }
    )
}

/// Boots Circle Drawer, clicks the canvas's centre and shoots the canvas with `variant` painted.
fn shoot(paint: u8, incremental: bool) -> (u64, Vec<u8>) {
    PROBE_PAINT.store(paint, std::sync::atomic::Ordering::SeqCst);
    let mut session = Session::start("circle-drawer", move || {
        stand::boot_circle_drawer(stand::options(incremental))
    })
    .expect("circle-drawer is a label");
    assert!(act(&mut session, &click("circle-canvas")).is_some());
    let answer = session.run(&screenshot_of("circle-canvas"));
    let digest = answer_digest(&answer);
    let Ok(Outcome::Shot { png, .. }) = answer else {
        panic!("the shot is accepted");
    };
    PROBE_PAINT.store(0, std::sync::atomic::Ordering::SeqCst);
    (digest, png)
}

fn decode(png_bytes: &[u8]) -> (u32, u32, Vec<u8>) {
    let decoder = png::Decoder::new(std::io::Cursor::new(png_bytes));
    let mut reader = decoder.read_info().expect("a PNG");
    let mut buf = vec![0; reader.output_buffer_size().expect("bounded")];
    let info = reader.next_frame(&mut buf).expect("a frame");
    buf.truncate(info.buffer_size());
    (info.width, info.height, buf)
}

/// The differing pixels of `host` against `linux`, as one text.
fn compare(host: &[u8], linux: &[u8]) -> String {
    let (w, h, a) = decode(host);
    let (lw, lh, b) = decode(linux);
    if (w, h) != (lw, lh) {
        return format!("size host {w}x{h} linux {lw}x{lh}");
    }
    let mut count = 0usize;
    let mut max_delta = 0u8;
    let mut le1 = 0usize;
    let (mut x0, mut y0, mut x1, mut y1) = (u32::MAX, u32::MAX, 0u32, 0u32);
    let mut first = Vec::new();
    for y in 0..h {
        for x in 0..w {
            let at = ((y * w + x) * 4) as usize;
            let p = &a[at..at + 4];
            let q = &b[at..at + 4];
            if p != q {
                count += 1;
                let d = p
                    .iter()
                    .zip(q)
                    .map(|(m, n)| m.abs_diff(*n))
                    .max()
                    .unwrap_or(0);
                max_delta = max_delta.max(d);
                if d <= 1 {
                    le1 += 1;
                }
                x0 = x0.min(x);
                y0 = y0.min(y);
                x1 = x1.max(x);
                y1 = y1.max(y);
                if first.len() < 12 {
                    first.push(format!("({x},{y}) host {p:?} linux {q:?}"));
                }
            }
        }
    }
    if count == 0 {
        return "equal to linux".to_owned();
    }
    format!(
        "differ {count} px · bbox ({x0},{y0})-({x1},{y1}) · max channel delta {max_delta} · \
         delta<=1 {le1} px · first: {}",
        first.join(" ")
    )
}

fn emit(line: &str) {
    let mut err = std::io::stderr().lock();
    let _ = writeln!(
        err,
        "PROBE {} {} {line}",
        std::env::consts::OS,
        std::env::consts::ARCH
    );
}

#[test]
fn probe_circle_drawer_canvas_digests() {
    for incremental in [false, true] {
        for (variant, paint) in VARIANTS {
            let (digest, png) = shoot(paint, incremental);
            let against = match std::fs::read(reference_path(variant, incremental)) {
                Ok(linux) => compare(&png, &linux),
                Err(_) => "no linux reference".to_owned(),
            };
            emit(&format!(
                "variant={variant} incremental={incremental} digest={digest:#018x} {against}"
            ));
        }
    }
}

/// Run on the linux dev host alone, with `--ignored`: writes the linux references.
#[test]
#[ignore = "writes the linux references on the dev host"]
fn probe_record_linux_references() {
    assert!(std::env::consts::OS == "linux");
    std::fs::create_dir_all(format!("{}/tests/probe_data", env!("CARGO_MANIFEST_DIR")))
        .expect("the reference directory");
    for incremental in [false, true] {
        for (variant, paint) in VARIANTS {
            let (digest, png) = shoot(paint, incremental);
            std::fs::write(reference_path(variant, incremental), &png).expect("written");
            emit(&format!(
                "recorded variant={variant} incremental={incremental} digest={digest:#018x}"
            ));
        }
    }
}
