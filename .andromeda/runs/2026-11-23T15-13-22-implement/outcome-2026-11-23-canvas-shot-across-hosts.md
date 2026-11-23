# Outcome — 2026-11-23-canvas-shot-across-hosts

**Outcome:** green (0 fix iterations; every non-operator gate green on its first firing) · smoke skipped — no
boot-path / UI-surface change.

## Files changed
- `tests/blitz-tests/tests/stand_canvas_cross_os.rs` — new, 323 lines: three canvas-only fixtures (`fill`, `edge`,
  `text`), the 12-row digest table `TABLE` (lines 33-47, per `.andromeda/runs/2026-11-23T15-13-22-implement/new-text.md`)
  and five tests.
- `docs/capabilities.md` — §Bundled fonts (146-148, 150), §Stated limits «A shot's PNG bytes» (410-413), §Unmeasured
  (493-494), §Where each claim is proven (607-608).

## Deviations
- none from the plan's steps. The digests were read once on the dev host's per-package build by a transient `eprintln!`
  edit to `reads_its_table`, run with `--nocapture`; the file was restored from its saved bytes and its sha256
  (`6ee680ea…56df`) re-read equal before the literals were written. Every pair of modes read the same digest: the
  whole and element rows are mode-independent on this host.
- scope record: none — `gate.py scope` reads `scope: clean — changed 2 · listed 2 · recorded 0`.

## Gates (`.andromeda/runs/2026-11-23T15-13-22-implement/gate-2026-11-23-canvas-shot-across-hosts.json`)
- `cargo test -p blitz-tests --locked --no-fail-fast --test stand_canvas_cross_os` — green · exit 0 · 1.83s
- `… --features blitz-paint/font-embolden --test stand_canvas_cross_os` — green · exit 0 · 5.06s
- `… --test stand_canvas_shot --test stand_cross_os_readings` — green · exit 0 · 4.94s
- `bash .github/scripts/ci-leg.sh fast` — green · exit 0 · 657.12s
- `bash .github/scripts/ci-leg.sh audit` — green · exit 0 · 1.38s
- `bash .github/scripts/ci-leg.sh stand-repeat` — green · exit 0 · 933.14s
- the hygiene, push and CI-read entries — `not run — leg operator`: the operator pass's.
- summary: `entries 9 · green 6 · red 0 · recorded 0 · timeout 0 · not-run 3`

## Controls (`{work_dir}/evidence/control-1.txt`)
`control: 4 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed` — `fill-one-unit`, `edge-moved`,
`glyphs-resized`, `page-text-beside`, each in place green, removed red, its case read failed.

## Surfaced / stuck
none.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo, rustc, escher-session, stand_canvas_cr | this run (gates, controls) | terminated — `census: none running` |

## Decisions & corrections
none.

## Next
The OPERATOR PASS: entries 7 (`gate.py hygiene`), 8 (the push) and 9 (`ci.py conclusion --sha HEAD --wait 2700`,
the cross-host reading on `Test (macos)`, `Test (windows)` and the linux `test` leg), then `/andromeda-wrap-session`.
