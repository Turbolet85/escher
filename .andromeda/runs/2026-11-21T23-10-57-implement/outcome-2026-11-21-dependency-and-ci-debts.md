# Outcome — 2026-11-21-dependency-and-ci-debts

**Outcome: green.** 9 of 9 non-operator gates green on the bare block (`gate-2026-11-21-dependency-and-ci-debts.json`,
the last run record); gate 13 `not run — defer` (gated, the Epoch 2 boundary audit not recorded); gates 10–12 are
`leg operator`, the operator pass's. Both plan controls hold (`evidence/control-1.txt`). Smoke skipped: no boot-path
or UI-surface change and no smoke entry.

**Next → the OPERATOR PASS**: entries 10 (`gate.py hygiene`), 11 (the push) and 12 (`ci.py conclusion --sha HEAD
--wait 2700`), then `/andromeda-wrap-session`. Entry 12's CI read is the first run of the four non-linux keys under
the verified archive. A key that no build asks for leaves that row's `Skia built from the verified archive` step red.

## Files changed
- new: `.github/skia-archives.toml`, `.github/scripts/skia_archive.py`, `.github/scripts/test_skia_archive.py`
- modified: `.github/scripts/ci-leg.sh`, `.github/scripts/test_ci_workflows.py`, `.github/workflows/ci.yml`,
  `deny.toml`, `Cargo.lock` (async-executor 1.14.0 → 1.14.1, only that entry)

## Deviations, with justifications
1. **`FORCE_SKIA_BINARIES_DOWNLOAD` is not set anywhere** — the operator's word at P1 (inputs#I7, «Drop FORCE, keep
   {key} (Recommended)»). In a crates.io build the flag sends skia-bindings 0.157.0 to its "for testing" branch
   (`download.rs:259-263`), whose key hash is `git rev-parse --short=20 HEAD` in the crate directory (`git.rs`).
   There that command reads `fatal: not a git repository`, exit 128, so the build falls to `STARTING A FULL BUILD`
   (`build.rs:118-124`): no download and no panic. Steps 5–7 export `SKIA_BINARIES_URL` alone.
2. **The URL is a `{key}` template** (inputs#I7): `skia_archive.py` prints `file://{DIR}/skia-binaries-{key}.tar.gz`,
   not the archive's own path (step 2's stdout wording). skia-bindings fills `{key}`, so a build asking for another
   key than the recorded one finds no file.
3. **The fallback, stated as a limit, in the plan's words** (inputs#I7). The plan says «Under
   `FORCE_SKIA_BINARIES_DOWNLOAD` a build asking for another key than the one recorded panics». Instead, a build
   asking for another key finds no file and falls back to a full source build of skia, with no panic. So does any
   failed read of the archive.
