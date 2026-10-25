# P5 controls — 2026-10-25-cold-agent-bar

Each control was fired as the entry itself (`gate.py run --plan escher-0.2.0/chunks/2026-10-25-cold-agent-bar/plan.md
--entry {n} --project-root {clone}`, no `--run-dir`). The clone is a `git clone --shared` of the project at HEAD
`e84dc818126cb27873a68e83b7f1990fb05f7413`, in the session scratchpad at `$TMPDIR/{scratchpad}/clone`, the chunk
folder copied in for the guard. Each entry fired first unmodified, reading what it read on the real tree, and the
clone was reset between controls (`git checkout --`). Times are the tool's log stamps, 2026-10-25.

| entry | input | reading |
|---|---|---|
| 8 the pipe's parse | unmodified | green · exit 0 (14:26:30Z) |
| 8 | must-fail: `if then fi (` appended to `scripts/cold-agent.sh` | red · exit 2 (14:26:31Z) |
| 9 the pipe census | unmodified | green · exit 1 · `0` (14:13:04Z) |
| 9 | must-fail: `x = os.environ` appended to `scripts/cold-agent.sh` | red · exit 0 (14:13:28Z) |
| 10 the silence census | unmodified (the two new checks absent, `cat` naming them) | green · exit 1 · `0` (14:13:05Z) |
| 10 | must-pass: both new checks present, a `//!` line each | green · exit 1 · `0` (14:13:29Z) |
| 10 | must-fail: `fn noisy() { println!("x"); }` appended to `stand.rs` | red · exit 0 (14:13:29Z) |
| 11 the no-engine-change count | unmodified | green · `0` (14:13:05Z) |
| 11 | must-pass: `stand.rs` edited | green · `0` (14:13:37Z) |
| 11 | must-fail: `packages/escher-driver/src/lint.rs` edited | red · last line `1` (14:13:37Z) |
| 13 the guard | unmodified (chunk folder copied in) | green · `guard: clean`, changed 0 (14:13:17Z) |
| 13 | must-pass: `stand.rs` edited | green · `allowed 1 — examples/seven_guis/src/stand.rs` · `guard: clean` (14:13:37Z) |
| 13 | must-fail: `lint.rs` edited | red · `guard: outside 1 files` (14:13:38Z) |
| 13 | must-fail: an untracked `tests/blitz-tests/tests/stray_extra.rs` | red · `guard: outside 1 files` (14:13:38Z) |
| 24 the evidence reader | unmodified (the record absent) | red · exit 2 (14:13:06Z) |
| 24 | must-pass: `evidence/live-verdict.json` written by `json.dump(indent=2)` with `"outcome": "passed"` | green · exit 0 · `1` (14:13:53Z) |
| 24 | must-fail: the same with `"outcome": "failed"` | red · exit 1 (14:13:53Z) |

Entry 12, the stand graph's platform adapters, was controlled by operand alone on the real tree. Its `run` with
`-p blitz-shell` in place of `-p seven_guis`, typed as the tool types an entry (`/usr/bin/bash -o pipefail -c …`), read
`1` and exit 0. So the probe reads an adapter where one stands.

Left in the scratchpad: the clone `$TMPDIR/{scratchpad}/clone`, with the copied chunk folder and an `evidence/` holding
the last control's file. A recursive removal is refused on this host, so it is named here, not removed.
