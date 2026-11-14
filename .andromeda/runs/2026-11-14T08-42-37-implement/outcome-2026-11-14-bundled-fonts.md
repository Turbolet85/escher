# Implement outcome — 2026-11-14-bundled-fonts

**Outcome: green** (gate block 7/7 green on its first firing; smoke skipped — no boot-path or UI-surface change).
Next → the OPERATOR PASS: entries 8 (hygiene), 9 (the push) and 10 (the CI read, `verdict: green` — the cross-OS
reading itself), then `/andromeda-wrap-session`.

## Files changed
- new `tests/blitz-tests/tests/stand_cross_os_readings.rs` — 7 tests: the six served tasks held as the session host
  holds them, both layout modes, read at boot and after one step (`click` / `type` / `advance`, never `press`) through
  `snapshot` (every part of Cells' 6), `lint` and `actions`; each answer's `answer_digest` held to one 48-row committed
  table (`ROWS` asserted; the six tasks × two moments × three verbs asserted present). A failure names mode, task,
  moment, verb and part only.
- new `tests/blitz-tests/tests/stand_bundled_font_families.rs` — 6 tests: `serif`, `monospace`, `system-ui`, an
  unavailable family, `cursive` and `fantasy` each read the `sans-serif` line's text extent (width and height) on a
  fixture booted through `fixtures::boot`, both layout modes, the `sans-serif` extent asserted non-zero first.
- no production file, manifest, workflow or committed check (entry 6, the guard probe: green, exit 0).

## Gates (trail `.andromeda/runs/2026-11-14T08-42-37-implement/gate-2026-11-14-bundled-fonts.json`)
```
  1 unit        green · exit 0 · 4.18s · 771 B → 1.log · cargo test -p blitz-tests --locked --test stand_cross_os_r… (65 chars)
  2 integration green · exit 0 · 4.22s · 1273 B → 2.log · cargo test --workspace --locked --test stand_cross_os_read… (97 chars)
  3 integration green · exit 0 · 1.88s · 757 B → 3.log · cargo test -p blitz-tests --locked --test stand_bundled_fo… (69 chars)
  4 integration green · exit 0 · 18.46s · 3721 B → 4.log · cargo test --workspace --locked --test stand_replay --test… (141 chars)
  5 probe       green · exit 0 · 0.01s · 2 B → 5.log · python3 -c "⏎import re⏎n = 0⏎for f in [\"tests/blitz-tests… (327 chars)
  6 probe       green · exit 0 · 0.0s · 0 B → 6.log · git diff --quiet 557f1c98d4e5639ec7e065025d092feda234b3b7 … (167 chars)
  7 integration green · exit 0 · 221.53s · 145079 B → 7.log · bash .github/scripts/ci-leg.sh fast
  8 probe       not run — leg operator (the letter drives it)
  9 probe       not run — leg operator (the letter drives it)
 10 probe       not run — leg operator (the letter drives it)
entries 10 · green 7 · red 0 · recorded 0 · timeout 0 · not-run 3
```
- The digest table was measured under the workspace build (step 3) and reads green under the per-package build too
  (entry 1) — the `writing-mode` feature split moves no digest. The two layout modes read all 48 rows equal (no row
  split, no stop).
- `cursive` and `fantasy` read `sans-serif`'s extent, as predicted: no stated limit, nothing for the residual or the
  incubator line.
- No gate entry is a `gate.py guard` entry (no guard pass line to copy). No deferral: the rust delta ran every gate.

## Controls (step 4) — `{work_dir}/evidence/control-1.txt`, `control-2.txt`
```
control digest-flip · entry 1 · holds — in place green, removed red · exit 0 ✗ (exit 101) · 37.67s · 2.65s → 1.without-digest-flip.log
control font-reach · entry 1 · holds — in place green, removed red · exit 0 ✗ (exit 101) · 37.67s · 2.71s → 1.without-font-reach.log
control serif-append · entry 3 · DOES NOT HOLD — in place green, removed green · 1.06s · 3.37s → 3.without-serif-append.log
control families-host · entry 3 · holds — in place green, removed red · exit 0 ✗ (exit 101) · 56.98s · 2.94s → 3.without-families-host.log
```
- digest-flip: red naming `incremental=false counter boot snapshot part 0`.
- font-reach (host system fonts in place of the bundled font): red on the first `snapshot` row at boot of all six
  tasks, as predicted.

## Deviations from plan, with justifications
1. **The plan's families control does not hold, and a substitute was fired.** Step 4's control for the families check
   (`GenericFamily::Serif`'s append removed from `build_single_font_ctx`) reads DOES NOT HOLD. With the append removed,
   `serif` still lays out in DejaVu Sans: on a collection holding one font, fontique's fallback reaches that font for a
   generic with nothing registered. This is also why `cursive` and `fantasy`, never registered, read `sans-serif`'s
   extent. The check holds the *reading* the acceptance asks for, not the *registration*. The substitute control
   `families-host` (the stand's font context switched to the host's system fonts, `control-2.toml`) turns `serif`,
   `monospace` and the unavailable family red. `system-ui`, `cursive` and `fantasy` stay green there, because this
   host resolves them to its sans face.
   → **A premise for the wrap (implementation reality, not a gap the code can close):** the plan's Goal/step 2 and the
   expected design-system amendment give the reason «`build_single_font_ctx` appends the one font to those three
   generics». The measured reason is wider: on the bundled set every generic, registered or not, reaches DejaVu Sans,
   because it is the collection's only font. The acceptance text (design) still holds as written. Its stated reason
   should be amended to the fallback reading by the wrap's fan-out. Nothing was authored here.
2. **A scratch measurement line.** Step 3's digests were read from a temporary `eprintln!` (and a temporary skip of the
   table compare) in the check, run once under the workspace build, then removed. The committed file prints nothing
   (entry 5: `0`).
3. The first `control-1.toml` was refused as not TOML (a newline in a `'…'` literal); it was re-written with a `'''`
   literal and re-fired. Nothing ran on the refused file.

Scope record: none needed — `gate.py scope` reads `scope: clean — changed 2 · listed 2 · recorded 0`.

## Matrix
- `v030-14` → `implemented`, ref `tests/blitz-tests/tests/stand_cross_os_readings.rs` (the cross-OS witness on the
  test leg, «Test (windows)» and «Test (macos)» of the operator's CI read). Its acceptance needs the CI read
  (entry 10): the cap is proven only when that run reads green on all three OSes. A digest that reads differently on
  macOS or Windows is v030-14's outcome failing — a finding for the wrap, never a re-baselined table.

## Surfaced / stuck
- Nothing surfaced as a blocking gap; nothing stuck. The premise correction in deviation 1 goes to the wrap.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / rustc / test binaries of entries 1–7 and the four control firings | this run (gate tool) | terminated — `gate.py census --comm cargo --comm rustc --comm escher-session --args stand_cross_os --args stand_bundled_font`: `census: none running` |
| control clones `~/.cache/andromeda-control/2ac18a16`, `99ea1cc6` | this run (gate.py control) | removed (`clone removed` on both summaries) |

## Decisions & corrections
- The operator relayed (inputs#I5): a `gate.py control` `case` is the test name alone, never prefixed by the binary id.
  The controls already met it, so nothing changed.
- Bash-guard refused a `cd` into the session scratchpad (it moves the session cwd). It was re-issued as a script run by
  absolute path.
- The operator's take-up directive stands (founder away; a fork goes to founder-forks.md, no dialog): no fork arose in
  this run, and no founder-forks.md was written.
