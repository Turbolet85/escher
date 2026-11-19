# P5 review — 2026-11-19-build-directory-held · «Build directory held» (escher-0.4.0, Epoch 1)

Plan: `doc:2026-11-19-build-directory-held/plan` rev 2, rendered at `.andromeda/cache/docs/2026-11-19-build-directory-held/plan.md`
to open in an editor. No plan stands in the chunk's folder; writing in that file changes no record until a `put`.
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-19-build-directory-held` diffs the
chunk's records.

## What research already measured (P3, no build)
- **The edit hypothesis is false on this host.** On a two-crate scratch workspace with cargo 1.99, a source edit added
  0 files to `deps/` and rebuilt in place. A feature flip minted new hashes for the crate, the crate above it and its
  test binary. What mints hashes is a change of features, lockfile, profile/flags or rustc version.
- **There are four feature resolutions of the engine**, read with `cargo tree` and no compile: `--workspace`,
  `-p blitz-tests` (49 packages resolved differently, blitz-dom 10 features against 16), `+font-embolden`, and
  `-p seven_guis` (55 different). Each builds every unit and test binary above the engine under its own hash.
- **Seven `Cargo.lock` changes since 2026-11-13**, the day the growth began; two moved taffy, below every engine
  unit. The `msrv` leg's rustc 1.91 also builds into the same `target/debug` as stable 1.99.
- **192 integration-test binaries, not 193.** `all.rs` is unbuilt.
- **The mutation tier builds under `target/mutants-tmp/`**, outside `target/debug/deps`.

## Your P4 word (inputs#I2)
The mend is a stale-hash sweep. The working day is a replayed one. The four resolutions are costed as a measurement
only, with whether two could be one. The sweep must be safe during a build or refuse then, with a control for each.

## Goal
Re-read the cause on the real tree, cost the four resolutions, and ship `scripts/target-sweep.py`. State a replayed
working day's growth without and with the sweep, in GB.

## Steps (9)
1. Footprint instrument in the chunk's evidence (not committed).
2. Cold cost per resolution, in the order workspace → `-p blitz-tests` → embolden → `-p seven_guis`: marginal GB,
   seconds and units owned alone. Control 1: the sweep fired during the cold build must refuse (exit 3, deps
   unchanged). This also measures the lock premise; if it fails, the chunk stops and surfaces it.
3. The hash mechanism on the real tree: an edit (predicted 0 new hashes) and one lockfile move (predicted one per
   unit above the moved crate). Both are restored by sha256, and the lock is asserted equal to the base commit.
4. Replayed day A, unmended: `fast`, agent-run boot/stand/session, `stand-repeat`, two `-p` crate tests, `doc`, a
   lock move with `fast`, then `fast` again. Growth = `du -sb` end − start.
5. `scripts/target-sweep.py {plan|run}`:
   - The keep set comes from cargo's JSON artifact lists over a closed list of the legs' and agent-run's selections,
     plus `escher-session` and both clippy lines.
   - It takes a non-blocking `flock` on `target/debug/.cargo-lock` before removing anything, and refuses if the lock
     is held.
   - It keeps units newer than its keep pass, and removes only within deps / .fingerprint / build / incremental.
   - Stdout is one JSON line of counts. Exits are 0 / 1 / 2 / 3. It refuses off unix.
6. `scripts/target-sweep.ps1`, the pass-through.
7. `.github/scripts/test_target_sweep.py` under a cargo shim, with nine cases. They include the held-lock refusal,
   the second taker excluded, a newer unit kept, and the state areas byte-equal.
8. Sweep, then replayed day B, ending on a sweep. Growth is read the same way. Control 2: after the sweep, every
   kept selection compiles nothing.
9. `footprint-report.md`: 13 column-0 lines (cause, 4 resolutions, could-be-one, edit, lock-move, llvm-cov, two day
   numbers, two controls).

## Touchpoints
- New: `scripts/target-sweep.py`, `scripts/target-sweep.ps1`, `.github/scripts/test_target_sweep.py`.
- Modified: none. No lockfile, manifest, source, profile or leg changes (entry 6 guards it).

## Test commands (9 entries)
1. sweep unittest — baseline red: the file is absent.
2. `ci-leg.sh ci-scripts` — baseline green.
3. `ci-leg.sh fast` — seen green at the last wrap.
4. `target-sweep.py plan` — baseline red: the script is absent.
5. Report line count, `last line 13` — baseline red: the report is absent. Must-pass and must-fail controls ran on a
   scratch clone.
6. `git diff --quiet b9a3bb19a… -- Cargo.lock Cargo.toml packages apps examples tests wpt .config` — baseline green.
   Its known-positive control read red on an edited lock.
7–9. Operator legs: hygiene, clean-tree push, `ci.py conclusion --sha HEAD --wait 2700`.

No smoke entry, because no boot-path is touched. The CI entry's note: no CI leg runs the sweep off linux, so no leg
witnesses its unix-only refusal.

## Concretization preview — matrix#v040-01
- **Current:** «The cause of the growth is measured and named; after the mend, a working day of ordinary gate runs on
  the dev host grows the build directory by a stated number of gigabytes, read before and after, and every check
  stays green.»
- **Proposed:**
  - The footprint report's `cause:` line names, from real-tree readings, the new hashes a source edit adds (predicted
    0) against those a lockfile move adds, and each resolution's share.
  - Its `day-unmended:` and `day-mended:` lines state in GB, read by `du -sb target` before and after, the growth of
    a working day's gate mix **replayed** on the dev host, without and with the sweep at its end. The replayed day is
    your chosen reading, inputs#I2.
  - Every leg the replay fires reads green, and `fast` and the pushed HEAD's CI read green.
- **The one narrowing:** «a working day of ordinary gate runs» becomes «a replayed working day of the ordinary gate
  mix», as you chose at P4. Nothing else is weakened.

## Notes for your review
- The sweep runs by hand. No leg, hook or skill fires it, because a new leg moves the leg pins and the capability
  statement. Without a routine caller, growth returns unless someone runs it.
- Steps 2–4 and 8 are long cold builds and replays, so implement's run will take hours of wall time.
- Expected amendments (wrap):
  - test-plan §9 — the cause, the sweep, the two day numbers;
  - architecture §Occupied Resources — the sweep as a remover of `target/debug` units alone;
  - security-plan §Input Validation — argv refused before acting, no env read;
  - obs-plan §9 — counts-only stdout.

## Beyond the plan
none
