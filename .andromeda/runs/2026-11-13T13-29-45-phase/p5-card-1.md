# P5 review — 2026-11-13-upstream-sync-determinism-and-exploration

**Chunk:** Upstream sync ahead of determinism and exploration · escher-0.3.0 · pending
**Plan:** `doc:2026-11-13-upstream-sync-determinism-and-exploration/plan` rev 3. To read it in an editor, open
`.andromeda/cache/docs/2026-11-13-upstream-sync-determinism-and-exploration/plan.md`.
- No plan stands in the chunk's folder.
- Writing in that file changes no record until a `put`.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-13-upstream-sync-determinism-and-exploration` shows a diff of the chunk's records.

## Goal
Take upstream `main` at `563d3db73` as **29 no-commit cherry-picks onto the registry pins**, the operator's answer (Q1, `inputs#I2`). The manifests and the lock are unchanged, and no git dependency or new listener comes in.

The take's behaviour changes for every document are ratified as upstream's by the founder's own word, «Утвердить как upstream (Рекомендую)» (Q2). They are: lossy page decoding, stylesheets decoded from bytes, the `font` element's `face`/`size`, `valign`, `cellspacing`/`cellpadding`, empty alt as decorative, and the default-font fallback.

The take carries `da271cde1`, which cures CI's Clippy red under rustc 1.100. The audit red, RUSTSEC-2026-0459 on `libsqlite3-sys`, is answered by an ignore by ID with a reason. That lean rests on `.claude/rules/security.md` §Dependencies: rusqlite ≥ 0.41 is not semver-compatible with 0.32, and upstream carries the same 0.30.1.

The plan also absorbs the seven carries:
- 17 unpinned actions in the three upstream-only workflows pinned to SHAs, and the pin check widened to every workflow;
- two headless frame tests (blending, an inline box's own decorations);
- the left-out delta and the merge surfaces re-read;
- the changed-set qualifier, owed to the wrap.

## Top acceptance criteria
- Under `cargo +1.100.0`, both clippy lines exit 0; `fast`, `doc`, `msrv` and `a11y` exit 0.
- `audit` prints `advisories ok, licenses ok`, the one new ignore being RUSTSEC-2026-0459.
- The manifests and `Cargo.lock` are byte-identical to `19596fd4b`. `evidence/lock.md` names the 29 picks and the 99 left out, each with its reason.
- The two frame tests pass in both layout modes, each read red under its control first.
- The re-recorded Counter checks replay green. `504e459e3` moves their boot bounds (bisected at P3).
- `run stand` and `run all` pass, against today's 262 · 0 · 7 and 768 · 0 · 12. The windowed smoke boots.
- Every live `uses:` of every workflow is a 40-hex SHA (17 at HEAD, 0 after).
- Fork CI on the pushed HEAD reads `verdict: green`. The per-OS tallies are re-read with each move named, and the sink census is recorded.

## Touchpoints
- the take's 38 files, from the trial;
- `deny.toml`;
- `tests/blitz-tests/checks/counter-{cli,mcp}.json`;
- `wpt.yml`, `publish-browser.yml`, `wpt-post-results.yml`;
- `.github/scripts/test_ci_workflows.py`;
- `tests/blitz-tests/tests/all.rs`;
- two new frame-test targets;
- seven upstream targets that arrive by the take.

## Test commands
20 `[[gate]]` entries:
- the no-pin-move probe;
- `fast`;
- agent-run `boot`, `run stand` and `run all`;
- the windowed smoke;
- two clippy lines under 1.100;
- `doc`;
- the two frame tests;
- `stand_replay` and seven_guis `recording`;
- `a11y`, `msrv` and `audit`;
- the pin count;
- the operator's hygiene, push and `ci.py` read.

Baselines on the untouched tree:
- **Green (10):** the no-pin-move probe, `fast`, `run stand`, `run all`, the smoke, `doc`, `recording`, `a11y`, `msrv`, `boot`.
- **Red, the subjects this chunk cures:** the 1.100 clippy lines (`needless_bool`), the audit (RUSTSEC-2026-0459) and the pin count (17). The audit and the pin count each also read green in a scratch clone with the plan's change applied (`p5-controls.md`).
- **Red only because the targets do not exist yet:** the two frame tests.

## Concretization preview
None. The chunk claims no capability: each of v030-08 to v030-15 is named by a later entry's hint.

## For your decision
- **Q1's authority.** The scope reads «the shape the founder chooses at P4». Q1 was answered as the operator's technical answer; the founder's own word covers Q2, the widenings. No playbook escalate pattern names the take's shape on its own. Confirm that the operator's Q1 stands, or have the founder rule it.
- **Host notes from P3.** Toolchain 1.100.0 was installed beside the default, and rustup self-updated 1.29.1 → 1.29.3 as a side effect. The trial worktrees `target/tmp/trial-det` and `target/tmp/trial-a` stand, with build dirs `target/trial` and `target/trial-a`, for removal on the operator's word. `target/tmp/trial-wt` is the founder's and is untouched.

## Beyond the plan
| # | proposal | owner / place |
|---|---|---|
| 1 | the 99 upstream commits this take leaves out (the 36 pin movers with their 39 chained conflicts and 3 build-reds, the 21 of the last take) — deferred to the next sync | «Upstream sync ahead of errors and explanation», an existing entry, by the wrap's route adaptation |
| 2 | RUSTSEC-2026-0459's ignore owned until the browser moves to `rusqlite` ≥ 0.41 | a new `residual` row, written by the wrap |
| 3 | the CDP server's served listener (`127.0.0.1` via `BLITZ_CDP_PORT`) arriving with any later take of `a1670ef1b` — a security-plan amendment first | the same next-sync entry, as a CARRY its take meets |

Apply? (yes / review / cancel)
