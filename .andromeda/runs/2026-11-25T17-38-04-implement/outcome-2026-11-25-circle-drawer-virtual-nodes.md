# Outcome — 2026-11-25-circle-drawer-virtual-nodes

**Outcome:** green — every non-operator gate entry green on the final tree (12 of 12), controls C1–C6 hold (7 of 7,
C5 on two entries), smoke skipped (no boot-path touchpoint, no smoke or self-verify entry, no headful self-verify).
**Run:** 2026-11-25T17-38-04-implement
**Next:** the OPERATOR PASS — entries 13 (`gate.py hygiene`), 14 (the clean-tree push), 15 (`ci.py conclusion --sha
HEAD --wait 2700`) and 16 (the CI windows job-log read for `unused import: `snapshot``, `--id` the run id entry 15
prints) — then `/andromeda-wrap-session`. The pass writes `{work_dir}/evidence/operator-pass.md` before it is cleared.

## Files changed
- New: `tests/blitz-tests/tests/stand_circle_nodes.rs`
- Modified (listed): `packages/blitz-dom/src/lib.rs`, `examples/seven_guis/src/tasks/circle_drawer.rs`,
  `packages/escher-driver/src/{canvas,json/canvas,json_tests,canvas_tests,coverage}.rs`,
  `tests/blitz-tests/tests/virtual_nodes_in_every_read.rs`, `tests/blitz-tests/tests/{session_common,common}/mod.rs`,
  `tests/blitz-tests/tests/stand_circle_drawer_canvas.rs`, `tests/blitz-tests/tests/stand_canvas_commands.rs`,
  `docs/capabilities.md`
- Modified (scope record): `packages/escher-driver/src/execute.rs` (companion, step 4),
  `packages/escher-driver/src/canvas_lint.rs` (in-intent, step 4), `tests/blitz-tests/tests/stand_canvas_carriers.rs`
  (companion, step 4) — `gate.py scope`: `scope: clean — changed 17 · listed 14 · recorded 3 (companion 2 · mechanical 0
  · in-intent 1 · widening 0) · absorbed 0 · excluded 190`

## Deviations, with justifications
1. **The content box is 696 × 396, not 700 × 400.** `.cd-canvas`'s `width: 700px; height: 400px` size its border box
   (2 px border), measured by the canvas summary. Per the plan's own implementation note, step 3's constants follow the
   measured box (`CANVAS_WIDTH` 696, `CANVAS_HEIGHT` 396) and the CSS stays; case (a) pins 696 × 396.
2. **Seven circles spill at 100, not three.** The fixture sequence is drawn on the measured 696 × 396 box; the spill
   table `CIRCLE_SPILLS` holds the seven measured (ids 19, 23, 24, 26, 46, 48, 85, each with its cut region).
3. **A glyph run's grouping bounds are its ink box.** Step 4 named «the box of its glyph origins». Those origins sit on
   the baseline, below and left of the label's centre; on the 100-circle fixture that rule grouped 95 of 100 labels
   under their own circle (the five misses each a label's run, read in a probe). The run's ink box — the canvas lint's
   `ink`, its glyphs' outline bounds, made `pub(crate)` (scope record, in-intent) — reads 100 of 100; the origins box
   stays the fallback where no glyph has an outline. The rule (nearest region centre, ties to the first) is unchanged.
4. **The exploration budget is 80, not 40.** Measured: the first path whose call clicks a `circle-canvas///c{id}` id is
   path 37, past the back button's branch, reached from a budget of 80 (60 reached none).
5. **The positional witness (step 7 (j)) is written through the attribute.** Every driver click lands at the canvas's
   centre, so c1 and c3 share one region and the tie reads c3 at index 1. The case pins that tie reading on the
   in-flow undo-and-click, then writes `348,198,40,1;100,100,40,2` and `348,198,40,1;100,100,40,3` to `data-circles`:
   c3 standing where c2 stood reads index 2 and its full text is byte-equal to c2's.
6. **`stand_canvas_carriers` restated (companion).** The plan read it as unmoved; its glyph-line key pin read red: the
   line tag ` n={k}` repeats the glyph line's existing `n=` key (its glyph count), so a glyph line holds two `n` keys,
   the count first and the tag last. The pin now reads `[font, size, brush, t, n, at, ids, n]` and the last pair `n=-`.
7. **C4's measured red is 76 of 100**, against the plan's off-tree 69 (made with every command's centre at its circle's).
8. Two reds of the first full firing, fixed: clippy `unnecessary_lazy_evaluations` on `circle_region`'s `bool::then`
   (now `then_some`) and rustdoc `private_intra_doc_links` on `Session::canvas`'s doc linking the private `group`.

## Gates (final firing)
1 lock probe green · 2 `ci-leg.sh build` green · 3 escher-driver lib green · 4 seven_guis lib green · 5
dioxus-native-dom lib green · 6 `stand_circle_nodes` green · 7 the three virtual-node targets green · 8 the seventeen
companions green · 9 `ci-leg.sh a11y` green · 10 `ci-leg.sh stand-repeat` green (1067.43 s) · 11 `ci-leg.sh fast`
green · 12 `ci-leg.sh doc` green · 13–16 `not run — leg operator`.
`entries 16 · green 12 · red 0 · recorded 0 · timeout 0 · not-run 4` (trail
`.andromeda/runs/2026-11-25T17-38-04-implement/gate-2026-11-25-circle-drawer-virtual-nodes.json`).
First firing: 11 `red · exit 0 ✗ (exit 101)` (clippy) and 12 `red · exit 0 ✗ (exit 101)` (rustdoc), both fixed and
re-run green before the final firing.
Controls: `control: 7 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`
(`{work_dir}/evidence/control-1.txt`). No `gate.py guard` entry in the block.
No deferral: every language touched ran its gates.

## Surfaced / stuck
none

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / rustc (gate entries, probes, the control clone's builds) | this run | terminated |
| the gate tool's control clone `~/.cache/andromeda-control/ef35c86e` | this run (`gate.py control`) | removed |
| a blocking `cat > /dev/null` shell | this run | terminated (TaskStop) |
`gate.py census --comm cargo --comm rustc --comm escher-session --comm seven_guis_nati --args stand_circle_nodes`:
`census: none running`.

## Decisions & corrections
- The plan's 700 × 400 content box followed the measured box, by the plan's own implementation note.
- Guard refusals, each re-issued once: a `cd` into the skill's references dir (the Bash guard), and a `cat` heredoc
  with a file target (the Bash guard) — both re-issued through Read / Write.
- The operator's word (inputs#I6, in this session): hygiene's one P1 row — the CI windows runner's drive path quoted at `.andromeda/runs/2026-11-25T16-18-13-phase/control-probes.md:32` — «Elide the path (Recommended)»: the parenthetical rewritten to `(…\tests\blitz-tests)`, the `Compiling blitz-tests v` fact and its counts kept; this run's `relay-1.md` elides it the same way, the verbatim text kept in the snapped copy; hygiene then read `clean`.
- Sweep hazard: the line tag ` n=` collides with the glyph line's `n=` glyph count; a reader keying a line's pairs into
  a map loses one.
