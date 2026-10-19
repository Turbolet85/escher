# Operator pass — 2026-10-19-recorded-check-replay

Each entry was fired by hand in plan order, on the operator's directive of 2026-10-19. The windows-gnu entry (7) read
green before any push (`evidence/windows-gnu-check-082208Z.txt`).

## Entry 14 — hygiene, before the pre-CI commit
Fired as written: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`.

    gate v1.21 · e2f3ad5e
    root . · case exact · planes rust · base HEAD (no --marker)
    hygiene: clean — read 48 (runs 43 · evidence 1 · inputs 4) · trails 15 not read · copies 2 not read by P1 — 0 host paths kept · binary 0 not read by P1

The operator read both committed checks (`tests/blitz-tests/checks/counter-{cli,mcp}.json`). They hold the stand
Counter's screens only, with no host path, name or secret.

## Pre-CI commit
`git add -A`, then the commit:
- sha `d2f010a2ae6afc05d30cff0ec69855c01a6c5c12`;
- subject `chore(2026-10-19-recorded-check-replay): operator pre-CI commit, for the run this chunk's verdict reads`;
- parent `6fd1d3daf6a111c49d478967611b1014359eae81`.

The tree was clean after it.

## Entry 15 — the local pre-push gate, the clean-tree guard, the push
Fired as `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py run --plan escher-0.2.0/chunks/2026-10-19-recorded-check-replay/plan.md --operator 15`.

    operator entry 15 · history tripwire: git
     15 probe       green · exit 0 · 146.99s · 107702 B → 15.log · history moved: refs/remotes/origin/build/escher-0.2.0 6fd1d3da→d2f010a2 · bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… (122 chars)
    entries 16 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 15

The `history moved` reading is the intended push: `origin/build/escher-0.2.0` moved 6fd1d3da → d2f010a2.

## Entry 16 — the CI conclusion on the pushed sha
Fired as written: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800`.

    ci v1.3 · 2b582a9b
    repo Turbolet85/escher (the push remote `origin`) · polled 27× over 807 s
    d2f010a2ae6a verdict: red · checks 17/17 · first-fail +789 s Test (macos) · runs CI#39225375577 in_progress/-
    failed 1: Test (macos) (failure)
    running 2: oldest Stand checks repeated 493 s
    run open: CI#39225375577 in_progress

**Verdict: red. The pass stopped here, on the operator's standing instruction. No fix was made and no comparison
was loosened.**

### The measured difference (job 117972311781, `Test (macos)`, aarch64-apple-darwin)
- `stand_replay`: both tests ran there and passed.
  - `each_committed_check_holds_on_the_unchanged_stand`: ok.
  - `each_committed_check_turns_red_at_its_first_activation_on_a_changed_counter`: ok.
  - So on macOS every kept `answer` of both committed checks, in both layout modes, is equal as JSON to the live
    answer. That covers the snapshot text, the bounds, the diffs and the shot's width and height.
- seven_guis `recording::the_committed_checks_are_what_the_binary_records`: FAILED at `recording.rs:250`, on the
  first file it compares:

      counter-cli.json: the binary records 3047 bytes where 3047 are committed; …

  The length is equal and the bytes differ. `counter-mcp.json` was not compared, because the loop stops at its
  first failing assert.
- The other three `recording` tests passed, including the CLI-vs-MCP same-check test, which compares two files
  recorded on that host.
- Run state at the read: linux `Test [default features]` success; `Test (windows)` and `Stand checks repeated`
  still running; every other job success.

### What the measurement narrows it to (inferred, not byte-read)
- A record's bytes are its call, its acceptance, its 16-hex `digest` and its `answer`.
- The calls are fixed by the script, and the acceptances and answers held as JSON values on macOS.
- The writer is the crate's own deterministic JSON writer.
- So the bytes that differ are the digests: an equal-length substitution of hex digits.
- The one digest that hashes something the kept answer does not hold is the `screenshot` step's, which is FNV-1a 64
  over the line with the PNG's base64. So macOS aarch64 paints the Counter's 800 × 600 shot to different PNG bytes
  than linux x86_64.
- The diagnostic withholds bytes by design, so the differing digest itself was not read. Fetching the macOS file
  needs a run that uploads or prints it.

### The run completed
CI#39225375577 ended `completed failure`. `Test (macos)` was its one red.
- `Test (windows)`: success. All three `stand_replay` tests ran there and passed, as did the off-unix refusal
  checks.
- `Stand checks repeated`: success.
- Every other job: success.

There was no red from any other cause.

## Fix after CI run 39225375577 (the operator's direction, 2026-10-19)
This is the plan's own narrowing (plan §Implementation notes: «keep the two scripts to steps that hold on every
OS»). The comparison stays byte-strict.
- `examples/seven_guis/tests/flows/record_checks.sh`:
  - the command-line session's `screenshot` call is dropped;
  - the MCP session's `screenshot` request is dropped, its later ids moved down by one;
  - the failing-step numbers follow.
- Both checks were re-recorded through the binary (`cargo build --locked -p seven_guis --bin escher-session`, then
  `sh examples/seven_guis/tests/flows/record_checks.sh target/debug/escher-session tests/blitz-tests/checks`):
  - `counter-cli.json` went from 3,047 to 2,929 B;
  - `counter-mcp.json` went from 3,838 to 3,720 B.
- `git diff` removes exactly the two `screenshot` records. In the CLI check, the record before it loses its trailing
  comma. Every other byte is unchanged. The first activation steps stay at index 1 and 3.
- Local reads, all ok:
  - seven_guis `recording`: 4 passed;
  - seven_guis `cli_flow`: 2 passed;
  - blitz-tests `stand_replay`: 3 passed.
- Windows-gnu check (plan entry 7, re-fired in run `.andromeda/runs/2026-10-19T08-22-08-implement`):

      7 build       green · exit 0 · 0.74s · 662 B → 7.2.log · cargo check --tests --locked --target x86_64-pc-windows-gn… (105 chars)

## Measured limit (for the wrap)
- **The limit:** a shot's PNG bytes differ by OS. On CI#39225375577, macOS aarch64 recorded the Counter session
  through the binary into `counter-cli.json` with the same length and different bytes than linux x86_64. The one
  digest not fixed by an answer that held as JSON there is the `screenshot` step's, an FNV-1a 64 over the line
  holding the PNG's base64.
- **What follows:** a committed check holds no `screenshot` step.
- **Where the shot path stays covered:**
  - `stand_session_recording` records a `screenshot` step in process on each OS. It checks the step keeps
    `{"width","height"}` with no image, and replays it on a fresh session.
  - The operator's direction named the same-host CLI-vs-MCP test
    (seven_guis `recording::the_command_line_and_the_mcp_role_keep_the_same_check`). That test runs a `click` and a
    `snapshot` only and holds no `screenshot` step, so as it stands it does not cover the shot path. Measured by
    reading `recording.rs:129-164`; left unchanged here.
- **Still owed:** pixel comparison of shots stays out of this version (F3 B).
