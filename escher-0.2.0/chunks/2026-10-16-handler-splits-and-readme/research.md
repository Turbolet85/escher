# Codebase Research — 2026-10-16-handler-splits-and-readme

## Scope
- **Depth:** moderate · **Reads:** 14 · **Globs/Greps:** 9
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 1 Session Addition (the gate tool's `timeout` reading of exit 124; no boot smoke in this chunk, so it binds nothing) · `.claude/rules/testing.md` — read in full, 8 Session Additions (2026-10-06 platform-bound keys: read the `#[cfg]` lines above a `match` arm — `events/keyboard.rs` carries none, `events/pointer.rs`'s `handle_click` carries `#[cfg(feature = "file-input")]` and `#[cfg(feature = "tracing")]`, both on upstream arms this chunk leaves; 2026-10-09 the hand mutation control's save · restore · sha256 rule, which binds no step here since no mutation runs, inputs#I1)
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg
- **External inputs:** `inputs#I1` — the operator's directive: the README part is the founder's word, pipeline tools bare or read whole, no mutation run without the founder's new word, forks as dialogs, every deferral a residuals line naming his label · `inputs#I2` — the founder's README word (2026-10-16) and the overseer's reading: 0.1.0 done and verified, 0.2.0 planned, «Escher» capitalized in prose, crate, binary and path names unchanged

## Files inspected
- `packages/blitz-dom/src/events/pointer.rs` (370–905) — `handle_pointerdown` (383–579): escher's `ClickTarget::RangeInput { fraction }` variant (452–456), its classification arm computing the content-box fraction (463–474) and its match arm calling `apply_range_move` on the main button (517–525); `handle_click` (640–882): escher's range-focus block (670–681) and the button arm escher rewrote from upstream's submit arm (800–822), every other arm upstream's (checkbox 684–705, radio 706–734, summary 737–761, label 763–773, `a` 774–799 with its `#[cfg(feature = "tracing")]` sites at 791–792 and 796–797, `#[cfg(feature = "file-input")]` file arm 823–857)
- `packages/blitz-dom/src/events/keyboard.rs` (full) — `handle_key_or_input_event` (20–126): escher's range-key arm (103–117, its `_ => return` the function's last reachable statement) and Enter arm (118–124) after upstream's text-input arm (82–102); `handle_key_up` (129–148), `activates_on_enter` (152–165), `activates_on_space` (169–181), `dispatch_activation_click` (185–193), `apply_range_move` (232–251) — the escher helpers already named
- `packages/blitz-dom/src/node/range.rs` (215–305) — `parse_float` (234–294): sign (241–250) · integer digits (252–255) · fraction (257–262) · the "a number opens with a digit" refusal (264–266) · exponent (273–287) · finite filter and `-0` to `0` (289–293); its unit table `the_float_parser_reads_the_html_grammar` at 348
- `README.md` (full, 74 lines) — line 1 `# escher`; line 9 "The screen snapshot, the agent driver and its command-line and MCP tools are being built (see Plans)"; `## Plans` (50–61) "Version 0.1.0 is built in six stages" with six epoch lines and a "Beyond 0.1.0" line; prose naming the project at 1, 7, 9, 17, 59, 65, 69, 73; code spans `escher-session` at 38–48
- `.andromeda/runs/2026-10-15T10-44-59-code-audit/proposals.md` (1–36, 71–100) — M1's figures and proposal; the mutation tier's six units: 1,558 mutants, 1,285 caught, 0 missed, 6 timeouts
- `.andromeda/runs/2026-10-15T10-44-59-code-audit/record.json` (`tool_versions`, `commands`) — the complexity instrument: `rust-code-analysis-cli -m -O json`, every space of kind function, `cognitive.sum`, version 0.0.25
- `escher-0.1.0/chunks/2026-10-15-cold-agent-test/report.md` (250–299) — the cold-agent baseline: 43 tool calls, 0 wrong, four lean tasks, 0.3197 USD
- `escher-0.2.0/intent.md` (1–90), `escher-0.2.0/working-route.md` (9–53), `escher-0.2.0/requirements.md` (1–40) — the 0.2.0 plan the README states: five epochs, eleven requirements, §Out of this version
- `escher-0.3.0-incubator/README.md` (1–30) — excluded from git locally; the README names no path in it

## Graph impact (from the code-graph query; trace `.andromeda/runs/2026-10-16T14-07-11-phase/tree-query-2026-10-16-handler-splits-and-readme.json`)
- **handle_click** — 3 call sites, all blitz-dom: `events/mod.rs:14` (import), `events/mod.rs:215` (`handle_dom_event`), `events/pointer.rs:770` (its own label arm's recursion) — signature kept, no caller moves
- **handle_pointerdown** — `events/mod.rs:14` (import), `events/mod.rs:192` — signature kept
- **handle_key_or_input_event** — `events/mod.rs:12` (import), `events/mod.rs:218`, `events/mod.rs:232` — signature kept
- **parse_float** — 6 calls, all in `node/range.rs`: `RangeRules::new` at 91, 95, 106, 114, `RangeRules::sanitize_attribute` at 132, the unit test at 351 — signature `fn(&str) -> Option<f64>` kept
- **apply_range_move** — `events/keyboard.rs:117`, `events/pointer.rs:519` — the one range write path; stays the one
- **dispatch_activation_click** — `events/keyboard.rs:123`, `events/keyboard.rs:146` — the one activation path; stays the one
- No symbol crosses a crate: every caller is blitz-dom's, every touched item `pub(crate)` or private. (Editor lines: the trace's 0-indexed `line` + 1.)

