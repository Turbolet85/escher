//! PROBE (build/probe-canvas-dump, never merged): Circle Drawer's canvas read as draw commands
//! through blitz-paint's `custom_widget_scene` (fork F9 A), written by hand as one line per
//! command (F2 A), glyph runs in three forms (F3 A: counts, origins' box and a digest of the
//! glyph ids; B: glyph ids and origins; C: the text recovered from the font's cmap), fonts by
//! index and digest (F4 A), and the summary level (F6 A). It measures the dump's bytes at 1, 12
//! and 100 circles and holds each dump's FNV-1a 64 to one table, so a run on linux, windows and
//! macOS reading the one table is the cross-OS reading.

// A throwaway probe on a build/ branch: a missing canvas or scene is the probe's own failure.
#![allow(clippy::expect_used)]

use std::collections::HashMap;
use std::fmt::Write as _;

use anyrender::Paint;
use anyrender::recording::RenderCommand;
use blitz_dom::{LocalName, Namespace, QualName};
use peniko::kurbo::{BezPath, PathEl};
use seven_guis::stand;

/// One reading: circles, form, layout mode, bytes of the dump, FNV-1a 64 of the dump.
type Row = (usize, &'static str, bool, usize, u64);

const TABLE: &[Row] = &[
    (1, "full-a", false, 555, 0x961667dcff8505de),
    (1, "full-b", false, 540, 0xc51da4c6935db441),
    (1, "full-c", false, 540, 0xcad1363258f91b53),
    (1, "summary", false, 152, 0xe3e030b3d3c52a14),
    (12, "full-a", false, 5426, 0xc1ad360c0f6a5f6b),
    (12, "full-b", false, 5246, 0xeec68cc4d701f774),
    (12, "full-c", false, 5246, 0x202e5358e245a622),
    (12, "summary", false, 155, 0x900a87a647679fe2),
    (100, "full-a", false, 44993, 0x1c49755c3d10f702),
    (100, "full-b", false, 43493, 0xa06afded71345d50),
    (100, "full-c", false, 43493, 0x24a38e14eb32af1f),
    (100, "summary", false, 158, 0xe79dd6df43d981f9),
    (1, "full-a", true, 555, 0x961667dcff8505de),
    (1, "full-b", true, 540, 0xc51da4c6935db441),
    (1, "full-c", true, 540, 0xcad1363258f91b53),
    (1, "summary", true, 152, 0xe3e030b3d3c52a14),
    (12, "full-a", true, 5426, 0xc1ad360c0f6a5f6b),
    (12, "full-b", true, 5246, 0xeec68cc4d701f774),
    (12, "full-c", true, 5246, 0x202e5358e245a622),
    (12, "summary", true, 155, 0x900a87a647679fe2),
    (100, "full-a", true, 44993, 0x1c49755c3d10f702),
    (100, "full-b", true, 43493, 0xa06afded71345d50),
    (100, "full-c", true, 43493, 0x24a38e14eb32af1f),
    (100, "summary", true, 158, 0xe79dd6df43d981f9),
];

const COUNTS: [usize; 3] = [1, 12, 100];

const FORMS: [&str; 4] = ["full-a", "full-b", "full-c", "summary"];

fn fnv(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in bytes {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    h
}

/// A number rounded to 1/1000, written shortest, `-0` as `0`.
fn num(v: f64) -> String {
    let r = (v * 1000.0).round() / 1000.0;
    if r == 0.0 { "0".into() } else { format!("{r}") }
}

fn affine(a: peniko::kurbo::Affine) -> String {
    let c = a.as_coeffs();
    if c == [1.0, 0.0, 0.0, 1.0, 0.0, 0.0] {
        return "id".into();
    }
    c.iter().map(|v| num(*v)).collect::<Vec<_>>().join(",")
}

fn path(p: &BezPath) -> String {
    let mut out = String::new();
    for el in p.elements() {
        match el {
            PathEl::MoveTo(a) => write!(out, "M{},{}", num(a.x), num(a.y)),
            PathEl::LineTo(a) => write!(out, "L{},{}", num(a.x), num(a.y)),
            PathEl::QuadTo(a, b) => {
                write!(out, "Q{},{},{},{}", num(a.x), num(a.y), num(b.x), num(b.y))
            }
            PathEl::CurveTo(a, b, c) => write!(
                out,
                "C{},{},{},{},{},{}",
                num(a.x),
                num(a.y),
                num(b.x),
                num(b.y),
                num(c.x),
                num(c.y)
            ),
            PathEl::ClosePath => write!(out, "Z"),
        }
        .ok();
    }
    out
}

fn brush(p: &Paint) -> String {
    match p {
        Paint::Solid(c) => {
            let c = c.to_rgba8();
            format!("#{:02x}{:02x}{:02x}{:02x}", c.r, c.g, c.b, c.a)
        }
        _ => "other".into(),
    }
}

/// The fonts a dump names, by index, each with its bytes' digest.
#[derive(Default)]
struct Fonts {
    digests: Vec<u64>,
    cmaps: Vec<HashMap<u32, char>>,
    seen: HashMap<u64, usize>,
}

impl Fonts {
    fn index(&mut self, font: &peniko::FontData) -> usize {
        let bytes = font.data.data();
        let digest = fnv(bytes);
        if let Some(i) = self.seen.get(&digest) {
            return *i;
        }
        let mut cmap = HashMap::new();
        if let Ok(f) = skrifa::FontRef::from_index(bytes, font.index) {
            use skrifa::MetadataProvider as _;
            for (cp, gid) in f.charmap().mappings() {
                if let Some(ch) = char::from_u32(cp) {
                    cmap.entry(gid.to_u32()).or_insert(ch);
                }
            }
        }
        self.digests.push(digest);
        self.cmaps.push(cmap);
        let i = self.digests.len() - 1;
        self.seen.insert(digest, i);
        i
    }
}

/// The full level: one line per command in paint order, layers as indentation, then the
/// fonts' lines.
fn full(commands: &[RenderCommand], form: char) -> String {
    let mut fonts = Fonts::default();
    let mut out = String::new();
    let mut depth = 0usize;
    for cmd in commands {
        let pad = "  ".repeat(depth);
        let line = match cmd {
            RenderCommand::PushLayer(l) => {
                depth += 1;
                format!(
                    "layer alpha={} t={} clip={}",
                    num(f64::from(l.alpha)),
                    affine(l.transform),
                    path(&l.clip)
                )
            }
            RenderCommand::PushClipLayer(c) => {
                depth += 1;
                format!("clip t={} clip={}", affine(c.transform), path(&c.clip))
            }
            RenderCommand::PushMaskLayer(m) => {
                depth += 1;
                format!("mask t={} clip={}", affine(m.transform), path(&m.clip))
            }
            RenderCommand::PopLayer => {
                depth = depth.saturating_sub(1);
                continue;
            }
            RenderCommand::Fill(f) => format!(
                "fill brush={} t={} shape={}",
                brush(&f.brush),
                affine(f.transform),
                path(&f.shape)
            ),
            RenderCommand::Stroke(s) => format!(
                "stroke w={} brush={} t={} shape={}",
                num(s.style.width),
                brush(&s.brush),
                affine(s.transform),
                path(&s.shape)
            ),
            RenderCommand::GlyphRun(g) => {
                let font = fonts.index(&g.font_data);
                let head = format!(
                    "glyphs font={font} size={} brush={} t={}",
                    num(f64::from(g.font_size)),
                    brush(&g.brush),
                    affine(g.transform)
                );
                let xs = g.glyphs.iter().map(|q| f64::from(q.x));
                let ys = g.glyphs.iter().map(|q| f64::from(q.y));
                let bbox = format!(
                    "{},{},{},{}",
                    num(xs.clone().fold(f64::INFINITY, f64::min)),
                    num(ys.clone().fold(f64::INFINITY, f64::min)),
                    num(xs.fold(f64::NEG_INFINITY, f64::max)),
                    num(ys.fold(f64::NEG_INFINITY, f64::max))
                );
                match form {
                    'a' => {
                        let ids: Vec<u8> =
                            g.glyphs.iter().flat_map(|q| q.id.to_le_bytes()).collect();
                        format!(
                            "{head} n={} at={bbox} ids={:016x}",
                            g.glyphs.len(),
                            fnv(&ids)
                        )
                    }
                    'b' => {
                        let list = g
                            .glyphs
                            .iter()
                            .map(|q| {
                                format!("{}@{},{}", q.id, num(f64::from(q.x)), num(f64::from(q.y)))
                            })
                            .collect::<Vec<_>>()
                            .join(" ");
                        format!("{head} glyphs={list}")
                    }
                    _ => {
                        let text: String = g
                            .glyphs
                            .iter()
                            .map(|q| fonts.cmaps[font].get(&q.id).copied().unwrap_or('\u{fffd}'))
                            .collect();
                        format!("{head} at={bbox} text={text:?}")
                    }
                }
            }
            RenderCommand::BoxShadow(_) => "shadow".into(),
        };
        out.push_str(&pad);
        out.push_str(&line);
        out.push('\n');
    }
    for (i, d) in fonts.digests.iter().enumerate() {
        writeln!(out, "font {i} digest={d:016x}").ok();
    }
    out
}

/// The summary level: counts per kind, the content box, the deepest layer and the full dump's
/// digest (its form A), as one JSON line.
fn summary(commands: &[RenderCommand], width: u32, height: u32) -> String {
    let kinds = [
        "layer", "clip", "mask", "pop", "fill", "stroke", "glyphs", "shadow",
    ];
    let mut counts = [0usize; 8];
    let (mut depth, mut deepest) = (0usize, 0usize);
    for cmd in commands {
        let k = match cmd {
            RenderCommand::PushLayer(_) => 0,
            RenderCommand::PushClipLayer(_) => 1,
            RenderCommand::PushMaskLayer(_) => 2,
            RenderCommand::PopLayer => 3,
            RenderCommand::Fill(_) => 4,
            RenderCommand::Stroke(_) => 5,
            RenderCommand::GlyphRun(_) => 6,
            RenderCommand::BoxShadow(_) => 7,
        };
        counts[k] += 1;
        if k < 3 {
            depth += 1;
            deepest = deepest.max(depth);
        } else if k == 3 {
            depth = depth.saturating_sub(1);
        }
    }
    let kinds = kinds
        .iter()
        .zip(counts)
        .map(|(k, n)| format!("\"{k}\":{n}"))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"kinds\":{{{kinds}}},\"width\":{width},\"height\":{height},\"depth\":{deepest},\"digest\":\"{:016x}\"}}",
        fnv(full(commands, 'a').as_bytes())
    )
}

/// `n` circles at whole-pixel centres inside a `w` × `h` box, diameters 10 to 79, from a fixed
/// sequence.
fn circles(n: usize, w: u32, h: u32) -> String {
    let mut s: u64 = 0x2545_f491_4f6c_dd1d;
    let mut next = |m: u64| {
        s = s
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (s >> 33) % m
    };
    (0..n)
        .map(|_| {
            let x = 20 + next(u64::from(w.saturating_sub(40)).max(1));
            let y = 20 + next(u64::from(h.saturating_sub(40)).max(1));
            let d = 10 + next(70);
            format!("{x},{y},{d}")
        })
        .collect::<Vec<_>>()
        .join(";")
}

/// Every reading: circles, form, layout mode, the dump's bytes and its JSON-string bytes.
fn readings() -> Vec<(usize, &'static str, bool, String)> {
    let mut out = Vec::new();
    for incremental in [false, true] {
        for &n in &COUNTS {
            let mut harness = stand::boot_circle_drawer(stand::options(incremental));
            let canvas = harness.query("#circle-canvas").expect("the canvas");
            let time = harness.time();
            let mut doc = harness.base_mut();
            doc.resolve(time);
            let layout = *doc.get_node(canvas).expect("node").final_layout();
            let w = (layout.size.width - layout.border.left - layout.border.right).max(0.0) as u32;
            let h = (layout.size.height - layout.border.top - layout.border.bottom).max(0.0) as u32;
            let text = circles(n, w, h);
            {
                let mut m = doc.mutate();
                m.set_attribute(
                    canvas,
                    QualName::new(None, Namespace::from(""), LocalName::from("data-circles")),
                    &text,
                );
            }
            doc.resolve(time);
            let scene = blitz_paint::custom_widget_scene(&mut doc, canvas, 1.0).expect("a scene");
            for form in FORMS {
                let dump = match form {
                    "full-a" => full(&scene.commands, 'a'),
                    "full-b" => full(&scene.commands, 'b'),
                    "full-c" => full(&scene.commands, 'c'),
                    _ => summary(&scene.commands, w, h),
                };
                out.push((n, form, incremental, dump));
            }
        }
    }
    out
}

/// Prints the table rows and each dump's bytes as a JSON string (run with --nocapture).
#[test]
#[ignore = "probe: prints"]
fn probe_print() {
    for (n, form, incremental, dump) in readings() {
        let json_bytes = dump.len()
            + 2
            + dump
                .chars()
                .filter(|c| matches!(c, '"' | '\\' | '\n'))
                .count();
        println!(
            "    ({n}, {form:?}, {incremental}, {}, 0x{:016x}), // json-string {json_bytes} B",
            dump.len(),
            fnv(dump.as_bytes())
        );
        if n == 1 && !incremental {
            println!("--- {form} n=1 ---\n{dump}---");
        }
    }
}

/// Each dump reads its committed bytes and digest on this OS.
#[test]
fn probe_dump_matches_its_table() {
    let rows = readings();
    assert_eq!(rows.len(), TABLE.len(), "the table holds every reading");
    let mut differ = Vec::new();
    for ((n, form, incremental, dump), row) in rows.iter().zip(TABLE) {
        assert_eq!(
            (*n, *form, *incremental),
            (row.0, row.1, row.2),
            "row order"
        );
        let got = (dump.len(), fnv(dump.as_bytes()));
        if got != (row.3, row.4) {
            differ.push(format!(
                "n={n} {form} incremental={incremental}: {} B 0x{:016x}",
                got.0, got.1
            ));
        }
    }
    assert!(differ.is_empty(), "PROBE-DIFFERS {}", differ.join(" | "));
}
