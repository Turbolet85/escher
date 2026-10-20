# Outcome — 2026-10-19-visual-lint · green + smoke (smoke skipped: no boot-path / UI-surface change)

Mode: pending (normal). Step 12, the gated criterion, was not run: the founder's own word on F1 is not recorded.

## Gates
One firing of the block: `entries 22 · green 15 · red 0 · recorded 0 · timeout 0 · not-run 7` (trail
`.andromeda/runs/2026-10-20T00-14-31-implement/gate-2026-10-19-visual-lint.json`). The 7 not run are the operator
entries 16–19 and the `gated:` defers 20–22, none voided. No gate deferred under the source-delta rule (Rust delta 6).
Every forecast read exactly: `run stand` 183 · 0 · 7 over 45 files; `run text_extent` 3 · 0 · 0; a11y 43 over 6;
workspace 201 lines · 1055 passed · 0 failed · 12 ignored; `fast` and `doc` green
(`escher-0.2.0/chunks/2026-10-19-visual-lint/evidence/counts.md`). The stand at boot reads 0 findings on all four
lean tasks in both modes (`evidence/readings.md`). No `watch:` subject in the block.

Matrix: v020-05 is a gated cap, so no ref was written and its status stays `planned`.

## Files changed
- New: `packages/blitz-dom/src/text_extent.rs`, `packages/escher-driver/src/lint.rs`,
  `tests/blitz-tests/tests/text_extent.rs`, `tests/blitz-tests/tests/stand_lint.rs`
- Modified: `packages/blitz-dom/src/lib.rs`, `packages/escher-driver/src/lib.rs`
- Evidence: `escher-0.2.0/chunks/2026-10-19-visual-lint/evidence/{readings,counts}.md`
- Scope record: none needed, no edit outside research's lists (`gate.py scope`: `clean — changed 6 · listed 6 ·
  recorded 0`).

## Deviations from the plan, with justifications
1. **`Finding.bounds` is a lint-owned `Bounds`** (x · y · width · height in CSS px, the same four numbers), not
   blitz-dom's `BoundingRect`. escher-driver depends on neither blitz-dom nor style, so it cannot name the type.
   Adding the dependency moves `Cargo.toml` and `Cargo.lock`, which gate entry 7 (no tracked file outside the six)
   forbids. `Reason`'s rectangles are `Bounds` too.
2. **Computed `contain`, `overflow-x`/`-y` and `display` are read through the public
   `BaseDocument::resolved_style_value`** as keyword text matched against closed sets (`paint`/`strict`/`content`;
   `visible`). The reason is the same as item 1: stylo's `Contain`/`Overflow` cannot be named.
   `visible_region` is unchanged (constraint held).
3. **Overlap is read over the visible parts** (each box cut to its clip region), not over whole boxes. A CRUD row
   scrolled out of `crud-list` would otherwise overlap whatever lies under the list on screen. This reads the same
   as the plan's letter on every page tested (no clipped planted or stand box lies over another), but it is a
   definition choice the operator did not rule on, so it is listed for review.
4. `lib.rs` re-exports `Bounds` and `Reason` beside `Defect` and `Finding` (public field types), and carries one
   crate-doc paragraph on `Session::lint`.
5. `Reason::Truncated.visible` is `Option<Bounds>`: `None` when no part of the text box can be seen.
6. The planted root `fx-root` is 784 × 584 inside the body's default 8 px margin, not 800 × 600. An 800 × 600 root
   inside the margin would pass the viewport and read `clipped`. Out-of-flow placement would leave `body` with no
   height, a `zero-size` holding text.
7. `TextExtent.content` also subtracts a scrollbar's width and height (the CSS content box). There is none on the
   test pages.
8. `text_extent.rs`'s pages are HTML booted with `Harness::from_html_with(_, stand::options(incremental))`, the
   stand's options. The stale id is made by `remove_and_drop_node`.
9. The planted-page check has one extra precondition assert: `text_contrast` measures row 6. The CRUD check asserts
   exactly one `clipped`/`off-screen` finding on the last row.

## Surfaced
No spec↔reality gap. One observation for the wrap: under the plan's letter, a node that is clipped or off-screen and
holds text can also read `truncated` (its text is cut from view as well). The CRUD check does not assert this case,
and it is not measured.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / rustc / escher-session / seven_guis_native / agent-run.sh | this run (gate entries, direct cargo runs) | terminated — `gate.py census`: none running (trail `census-2026-10-19-visual-lint.json`) |

## Next
The OPERATOR PASS, entries 16 (hygiene), 17 (fast + clean-tree guard + push, after the pre-CI commit
`chore(2026-10-19-visual-lint): operator pre-CI commit, for the run this chunk's verdict reads`), 18 (CI conclusion)
and 19 (per-OS tallies into `evidence/ci.md`). Then `/andromeda-wrap-session`. Gated entries 20–22 wait on the
founder's F1 word (step 12).

## Decisions & corrections
- Guard refusal: the Bash guard refused a `cd` into the chunk's `inputs/` dir ("moves the session cwd"). It was
  re-issued once with absolute paths and ran.
- Diagnostics: a temporary assertion message printed the planted findings and the last CRUD row's reading, two
  runs. Each time the test file was restored from a scratch copy, and the second restore was checked equal by sha256.
