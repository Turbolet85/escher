# Outcome — 2026-11-22-epoch-2-corrective (run 2026-11-22T17-47-29-implement)

**Outcome: green** — the block's second firing reads `entries 20 · green 17 · red 0 · recorded 0 · timeout 0 · not-run 3`
(the three `leg = 'operator'` entries, the operator pass's). Smoke: skipped — no boot-path / UI-surface change.
Trail: `.andromeda/runs/2026-11-22T17-47-29-implement/gate-2026-11-22-epoch-2-corrective.json`.

## Files changed
- new: `packages/escher-driver/src/command_tests.rs` — `command.rs`'s test module moved out whole (619 lines, five `//!` lines added)
- modified: `packages/escher-driver/src/command.rs` — the `#[cfg(test)] #[path = "command_tests.rs"] mod tests;` declaration
- modified: `packages/blitz-dom/src/accessibility.rs` — escher's reads out of `build_accessibility_node` into `write_escher_states` and its five readers
- modified: `packages/blitz-dom/src/events/keyboard.rs` — `close_open_list_before_tab` and `press_control_key`, one call each
- modified: `packages/blitz-dom/src/stylo/presentational_hints.rs` — `list_box_hints`, one call after the attribute loop
- modified: `Cargo.lock` — `url` 2.5.8 → 2.5.9 and `form_urlencoded` 1.2.2 → 1.2.3 alone (4 lines in, 4 out)

## Deviations, with their justification
1. **The lock move (plan step 5) — on the operator's word inputs#I5.** The plan's `cargo update -p url --precise 2.5.9`
   also moves the `syn` edge of `bindgen` 0.73.2 and of `cssparser-macros` 0.7.1 from 3.0.6 to 2.0.119; `bindgen`'s
   `prettyplease` 0.3.0 stays on `syn` 3.0.6, so `bindgen` fails to compile (`error[E0277]: the trait bound
   syn::file::File: syn::parse_quote::ParseQuote is not satisfied`, bindgen-0.73.2/lib.rs:1017). The first firing
   read entries 8–14 red on that error alone (`gate.py show … --n 8`, `--n 13`, `--n 14`). Surfaced; the operator
   ruled «Keep syn edges (Recommended)» against the plan's rejection of a hand-edited lock, with three conditions, all
   met: (1) `cargo metadata --locked` exit 0, lock unchanged, and a full `cargo build --workspace --locked
   --all-targets` under the verified skia URL `Finished` exit 0, no warning, lock unchanged — both before the re-run;
   (2) entry 5's `expect` mended to the four `form_urlencoded` / `url` lines (plan rev 6 → 7), recorded in
   `.andromeda/cache/chunks/2026-11-22-epoch-2-corrective/evidence/entry-5-mend.md`, and it reads green; (3) the hazard
   is reported below.
2. **The control-key call's place (plan step 3).** The plan puts it "after the text-input arm"; that does not compile
   — upstream's arm takes the event by value (`match event`), so a use after the `if` reads a possibly moved value.
   The call stands one statement before the arm, after the target check, in the same `if let Some(node_id)` scope;
   `press_control_key` returns first on an element holding text-input data, and upstream's arm acts only on such an
   element, so exactly one of the two acts on a key, as before. Upstream's text is otherwise unchanged: the diff of
   the function against `547d7514b` is the two call lines alone. `press_control_key` also hoists the "pressed
   `KeyPress`" condition every one of the five arms carried to its head — the same gate, read once.
3. **`list_box_hints` reads every `size` attribute** (plan step 4), as the loop's arm did, instead of `elem.attr`'s first
   — exact behaviour for an element holding two `size` attributes in different namespaces.
4. **Doc comments' spec URLs wrapped in `<…>`** on the moved functions (rustdoc's `bare_urls`); the spec-link text is unchanged.

Scope record: none — `gate.py scope` reads `scope: clean — changed 6 · listed 6 · recorded 0 … · excluded 195`.

