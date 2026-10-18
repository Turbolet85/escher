# Re-count — 2026-10-18-upstream-sync-determinism-and-traces

Measured by /andromeda-implement, run `.andromeda/runs/2026-10-18T06-16-11-implement/`, 2026-10-18, on the work tree
after the take. All entry lines are from the trail `gate-2026-10-18-upstream-sync-determinism-and-traces.json`.
The first full firing ran 30 entries: 25 green · 0 red · 1 recorded · 4 not run (`leg operator`).

## Workspace tally (`bash .github/scripts/ci-leg.sh fast`, entry 11 green, `target/ci-logs/test.log` fresh)
- Entry 12: `lines 192 passed 991 failed 0 ignored 12`.
- Against step 1 (`evidence/before.md`, 192 · 990 · 0 · 12): **+1 passed**. Line count, failed and ignored are unmoved.
- Attribution, per target: every `Running …` result line of the engine-debts wrap's fast leg (its gate log, the same
  run as `before.md`'s `test.log`) was paired with this run's, entry 11's log. 192 lines before and 192 after; exactly
  **one** line moved: blitz-dom's lib (`Running unittests src/lib.rs (target/debug/deps/blitz_dom-…)`), `97 passed` →
  `98 passed`. The +1 is upstream's `document::hover_state_tests::hovering_below_content_reports_default_cursor`, which
  appears once in `test.log` as `... ok`. That is the prediction, and no other line moved.
- Entry 9 (`cargo test -p blitz-dom --locked --lib hover_state_tests`): `running 3 tests`. The two standing tests and
  `hovering_below_content_reports_default_cursor ... ok` read `3 passed; 0 failed; 0 ignored; 0 measured; 95 filtered out`.
- Entry 10 (`cargo test -p blitz-tests --locked --test pointer_leave`): green, `Running tests/pointer_leave.rs`, no `FAILED`.

## blitz-tests by name
- `ls tests/blitz-tests/tests/*.rs | wc -l`: **132 files**.
- Entry 13: `0` absent. Every file but `all.rs` ran as its own target in the workspace leg, **131 targets**.
- Entry 21: `ls tests/blitz-tests/tests/stand_*.rs | wc -l` reads **41**.

## CI scripts (entry 14, recorded)
```
Ran 100 tests in 8.581s
OK
```
Unmoved against `before.md` (`Ran 100 tests`, `OK`).

## a11y leg (entries 17, 18)
- Entry 18: `lines 6 passed 43 failed 0`. Per target, from `target/ci-logs/a11y.log`: `accessibility_hidden` 6 ·
  `accessibility_roles` 6 · `focusability_updates` 4 · `stand_accessibility_ids` 8 · `stand_contrast` 5 ·
  `stand_keyboard` 14, all with 0 failed.
- Entry 24: no platform accessibility adapter in the stand graph (`0`, exit 1).

## Agent-run contract (entries 19, 20, 22)
- Entry 19 `boot`: green.
- Entry 20 `run stand`:
  `{"event": "run.end", "ts": "2026-10-18T06:23:46Z", "selection": "stand", "passed": 170, "failed": 0, "ignored": 7, "cargo_exit": 0, "outcome": "passed"}`.
  This equals the baseline (170 · 0 · 7). The stand's geometry, Tab order, contrast rows and screenshots are asserted
  inside those checks, both layout modes, and none moved.
- Entry 22 `run all`:
  `{"event": "run.end", "ts": "2026-10-18T06:24:51Z", "selection": "all", "passed": 562, "failed": 0, "ignored": 12, "cargo_exit": 0, "outcome": "passed"}`.
  This equals the baseline (562 · 0 · 12). blitz-tests holds no blitz-dom unit test, so the +1 does not reach it.

## Doc, build, smoke
- Entry 15 `ci-leg.sh doc`: green.
- Entry 25 `cargo build -p seven_guis --bin seven_guis_native --locked`: green.
- Entry 26 (windowed smoke, `WAYLAND_DISPLAY` set): green, the window up to the 10 s bound. Its log:
  `2026-10-18T06:25:01.228571Z INFO escher_telemetry service.name=seven_guis service.version=0.1.0 message=telemetry installed`.
  The smoke cannot see a cursor's visibility. The windowed effect of `fb8d6c15e` is witnessed by upstream's unit test
  alone, a recorded limit and not a claim.
