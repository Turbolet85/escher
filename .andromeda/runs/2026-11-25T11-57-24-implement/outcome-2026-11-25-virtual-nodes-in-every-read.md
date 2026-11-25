# Outcome — 2026-11-25-virtual-nodes-in-every-read

**Outcome:** green · run `.andromeda/runs/2026-11-25T11-57-24-implement/` · smoke skipped — no boot-path or UI-surface touchpoint, no smoke or self-verify entry.

## Files changed
- New: `tests/blitz-tests/tests/virtual_nodes_in_every_read.rs`
- Modified: `packages/dioxus-native-dom/src/graph.rs` · `packages/dioxus-native-dom/src/actionable.rs` · `packages/escher-driver/src/execute.rs` · `packages/escher-driver/src/actions.rs` · `packages/escher-driver/src/layout.rs` · `packages/escher-driver/src/refusal.rs` · `packages/escher-driver/src/schema.rs` · `packages/escher-driver/src/coverage.rs` · `tests/blitz-tests/tests/fixtures/mod.rs` · `tests/blitz-tests/tests/virtual_nodes_as_elements.rs` · `tests/blitz-tests/tests/virtual_nodes_declared.rs` · `docs/capabilities.md` · `tests/blitz-tests/tests/stand_act_spans.rs` (scope record, companion)

## Gates (the final full firing, gate trail `.andromeda/runs/2026-11-25T11-57-24-implement/gate-2026-11-25-virtual-nodes-in-every-read.json`)
- `git diff --quiet b0e2d1c31… -- Cargo.lock Cargo.toml …` — green
- `bash .github/scripts/ci-leg.sh build` — green
- `cargo test -p dioxus-native-dom --locked --lib` — green
- `cargo test -p escher-driver --locked --lib` — green
- `cargo test -p blitz-tests --locked --test virtual_nodes_in_every_read` — green (7 cases)
- `cargo test -p blitz-tests --locked --test virtual_nodes_as_elements --test virtual_nodes_declared` — green
- `cargo test -p blitz-tests --locked --test stand_graph … --test stand_act_refused` — green (red on the first firing: `stand_act_spans` `each_driver_command_leaves_one_line_holding_nothing_it_handled`, «virtual fixture: call 45: the call returns the shape stated for it» — fixed, see deviations)
- `cargo test -p seven_guis --locked --test cli_commands --test mcp_server --test host_log --test mcp_log` — green
- `bash .github/scripts/ci-leg.sh a11y` — green
- `bash .github/scripts/ci-leg.sh fast` — green (red on the first firing at the same `stand_act_spans` case)
- `bash .github/scripts/ci-leg.sh doc` — green
- the three `leg = 'operator'` entries (hygiene, push, CI read) — not run, the operator pass's
- No deferral, no `defer` entry, no `gate.py guard` entry.

## Controls (`.andromeda/runs/2026-11-25T11-57-24-implement/control-1.toml`, evidence `.andromeda/cache/chunks/2026-11-25-virtual-nodes-in-every-read/evidence/control-1.txt`)
`control: 8 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed` — W1–W7 on the new target, L1 on `virtual_nodes_as_elements`, each named case read failed.

## Deviations (with justification)
- **a's measured bounds.** Plan step 9 put `a` at (47, 37, 60, 30) on the listened page. The fixture measures (55, 45) there and (15, 15) on the first page: the body's 8 px margin, which the plan's forecast left out. The test pins the measured literals (testing rule: pin the measured value). The `at` offsets (7, 7) and (47, 27) read as planned.
- **W6 and W7 red readings.** W6 reads red on the first page already, whose canvas also stands off the origin by the margin (predicted on the listened page). W7 reads red at the recording, whose virtual click is refused before any replay (predicted: the replay stops).
- **Step 3's table is keyed on the verb's `VerbSpec`** (`takes_virtual_id`, `resolve_for`), not on a `Command`, so the in-process `Session::layout`, which holds no command, reads the same table as `click` and `actions`.
- **A unit test beyond the plan:** `execute::tests::click_and_layout_alone_take_a_virtual_id` pins the table to the target verbs `click` and `layout`.
- **`actions` for a virtual node whose canvas does not resolve** reads `stale` on every verb, as plan step 4 says; the real `type` / `scroll` / `screenshot` / `canvas` call on such an id would read `not-found` (resolve's head arm comes first). Unreachable: the snapshot lists no virtual node without its canvas, so `actions` never lists one.
- **Scope record** — companion: `tests/blitz-tests/tests/stand_act_spans.rs` · serves step 3 · self. Its virtual-fixture block drove `layout` on `vn-canvas///a` expecting `Refused(NotFound)`, a pin of the lifted limit; restated to `Expect::Laid`, and its module doc's «a refused `layout`» to «a `layout`». The refused `layout` stays driven on the CRUD task (`layout(NOBODY)`), so the eight causes and the 46 calls hold. `gate.py scope`: `scope: clean — changed 14 · listed 13 · recorded 1 (companion 1 …)`.

## Measured, as the plan asked
- F19 A's check (M6's prediction): a replayed `click vn-canvas///a` on the listened page reaches the canvas's own `onclick`, recorded under `vn-canvas`; a check of `click vn-drop` lists `(vn-canvas, click)` unexercised; neither coverage line holds `///`.
- The fixture readings `b` disabled, `d` covered, `c` off-screen hold as forecast.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / rustc / test binaries of the gate entries and the controls | this run (the gate tool) | terminated — `gate.py census --comm cargo --comm rustc --comm escher-session --args virtual_nodes`: `census: none running` |

## Decisions & corrections
- Guard refusal: bash-guard refused a `cat` heredoc to a scratch script («a cat/tee heredoc with a file target»); re-issued through the Write tool and run by path.
- Guard refusal: bash-guard refused a `cd` into the skill's references directory («cd … moves the session cwd»); re-issued with an absolute path.
- Recurrence of a standing learning: `testing.md`'s 2026-10-10 rule (search `tests/` for checks that pin a changed behaviour before calling them unaffected) — `stand_act_spans` pinned the refused virtual `layout` and read red at the first full gate pass.
- none other.

## Next
The OPERATOR PASS: entries 12 (`gate.py hygiene`), 13 (the push) and 14 (`ci.py conclusion --sha HEAD --wait 2700`), then `/andromeda-wrap-session`.
