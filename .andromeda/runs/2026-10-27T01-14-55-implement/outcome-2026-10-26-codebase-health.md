# Implement outcome — 2026-10-26-codebase-health

**Outcome: green + smoke.** The chunk is functionally complete. Next → the OPERATOR PASS, plan entries **19**
(`gate.py hygiene`, before the pre-CI commit), **20** (the pre-CI commit, subject exactly `chore(2026-10-26-codebase-health):
operator pre-CI commit, for the run this chunk's verdict reads`, then `fast`, the clean-tree guard and the push),
**21** (`ci.py conclusion --sha HEAD --wait 1800`) and **22** (the per-job tallies, `<id>` the run that entry
prints, into `evidence/ci.md`). Then `/andromeda-wrap-session`.

Run dir `.andromeda/runs/2026-10-27T01-14-55-implement/` · gate trail `gate-2026-10-26-codebase-health.json`.

## Files changed
- `packages/escher-driver/src/lint.rs` — `findings`'s per-entry ladder moved into two private module-level items:
  `readings` (the entry's `(Defect, Reason)` pairs in push order: zero-size alone for a box with no area, else
  placement, then truncated, then contrast) and `placement` (the off-screen / clipped match). `findings` keeps its
  signature, its overlap pushes first, and `ordered`. `Session::lint`, `flatten`, `overlaps` and every per-defect
  reader are untouched.
- `packages/escher-driver/src/mcp_tests.rs` — the listing test keeps its outer assertions and its row walk. Each
  row's name assert stays inline; the rest goes to three `#[track_caller]` helpers, `assert_described`,
  `assert_schema` and `assert_property`, which hold every former assertion with its expectation and the
  `row {row}` message.
- `tests/blitz-tests/tests/stand_requirement_sweep.rs` — the 0.2.0 sweep test keeps its table-order and leg checks.
  The rows go to `#[track_caller]` `assert_0_2_0_row` and `assert_0_2_0_witness`, and the target predicate to
  `inside_stand_repeat` (it now holds the two path-prefix constants). Every message is unchanged.
- Chunk records: `evidence/complexity.md`, `evidence/counts.md`, `evidence/row-controls.md`.
- Ledger: `escher-0.3.0/verification-matrix.json` — v030-01 `planned → implemented`, with its ref.

3 source files modified · 0 new · 0 scope-record lines (`gate.py scope`: `scope: clean — changed 3 · listed 3 ·
recorded 0 (companion 0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 559`).

## Gates
One firing of the whole block, green on the first run: `entries 22 · green 18 · red 0 · recorded 0 · timeout 0 ·
not-run 4` (19–22 `leg operator`). No deferral: Rust carries this chunk's delta, so every gate ran. Every count
equals its baseline or prediction (`evidence/counts.md`):
- complexity `over 15: 85` (88 at base), the three rows gone, `clear_interaction_state_for_removed_node 17.0`
  standing, `files read 429`;
- escher-driver 100 passed; stand checks 205 · 0 · 7; a11y 14 · 85 · 0 · 0; stand-repeat 3 of 3 rounds, 204 · 864 ·
  0 · 21; workspace 208 · 1117 · 0 · 12; CI scripts 108 OK; doc green;
- coverage: escher-driver 4930/5031 = 97.99 %, against 4918/5019 = 97.99 % before. The factoring adds 12 counted
  lines, all covered.

Each function's own reading: `findings` 7 (was 16), the listing test 3 (was 18), the sweep test 3 (was 18). Every
new helper reads 1 to 4, and `overlaps` is unmoved at 13 (`evidence/complexity.md`).

**The guard entry (10), its pass line copied whole** (`gate.py show --n 10`):

```
root . · base 19f9ff91 · allow set · record 0 · plan steps 6 · listed 3
control: plan · serves · allow — each fired on its synthetic known positive
scope-record.md: absent — no companion is excused
guard: allowed 3 — packages/escher-driver/src/lint.rs, packages/escher-driver/src/mcp_tests.rs, tests/blitz-tests/tests/stand_requirement_sweep.rs · companions excluded 0 · changed 3 · excluded 0
guard: clean
```

The three allowed files are exactly the plan's three files to modify. No file was excluded.

**Row-guard controls (step 4)** — five of five red at an assertion (exit 101, 0 `error[` lines), each restored
sha256-equal, the three tests green after (`evidence/row-controls.md`).

