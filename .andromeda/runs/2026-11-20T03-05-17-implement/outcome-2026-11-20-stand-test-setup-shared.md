# Outcome — 2026-11-20-stand-test-setup-shared

**Outcome:** green — every non-operator gate green on its first firing (0 fix iterations); smoke skipped — no boot-path or UI-surface change (test code only, the plan lists no smoke or self-verify entry). Next → the operator pass, entries 7 (hygiene), 8 (push) and 9 (CI read), then `/andromeda-wrap-session`.

## Files changed (21, all on research's Files-to-modify list; 0 new)
- `tests/blitz-tests/tests/session_common/mod.rs` — steps 1, 4, 5, 6, 8: `tool_call`, `serve_lines`, `mcp_tool_result`; `lean`, `hold_served`; `Held` (`task`, `settled_step`), `assert_modes_agree`; `edges`, `centre`, `has_area`, `inside`, `viewport`; the `cfg(unix)` `record_check`
- `tests/blitz-tests/tests/common/mod.rs` — step 7: `ElementMark`, `element_marks`
- `stand_act_error_text.rs`, `stand_act_errors.rs`, `stand_order_served.rs` — steps 2, 8
- `stand_mcp_masked.rs` (gains `mod session_common;`), `stand_snapshot_parts.rs`, `stand_masks_on_new_answers.rs` — step 2
- `stand_mcp_tools.rs` — steps 2, 9
- `stand_session_state.rs` (gains `mod session_common;`) — step 3
- `stand_actions.rs`, `stand_layout.rs` — step 4
- `stand_act_diff.rs`, `stand_act_states.rs` — step 5
- `stand_act_obstructed.rs`, `stand_act_scroll.rs` — step 6
- `stand_actionable_keys.rs`, `stand_non_lean_tasks.rs` — step 7
- `stand_load_page.rs`, `stand_replay.rs`, `stand_act_transformed.rs` — step 9

## Gates (run 1 of `.andromeda/runs/2026-11-20T03-05-17-implement/gate-2026-11-20-stand-test-setup-shared.json`)
- `bash .github/scripts/ci-leg.sh fast` — green · exit 0 · 540.1 s
- `bash .github/scripts/ci-leg.sh stand-repeat` — green · exit 0 · `contains stand-repeat: 3 of 3 rounds green` held · 848.17 s
- `bash .github/scripts/ci-leg.sh a11y` — green · exit 0 · 44.76 s
- `python3 …/evidence/fold_guard.py 1b0125c10…` — green · `fold guard: PASS` (`files 197 · tests 930 · literals 3636`)
- `git diff --quiet 1b0125c10… -- Cargo.lock Cargo.toml .config .github scripts packages apps examples wpt tests/blitz-tests/Cargo.toml tests/blitz-tests/lib.rs tests/blitz-tests/checks` — green · exit 0
- the jscpd recipe + `stand_reading.py … 47 550` — green · `stand reading: 42 pairs / 358 lines · bar under 47 / 550 · stand reading bar: PASS`; whole `{"pct": 3.74, "duplicated_lines": 6209, "total_lines": 165838, "clones": 579, …, "test": {"pairs": 334, "lines": 3667}}`; report `evidence/jscpd-20261120T033744Z/` written 03:37:44 UTC in this run (fresh); the gate tool timed the entry at 0.31 s, and a scratch run of the same recipe on the same tree read the same 42 / 358
- entries 7, 8, 9 — `not run — leg operator` (the operator pass)

## Deviations, with justifications
- Step 2, `stand_mcp_tools.rs`: its `serve` keeps its name and signature, its body now one call of `session_common::serve_lines` over the stand's four apps (the plan kept the function because its boot differs; the shared exchange takes the boot as a parameter, so the body folds with no message changed).
- Step 9, `stand_mcp_tools.rs`' covered and no-font exchanges (base 531-538 / 560-567): folded by calling `session_common::serve_lines` inside the same `with_default(Recorder::default(), …)` rather than by a private helper; the reply count and messages are unchanged, the two `expect` messages now read from the shared exchange.
- Step 5, `stand_act_diff.rs`' `step` now calls `settled_step` and asserts `named` after it; the step index is read before the call, so its message reads the same index.
- Step 9, `stand_load_page.rs`: only the byte-equal tail (the diff adds the message, no time moved, the next screen reads it) folds into `assert_loaded`; the two lines whose messages name the click or the press stay in place.
- Step 9, `stand_replay.rs`: three private `cfg(unix)` helpers — `recorded_counter`, `assert_kept_and_holding`, `stopped_at_step_3` (the last returns the stopped step's keys; each caller asserts its own key with its own message).
- scope record: none — `gate.py scope` clean (`changed 21 · listed 21 · recorded 0`).

## Not obtainable here
- An off-unix compile of blitz-tests (`cargo check --tests --locked --target x86_64-pc-windows-gnu -p blitz-tests`, the testing rule after moving `cfg(unix)` items): exit 101 — `aws-lc-sys` needs `x86_64-w64-mingw32-gcc`, absent on this host. The CI windows leg (entry 9) is the witness.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / rustc / the test binaries (entries 1-3, the Windows check) | this run | terminated — `gate.py census --comm cargo --comm rustc --comm node --comm escher-session --comm seven_guis_native --args jscpd`: `census: none running` |
| node (jscpd, entry 6 and the scratch reading) | this run | terminated — same census |

## Decisions & corrections
- The operator's directive (`inputs#I3`): F1 unanswered — the upstream cdp-server and vibey-script tests kept, no upstream file touched; on green, the operator pass by hand in plan order in this window, recorded in `evidence/operator-pass.md` and landed; stop on any red.
- Guard refusals, each re-issued once and run: two `cd` into a directory outside the root (re-issued with absolute paths), one `cat` heredoc with a file target (re-issued as a Write-tool script).
- A control on the ci-scripts unittest entry: none fired — the plan's prose names no one-shot control for implement (its controls ran at P5, `p5-controls.md`).
