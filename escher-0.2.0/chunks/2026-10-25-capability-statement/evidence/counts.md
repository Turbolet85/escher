# Counts — 2026-10-25-capability-statement (/implement, run `.andromeda/runs/2026-10-25T10-05-33-implement/`)

Gate trail: `.andromeda/runs/2026-10-25T10-05-33-implement/gate-2026-10-25-capability-statement.json`.

- Whole block, first firing: `entries 16 · green 11 · red 0 · recorded 0 · timeout 0 · not-run 5` (the five `operator`
  entries 12-16 not run, the operator pass's).
- After the statement's a11y wording edit, re-fired `--only 1,4,6,9,10`: `entries 16 · green 5 · red 0 · …`; after the
  sentence's last tightening, `--only 1,4,6`: `entries 16 · green 3 · red 0 · …` — entries 1, 4 and 6 green on the final
  text.
- Entry 1, the pin: `Ran 4 tests` · `OK` (log 1.3).
- Entry 2: README's «the eleven driver commands» count 0 (exit 1). Entry 3: README's `docs/capabilities.md` line count 1.
- Entry 4, the secret search over the two new files: 0 (exit 1).
- Entry 5, README numstat against the base: `1 1`.
- Entry 6, the guard: `guard: allowed 3 — .github/scripts/test_capability_statement.py, README.md, docs/capabilities.md ·
  companions excluded 0 · changed 3 · excluded 0` · `guard: clean`.
- Entry 7, the Viola pointer: 3 lines `repository:` · `path:` · `commit:`. Entry 8: the pointer ignored by Viola's
  `.gitignore` (exit 0).
- Entry 9, `ci-leg.sh fast`: exit 0 (100.79 s on the re-fire); its `ci-scripts` leg `Ran 104 tests` · `OK`;
  `target/ci-logs/test.log` written 10:14:17Z, inside that firing.
- Entry 10, the workspace tally: `lines 206 passed 1105 failed 0 ignored 12` — as predicted, unchanged, no Rust test
  added.
- Entry 11, `ci-leg.sh doc`: exit 0.
- The pin's red-before-green controls: `evidence/pin-controls.md`.
