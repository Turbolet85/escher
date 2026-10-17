# P5 review — 2026-10-16-fast-failing-mutants

**Plan:** `escher-0.2.0/chunks/2026-10-16-fast-failing-mutants/plan.md` (384 lines, 8 steps, 22 gate entries, 8
criteria) · **research:** `research.md` · **scope:** `scope.md` (one bullet added at validation-1, the nextest profile)

## Goal
Each of the 0.1.0 audit's six timeout-only mutants fails a named test fast, and a mutation run over their three files
reads 0 missed and 0 timeout. The run stays inside your limits (inputs#I1). The work is test-only: nothing shipped
changes, and `mutation_writer.rs` stays byte-identical to upstream.

## What P3 measured, which shaped the plan
- All six hang in a blitz-tests integration binary that cargo runs before any crate's unit tests: `stand_accessibility_ids`,
  `stand_act_diff` ×2, `stand_mcp_masked` ×2 and `harness_smoke`. Each spins on its own test thread, so under
  `cargo test` a fast unit test beside the code never gets to run.
- Your P4 answer, «nextest profile (Recommended)»: three direct unit tests, plus a committed nextest `mutants` profile
  that runs them first and ends a run at its first failure.

## Touchpoints
- `examples/seven_guis/src/tasks/timer.rs` — a test module appended after line 196: `TimerTicks::next` polled by hand
- `packages/escher-driver/src/mcp_tests.rs` — `read_request` over `b"a\n\nb\n"`, plain and one byte at a time
- `packages/dioxus-native-dom/src/dioxus_document_tests.rs` — `create_text_node` on a fresh `MutationWriter`
- `.config/nextest.toml` (new) — profile `mutants`: the three tests at `priority = 100`, `fail-fast = { max-fail = 1,
  terminate = "immediate" }`, `test-threads = 4`
- `evidence/`: six hand mutation controls, and the three runs' compact records

## Test commands (22 entries)
- Unit counts: seven_guis lib 1 → 2, escher-driver 73 → 74, dioxus-native-dom 68 → 69 (baselines red at the old counts).
- The profile parses with no ignored key and lists the three tests.
- **Three mutation runs** (`operator` entries, fired once by /implement, about 1.5 h in all). Each is cargo-mutants
  27.1.0 on one file with `--test-tool nextest`, `NEXTEST_PROFILE=mutants`, the audit's test set for that unit, `-j 2`,
  `taskset -c 12-15,28-31` and `--timeout 600`, into a fresh `target/tmp/ffm-{file}-{stamp}`. There is no recursive
  `rm`: the host refuses it.
- Readers over the records: the six read `CaughtMutant` with a test phase under 120 s; 0 `MissedMutant` / `Timeout` /
  `Failure`; 137 mutants run.
- Guards: `mutation_writer.rs` identical to `434e0661`; `timer.rs` lines 1–196 unchanged; no other tracked code file
  moves; the only new code-tree file is `.config/nextest.toml`; the silence probe reads 0.
- Then the a11y leg, `ci-leg.sh fast` and `doc`, and the operator's hygiene, push and CI read.
- Every new entry has its baseline, and every green guard has its must-fail / must-pass control fired as the entry in a
  scratch clone. Entries 8–10 were controlled over records the entries' own `jq` built from real `outcomes.json`
  (`.andromeda/runs/2026-10-16T18-14-30-phase/controls.md`).

## Concretization preview — v020-02 (claimed by this chunk on your word)
**Current:** "A mutation run over the units that held the six timeout-only mutants catches each of them with a failing test
well inside the timeout, and the run's score holds at 0 survivors."

**Proposed:** "Under cargo-mutants 27.1.0 with `--test-tool nextest` and `.config/nextest.toml`'s `mutants` profile,
one mutation run per file over the three files that hold the six timeout-only mutants (`timer.rs`,
`mcp/framing.rs`, `mutation_writer.rs`, 137 mutants at the base) exits 0, each file against the 0.1.0 audit's test set
for its unit, at `-j 2`, `taskset -c 12-15,28-31` and `--timeout 600` (inputs#I1). 0 mutants read missed or timeout.
Each of the six reads `CaughtMutant` with a test phase under 120 s, and each turns its named direct test red in a hand
mutation control restored sha256-equal."

What moves, for your never-weaken check:
- **"units" → the three files.** This is your directive (inputs#I1). The outcome bar, 0 survivors, stays.
- **The instrument is now nextest's `mutants` profile.** Under plain `cargo test` the six still read `timeout`, because the
  hung blitz-tests binaries come first. The plan records that as a test-plan §10 amendment, so the next version-end
  audit's mutation tier runs with the profile.
- **"well inside the timeout" → each test phase under 120 s, against your 600 s bound.**

## Notes
- Disk: `target/` 572 GB, 352 GB free. The runs build in temp copies, two at a time. Anything cargo-mutants leaves
  behind is named in the report for you to clear, since the host refuses a recursive `rm`.
- P3 and P5 ran mutation runs inside your limits: the six alone, about 48 min, to measure where each hangs, and one
  caught mutant (`timer.rs:21:9`) for a real outcome record. Their outputs sit under `target/tmp/ffm-p3/` and
  `target/tmp/ffm-p5-20261016T193004Z/`.

## Beyond the plan
none