## Gates (second firing; the first firing's reds above)
1 `cargo test -p escher-driver --locked --lib command::tests` green · 2 the listing, last line 7, green · 3
`tests_moved_whole.py` `tests moved whole: PASS — 619 lines` green · 4 preservation guard green · 5 lock move (mended)
green · 6 `ci-leg.sh audit` `advisories ok, licenses ok` green · 7 added-lines census `last line 0`, exit 1, green · 8–11
the four per-body behaviour runs green · 12 `ci-leg.sh a11y` green · 13 `ci-leg.sh fast` green (its sweep `swept`) · 14
`ci-leg.sh coverage` green — escher-driver 97.80 % (5653/5780, floor 97), dioxus-native-dom 90.31 %, blitz-test-harness
96.13 %, 3 of 3 at or above · 15 `fn_scores.py` `fn scores: PASS` — 13 · 46 · 167, every fork-own function ≤ 14 · 16
`bars.py` `ours over 800 outside the seven left for a later entry: 0`, `functions over 15: 93 · ours 0`, `bars: PASS`
· 17 stand reading `44 pairs / 372 lines · stand reading bar: PASS` · 18–20 `not run — leg operator`.
No `gate.py guard` entry in the block. No deferral.

## Surfaced / residual candidate (the operator's condition 3)
The `url` update re-resolves two `syn` edges into a lock that does not build. Measured on a scratch clone at
`40ec4749e` (the base lock): `cargo update -p url` — alone, `--precise 2.5.9`, or beside `-p form_urlencoded` — moves
both edges to `syn 2.0.119`, and any build reaching `bindgen` then fails E0277; `cargo update -p log` and `-p regex`
move no `syn` edge; a whole `cargo update` moves every `syn 3` edge, these two included, to `syn 3.0.10` (its build
unmeasured). Reproduce: `cargo update -p url --precise 2.5.9 && git diff -U0 Cargo.lock` (two `- "syn 3.0.6", +
"syn 2.0.119",` pairs), then `cargo build --workspace --locked`. The tree's lock now holds a state cargo's own url
update does not produce, so a later `cargo update -p url` re-breaks it. A residual candidate for the wrap.

## Hygiene before the land
`gate.py hygiene` first read `refused 5 files — P1 5`; after the rewrites below it reads `hygiene: clean — read 228 (runs
193 · evidence 18 · inputs 17) · trails 23 not read · copies 13 not read by P1 — 0 host paths kept · binary 0 not read by P1`.
- respelled `.andromeda/cache/chunks/2026-11-22-epoch-2-corrective/evidence/audit-40ec4749e.log` ×2 (`gate.py respell`)
- respelled `…/evidence/jscpd-20261122T171732Z/jscpd-report.json` ×1184, `…/jscpd-20261122T175306Z/jscpd-report.json`
  ×1184, `…/jscpd-20261122T181925Z/jscpd-report.json` ×1184 (`gate.py respell`; the last two written by this run's
  two firings of the stand duplication entry)
- `…/evidence/audit-url-2.5.9-scratch.log` line 2: the phase scratch clone's temp prefix rewritten to `$TMPDIR/{scratchpad}/`
  (one hit, a cargo-deny warning's file pointer; `respell` takes `in-root` rows alone) — the leg's summary line the plan
  cites, `advisories ok, licenses ok`, unchanged

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / rustc of the gate entries and the build | this run | terminated (census: no `rustc`, `escher-session` or `seven_guis` running; `cargo test` none) |
| `cargo metadata … ~/dev/projects/viola/Cargo.toml` (pid 985451), `cargo metadata … rustlib/src/rust/library/Cargo.toml` (pid 985488) | NOT DERIVED — another project's tooling (ppid 1371801), not this run | left running — not this run's |

## Decisions & corrections
- The operator's word inputs#I5 «Keep syn edges (Recommended)», with three conditions: the lock keeps both `syn` edges at 3.0.6 by edit, measured by `cargo metadata --locked` and a full `--locked` workspace build before the re-run; entry 5 mended in the plan with its evidence; the hazard reported as a residual candidate.
- Correction of this run's own claim: the question card said any `cargo update` re-resolves the two edges; re-measured, only a `url` update does (`-p log`, `-p regex` move none; a whole update moves all to 3.0.10).
- Sweep hazard: a phase control that fires only the audit leg on a lock move proves no build — the P3 measurement of revision 1 never compiled the moved lock.
- Guard refusal: the Bash guard refused a `cd` into the skill's references dir (session cwd move); re-issued as `command grep` on an absolute path.
- Evolve refused one smoke record whose `ts` was typed ahead of the clock (exit 3); re-fired with the clock's `ts`.
