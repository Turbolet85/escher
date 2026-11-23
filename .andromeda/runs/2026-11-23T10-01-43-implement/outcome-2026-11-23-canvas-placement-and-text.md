# Outcome — 2026-11-23-canvas-placement-and-text

**Run:** 2026-11-23T10-01-43-implement · **Outcome:** green, with one red that is not this chunk's (the audit leg, RUSTSEC-2026-0594). The smoke was skipped: no boot path changed and no self-verify is available.

## Files changed
- New: `tests/blitz-tests/tests/stand_canvas_placement.rs` · `tests/blitz-tests/tests/stand_canvas_pointer_frame.rs` · `tests/blitz-tests/tests/stand_canvas_text.rs`
- Modified: `packages/blitz-paint/src/lib.rs` · `packages/blitz-paint/src/render.rs` · `packages/blitz-dom/src/events/mod.rs` · `packages/blitz-dom/src/node/custom_widget.rs` · `packages/blitz-dom/src/document.rs` · `tests/blitz-tests/Cargo.toml` · `Cargo.lock`
- Scope: `gate.py scope` printed `scope: clean — changed 10 · listed 10 · recorded 0`, so the scope record has no lines.

## Gates (by `run`)
- `cargo test -p blitz-dom --locked --no-fail-fast` — green
- `cargo test -p blitz-paint --locked --no-fail-fast` — green
- `… --test stand_canvas_placement` — green, `3 passed`. Read red before step 2 (`evidence/red-placement.log`): paint handed `(130, 90)`, the third test green.
- `… --test stand_canvas_pointer_frame` — green, `1 passed`. Read red before step 4 (`evidence/red-pointer-frame.log`): it heard `(42, 42)` and `(112, 72)`, page = client, exactly the prediction. Firings 2 and 3 read the same red after step 4's edit: they ran on a stale blitz-dom build, the second with no edit between (recorded below).
- `… --test stand_canvas_text` — green, `1 passed`. Read red before step 7 (`evidence/red-text.log`): «the widget is handed a font context before it paints». Firings 1–3 were compile errors: two came from the stale blitz-dom build, the third from the test calling `font_ctx()` on `DioxusDocument` (fixed through `Document::inner()`).
- `… --features blitz-paint/font-embolden --test stand_canvas_text` — green
- `… --test stand_canvas_shot --test custom_widget_layout` — green
- `cargo test -p blitz-tests --locked --no-fail-fast` — green, 336.67 s
- `git diff --numstat d43bc0e7f… -- Cargo.lock | awk …` — green, last line `1+0`
- `bash .github/scripts/ci-leg.sh fast` — green, 506.46 s (firing 2). Firing 1 was stopped on the operator's hold (inputs#I6); a killed call leaves no trail record. The fast leg's last line, the dev-host sweep: `{"verb": "run", "outcome": "swept", …}`.
- `bash .github/scripts/ci-leg.sh doc` — green, 99.23 s
- `bash .github/scripts/ci-leg.sh audit` — **red — not this chunk's**: `advisories FAILED, licenses ok`, `RUSTSEC-2026-0594` on `cookie 0.18.2`. A clone of the parent `d43bc0e7f` reads the same (`evidence/audit-base.md`). The owner is the operator, who answers it on the card (inputs#I7); the lock is not moved in this run.
- `bash .github/scripts/ci-leg.sh stand-repeat` — green, 919.8 s
- The hygiene, push and CI-read entries are `leg = 'operator'`: not run here, they belong to the operator pass.

## Controls (step 8)
`gate.py control` with `control-1.toml` (kill 1200 s) printed `control: 4 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed` (`evidence/control-1.txt`): `paint-size`, `paint-origin`, `pointer-frame` and `font-handoff` each hold.

## Deviations
- `stand_canvas_text` reaches the document's font context as `harness().doc.inner().font_ctx()`. The plan says "the harness document's `font_ctx()`", but `DioxusDocument` has no such method; the base document is reached through the `Document` trait.
- `stand_canvas_placement` reads the handed size after the shot and before the pixels, in one helper `assert_paint`. The handed size is written only at paint time.
- Step 4's escher function takes the node and `pos` (`custom_widget_content_origin(node, pos)`). It is called once, just before the widget arm, because the arm holds `node` mutably borrowed. The arm passes the result in place of `pos`; the sub-document arm is untouched.
- A `cargo clean -p blitz-dom` (2.8 GiB removed) cleared a stale build. Cargo had read blitz-dom `Fresh` after the step 4 and 5 edits, even after a `touch`. The editor's rust-analyzer `cargo check --workspace` runs on the shared `target/`.
- The red re-fires with no edit between them were not written to a file before the re-fire, as the rule asks: gate 4 firing 3 heard `(42, 42)`/`(112, 72)` and gate 5 firing 2 read `E0407: method fonts is not a member of trait Widget`. They are recorded here, late, and the later greens do not erase them.

## Surfaced / stuck
- Nothing surfaced or stuck. For the operator's card: **RUSTSEC-2026-0594** — «Private cookie nonces can repeat in processes that fork». `cookie 0.18.2` reaches the lock through `cookie_store 0.22.1` → `reqwest 0.13.5` → `blitz-net` (and `http-cache-reqwest`, `reqwest-middleware`), then into blitz, dioxus-native, browser, rdme, seven_guis and blitz-tests. The advisory's fix: upgrade to `>=0.18.3` (`cargo update -p cookie`).

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / rustc of the gate entries, ci-leg.sh legs | this run (the gate tool) | terminated. `gate.py census --comm cargo --comm rustc --comm ci-leg.sh --comm cargo-deny --comm escher-session` printed `census: none running` |
| the fast leg's first firing (ci-leg.sh and its cargo) | this run | terminated: TaskStop on the operator's hold; census then showed no ci-leg.sh |
| `cargo check --workspace` (pid 2979501) with rustc children | not this run: the editor's rust-analyzer (ppid 2956594) | not ours; gone at the final census |
| the control clone `~/.cache/andromeda-control/d16a55bc` | this run (gate.py control) | removed (`clone removed`) |
| the scratch clone `auditclone` in the session scratchpad | this run | left in the scratchpad (files only, no process) |

## Decisions & corrections
- The operator's hold (inputs#I6): stop the gate run in flight and start no cargo. Done: the fast leg was stopped and entries 10 and 13 were owed.
- The operator's go (inputs#I7): re-fire with `--only 10,13`, then the step 8 controls, then finish. Do not move the lock for the audit red; put RUSTSEC-2026-0594 on the card. The step 8 controls had already read 4 holds on this same tree, which has not been edited since, so they were not re-fired.
- Sweep hazard: cargo can read a crate `Fresh` against edited sources when the editor's own cargo shares `target/`. Neither a `touch` nor a re-run cleared it; `cargo clean -p {crate}` did.
- Guard refusals, each re-issued once: a `cd` into the skill's references directory (bash-guard) and a `cat` heredoc writing a scratch script (bash-guard).
