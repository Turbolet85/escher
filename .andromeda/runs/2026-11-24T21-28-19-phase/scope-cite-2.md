old: `[inferred]` The region before virtual nodes exist is the canvas's content box and any clip
new: Verified at P3 (circle_drawer.rs:150-154, the widget pushes one clip at its content box; canvas.rs:367-370 reads it): the region before virtual nodes exist is the canvas's content box and any clip
old: - `[inferred]` Other lint-like checks the commands already support without new reads (a command wholly outside the
new: - Verified at P3 (canvas.rs:355-462, every command's geometry is read): other lint-like checks the commands already support without new reads (a command wholly outside the
old: - `[inferred]` The driver's lint (`packages/escher-driver/src/lint.rs`, `Session::lint`, the `lint` verb's
new: - Verified at P3 (lint.rs:223, canvas.rs:270, schema.rs:431): the driver's lint (`packages/escher-driver/src/lint.rs`, `Session::lint`, the `lint` verb's
old: - `[inferred]` The glyph run's measured extent: the reading today holds the box of glyph ORIGINS only, not the
new: - Verified at P3 (canvas.rs:408-417; escher-driver names no skrifa, the workspace's skrifa 0.49 is one version in Cargo.lock): the glyph run's measured extent: the reading today holds the box of glyph ORIGINS only, not the
old: - `[inferred]` The cancellation is the push of `239c197c1` superseding the run (no failing subject), so it
new: - Verified at P3 against the run (CI#45488004344, every check `cancelled`, none failed; CI#45490290017 on `239c197c1` still `in progress`, its `Stand checks repeated` at 844 s, at P3): the cancellation is the push of `239c197c1` superseding the run (no failing subject), so it
