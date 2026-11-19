# P5 review, card 2 — 2026-11-19-build-directory-held · «Build directory held»

Plan: `doc:2026-11-19-build-directory-held/plan` rev 4, rendered at `.andromeda/cache/docs/2026-11-19-build-directory-held/plan.md`
to open in an editor. No plan stands in the chunk's folder; writing in that file changes no record until a `put`.
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-19-build-directory-held` diffs the
chunk's records. Your review is snapped as `inputs#I3`.

## What your review changed
- **Step 8 (new), `.github/scripts/ci-leg.sh` modified.**
  - The `fast` arm keeps its five-leg loop and its stop-at-first-red.
  - After the loop reads green it runs `python3 "$SCRIPTS/../../scripts/target-sweep.py" run` when `CI` is unset or
    empty.
  - The sweep's exit is discarded, so `fast` still `exit 0`s, and its one JSON line is printed.
  - No `LEGS` or `FAST_LEGS` entry is added; no ci.yml job changes.
- **Every sweep ending prints one JSON line.** It holds `verb`, `outcome` (swept · planned · refused · failed) and, on a
  refusal or failure, `cause`. A refusal therefore still prints a line in `fast`.
- **Step 7 adds four fast-leg contract cases.** They drive the real `ci-leg.sh fast` under a `cargo` shim and a `python3`
  shim, as `test_ci_workflows.py`'s `run_leg` does:
  - `CI` absent: the sweep is called once, after embolden, and its line is last;
  - a refused or failed sweep: `fast` still exits 0;
  - `CI=true`: no call;
  - a red leg: no call.
- **Gate entries.**
  - 3 — `ci-leg.sh fast` now expects `contains "verb": "run"`.
  - 4 (new) — `CI=true bash .github/scripts/ci-leg.sh fast` expects exit 0 and `lacks "verb":`.
- **Step 9, day B, fires no sweep by hand.** It opens on `fast` (its sweep clears day A's hashes) and ends on the mix's
  last `fast`. Day A (step 4) runs before the wiring lands, so it stays unmended.
- **The report gains a 14th line, `fast-sweep-cost:`** (entry 6 now `last line 14`). Its scratch-clone controls were
  re-run: 14 lines read green, 13 read red.
- **A lean for you to confirm or overturn.** On a red verdict no sweep runs: `fast` stops at its first red leg, as today.
  The sweep never reddens either way.

## Baselines (rev 4)
- 1 red — the unittest file is absent.
- 2 green, 13.83 s.
- 3 red — exit 0 with every leg green (516 s from cold), only the sweep's `"verb": "run"` line absent.
- 4 green — 419 s, no sweep line.
- 5 red — the script is absent.
- 6 red — the report is absent.
- 7 green.

Entries 8–10 (hygiene, clean-tree push, `ci.py`) are unchanged.

**A side reading.** Those two `fast` baselines alone left `target/` at **101G**. Step 2 removes `target/debug`, since
every unit in it is this chunk's own build, before its cold measurement.

## Cost the wiring brings
The sweep's keep pass builds any kept selection not yet built: agent-run's `-p blitz-tests` set, `-p seven_guis` and
`escher-session`. A `fast` on a day where those have not run pays that build. Step 9 measures it as
`fast-sweep-cost:`.

## Concretization preview — matrix#v040-01
- **Current:** «The cause of the growth is measured and named; after the mend, a working day of ordinary gate runs on
  the dev host grows the build directory by a stated number of gigabytes, read before and after, and every check
  stays green.»
- **Proposed:**
  - The footprint report's `cause:` line names, from real-tree readings, the hashes a source edit adds (predicted 0)
    against a lockfile move's, and each resolution's share.
  - `day-unmended:` and `day-mended:` state in GB, by `du -sb target` before and after, the growth of a working day's
    gate mix **replayed** on the dev host: without the sweep, then with it wired into `fast`. Replayed is your
    chosen reading, inputs#I2.
  - Every leg the replay fires reads green; `fast` and the pushed HEAD's CI read green.
- **The one narrowing:** «ordinary gate runs» becomes «a replayed working day of the ordinary gate mix», your P4
  choice.

## Expected amendments (wrap), widened by the review
- **test-plan §9:** the cause, the sweep, the two day numbers, and `decision/local-pre-push-gate` gaining the sweep.
- **architecture:** `contract/ci-leg-legs/fast` gains the sweep step. §Occupied Resources gains the sweep as a remover
  of `target/debug` units, and **`CI`** as an env name `ci-leg.sh` reads, a flag carrying no content.
- **security-plan:** the sweep's argv is refused before acting; the environment values gain `CI`.
- **obs-plan §9:** the sweep's counts-only stdout.

## Beyond the plan
none