## Patterns detected
- **The complexity instrument reproduces the audit at HEAD** (scratch measure, `rust-code-analysis-cli -m -O json -p` on the three files, function spaces, `cognitive.sum`): `handle_key_or_input_event` 56 · `handle_click` 74 · `handle_pointerdown` 40 · `parse_float` 19 — the audit's four figures exactly. Closures are their own `<anonymous>` spaces (`events/pointer.rs:686` 4, `:710` 5, `events/keyboard.rs:32` 5), so a `generate_focus_events` closure never counts toward its parent.
- **Upstream's share of each handler** (the same instrument on `git show 434e0661:` — the merge base the arch's Upstream sync line names): `handle_click` 69 · `handle_pointerdown` 35 · `handle_key_or_input_event` 50. escher's arms add 5 · 5 · 6. So no split of escher's arms alone takes a parent under the ceiling of 15; each parent's floor without reshaping upstream's arms is its upstream figure.
- **escher's arms, by the merge-base diff** (`git diff 434e0661 HEAD -- packages/blitz-dom/src/events/{pointer,keyboard}.rs`, 262 lines): pointer.rs — the `RangeInput` variant, its classification arm, its match arm, the range-focus block in `handle_click`, the button arm's guard and body; keyboard.rs — the range-key arm, the Enter arm, `handle_key_up`, `activates_on_enter`, `activates_on_space`, `dispatch_activation_click`, `apply_range_move`, one `changed_nodes.insert` in `apply_generated_text_input_event`. Every other line of the three handlers is upstream's at `434e0661`.
- **The already-named precedent** (`events/keyboard.rs:152–193`): the activation gates already live in named private functions with a `///` WHATWG citation (`activates_on_enter`, `activates_on_space`, `dispatch_activation_click`) — the shape the new extractions follow.
- **Borrow shape in `handle_click`** (`events/pointer.rs:651–663`, 809–821): `el` is a `&mut ElementData` borrowed from `doc.nodes`; the button arm reads `submits` from `el` before `generate_focus_events(doc, …)`, so a body extracted as a function of `(doc, node_id, submits, dispatch_event)` keeps the borrow order the arm has now.

## Conventions to follow
- **Crate-internal visibility**: new functions private (`fn`) beside their callers, as `activates_on_enter` is (`events/keyboard.rs:152`); no `pub`, no re-export (arch §Standard Contracts → Nodes and widgets).
- **Docs move with code**: the WHATWG range-state citation (`events/keyboard.rs:108`) and the float-parsing rules link (`node/range.rs:233`) travel with the code they annotate; `///` on each new function (arch §Conventions → Documentation; rustdoc `-D warnings` over the workspace).
- **Unit tests inline**: any step test of `parse_float` joins `node/range.rs`'s tail `#[cfg(test)] mod tests` (testing.md §Framework).
- **Event-producing functions take the parent's `dispatch_event`** (`&mut dyn FnMut(DomEvent)`, as `dispatch_activation_click` takes it, `events/keyboard.rs:189`).

## New files to create
- `escher-0.2.0/chunks/2026-10-16-handler-splits-and-readme/evidence/` — the complexity readings before and after (the instrument's command and its per-function lines), the gate logs' verdicts, the README's before/after facts

## Files to modify
- `packages/blitz-dom/src/events/pointer.rs` — escher's range classification and range arm out of `handle_pointerdown`, escher's range-focus block and button-arm body out of `handle_click`, as named private functions; upstream's arms untouched
- `packages/blitz-dom/src/events/keyboard.rs` — escher's range-key arm out of `handle_key_or_input_event` as a named private function; the Enter arm's gate stays `activates_on_enter`
- `packages/blitz-dom/src/node/range.rs` — `parse_float` split into its steps as private functions; its unit table untouched
- `README.md` — the current state: 0.1.0 done and verified, 0.2.0 planned; «Escher» capitalized in prose, code names lowercase

## Sweeps
- `\b(parse_float|handle_click|handle_pointerdown|handle_key_or_input_event)\b` over `packages apps examples tests wpt` (`*.rs`): 17 hits in 4 files · 3 files changed (`events/pointer.rs`, `events/keyboard.rs`, `node/range.rs`) · 1 no-change (`events/mod.rs` — imports and calls of signatures that stay)
- `readme` over `.github/ scripts/ Cargo.toml justfile` (case-insensitive) and `readme =` over every member manifest: 1 hit, `Cargo.toml:21` `"apps/readme"` (the markdown viewer crate) · 0 changed — no build, CI or manifest step reads `README.md`

## Open questions
- none — the parents' floor (each stays over the ceiling at its upstream figure) is decided by the audit's own proposal ("leave the upstream-merged entrants to upstream") and arch's upstream-sync line; P4 states it as a lean.
