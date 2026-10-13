# Implement outcome — 2026-10-13-stand-contrast-harness

**Outcome: green — smoke skipped (no boot-path / UI-surface change).** The gates were green on the first firing:
14 green · 0 red · 3 not run (the operator legs 15, 16, 17). One iteration, no soft-exit, no deferral, nothing surfaced.

## Files changed
- `packages/blitz-test-harness/src/contrast.rs` (new): `contrast_ratio`, `TextContrast` (`is_large_text`,
  `required_ratio`), `Harness::text_contrast`, and 5 inline unit tests (the ratio table, the threshold table, and three
  `from_html` fixtures).
- `packages/blitz-test-harness/src/lib.rs`: `mod contrast;`, `pub use contrast::{TextContrast, contrast_ratio};`, and
  one crate-doc bullet.
- `packages/blitz-test-harness/Cargo.toml`: `color = { workspace = true }`.
- `Cargo.lock`: one line, `"color",` in blitz-test-harness's dependencies. No `[[package]]` (entry 5 read 0).
- `tests/blitz-tests/tests/stand_contrast.rs` (new): the stated 18-row `RECORD`, plus
  `every_stand_control_reads_its_recorded_pair` and `the_painted_background_is_the_measured_background`, both modes.
- `escher-0.1.0/chunks/2026-10-13-stand-contrast-harness/evidence/pairs.md`: the per-pair record, with declared values
  beside each row, the counts by verdict and the hypothesis reading.
- `escher-0.1.0/chunks/2026-10-13-stand-contrast-harness/evidence/counts.md`: run stand, workspace, `mod common;`, lock
  delta, audit.

## The measurement
- Verdicts: 9 pass · 5 fail · 3 exempt-inactive · 1 no-text. Every threshold is 4.5, since no text is large.
- The scope's hypothesis held. The five fails are `counter-increment`, `flight-one-way`, `flight-book`, `timer-reset`
  and `crud-create`, each `#ffffff` on `#4a6cf7` at 4.39.
- No measured colour differs from its declared value.
- The painted witness matched the measured background within 1 per channel on all 17 measured rows, in both modes.

## Counts
- `run stand`: 37 files · passed 141 · failed 0 · ignored 7 (standing 139 · 0 · 7 over 36).
- Workspace (`target/ci-logs/test.log`, this run's): 180 result lines · 900 passed · 0 failed · 12 ignored (standing
  179 · 893 · 0 · 12). That is +1 target and +7 tests, exactly the new ones.
- `mod common;` readers: 18 (17 → 18).
- Audit: `advisories ok`.

## Deviations from plan, with justifications
- **Step 4, the background-image check.** It reads `BaseDocument::resolved_style_value(node, "background-image")` and
  treats any layer other than `none` as an image, rather than matching stylo's computed `Image::None`. Naming that type
  needs a `style` dependency in blitz-test-harness, which the plan does not list. Opacity and filter are read from
  computed styles (`clone_opacity`, `get_effects().filter`), as planned. No scope-record line: the file is listed and
  the manifest is unchanged beyond `color`.
- **Step 7, the record's shape.** The expected results are tuple variants over one `Pair` struct
  (`Pass(pair(text, background, ratio, size, weight))`) rather than struct variants that repeat the five fields. The
  fields and their statement (8-bit hex, ratio at two decimals, whole px, weight) are as planned.
- **Step 8, the snapshot check.** "The snapshot lists every control id" is asserted per row inside `measure`, beside the
  `query` resolution, not as a separate pass first. Every message names the layout mode, the task and the row index only.
- **Step 9.** The reading was taken by a temporary `dump_the_reading` test whose panic message carried the rows. It was
  removed before the gates ran; entry 7's print census reads 0.

## Surfaced
None. No spec↔reality gap.

## Scope read
`scope: clean — changed 5 · listed 5 · recorded 0 · absorbed 0 · excluded 102`. No `scope-record.md` is needed.

## Verification matrix
`matrix.py show --chunk` reads 0 claimed by this chunk, so no ref was written, as scope.md expected.

## Next
The OPERATOR PASS, before `/andromeda-wrap-session`:
- entry 15: `gate.py hygiene`, before the pre-CI commit;
- entry 16: `ci-leg.sh fast`, then the clean-tree guard, then the push, after
  `chore(2026-10-13-stand-contrast-harness): operator pre-CI commit`;
- entry 17: `ci.py conclusion --sha HEAD --wait 1800`.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo test / rustc builds of gate entries 1–4 | this run (through the gate tool) | terminated |
| `scripts/agent-run.sh boot` and `run stand` (cargo, the stand test binaries) | this run (through the gate tool) | terminated |
| `ci-leg.sh audit`, `fast` and `doc` (cargo-deny, fmt, clippy, workspace tests, CI scripts, rustdoc) | this run (through the gate tool) | terminated |
| the P1 runs `cargo test -p blitz-test-harness`, `cargo test --test stand_contrast` (the dump and the first green) and `cargo fmt -p blitz-tests` | this run (by hand) | terminated |

Measured against `ps -eo pid,comm,args` at 2026-10-13T17:37Z: no `cargo`, `rustc`, `seven_guis`, `escher-session`,
`stand_*`, `blitz_tests` or `agent-run` process. The one hit was the census command's own shell.

## Decisions & corrections
- The gate header's log line read `$TMPDIR/andromeda-gate/…` while `TMPDIR` is unset in this session's shell. A read
  through the literal variable failed, and the logs were under the host's temp root (`andromeda-gate/…` directly under it). This cost one re-read and was
  logged to evolve.
- Standing learning (host-linux.md, 2026-10-13): pipeline tools were run bare or redirected to a file and read whole.
  No recurrence.
