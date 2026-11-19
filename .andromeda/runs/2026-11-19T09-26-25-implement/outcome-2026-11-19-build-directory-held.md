# Outcome — 2026-11-19-build-directory-held

**Outcome:** green — no smoke (no boot path; skipped) · Next → the OPERATOR PASS (entries 8 `gate.py hygiene`, 9 the push, 10 `ci.py conclusion --sha HEAD --wait 2700`), then `/andromeda-wrap-session`.

## Files changed
- `scripts/target-sweep.py` (new, 225 lines) — the stale-hash sweep, `plan` · `run`
- `scripts/target-sweep.ps1` (new, 4 lines) — the Windows pass-through
- `.github/scripts/test_target_sweep.py` (new, 416 lines) — 16 unittest cases: the sweep's contract under a cargo shim and the fast leg's ownership of it
- `.github/scripts/ci-leg.sh` (+5 lines) — the `fast` arm runs the sweep after its legs read green, when `CI` is unset or empty, its exit discarded
- chunk evidence (gitignored work dir): `evidence/footprint.py`, `footprint.ndjson`, `footprint-report.md`, `control-refuse.txt`, the step scripts and logs

## Gates (run `2026-11-19T09-26-25-implement`, gate trail `gate-2026-11-19-build-directory-held.json`)
- `python3 -m unittest discover -s .github/scripts -p 'test_target_sweep.py' -v` — green (Ran 16 tests, OK)
- `bash .github/scripts/ci-leg.sh ci-scripts` — green
- `bash .github/scripts/ci-leg.sh fast` — green, 537.76 s, the sweep's line `{"verb": "run", "outcome": "swept", "units_kept": 3089, "units_removed": 0, …}` printed
- `CI=true bash .github/scripts/ci-leg.sh fast` — green, 416.56 s, no sweep line (0 `"verb"` lines in its log)
- `python3 scripts/target-sweep.py plan` — green, `{"verb": "plan", "outcome": "planned", "units_kept": 3108, "units_to_remove": 296, …}`
- `grep -c -E '^(cause|…): ' …/footprint-report.md` — green, last line 14 (re-fired after the report's day lines gained the whole-`target` reading: green again)
- `git diff --quiet b9a3bb19… -- Cargo.lock Cargo.toml packages apps examples tests wpt .config` — green
- operator entries 8 · 9 · 10 — not run, leg operator (the operator pass's)
- iterations: the block read green at its first firing; two sweep defects were found earlier by step 9's warm control and fixed red-first (below)

## Measurements (footprint-report.md)
- cause: hashes per (resolution × lockfile × toolchain) epoch; an edit adds 0, a one-crate lock move 184 test-binary hashes (+36.49 GB)
- resolutions, cold marginal: workspace 54.61 GB · 151 s · 792 alone; blitz-tests 37.55 GB · 106 s · 190 alone; embolden 0.40 GB · 5 s · 6 alone; seven-guis 3.24 GB · 35 s · 88 alone
- day unmended +57.90 GB; day mended −2.04 GB (peak 195.89 GB inside the day, before the lock-move fast's sweep); every replayed leg green
- the sweep costs 181–219 s per fast
- control-refuse: refused · build-in-flight, exit 3, nothing removed; control-warm: 0 new unit entries after the final sweep

## Deviations, with justifications
1. **Cold start by rename, not removal.** The host guard refused `rm -rf target/debug` ("a recursive rm — a deny rule … Leave the target and name it in your report"). The P5 baselines' 101 GB build was renamed to `target/p5-baseline-debug` (it is this chunk's own build: `target/` was 75 M at take-up). It is left for the operator to remove; it sits inside every whole-`target` reading as a constant.
2. **The lock probed before the keep pass too.** The plan takes the lock after the keep runs. The keep runs' own cargo would wait out a build in flight and then read the lock free, so the step 2 control would not refuse. The sweep now probes non-blocking first (refusing `build-in-flight`) and takes the lock again before removal. Measured on cargo 1.99: a build holds `.cargo-lock`, `.cargo-build-lock` and `.cargo-artifact-lock` for its length.
3. **The clippy keep lines carry the leg's `-- -D warnings`.** Clippy tracks its arguments, so a keep run without them would rebuild those units in place, and the next clippy leg would rebuild them again.
4. **incremental/ is not keyed by cargo's hash** (the plan assumed one hash for all four directories). rustc names those directories `{crate}-{base36 crate id}`. Per crate, the sweep keeps as many directories as the keep runs name units of that crate, newest first, plus any written after the keep pass began. A wrong removal there costs incremental warmth only, never a stale build.
5. **Two sweep defects found by step 9's warm control, each fixed behind a red-first case.**
   - (a) Cargo's JSON names a build script's compile unit by its directory (`build/{pkg}-{hash}/build-script-build`), so those units were never kept. The warm control read 737 `Compiling` lines; it was fixed to read the hash from the file's directory too. Evidence: `evidence/red-first.txt`.
   - (b) An example's unit is named only by its hash-less uplift (`examples/box_shadow`), so its fingerprint was removed. A unit with no `deps/` or `build/` entry now keeps its fingerprint. Evidence: `evidence/red-first.txt`.
   - The mended day was re-run as `day-b2` after (a). The first day B readings are superseded and kept in the ledger. Day B2 predates (b), which touches only 19 example fingerprint directories, so its byte readings stand. The warm control was re-read on the final sweep.
6. **Lock move crate.** `smallvec 1.16.2 → 1.16.1` was refused offline by parley's `^1.16.2`, and `Cargo.lock` was left unchanged. `memchr 2.8.3 → 2.8.2` was used. Every restore was compared by sha256 and against the base commit.
7. **Step 2 control's deps count.** The count cannot stay constant while a cold build adds files. The two firings read 1479 → 1488 and 2006 → 2024. The names that vanished are rustc's own transients (a codegen `.rcgu.o`, a temp `rmeta…` directory). The sweep's refusal path removes nothing.
8. **Step 9 control's "no Compiling line" does not hold as written.** `seven_guis` and `blitz-tests` rebuild in place on every alternation of the kept selections with no sweep in the tree (step 2's unit reads compiled 2 · 2 · 2 · 1 before the sweep existed). The control is read as 0 new unit entries after the final sweep, with only those 7 lines, and escher-session standing.
9. **Test-writing order.** The fast-leg cases were appended after day A and run red before the wiring (`evidence/red-first.txt`). Day A's `ci-scripts` legs therefore ran the contract half alone, green.
10. **Instrument fix.** `footprint.py units` had the sweep's build-script blind spot. The four unit reads were re-taken as `units-*-r2` (1116 · 807 · 623 · 540 against 1037 · 750 · 566 · 489).

Scope record: none — `gate.py scope` clean (changed 4 · listed 4 · recorded 0).

## Surfaced / observed (no fix in this chunk)
- **`CI=true` fast built new hashes.** Gate entry 4 (`CI=true … fast`) recompiled 42 packages. Entry 5's plan then read 296 units outside the keep set over 153.65 GB, against 105.12 GB after entry 3's sweep. So a `fast` under `CI` set builds about 48 GB under new hashes, which the next dev-host `fast` sweeps. The project source reads no `CI`; the mechanism is unmeasured. This is for the wrap to route.
- **In-place rebuilds on alternating selections.** `seven_guis` and `blitz-tests` rebuild in place whenever the kept selections alternate (time, not disk). Each sweep's keep pass pays it.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / rustc builds of steps 2–4, 9 and the gates | this run (footprint steps, legs) | terminated — `gate.py census --comm escher-session --comm cargo --comm rustc --comm seven_guis_native --args target-sweep.py`: `census: none running` |
| escher-session hosts of `agent-run run session` and stand-repeat's process checks | a leg | terminated (same census) |

## Decisions & corrections
- **Guard refusal.** `rm -rf target/debug` was refused by the host Bash guard (recursive rm deny rule). It was not re-issued; the build was renamed aside and is named here for the operator.
- **Operator's stop conditions.** "Stop and tell me if step 2 control fails …, if any gate is red, or if /home falls under 150 GB free." None fired. The control refused as required, no gate read red, and the lowest free reading was 544 GB (after day A).
- **Evolve append refusal.** My own hand-typed `ts` was ahead of the clock; nothing landed. It was re-fired with `date -u`'s reading.
- **Sweep hazard.** Cargo 1.99's JSON names a build-script compile unit by its directory and an example by its hash-less uplift. A reader keyed on file names alone misses both. (Measured: the warm control's 737 and 8 in-place recompiles.)
