# Outcome — 2026-11-24-canvas-draw-commands

**Outcome:** green — 11 of 11 non-operator gate entries green on the first firing (`gate-2026-11-24-canvas-draw-commands.json`), 4 of 4 controls hold (`evidence/control-1.txt`). Smoke: skipped — no boot-path or UI-surface change; seven_guis' four process checks drove both binary roles as a gate.

**Next:** the OPERATOR PASS — the plan's `operator` entries 12 (`gate.py hygiene`), 13 (the push of `build/escher-0.4.0`) and 14 (`ci.py conclusion --sha HEAD --wait 2400`), then `/andromeda-wrap-session`. The pass writes `{work_dir}/evidence/operator-pass.md`.

## Files changed
- New (6): `packages/escher-driver/src/canvas.rs`, `packages/escher-driver/src/canvas_tests.rs`, `packages/escher-driver/src/json/canvas.rs`, `tests/blitz-tests/tests/stand_canvas_commands.rs`, `tests/blitz-tests/tests/stand_canvas_carriers.rs`, and the scope record.
- Modified (38): `packages/blitz-paint/src/lib.rs`, `packages/blitz-test-harness/src/screenshot.rs`, `packages/escher-driver/Cargo.toml`, `Cargo.lock`; escher-driver's `schema.rs`, `schema/spec.rs`, `schema_tests.rs`, `command.rs`, `command_tests.rs`, `execute.rs`, `json.rs`, `json_tests.rs`, `explore.rs`, `explore_tests.rs`, `actions.rs`, `describe.rs`, `mcp.rs`, `mcp_tests.rs`, `cli_tests.rs`, `recording_tests.rs`, `lib.rs`; blitz-tests' `fixtures/mod.rs`, `session_common/mod.rs`, `stand_actions.rs`, `stand_cross_os_readings.rs`, `stand_act_spans.rs`, `stand_act_scroll.rs`, `stand_mcp_tools.rs`, `stand_masks_on_new_answers.rs`, `stand_requirement_sweep.rs`; seven_guis' `cli_commands.rs`, `mcp_server.rs`, `host_log.rs`, `mcp_log.rs`, `common/mod.rs`; `scripts/cold-agent.sh`, `.github/scripts/test_cold_agent.py`, `docs/capabilities.md`, `README.md`.

## Deviations, with their justifications
- `Harness::canvas_scene` takes a `NodeId`, not the plan's `usize`: the harness's node type is the versioned `NodeId` (CLAUDE.md, «NodeId is versioned»), and `custom_widget_scene` takes it as `process_custom_widget_node` does.
- The canvas writer is a child module, `json/canvas.rs`, as `json/graph.rs` and `json/layout.rs` are: scope record `in-intent`.
- The line form's open choices, settled from neighbouring code and stated in the field help: `blend=` is `{mix},{compose}`; `cap=` is one word where both caps agree, else `{start},{end}`; `dash=` is the lengths then `@` and the offset; a glyph run of no glyph writes `at=none`; a `Custom` paint, which a recording keeps as transparent solid, reads as the solid `#00000000`.
- Step 10's fixture condition is read from the canvas's `data-circles` attribute, not «the snapshot's circle count»: Circle Drawer's circles leave the DOM and no snapshot node counts them.
- Step 17's «canvas-pixels-alone sentence» is not in `docs/capabilities.md` (it stands in the masters, the wrap's); nothing there was re-worded. The statement's actions-answer limit moves 118,292 → 130,496 bytes, the measured Cells size.
- The verb-count sweep missed two literal pins, `assert_eq!(VERBS.len(), 15)` in `tests/blitz-tests/tests/stand_act_scroll.rs:488` (on no list: scope record `companion`) and `examples/seven_guis/tests/cli_commands.rs:161`, both moved to 16 when the gates read them red.
- `tests/blitz-tests/tests/stand_mcp_tools.rs` is named by the plan (step 14) but not by research's lists: scope record `in-intent`.
- Control `glyph-ids-written` edits two files: the reading holds the ids' digest alone by construction, so `json.rs` has no glyph id to write; the control keeps the first glyph id in `canvas.rs` and writes it in decimal in `json/canvas.rs`. Control `budget-doubled` loosens `canvas_parts`' cut alone: cargo runs `stand_canvas_carriers` first and the entry has no `--no-fail-fast`, so a doubled whole-or-paged decision would redden the carriers' paged fixture and leave the named case unrun.
- The measured sizes run above the probe's: full text 527 · 5,915 · 49,266 bytes at 1 · 12 · 100 circles against the probe's 555 · 5,426 · 44,993 (+9.0 % and +9.5 % at 12 and 100: the `pop` lines and the fields the probe did not write); 100 circles read in 6 parts. Cells' actions answer grows by 12,204 bytes over 678 nodes, 18.0 per node against the plan's predicted 17.
- Beyond the plan: an `execute.rs` unit test `a_canvas_dump_is_cut_into_parts_of_whole_lines_within_the_budget`, beside graph's.

Scope record (`gate.py scope`: `scope: clean — changed 44 · listed 41 · recorded 3 (companion 1 · mechanical 0 · in-intent 2 · widening 0) · absorbed 0 · excluded 188`):
- companion: `tests/blitz-tests/tests/stand_act_scroll.rs` · serves `packages/escher-driver/src/schema.rs` · self
- in-intent: `packages/escher-driver/src/json/canvas.rs` · serves `packages/escher-driver/src/json.rs` · self
- in-intent: `tests/blitz-tests/tests/stand_mcp_tools.rs` · serves step 14 · self

`gate.py guard` entries: none in the block.

## Surfaced / stuck
none

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo, rustc and the test binaries of entries 1–11 | this run (gate.py run) | terminated — `gate.py census --comm escher-session --comm cargo --comm rustc --args stand_canvas_carriers --args cli_commands --args canvas_host`: `census: none running` |
| the socket hosts `canvas_host_full` · `canvas_host_incremental` · seven_guis `canvas_host` | a leg (entries 2, 4, 11) | terminated — the same census |
| the control clone `~/.cache/andromeda-control/875780d4` and its firings | this run (gate.py control) | terminated — `clone removed` |

## Decisions & corrections
- Guard refusal: bash-guard refused `cd` into the skill's references dir (Setup) — re-issued as Read; refused a `cat >>` heredoc append to `json_tests.rs` — re-issued through the Edit tool, the form it names.
- Guard refusal: wake-guard refused a ScheduleWakeup while the gate block ran — the block's notification was the wake.
- Sweep hazard: a verb-count sweep on the words «fifteen | eleven» misses a literal count pin (`assert_eq!(VERBS.len(), 15)`), two of them here; the literal is found only by a red run unless `VERBS.len()` is swept too.
- Control hazard: on an entry that names two `--test` targets with no `--no-fail-fast`, cargo runs them in name order and stops at the first red one, so a control's case in the later target reads «not run» unless the edit leaves the earlier target green.