4. **A witness step on the CI matrix** (inputs#I7, «if it can run at no real cost, add it»). `Skia built from the
   verified archive` follows `test` on every row. It reads `target/{target}/debug/build/skia-bindings-*/output` for
   one file holding both `FROM: file://` and `DOWNLOAD AND INSTALL SUCCEEDED`, else it exits 1. It is a shell loop
   with no cost beyond the read, so a silent source build on a non-linux key reads red. It is pinned by
   `test_p_matrix_builds_skia_from_the_verified_archive`.
5. **Control 2 re-aimed** (inputs#I7): it now removes the `SKIA_BINARIES_URL` export, the FORCE export being gone. Its
   case, `test_skia_legs_build_from_the_verified_archive`, is named as the plan names it.
6. **The sweep runs under the verified URL** — a fix in a listed file (`ci-leg.sh`), found by gate 5's red. `fast`'s
   closing `target-sweep.py` keep runs called cargo without the variable, so skia-bindings re-ran its build script back
   to the https URL and rebuilt its dependents on every dev-host `fast`. It now runs under `use_verified_skia`.
   `test_fast_leg_sweeps_under_the_verified_archive` pins it, hand-controlled (`evidence/control-sweep-hand.md`).
7. **Order:** steps 5–9 were written before step 4's measurement ran; the plan orders step 4 before them. All five
   archives then measured equal (gate 8 green, `evidence/skia-archives-measured.md`, gate 9 last line 5). The stop
   condition never fired, and nothing after it would have changed.
8. **Cases beyond the plan's list:**
   - `test_skia_archive.py`: a local copy whose sum differs is fetched again · a bad call exits 2 · the record's URL
     is the release template.
   - `test_ci_workflows.py`: a caller's `SKIA_BINARIES_URL` is overwritten (the plan's rejected approach, pinned) ·
     the fast sweep case of item 6.
   - The leg cases assert `FORCE=unset` in place of `FORCE_SKIA_BINARIES_DOWNLOAD=1`.
9. **`run_leg`'s `python3` shim keeps one a test already wrote**, so `test_fast_leg_ends_on_the_font_embolden_witness`
   stays byte-unchanged as step 6 requires.

Scope record: none — `gate.py scope` clean (changed 8 · listed 8 · recorded 0).

## Surfaced
- The plan's premise on `FORCE_SKIA_BINARIES_DOWNLOAD` (Goal, step 5, step 7, Implementation notes, the CI
  acceptance's «Under `FORCE_SKIA_BINARIES_DOWNLOAD` a build asking for another key than the one recorded panics»)
  is false for a crates.io build. It was resolved in this run by the operator's word (inputs#I7); the wrap amends
  what the masters state.

## Gate verdicts (the bare block, `.andromeda/runs/2026-11-21T23-10-57-implement`)
- `python3 .github/scripts/test_skia_archive.py` — green · exit 0 · contains OK
- `python3 .github/scripts/test_ci_workflows.py` — green · exit 0 · contains OK
- `bash .github/scripts/ci-leg.sh audit` — green · advisories ok, licenses ok · lacks warning[yanked]
- `bash .github/scripts/ci-leg.sh fast` — green · exit 0 (584.73 s)
- `grep -l 'FROM: file://' target/debug/build/skia-bindings-*/output` — green · contains target/debug/build/skia-bindings-
- `cargo tree --locked -p seven_guis … | grep -cE '^accesskit_(xplat|winit) '` — green · exit 1 · last line 0
- the advisory-standing count — green · last line 11
- the five archives fetched and verified — green · last line 5
- the measured-lines count — green · last line 5
- `gate.py hygiene`, the push, `ci.py conclusion` — not run — leg operator
- the boundary-audit metric query — not run — defer (gated: the block stands)

Reds recorded before their re-fires: gate 5 read red at firings 1 and 2 on an unchanged tree
(`evidence/red-gate5-firing1.md`); the cause was item 6, fixed before firing 3.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / rustc / clippy-driver (gates 3–6, the fast leg's sub-legs and sweep) | this run | terminated — `gate.py census` reads none |
| skia_archive.py, target-sweep.py, ci-leg.sh | this run | terminated — census reads none |
| rust-analyzer ×2 (pids 1371801, 2956594) | NOT DERIVED — the operator's editors, alive before this run | left running: not this run's — the operator's |

## Decisions & corrections
- The operator's ruling at P1 (inputs#I7): FORCE dropped, `{key}` kept in the URL, the fallback stated as a limit,
  and a CI file-read witness added because it costs nothing.
- Sweep hazard: any cargo run without `SKIA_BINARIES_URL` on the shared `target/` re-runs skia-bindings' build script
  (`cargo:rerun-if-env-changed=SKIA_BINARIES_URL`), and gate 5 reads whichever run came last. The first red was
  attributed to rust-analyzer from a census alone. Cargo's `run-build-script` fingerprints (`{'var':
  'SKIA_BINARIES_URL', 'val': None}`) named the sweep, so the first attribution is withdrawn.
- The testing rule's 2026-11-14 note that a `ci-scripts` control reads INDETERMINATE «no result line» did not recur.
  `gate.py control` v1.38 read both unittest cases `holds`.
