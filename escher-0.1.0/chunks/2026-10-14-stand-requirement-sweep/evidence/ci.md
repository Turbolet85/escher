# Operator pass — hygiene, the push and the CI read

Fired on the operator's word of 2026-10-14 in this session ("Run the operator pass and stop before the wrap"):
- entry 15 as written, by its own command line;
- the pre-CI commit;
- entry 16 through `gate.py run --plan … --operator 16`, with no run dir, so it left no trail;
- entry 17 as written.

The implement run's gate record (`.andromeda/runs/2026-10-14T21-20-05-implement/gate-2026-10-14-stand-requirement-sweep.json`)
lists all three as `not run — leg operator`.

## Entry 15 — hygiene, before the pre-CI commit
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`, expect `['exit 0', 'contains hygiene: clean']`:
held (exit 0).

```
gate v1.17 · 383c6f7b
root . · case exact · planes rust · base HEAD (no --marker)
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P1 in-root home · P1 in-root drive · P2 · P3 rust — each fired on its synthetic known positive
hygiene: clean — read 69 (runs 60 · evidence 3 · inputs 6) · trails 13 not read · copies 4 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## The pre-CI commit
`55a256384c801a50ea86cbd34c16c908bd05cd79`:
- subject `chore(2026-10-14-stand-requirement-sweep): operator pre-CI commit, for the run this chunk verdict reads`;
- committed 2026-10-14 23:41:27 +0200 by `git add -A`;
- 92 files, 12162 insertions, 30 deletions;
- the tree clean after it (0 `git status --short` rows).

## Entry 16 — local pre-push gate, clean-tree guard, push
`bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.1.0`,
fired through `--operator 16` (`operator entry 16 · history tripwire: git`):

```
 16 probe       green · exit 0 · 135.65s · 98884 B → 16.log · history moved: refs/remotes/origin/build/escher-0.1.0 19756f2d→55a25638 · bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… (122 chars)
entries 17 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 16
```

The history move is the push the entry exists for, so it was intended. Read from its log,
`$TMPDIR/andromeda-gate/2026-10-14-stand-requirement-sweep/run-20261014T214137Z/16.log` (2087 lines, 98884 B):
- 185 `test result:` lines, all `ok`: 951 passed · 0 failed · 12 ignored, the implement pass's figure.
- The CI scripts: `Ran 79 tests in 5.480s` · `OK`.
- Its last lines are the push: `To https://github.com/Turbolet85/escher.git` ·
  `19756f2d6..55a256384  build/escher-0.1.0 -> build/escher-0.1.0`.

`git rev-parse origin/build/escher-0.1.0` = `55a256384c801a50ea86cbd34c16c908bd05cd79`, the same as local HEAD.

## Entry 17 — the run on the pushed sha
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800`, fired as written,
exit 0. Expect `['exit 0', 'contains verdict: green']`: held.

```
ci v1.3 · c2400620
repo Turbolet85/escher (the push remote `origin`) · polled 30× over 900 s
55a256384c80 verdict: green · checks 16/16 · wall 886 s · runs CI#38642493451 completed/success
runs: CI#38642493451 push completed/success
checks: Accessibility (a11y) tests · Build [default features] · Build counter example · Build wasm examples · Clippy
  Coverage report · Dependency audit · Documentation · MSRV Build [Rust 1.91] · Rustfmt · Test (android) · Test (ios)
  Test (macos) · Test (windows) · Test CI scripts · Test [default features]
```

The run: sha `55a256384c801a50ea86cbd34c16c908bd05cd79` · CI#38642493451 · verdict **green**, 16/16 checks, wall 886 s.

### C9 on the macOS leg (step 10)
The `Test (macos)` job, 116123587286, ended `success` (21:49:43Z → 21:56:22Z). Its log was read with
`gh run view 38642493451 -R Turbolet85/escher --job 116123587286 --log` (2497 lines):
- The command is `cargo test --all --tests --target aarch64-apple-darwin --locked`.
- The block `Running tests/cli_commands.rs (target/aarch64-apple-darwin/debug/deps/cli_commands-25e84f754e773f7e)`
  holds 11 tests and ends `test result: ok. 11 passed; 0 failed; 0 ignored`.
- Among them, at 21:56:05Z:
  `test a_state_directory_of_90_bytes_holds_a_session_and_one_of_91_is_refused_with_its_bound ... ok`. So 90 bytes
  holds a session, and 91 ends `state-dir-too-long` with status 3, its message stating 90 and holding no path.
- The Linux test, `…_of_94_bytes_…`, appears 0 times in that log: it is compiled out off Linux, as written.
- The job's log has 166 `test result: ok` lines and 0 `test result: FAILED`.
