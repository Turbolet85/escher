# Outcome — 2026-11-25-virtual-nodes-declared · /andromeda-implement run 2026-11-25T02-42-47-implement

**Outcome: green** (0 fix iterations: every non-operator gate green on its first firing) · smoke skipped — no boot-path / UI-surface change, no self-verify entry.

## Files changed (7: 5 modified · 2 new)
- `packages/blitz-dom/src/node/custom_widget.rs` — `VirtualNodeDeclaration` (key · `accesskit::Role` · name · content-box `BoundingRect`) and the defaulted `Widget::virtual_nodes(&self)` returning an empty `Vec`, placed after `paint`; 28 added lines, 0 removed.
- `packages/blitz-dom/src/node/mod.rs` — `#[cfg(feature = "custom-widget")] pub use custom_widget::VirtualNodeDeclaration;` (2 added lines).
- `packages/blitz-dom/src/lib.rs` — `#[cfg(feature = "custom-widget")] pub use crate::node::VirtualNodeDeclaration;` (2 added lines).
- `packages/dioxus-native-dom/src/virtual_node.rs` (new) — `VirtualNode`, `VirtualNodeRefusal`, `RefusedDeclaration`, `VirtualNodes`, `VirtualNodesUnread`, `DioxusDocument::virtual_nodes`, the crate-private `validate`, and nine unit tests.
- `packages/dioxus-native-dom/src/lib.rs` — `mod virtual_node;` and the five re-exports.
- `packages/dioxus-native-dom/src/element_id.rs` — the reserved form's module doc and `virtual_node_id`'s `///`, re-worded: produced in process by `DioxusDocument::virtual_nodes`, written by no walk, served by no carrier. Body unchanged.
- `tests/blitz-tests/tests/virtual_nodes_declared.rs` (new) — three cases, both layout modes.

## Gates (trail `.andromeda/runs/2026-11-25T02-42-47-implement/gate-2026-11-25-virtual-nodes-declared.json`)
```
  1 probe       green · exit 0 · 0.0s · 0 B → 1.log
  2 probe       green · exit 0 · 0.01s · 2 B → 2.log
  3 probe       green · exit 0 · 0.0s · 0 B → 3.log
  4 build       green · exit 0 · 12.79s · 2125 B → 4.log · bash .github/scripts/ci-leg.sh build
  5 unit        green · exit 0 · 0.77s · 11133 B → 5.log · cargo test -p dioxus-native-dom --locked --lib
  6 integration green · exit 0 · 2.12s · 673 B → 6.log · cargo test -p blitz-tests --locked --test virtual_nodes_declared
  7 integration green · exit 0 · 9.4s · 3354 B → 7.log · (the seven companions)
  8 lint        green · exit 0 · 653.61s · 182582 B → 8.log · bash .github/scripts/ci-leg.sh fast
  9 build       green · exit 0 · 80.45s · 3711 B → 9.log · bash .github/scripts/ci-leg.sh doc
 10–12          not run — leg operator (the operator pass's)
entries 12 · green 9 · red 0 · recorded 0 · timeout 0 · not-run 3
```
No `gate.py guard` entry in the block. No deferral: Rust changed, every Rust gate ran.

## Controls (step 7) — `control: 14 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`
K1–K14 each read `holds`, every named case `failed` in its removed log; none widened. Lines verbatim in
`.andromeda/cache/chunks/2026-11-25-virtual-nodes-declared/evidence/control-1.txt`; the file fired is
`.andromeda/runs/2026-11-25T02-42-47-implement/control-1.toml`.

## Ledger
No capability row is owned by this chunk (`coverage where chunk = …` → 0 rows): v040-11 stays pooled, as the plan
states. No `matrix.py implement` call.

## Deviations from plan
- **Re-export placement.** The plan put each blitz-dom re-export pair *after* the existing line (`lib.rs` after
  `pub use crate::node::Widget;`, `node/mod.rs` beside the list). rustfmt (the PostToolUse hook) sorted each pair
  *before* the existing `pub use` it sits beside. Still added lines only — probe 2 reads `last line 0` removed.
- **Unit-test role.** Plan step 5 names no role for the unit fixtures. They use `Role::default()` (accesskit's
  `#[default] Unknown`) through `Default::default()`, so the test module names no `accesskit` item: dioxus-native-dom's
  `accesskit` dependency is optional under `accessibility`. Validation reads no role.
- **Unit `an_unlaid_canvas_reads_not_laid_out` fixture.** One component holds `div#plain` and, behind a global
  `SHOW` signal, `object#canvas`; the stale case drops the canvas by re-rendering through that signal, as
  `element_id`'s `a_removed_element_reads_no_id` does. The premise (arch contract/nodes-widgets/try-layout-data,
  «None before the first layout») held: the case reads `Err(NotLaidOut)` after `initial_build()` alone.
- **Integration fixture.** `vn-plain` is styled `PADDED` like `vn-canvas` so it is laid out as a block; the plan
  named no style for it. The check builds `snapshot` and `graph` calls as `escher_driver::Call` in-file (it declares
  `mod fixtures;` only, per step 6) and reads each answer through `Outcome::to_json`.
- Scope record: none — no file outside research's lists was edited (`scope: clean — changed 7 · listed 7 · recorded 0`).

## Surfaced / stuck
None.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / rustc (the gate block's and the controls' builds) | this run (through `gate.py run` / `gate.py control`) | terminated — `census: none running` (comm cargo, rustc, escher-session) |

## Decisions & corrections
- Guard refusal: the Bash guard refused `cd` into the skill's references directory (`Blocked: cd … moves the session
  cwd`); re-issued once as Read calls.
- Two scratch build logs this run wrote by hand before the gates (`target/tmp-vn-unit.log`, `target/tmp-vn-int.log`)
  sit under the gitignored `target/`.

## Next
The OPERATOR PASS — entries 10 (hygiene), 11 (push) and 12 (CI read), which this skill never fires — then
`/andromeda-wrap-session`.