## Smoke
Smoke ✓, recorded from P2 and not re-run. The plan's boot smoke is the agent-run pair, entries 11–12, which ran green
in this run. `agent-run.sh status`, read by hand afterwards, gives boot `2026-10-27T01:18:54Z` (after this run began
at 01:14:55Z) and `run stand` passed 205 · failed 0 · ignored 7. Entry 5 drove the `escher-session` binary itself
(the hosted `lint`, the MCP lint tool call).

## Deviations from plan
- **Step 4 (a)** — `.github/scripts/ci-leg.sh` holds `--test select_interaction` twice: the `a11y` leg at :39 and
  `stand-repeat` at :57. The sweep test reads the whole script by `contains`, so removing only the stand-repeat
  site leaves it green. The control removed both sites, which reads the plan's "removed from ci-leg.sh" literally.
  It read red at `named target 8: the leg runs it`. Note for the wrap: this guard cannot tell a name dropped from
  stand-repeat while another leg still names it. That predates this chunk and is out of its scope, with no change
  made.
- **Step 5** — the complexity entry also prints `packages/blitz-dom/src/document.rs get_text_selection_ranges 32.0`,
  because `document.rs` is a named file. It was over the ceiling at the base as well, is not touched, and is
  counted in both 88 and 85. No atom reads it.
- No other deviation. Nothing was surfaced and no spec↔reality gap arose.

## Expected for the wrap (plan §Implementation notes, unchanged)
The line citations into the three files are moved by the factoring. Measured on this tree: `findings` at
`lint.rs:247`, `readings` :282, `placement` :303, `ordered` :315 (was :299), `overlaps` :350 (was :334), the file 620
lines. The listing test at `mcp_tests.rs:399` (unmoved), its helpers :422 · :437 · :470, the next test at :499. The
sweep test at `stand_requirement_sweep.rs:1016` (unmoved), its helpers :1053 · :1078 · :1097, the file 1109 lines.
The masters' 38 citations are the wrap's citation sweep. `mcp_tests.rs`'s `env!` reads stay two:
`CARGO_PKG_VERSION` at :247 (unmoved) and `CARGO_MANIFEST_DIR` at :650 (was :633). So `security-plan.md:295`'s
`mcp_tests.rs:633-634` needs re-pointing, as the plan expected.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / rustc builds and test binaries (entries 2–5, 11–18; the five controls) | this run | terminated — `census --comm escher-session --comm cargo --comm rustc --comm rust-code-analysis-cli --args stand_requirement_sweep --args stand_lint`: `census: none running`; `census --args target/debug/deps --args target/llvm-cov-target --args agent-run.sh`: `census: none running` |
| `escher-session` hosts (the session checks in entries 5, 12, 14, 15, 16) | a leg | terminated — none running (same census) |
| `rust-code-analysis-cli` (entry 1; the per-function reading) | this run | terminated — none running (same census) |

## Decisions & corrections
- overseer1's directive at invocation: the phase's friction line `2026-10-27T00:38:38Z-d` (the mid-run tool move,
  uncommitted) now carries «tools moved mid-run, inert (overseer1, F362)», appended to its `what`. It was done by a
  validated python rewrite of that one line, because `evolve.py` only appends. The record is in this run's `code`
  step record (`problem`: process · overridden).
- The founder is asleep and asked for autonomous work, so forks were to go to the operator as a dialog. None arose.
- overseer1: "if a gate is red with no assertion failing on a value, stop and say so before a re-run". No gate was
  red, and nothing was re-run.
- The Bash guard's `cd` arm refused one Setup read of the skill's references. It was re-issued once with absolute
  paths (`guard-refused:1` in the `code` record).
- Standing learnings met and held: grep exit-code handling (2026-10-07), with `command grep` used; the per-package
  `Intrinsic` warning (2026-10-15) appeared in the compile check and is not this change's. The hand-mutation restore
  rule (2026-10-09) was followed: bytes written back and sha256 compared before the green reading.
- The handoff at session start was stale. Its Branch line names `build/escher-0.2.0` and Status `clean`, while the
  tree is on `build/escher-0.3.0` with the phase's uncommitted output. The phase's take-up already recorded this, and
  the wrap refreshes the handoff.
