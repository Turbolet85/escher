//! A canvas's draw commands: a `canvas` answer's summary, its command lines and its full dump.

use std::fmt::Write as _;

use crate::canvas::{
    COMMAND_KINDS, CanvasBrush, CanvasCommand, CanvasReading, PathStep, Rgba, Transform,
};

use super::{Object, list, string};

/// A number rounded to 1/1000 and written as the shortest decimal that reads back the same,
/// `-0` as `0`; a value that is not finite as `null`.
fn number(out: &mut String, value: f64) {
    if !value.is_finite() {
        out.push_str("null");
        return;
    }
    let rounded = (value * 1000.0).round() / 1000.0;
    // Past about 1e305 the scaling overflows; such a value has no fraction to round.
    let rounded = if rounded.is_finite() { rounded } else { value };
    if rounded == 0.0 {
        out.push('0');
    } else {
        let _ = write!(out, "{rounded}");
    }
}

/// Numbers joined by `,`.
fn numbers(out: &mut String, values: &[f64]) {
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        number(out, *value);
    }
}

fn transform(out: &mut String, transform: &Transform) {
    if transform.is_identity() {
        out.push_str("id");
    } else {
        numbers(out, &transform.0);
    }
}

/// A path as SVG letters with their points.
fn path(out: &mut String, steps: &[PathStep]) {
    for step in steps {
        match *step {
            PathStep::Move(x, y) => {
                out.push('M');
                numbers(out, &[x, y]);
            }
            PathStep::Line(x, y) => {
                out.push('L');
                numbers(out, &[x, y]);
            }
            PathStep::Quad(x1, y1, x, y) => {
                out.push('Q');
                numbers(out, &[x1, y1, x, y]);
            }
            PathStep::Cubic(x1, y1, x2, y2, x, y) => {
                out.push('C');
                numbers(out, &[x1, y1, x2, y2, x, y]);
            }
            PathStep::Close => out.push('Z'),
        }
    }
}

fn colour(out: &mut String, Rgba([r, g, b, a]): Rgba) {
    let _ = write!(out, "#{r:02x}{g:02x}{b:02x}{a:02x}");
}

fn brush(out: &mut String, brush: &CanvasBrush) {
    match brush {
        CanvasBrush::Solid(rgba) => colour(out, *rgba),
        CanvasBrush::Gradient { kind, stops } => {
            out.push_str(kind);
            out.push(':');
            for (index, stop) in stops.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                number(out, stop.offset);
                colour(out, stop.colour);
            }
        }
        CanvasBrush::Image(index) => {
            let _ = write!(out, "image:{index}");
        }
        CanvasBrush::Resource => out.push_str("resource"),
        CanvasBrush::Pattern => out.push_str("pattern"),
    }
}

/// One `key=value` pair, written after a space.
fn pair(out: &mut String, key: &str, value: impl FnOnce(&mut String)) {
    out.push(' ');
    out.push_str(key);
    out.push('=');
    value(out);
}

/// One command's line, its fixed word then its `key=value` pairs, with no indent and no end.
fn command_line(out: &mut String, command: &CanvasCommand) {
    out.push_str(COMMAND_KINDS[command.kind()]);
    match command {
        CanvasCommand::Layer {
            alpha,
            mix,
            compose,
            fill,
            transform: t,
            clip,
            filter,
            backdrop,
        } => {
            pair(out, "alpha", |out| number(out, *alpha));
            pair(out, "blend", |out| {
                out.push_str(mix);
                out.push(',');
                out.push_str(compose);
            });
            pair(out, "fill", |out| out.push_str(fill));
            pair(out, "t", |out| transform(out, t));
            pair(out, "clip", |out| path(out, clip));
            if *filter {
                out.push_str(" filter");
            }
            if *backdrop {
                out.push_str(" backdrop");
            }
        }
        CanvasCommand::Clip { transform: t, clip } => {
            pair(out, "t", |out| transform(out, t));
            pair(out, "clip", |out| path(out, clip));
        }
        CanvasCommand::Mask {
            mode,
            transform: t,
            clip,
        } => {
            pair(out, "mode", |out| out.push_str(mode));
            pair(out, "t", |out| transform(out, t));
            pair(out, "clip", |out| path(out, clip));
        }
        CanvasCommand::Pop => {}
        CanvasCommand::Fill {
            fill,
            brush: b,
            brush_transform,
            transform: t,
            shape,
        } => {
            pair(out, "fill", |out| out.push_str(fill));
            pair(out, "brush", |out| brush(out, b));
            if let Some(bt) = brush_transform {
                pair(out, "bt", |out| transform(out, bt));
            }
            pair(out, "t", |out| transform(out, t));
            pair(out, "shape", |out| path(out, shape));
        }
        CanvasCommand::Stroke {
            width,
            join,
            start_cap,
            end_cap,
            miter,
            dashes,
            dash_offset,
            brush: b,
            brush_transform,
            transform: t,
            shape,
        } => {
            pair(out, "w", |out| number(out, *width));
            pair(out, "join", |out| out.push_str(join));
            pair(out, "cap", |out| {
                out.push_str(start_cap);
                if start_cap != end_cap {
                    out.push(',');
                    out.push_str(end_cap);
                }
            });
            pair(out, "miter", |out| number(out, *miter));
            if !dashes.is_empty() {
                pair(out, "dash", |out| {
                    numbers(out, dashes);
                    out.push('@');
                    number(out, *dash_offset);
                });
            }
            pair(out, "brush", |out| brush(out, b));
            if let Some(bt) = brush_transform {
                pair(out, "bt", |out| transform(out, bt));
            }
            pair(out, "t", |out| transform(out, t));
            pair(out, "shape", |out| path(out, shape));
        }
        CanvasCommand::Glyphs {
            font,
            size,
            brush: b,
            alpha,
            transform: t,
            count,
            origins,
            ids,
        } => {
            pair(out, "font", |out| {
                let _ = write!(out, "{font}");
            });
            pair(out, "size", |out| number(out, *size));
            pair(out, "brush", |out| brush(out, b));
            if *alpha != 1.0 {
                pair(out, "alpha", |out| number(out, *alpha));
            }
            pair(out, "t", |out| transform(out, t));
            pair(out, "n", |out| {
                let _ = write!(out, "{count}");
            });
            pair(out, "at", |out| match origins {
                Some(origins) => numbers(out, origins),
                None => out.push_str("none"),
            });
            pair(out, "ids", |out| {
                let _ = write!(out, "{ids:016x}");
            });
        }
        CanvasCommand::Shadow {
            transform: t,
            rect,
            radii,
            offset,
            spread,
            deviation,
            colour: c,
            kind,
        } => {
            pair(out, "t", |out| transform(out, t));
            pair(out, "box", |out| numbers(out, rect));
            pair(out, "radii", |out| numbers(out, radii));
            pair(out, "offset", |out| numbers(out, &[offset.0, offset.1]));
            pair(out, "spread", |out| number(out, *spread));
            pair(out, "dev", |out| number(out, *deviation));
            pair(out, "colour", |out| colour(out, *c));
            pair(out, "kind", |out| out.push_str(kind));
        }
    }
}

