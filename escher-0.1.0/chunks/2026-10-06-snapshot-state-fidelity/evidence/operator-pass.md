# Operator pass — entries 17 · 18 · 19

Driven by the agent on the operator's direction (2026-10-06), each by hand, in plan order.

## Entry 17 — hygiene
- run: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`
- exit 0 · atom `contains hygiene: clean` ✓
- First read (2026-10-06T22:16Z): `hygiene: clean — read 41 (runs 36 · evidence 2 · inputs 3) · trails 14 not read · copies 1 not read by P1 — 0 host paths kept · binary 0 not read by P1`.
- It was re-fired after the lines above were written, over the evidence set the pre-CI commit carries. That run read `hygiene: clean — read 42 (runs 36 · evidence 3 · inputs 3) · trails 14 not read · copies 1 not read by P1 — 0 host paths kept · binary 0 not read by P1` at exit 0.

## Pre-CI commit
- `70b9997d` `chore(2026-10-06-snapshot-state-fidelity): operator pre-CI commit, for the run this chunk's verdict reads` (`git add -A`, the whole tree: 55 files).

## Entry 18 — fast · clean-tree guard · push
- run: `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.1.0`
- exit 0 (2026-10-06T22:18Z) · fast green (workspace 504 · 0 · 5 over 129 result lines from `target/ci-logs/test.log`; `Ran 64 tests`) · tree clean · push `52616c18..70b9997d  build/escher-0.1.0 -> build/escher-0.1.0` · after it, `HEAD` = `origin/build/escher-0.1.0` = `70b9997d4318`.

## Entry 19 — CI conclusion: RED
- run: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800`
- exit 0 · atom `contains verdict: green` ✗ — the entry is red (read 2026-10-06T22:24Z)
- `70b9997d4318 verdict: red · checks 16/16 · first-fail +349 s Test (macos) · runs CI#37539756517 in_progress/-` (repo Turbolet85/escher, polled 13× over 372 s); `failed 1: Test (macos) (failure)`.
- The failed job (112530567010), from its log: `stand_snapshot_state` read `test result: FAILED. 6 passed; 1 failed`; the one failure is `a_disabled_control_reads_disabled_until_the_app_enables_it`, at `stand_snapshot_state.rs:161`, message `incremental=false: a valid start date enables Book again`, left `Some(false)`, right `Some(true)`.
- Basis, read from the source: that assertion follows `flight.press(Key::Backspace)`, and the editor's `Key::Backspace` arm is compiled only off macOS (`packages/blitz-dom/src/node/text.rs:330-331`, `#[cfg(not(target_os = "macos"))]` — on macOS the delete arrives as an Apple standard key binding, which the harness does not synthesize). So on the macOS runner the key press deletes nothing and Book stays disabled. The step is the one /implement added to the plan's sequence; the host (Linux) cannot reproduce the red.
- At the last read (22:25Z) the run was still open: `Test (windows)` and `Coverage report` in progress, the other 13 jobs success.
- The pass stopped here on the operator's instruction. Everything from the pre-CI commit section to this line was written after the push, so it rides the fix commit.

## Fix (the operator's direction, 2026-10-06: option A — clicks and typing only)
- `tests/blitz-tests/tests/stand_snapshot_state.rs`, `a_disabled_control_reads_disabled_until_the_app_enables_it`: the `Key::Backspace` step is gone. In return mode one character typed into `flight-return-date` makes `flight-book` read `Some(false)`; a click on `flight-one-way` makes it read `Some(true)` again and `flight-return-date` `Some(false)` again; the plan's own step (a character typed into `flight-start` disables Book) follows unchanged. No key but characters, Tab and Shift+Tab is pressed anywhere in the file, and `Key::Backspace` is the only platform-gated arm on the typing path (`grep cfg(target_os` over `node/text.rs` and `events/keyboard.rs` → that one line).
- Local reading before the fix commit: the plan's block through the gate tool, `entries 19 · green 16 · red 0 · recorded 0 · timeout 0 · not-run 3`; `stand_snapshot_state` 7 passed; `run stand` 48 ok.
- The first run's final state, read after it closed: CI#37539756517 `completed · failure`, `Test (macos)` its only non-success job (Windows and coverage passed).

## For the wrap's route pass — a fifth engine defect that needs an owner
The plan's Implementation notes list four (the password painted in the clear, the bare `disabled` control that stays focusable, the plain-button click that clears focus, the missing `select` and range interaction model). The operator added a fifth (2026-10-06):
- **On macOS the editor has no Backspace arm.** `packages/blitz-dom/src/node/text.rs:330-331` compiles `Key::Backspace` only off macOS and leaves the delete to the Apple standard key bindings, so a headless key press deletes nothing there (measured: the macOS job above). The driver will need typed deletion to work headless on every platform.
