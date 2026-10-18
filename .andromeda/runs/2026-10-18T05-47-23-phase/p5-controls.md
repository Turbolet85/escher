# P5 controls — 2026-10-18-upstream-sync-determinism-and-traces

Each control was fired as the entry, `gate.py run --plan {plan} --entry {n} --project-root {clone}` with no run dir, over
scratch clones (`git clone --shared` of the project at HEAD `fb842671137fc4f5e48059f269e796b57cf25017`). Only the
verdicts are recorded here; the inputs lived in the clones.

## Clone `$TMPDIR/{scratchpad}/clone-cp` — HEAD fb842671, input: `git cherry-pick --no-commit fb8d6c15e` (exit 0; `M  packages/blitz-dom/src/document.rs` staged alone; no CHERRY_PICK_HEAD)
- entry 1 (patch present): green · exit 0 — must-pass
- entry 2 (exact take): green · exit 0 · no output — must-pass
- entry 3 (preservation guard): green · exit 0 — must-pass (document.rs excluded)
- entry 4 (no git dependency): green · exit 1 · last line 0
- entry 5 (registry pin): green · exit 0 · last line 1
- entry 7 (keyboard_compat upstream): green · exit 0 · last line 1
- entry 8 (coverage exclusions): green · exit 0 · last line 2
- entry 9 (upstream's unit test), with `CARGO_TARGET_DIR` in the scratchpad: green · exit 0 · 38.8 s — `hover_state_tests` 3 passed, `hovering_below_content_reports_default_cursor ... ok` — must-pass for its copied test name

## Clone `$TMPDIR/{scratchpad}/clone-ctl` — HEAD fb842671, input: the same cherry-pick, then three planted edits, uncommitted: `Cargo.toml`'s taffy line given `git = "https://github.com/DioxusLabs/taffy", rev = "ea07c4c3…"` in place of `version = "0.15"`; one `--ignore-filename-regex 'keyboard_compat\.rs$'` dropped from `.github/scripts/ci-leg.sh`; one comment line added above `get_cursor` in `document.rs`
- entry 2: red · exit 1 (the extra line) — must-fail
- entry 3: red · exit 1 — must-fail
- entry 4: red · exit 0 (expected exit 1) — must-fail
- entry 5: red · exit 1 (expected exit 0) — must-fail
- entry 8: red · last line 1 (expected 2) — must-fail

## Typed controls (each differs from its entry's `run` by one input operand alone, run as `bash -o pipefail -c`)
- entry 6, the sha operand set to our `36877a237318b73df6116b5899d9e0bb341f9068`: 8 at exit 0 — the regex reads a commit that adds such lines
- entry 7, the path operand set to `apps/bump/Cargo.toml`: 0 at exit 0 — the read counts 0 for a path the sha does not hold

## The operator CI-tally entry (gate 30)
- fired at P4, 2026-10-18T06:06Z, on the completed green run CI#39100630123 of fb842671: Test [default features] 192 · 990 · 0 · 12; Test (macos) 173 · 987 · 0 · 12; Coverage report 173 · 987 · 0 · 12; Test (windows) 173 · 963 · 0 · 9; Accessibility (a11y) tests 6 · 43 · 0 · 0; Stand checks repeated 144 · 582 · 0 · 21; exit 0
