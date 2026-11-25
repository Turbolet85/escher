# Outcome — 2026-11-25-virtual-nodes-as-elements

**Outcome:** green — the operator's V2 ruling applied (inputs#I5). Smoke: skipped (no boot path or UI surface changed).

## Files changed
- Listed (research): `packages/blitz-dom/src/node/custom_widget.rs`, `packages/dioxus-native-dom/src/{virtual_node,dioxus_document,snapshot,graph,element_id}.rs`,
  `packages/escher-driver/src/{execute,lint,schema,refusal}.rs`, `tests/blitz-tests/tests/virtual_nodes_declared.rs`,
  `.github/scripts/{ci-leg.sh,test_ci_workflows.py}`, `docs/capabilities.md`; new `tests/blitz-tests/tests/virtual_nodes_as_elements.rs`.
- Recorded (scope record rev 3, four lines; `gate.py scope`: clean — changed 19 · listed 15 · recorded 4):
  - `tests/blitz-tests/tests/fixtures/mod.rs` · in-intent · serves step 11 · self
  - `tests/blitz-tests/tests/stand_act_spans.rs` · in-intent · serves step 13 · self
  - `packages/escher-driver/src/schema_tests.rs` · companion · serves `packages/escher-driver/src/schema.rs` · self
  - `.github/scripts/test_capability_statement.py` · companion · serves `.github/scripts/ci-leg.sh` · self

## Deviations, with justifications
1. **Step 8's lint skip dropped** — the operator's word (inputs#I5): «A guard no input can turn red is not kept. Drop
   the inert skip; report V2 as having no guard to control and the research premise as disproved.» Control V2 read
   `DOES NOT HOLD`: a virtual node's lint entry has no element, so no clip region and no visible part; `overlaps`
   skips an entry with no visible part and `readings` one with no element. The lint holds no virtual node by
   construction, pinned by `virtual_nodes_as_elements::the_graph_actions_and_lint_hold_no_virtual_node` (the lint
   answer on the declaring fixture holds no `///`, though `e` overlaps `a`). The `///` doc's limit «a canvas's
   virtual nodes are not linted» stays, true as read.
2. **`VirtualPlace` holds the bounds alone** — plan step 3 gave it the canvas's `NodeId` too; nothing reads it, so
   it warned dead under the clippy leg's `-D warnings` and was dropped.
3. **`Cause::Covered`'s meaning is also pinned as a literal** in `refusal.rs`'s unit test (the plan's note said
   `not-found` alone); both literals moved in the same file.
4. **Two pins outside the plan's sweep moved as companions**: `schema_tests.rs` pins `click`'s help (entry 4 read red
   at the first pass, `row 8`), and `test_capability_statement.py` pinned the a11y leg's file count `19` (entry 10's
   ci-scripts step read `20 != 19`).
5. **The hidden-canvas case** toggles a class `vn-hidden` set by a stylesheet in the fixture page (`style { … }`),
   a button `vn-hide` doing it, as the plan's «class toggle».
6. **`tree_id` answers `Option`**: `None` past 2³¹ − 1 nodes a canvas, read by checked arithmetic.

## Gates (final firing on the finished tree, trail `gate-2026-11-25-virtual-nodes-as-elements.json`)
entries 14 · green 11 · red 0 · recorded 0 · timeout 0 · not-run 3 — entries 12 to 14 are `leg = 'operator'`,
the operator pass's (hygiene, push, CI read). Earlier firings: the first full run read red on entry 4
(`schema::tests::every_verb_states_its_help_its_arguments_and_a_result`, `row 8`) and entry 10 (the same test);
the `--only 4,10` re-run read 4 green and 10 red (`test_the_leg_counts_are_stated_in_words`, `20 != 19`); the
second full run read 11 green; the third, after the V2 ruling, 11 green.
The a11y leg read 20 · 199 · 0 · 0 (result lines · passed · failed · ignored), the line `docs/capabilities.md` states.

## Controls (`.andromeda/cache/chunks/2026-11-25-virtual-nodes-as-elements/evidence/control-1.txt`)
control: 7 holds · 0 timeout · 1 does not hold · 0 unreproduced · clone removed — V1, V3–V8 hold; V2's guard,
the inert lint skip, is dropped on the operator's word, so V2 has no guard to control.

## Surfaced / stuck
none. Disproved: research's Mechanism equality «the lint meets a virtual node only through `overlaps` … two
overlapping virtual regions … would read `overlap` naming a `///` id» — `overlaps` pairs visible parts only.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / rustc / test binaries of every gate entry and control firing | the gate tool (this run) | terminated — `gate.py census --comm cargo --comm rustc --comm escher-session --comm virtual_nodes_as --comm stand_act_spans`: `census: none running` |
| the manual a11y leg and the two manual `cargo test` runs | this run | terminated (same census) |

## Decisions & corrections
- The operator's quiet window: «let the gate re-run in flight end, then start NO cargo … wait for my go» — held; the
  controls fired after «Go: the host is yours».
- The operator's V2 ruling (inputs#I5): a guard no input can turn red is not kept; the fact is pinned by a reading.
- Guard refusal: the bash guard refused a `cd` into the cargo registry (it moves the session cwd); re-issued with an
  absolute path.
- Sweep hazard: a pin of a changed help or meaning text may hold only a prefix (`json_tests.rs` reads the `covered`
  text by `contains`, unmoved) or the whole text (`schema_tests.rs`, `refusal.rs`'s unit test) — grep each edited
  text's first clause, not its new words.
