# Outcome — 2026-11-23-canvas-placement-and-text

**Run:** 2026-11-23T11-22-00-implement · **Outcome:** green. The run re-entered the revised plan (rev 5, revision 1) at step 10. The smoke was skipped: no boot path or UI surface changed, and no self-verify is available.

## Files changed (this run)
- Modified: `Cargo.lock`. Its `cookie` entry moves from `version = "0.18.2"` to `"0.18.3"` (line 2042), and the checksum moves from `1a373e36…f2f87` to `d55f2d6c…615f8a` (line 2044). The edit was made with the Edit tool, those two lines and nothing else. No `cargo update` was run, no manifest line changed and no `deny.toml` line changed.
- The chunk's whole changed set, steps 1–9 from run `2026-11-23T10-01-43-implement` included, is 10 files: 3 new and 7 modified.
- Scope: `scope: clean — changed 10 · listed 10 · recorded 0 (companion 0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 339`. The scope record has no lines.

## Gates (step 10: 9, 14, 12, fired one entry per call in that order)
- `git diff --numstat d43bc0e7f7f9c992760230e71eeba19c708935e8 -- Cargo.lock | awk '{print $1 "+" $2}'` — green · exit 0 · last line `3+2`, 0.01 s.
- `bash .github/scripts/ci-leg.sh build` — green · exit 0 · lacks `warning:`, 9.37 s. Its log shows `Compiling cookie v0.18.3`, then `cookie_store v0.22.1`, `reqwest v0.13.5`, `blitz-net` and every workspace crate. Last line: `Finished dev profile … in 9.22s`.
- `bash .github/scripts/ci-leg.sh audit` — green · exit 0 · last line `advisories ok, licenses ok`, 1.18 s. This clears RUSTSEC-2026-0594, the red run `2026-11-23T10-01-43-implement` recorded.
- After every `--locked` run the lock still reads `3+2` against `d43bc0e7f`.
- Not re-fired, by the plan's step 10 rationale: entries 1–8, 10, 11 and 13 (unit, stand, `fast`, `doc`, `stand-repeat`). They stand green as `2026-11-23T10-01-43-implement` recorded them, on a tree that differs only by the two `cookie` lines.
- The operator entries, not run here: 15 (hygiene), 16 (push) and 17 (CI read). They belong to the operator pass.

## Deviations
- none in this run. The earlier run's deviations stand in its outcome file and the report.

## Surfaced / stuck
- none.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / rustc / cargo-deny of entries 14 and 12 (ci-leg.sh) | this run (the gate tool) | terminated. `gate.py census --comm cargo --comm rustc --comm ci-leg.sh --comm cargo-deny --comm escher-session` printed `census: none running` |

## Decisions & corrections
- Guard refusal, re-issued once: bash-guard refused a `cd` into the skill's references directory (its cd arm). It was re-issued with absolute paths.
- Residual `res-evkq636g` gains its second instance here: the `syn` edges of `bindgen` and `cssparser-macros` stay at 3.0.6 because the lock was edited by hand, not re-resolved (inputs#I10).
