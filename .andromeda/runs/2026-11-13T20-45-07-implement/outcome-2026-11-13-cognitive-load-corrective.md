# Implement outcome — 2026-11-13-cognitive-load-corrective

**Outcome: green + smoke.** Next → the OPERATOR PASS: entries 9 (hygiene), 10 (push) and 11 (CI read), which this
skill never fires; then `/andromeda-wrap-session`.

## Files changed (scope: clean — changed 4 · listed 4 · recorded 0)
- `tests/blitz-tests/tests/click_default_actions.rs` (new): the six click default actions no check reached, in both
  layout modes. It covers a link leaving the document, a fragment link, a bare `<a>` walking on to its label, an
  unmatched click clearing focus, a disabled submit button stopping the walk, a label bound to a `multiple` select,
  and a double click with and without 500 ms between the clicks.
- `packages/blitz-dom/src/events/pointer.rs`: `handle_click` (97 → 2) is now a walk (`click_walk` 10), a dispatch
  (`click_element` 3) and one private function per arm: `click_checkbox` 2 · `click_radio` 7 · `click_summary` 3 ·
  `click_select` 2 · `click_option` 2 · `click_label` 3 · `click_link` 8 · `click_file_input` 5 (under `file-input`).
  The optgroup arm calls the existing `click_lands_on_row` directly, and the button arm calls the existing
  `click_button`.
- `packages/escher-driver/src/execute.rs`: `Session::execute` (32 → 1) has one call per arm: `snapshot` 2 · `click` 0
  · `type_into` 0 · `type_step` 7 (the acting closure's order as a named function) · `press_key` 0 · `advance` 2 ·
  `scroll` 0 · `screenshot` 5 · `graph` 1. Lint and actions stay one-line arms, and the span stays in `run`.
- `packages/dioxus-native-dom/src/graph.rs`: `DioxusDocument::graph` (50 → 4) now calls `state_readings` 2 ·
  `ran` 1 · `node_handlers` 8 · `handler_write` 4 · `node_reads` 6 · `graph_states` 0.

## Gates (trail `.andromeda/runs/2026-11-13T20-45-07-implement/gate-2026-11-13-cognitive-load-corrective.json`)
`entries 11 · green 8 · red 0 · recorded 0 · timeout 0 · not-run 3` on the first full firing, with no fix iteration.
- 1 complexity: green, last line 3 (touch_action_pan_axes 17, handle_pointermove 77, handle_pointerdown 39). The
  before reading was 6 (`{work_dir}/evidence/before.md`), and the after reading and every function's own reading
  are in `{work_dir}/evidence/after.md`.
- 2 census: green — pointer.rs 13/13, execute.rs 3/3, graph.rs 3/3 equal.
- 3 mutator.rs/document.rs: green (exit 0).
- 4 click_default_actions: green, 6 passed.
- 5 click-arm checks (workspace build): green — 15 suites, 114 passed (baseline 114).
- 6 escher-driver: green — 104 passed (baseline 104).
- 7 dioxus-native-dom --lib: green — 109 passed (baseline 109).
- 8 `ci-leg.sh fast`: green — 234 suites, 1406 passed, 0 failed, 54 `stand_` targets among them.
- 9, 10, 11: `not run — leg operator`. They belong to the operator pass.
- No deferral. No `gate.py guard` entry is in the plan.

## Controls (step 2, before any arm moved) — `{work_dir}/evidence/control-1.txt`
`control: 6 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`. Each case read red under its own
control, and the untouched tree read green. The label re-dispatch control also turns the bare-link case red, because
that click reaches its checkbox through the same re-dispatch.

## Smoke — `{work_dir}/evidence/smoke.txt`
The plan has no smoke entry. The `escher-session` binary was built and driven by hand on a Counter session:
`start`, `snapshot`, `graph`, a refused `type` (`not-found`, exit 1), then `click counter-increment`, which counted
to 1 and focused the button, then `stop`. Every answer was as expected.

## Deviations from the plan, with justifications
1. **Case (c)'s assertion.** The plan says an unmatched click "clears focus". `BaseDocument::get_focussed_node_id`
   reads the root element when no node holds focus, so the check asserts that `focused()` is the `html` element. The
   `//!` says so. This is the engine's existing reading, not a change. The control that removes the `clear_focus`
   call turns the case red.
2. **Order of steps 3–5.** `execute.rs` and `graph.rs` (steps 4 and 5) were written while the step-2 controls ran in
   their clone, after the clone had been made. `pointer.rs` was edited only after all six controls had read.
3. **Control (b)'s form.** The plan names removing "the walk-on after the match". The control replaced
   `maybe_node_id = doc.nodes[node_id].parent;` with `maybe_node_id = None;`, because removing the line outright
   would loop forever.
4. **Re-reads in the arm functions.** As step 3 asks, `click_radio`, `click_link` and `click_file_input` read their
   element again with a `let … else` whose `else` returns false (or returns). That branch is never taken: the
   dispatch has just read the same element. `click_select` reads `is_single_select` through `is_some_and`. No new
   `unwrap`, `expect` or log site was added (the census reads equal).
5. **The smoke.** The plan lists no `role = 'smoke'` entry, so the smoke was a hand-driven CLI flow (above).

## Surfaced / stuck
None. No capability is claimed (the coverage query reads 0 rows), so the matrix has no ref to write.

## Process hygiene
`gate.py census --comm escher-session --comm seven_guis_native --args smoke-clc` → `census: none running`.

| process | started by | final state |
|---|---|---|
| escher-session host (smoke session 1, pid 264605) | this run | terminated (`stop` → `{"stopped":true}`) |
| escher-session host (smoke session 2) | this run | terminated (`stop` → `{"stopped":true}`) |
| cargo test / clone builds under the control | a leg (gate.py control) | terminated (clone removed) |

## Decisions & corrections
- A guard refusal: the Bash guard refused a `cd` into the references directory and a log directory, and a `cat`
  heredoc with a file target. Each was re-issued once, through Read, an absolute path or the Write tool.
- A tool refusal: `gate.py control` refused the id `e-label-redispatch` (over 16 characters) before anything was
  made. It was renamed `e-label` and re-fired.
- An engine fact met: `get_focussed_node_id` falls back to the root element, so "focus cleared" reads as the root
  element being focused. Measured by instrumenting the check temporarily; the instrumentation was removed by
  restoring a byte copy.