/// Each command's line, in order, indented two spaces per layer open before it — `depth` open
/// where the first stands, a `pop` at the indent of the layer it closes — closed by its node's
/// tag, ` node={k}` for the 1-based index `nodes` holds at its place or ` node=-` for none — a
/// key no command's own pairs write — and ended by `\n`.
pub(crate) fn canvas_lines(
    commands: &[CanvasCommand],
    nodes: &[Option<usize>],
    depth: usize,
) -> Vec<String> {
    let mut depth = depth;
    commands
        .iter()
        .enumerate()
        .map(|(index, command)| {
            if matches!(command, CanvasCommand::Pop) {
                depth = depth.saturating_sub(1);
            }
            let mut line = "  ".repeat(depth);
            command_line(&mut line, command);
            match nodes.get(index).copied().flatten() {
                Some(node) => line.push_str(&format!(" node={node}")),
                None => line.push_str(" node=-"),
            }
            line.push('\n');
            if command.opens() {
                depth += 1;
            }
            line
        })
        .collect()
}

/// The commands' lines as one text: a full dump's `text`.
pub(crate) fn canvas_text(
    commands: &[CanvasCommand],
    nodes: &[Option<usize>],
    depth: usize,
) -> String {
    canvas_lines(commands, nodes, depth).concat()
}

/// The written length of each command's line inside a JSON string, its tag included, in order,
/// from depth 0.
pub(crate) fn canvas_line_lengths(
    commands: &[CanvasCommand],
    nodes: &[Option<usize>],
) -> Vec<usize> {
    canvas_lines(commands, nodes, 0)
        .iter()
        .map(|line| {
            let mut written = String::new();
            string(&mut written, line);
            written.len() - 2
        })
        .collect()
}

/// A list of digests, each as 16 lowercase hexadecimal digits.
fn digests(values: &[u64]) -> String {
    list(values.iter().map(|value| format!("\"{value:016x}\"")))
}

/// The written length of a reading's `fonts` and `images` lists, less their brackets.
pub(crate) fn canvas_resource_lengths(reading: &CanvasReading) -> usize {
    digests(&reading.fonts).len() - 2 + digests(&reading.images).len() - 2
}

/// What `canvas` answers without `full` or `part`.
pub(crate) fn canvas_summary(
    reading_canvas: bool,
    (width, height): (f64, f64),
    kinds: &[usize; 8],
    depth: usize,
    digest: u64,
) -> String {
    let mut object = Object::new();
    object.flag("canvas", reading_canvas);
    let mut written = String::new();
    number(&mut written, width);
    object.json("width", &written);
    written.clear();
    number(&mut written, height);
    object.json("height", &written);
    let mut counts = Object::new();
    for (word, count) in COMMAND_KINDS.iter().zip(kinds) {
        counts.count(word, u64::try_from(*count).unwrap_or(u64::MAX));
    }
    object.json("kinds", &counts.end());
    object.count("depth", u64::try_from(depth).unwrap_or(u64::MAX));
    object.text("digest", &format!("{digest:016x}"));
    object.end()
}

/// What `canvas` answers with `full`: `canvas`, the lines from `depth`, `fonts` and `images`,
/// and for a part `part` and `parts`.
pub(crate) fn canvas_full(
    reading: &CanvasReading,
    depth: usize,
    part: Option<(u32, u32)>,
) -> String {
    let mut object = Object::new();
    object.flag("canvas", reading.canvas);
    object.text(
        "text",
        &canvas_text(&reading.commands, &reading.nodes, depth),
    );
    object.json("fonts", &digests(&reading.fonts));
    object.json("images", &digests(&reading.images));
    if let Some((part, parts)) = part {
        object.count("part", u64::from(part));
        object.count("parts", u64::from(parts));
    }
    object.end()
}
