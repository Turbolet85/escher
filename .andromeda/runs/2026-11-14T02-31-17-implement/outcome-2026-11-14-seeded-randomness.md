# Outcome — 2026-11-14-seeded-randomness: green + smoke

## Files changed
- new `packages/dioxus-native-dom/src/seed.rs`: `DEFAULT_SEED` (0); `SeededRandom`, a splitmix64 whose clones share one `Rc<Cell<u64>>` state, with a hand `Debug` that writes the type name alone; `from_seed`, `next_u64`, `below`, the last by a 128-bit multiply-high with no panic path; `use_seeded_random`, which reads the root's generator through `try_consume_context`, else a private `from_host` seeded from std's `RandomState`. It has 6 inline unit tests.
- new `tests/blitz-tests/tests/stand_seeded_randomness.rs`: the dice fixture and 6 tests, plus 1 `#[ignore]` child. All run in both layout modes, and the replay test is `cfg(unix)`.
- modified `packages/dioxus-native-dom/src/lib.rs`: `mod seed;`, ungated, and its three re-exports.
- modified `packages/blitz-test-harness/src/harness.rs`: `from_vdom` provides `SeededRandom::from_seed(DEFAULT_SEED)` at the root when `runtime().has_context::<SeededRandom>(ScopeId::ROOT)` reads none. `///` documents it.
- `packages/blitz-test-harness/Cargo.toml`: unchanged, because no dependency was missing. `Cargo.lock` is unchanged.

## Gates
`gate.py run` trail: `.andromeda/runs/2026-11-14T02-31-17-implement/gate-2026-11-14-seeded-randomness.json`. Summary: `entries 11 · green 8 · red 0 · recorded 0 · timeout 0 · not-run 3`.
- Entries 1–7 were green. Entry 8, `ci-leg.sh fast`, was green in 274 s.
- Entries 9, 10 and 11 are `not run — leg operator`. They are the operator's.
- The fix loop took no iterations, and no gate was deferred.

## Controls
`gate.py control`; evidence in `.andromeda/cache/chunks/2026-11-14-seeded-randomness/evidence/control-1.txt`. Summary: `control: 2 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`.
- **no-default-seed** removed the provide in `from_vdom` and fired entry 3.
  - Its own case is `a_run_given_no_seed_reads_the_default_seed`.
  - The removed firing's log `3.without-no-default-seed.log` shows that case ran and failed: `test a_run_given_no_seed_reads_the_default_seed ... FAILED`, panicked at `stand_seeded_randomness.rs:165:9`.
  - It was the only failure: `5 passed; 1 failed; 1 ignored`.
- **below-constant** made `below` return 0 and fired entry 3.
  - Its own case is `the_fixture_reads_the_seeded_source`.
  - The removed firing's log `3.without-below-constant.log` shows that case ran and failed: `test the_fixture_reads_the_seeded_source ... FAILED`, panicked at `:136:9`.
  - `two_seeds_read_different_states` (`:157`) and `a_kept_check_replays_its_seeded_run` (`:257`) also failed: `3 passed; 3 failed; 1 ignored`.
- With the guard in place, entry 3 read green in the clone (`3.2.log`).

## Smoke (P3)
The smoke fired because `from_vdom` is the boot of every driver session. I drove it by hand, because the plan holds no `role = 'smoke'` entry. The build was `cargo build --locked -p seven_guis --bin escher-session`, then `escher-session` with `--session target/tmp/seed-smoke`, each call under `timeout 60`:

| call | exit | result |
|---|---|---|
| `start counter` | 0 | |
| `snapshot` | 0 | |
| `click --id counter-increment` | 0 | `settled:true`; `counter-value` changed to `1` |
| `snapshot` | 0 | |
| `stop` | 0 | `{"stopped":true}` |

Result: smoke ✓.

## Deviations from plan
- **The check reads `session_common`** for its call builders (`click`, `snapshot`) and `state_dir` (under `CARGO_TARGET_TMPDIR`). The plan names neither module.
  - Why: this is the convention for checks that hold a session (`.claude/rules/testing.md` §Framework).
  - Wrap: this makes it a new reader of `tests/session_common/mod.rs`, so the counts of readers in the leaves move.
- **Control edits.** The plan says "the provide removed". The control replaced the provide line with `let _ = (SeededRandom::from_seed(DEFAULT_SEED), &vdom);`, so the imports stay used. That is still a removal of the provide. "`below` returning a constant" was done as `.map(|_| 0)`.
- **No scope-record lines.** `gate.py scope` reads `scope: clean — changed 4 · listed 4 · recorded 0`.

## Surfaced
- **matrix#v030-13 is unclaimed.** The plan's acceptance names `matrix#v030-13`, but the ledger reads `planned, unclaimed` with a null chunk.
  - `matrix.py implement --id v030-13` answered `v030-13: chunk null — claim first (phase P5.4 owns the claim)`, exit 3, and wrote nothing.
  - So no ref and no `implemented` status was written. The witness ref, once claimed, is:
    - `tests/blitz-tests/tests/stand_seeded_randomness.rs::one_seed_two_runs_read_the_same_state_at_every_step`
    - `::a_second_process_reads_the_same_state`
    - `::a_kept_check_replays_its_seeded_run`
  - Who resolves it is the operator's or the wrap's call.
- **Spec↔reality gaps:** none.

## Process hygiene
`gate.py census --comm escher-session --comm stand_seeded_randomness --comm cargo` read `census: none running`.

| process | started by | final state |
|---|---|---|
| escher-session host (smoke) | this run | terminated (`stop` → `{"stopped":true}`; census none running) |
| cargo / test binaries (gate entries, controls) | this run (the gate tool) | terminated (census none running) |

## Next
The operator pass, entries 9 (hygiene), 10 (push) and 11 (CI read). Then `/andromeda-wrap-session`.

## Decisions & corrections
- Operator direction for this run: "For every gate.py control you fire, write in the outcome: the guard's own case, and whether the removed firing's log shows that case run and failed." It was applied under §Controls above.
- Guard refusals:
  - The bash guard refused `cd` into the cargo registry. I re-issued the command with absolute paths.
  - It also refused `cd` into the skill's references dir. I re-issued the command with an absolute path.
