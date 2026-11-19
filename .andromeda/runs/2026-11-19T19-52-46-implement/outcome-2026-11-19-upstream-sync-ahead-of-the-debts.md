# Outcome — 2026-11-19-upstream-sync-ahead-of-the-debts

**Run:** 2026-11-19T19-52-46-implement · **Outcome:** green — the operator pass next (gate entries 11 hygiene, 12 the push after the merge commit, 13 the CI read, 14 the two-parent check), then the wrap.

## What was done
- `git -c rerere.enabled=false merge --no-commit --no-ff 547d7514ba2adb357d41d5e4655a63ddd5cdc5ec` on `build/escher-0.4.0` (`d9af170dc`): 52 conflicted files, all resolved; `MERGE_HEAD` stands staged for the operator's pre-CI commit.
- The founder's W1–W9 (inputs#I4, all option A) applied: W1 the CDP crate in, `devtools` and `embedded-devtools-frontend` out of `apps/browser`'s default (no default turns devtools on); W2 `packages/parley_code_editor` out of the members and the tree; W3 rustls taken; W4 skia default taken in `apps/browser` and `wpt/runner`, its build-time fetch measured (`evidence/lock.md`); W5 the event queue taken with the two silences kept (`clear_focus_silently` for the `inert` fix-up, `set_focus_silently` for the driver's select focus); W6 `isTrusted` taken; W7 the cancelled-pointerup click taken; W8 SVG presentation and `hr` hints taken, the mask reach measured (no fetch round the stamping constructor; a fragment `url(#m)` fetched as a stamped `data:` URL, recorded and flagged upstreamable); W9 `visibility: collapse` taken, a select's option still kept under `display: none`.
- Coupled set taken (Stylo 0.24 · selectors 0.43 · parley 0.13 · skrifa 0.49 · winit beta.4 · svgtypes 0.16.2), Dioxus held at 0.7.3 (`evidence/limits.md`).
- Escher's surfaces carried onto upstream's code: the changed set behind `has_changes` / `clear_changes` and the shell's post-resolve refresh; the `click_*` arms, select / range / typeahead / list keys on the event queue; the stamped request; escher's builder with W9; the select `size` hint into `stylo/presentational_hints.rs`; `RunPaint` / `stroke_text_as` with the synthesized bold onto upstream's `text.rs`; the select list's wheel into `events/wheel.rs`; the document clock in the smooth-scroll progress.
- An upstream defect found and fixed (inputs#I7, inputs#I8): parley 0.13 drops an in-flow inline box whose U+FFFC placeholder no font can shape; `layout/inline/build.rs` `push_inline_box` falls back to a private family registered in every document (`PLACEHOLDER_FONT_FAMILY`); witness `document_inline_box_placeholder_tests.rs`, red on upstream `547d7514b`, flagged upstreamable.
- Four ruled lock updates (inputs#I6): `pulldown-cmark` 0.13.6, `rustls` 0.23.46, `rustls-webpki` 0.103.16, `webpki-root-certs` 1.0.10.

## Gates (the plan's `## Test Commands`, by `run`)
First firing (red): `bash .github/scripts/ci-leg.sh fast` red · exit 101 (`font_face_sniffed_source`); `bash .github/scripts/ci-leg.sh audit` red · exit 1 (RUSTSEC-2026-0414, -0433, -0346, -0347, -0348, -0349, -0382, -0383). Recorded before the re-fire; both fixed (above).
Second firing (`--only 1..10`), the standing verdicts:
- `test "$(git rev-parse MERGE_HEAD)" = 547d7514ba2adb357d41d5e4655a63ddd5cdc5ec` — green · exit 0
- `git grep -n -I -E '^(<<<<<<<|>>>>>>>) ' -- . ':(exclude)*.md'` — green · exit 1, no output
- `test ! -e AGENTS.md` — green · exit 0
- `grep -c '^source = "git+' Cargo.lock` — green · last line 1
- `bash .github/scripts/ci-leg.sh build` — green · exit 0
- `bash .github/scripts/ci-leg.sh fast` — green · exit 0 (668 s)
- `bash .github/scripts/ci-leg.sh doc` — green · exit 0
- `bash .github/scripts/ci-leg.sh a11y` — green · exit 0
- `bash .github/scripts/ci-leg.sh audit` — green · exit 0 (`advisories ok, licenses ok`; re-read again after the `deny.toml` reason wording: the same)
- `bash .github/scripts/ci-leg.sh stand-repeat` — green · `stand-repeat: 3 of 3 rounds green`
- entries 11–14 (`gate.py hygiene`, the push, `ci.py conclusion`, `HEAD^2`) — `not run — leg operator`, the operator pass's.
- `stand_cross_os_readings` (the committed digest table of the seven served tasks) green in `fast` and in each of the three `stand-repeat` rounds: no digest moved with the inline-box fix, as the operator asked (inputs#I8).
- Smoke: `bash scripts/agent-run.sh boot` ready, `run session` 2 of 2 passed; the windowed `seven_guis_native` ran 15 s under `timeout -s TERM -k 5 15` (exit 124), no panic, no output, no survivor.

## Deviations (with justifications)
- The four precise lock updates past upstream's lock, against the plan's «no `cargo update`» — the operator's ruling on the audit red (inputs#I6), each advisory id in `evidence/lock.md`.
- The lock settled by `cargo metadata` (the same resolver step) in place of the plan's unlocked `cargo check`, then checked `--locked`.
- The shell's accessibility refresh moved from `View::poll` to `View::redraw` after resolve (upstream's site), reading the changed set; the plan's step 4 left the site open.
- `examples/seven_guis/src/app.rs` kept as HEAD's (`6dd69eb1c`'s stand hunk left out — it does not apply to the stand's `app()` shape).
- The engine fix for the inline-box placeholder (`layout/inline/build.rs`, `document.rs`'s registration, `lib.rs`'s family constant) and its witness, on the operator's word (inputs#I7, inputs#I8) — wider than the "mark the ancestors" first proposed: the cause was parley 0.13's placeholder, not a stale layout, and the fallback font had to be registered in given contexts too.
- `.github/scripts/test_blitz_tests_targets.py` unchanged: it pins the manifest's shape, not a target list, so upstream's eight new files are targets with no edit.
- `deny.toml`'s ignore reasons re-worded to the merged chain (`winit 0.31.0-beta.4`, `accesskit_unix 0.24.1`); the ignores themselves unchanged.
- The temporary sniff probe target and the temporary `eprintln!` lines in blitz-dom (located the cause) removed; each instrumented file restored byte-identical (sha256).
- Scope record (`gate.py scope`: `scope: clean — changed 185 · listed 182 · recorded 3`): mechanical — `CONTRIBUTING.MD`, `docs/licensing.md`, `justfile`, each serving step 3 (upstream's clean takes), self.

## Surfaced
- None open. Measured facts carried for the wrap: the W8 fragment `data:` fetch (upstreamable); the inline-box placeholder defect (upstreamable, fixed here); upstream's `docs/licensing.md` row says `webpki-root-certs` is depended on "only for Wasm targets", while the audit's tree reads it ← `rustls-platform-verifier 0.7.1` ← `reqwest` (native graph).

## Process hygiene
| Process | Started by | Final state |
|---|---|---|
| `seven_guis_native` (windowed smoke) | this run | terminated (timeout 15 s, exit 124) |
| `escher-session` hosts of `cli_flow` | the harness for this run | terminated (census: none running) |
| cargo / rustc of the gate legs, the probes and the upstream clone's check | this run | terminated |
| `cargo check --message-format=json --keep-going` (pid 2541499) with two `rustc` children | rust-analyzer (pid 3998868), not this run | left running — the editor's flycheck; the editor stops it |
Census: `gate.py census --comm seven_guis_native --comm escher-session --comm cargo --comm rustc --args upstream-547d` → `census: 3 running`, the three rows rust-analyzer's.
Left on disk for the operator: `target/tmp/upstream-547d` (the upstream clone and its target dir).

## Decisions & corrections
- The operator held the host for another project after the first gate block: no cargo call until «host free» (his message, this session).
- Ruling inputs#I6: the four precise lock updates; the sniff failure located before any fix.
- Ruling inputs#I7: `--no-fail-fast` once; confirm on an upstream clone; fix in the engine, own blitz-dom test, upstreamable; the sniff test unchanged.
- Ruling inputs#I8: option 1 (the placeholder fallback); prove no move where a font exists; W8's fragment fetch recorded as measured, upstreamable.
- Recurrence of the host-linux rerere learning: `git apply --3way` during the merge wrote preimages to the shared rr-cache; cleared with `git rerere clear`, later applies under `-c rerere.enabled=false`.
- Guard refusals, each re-issued once: a `cd` into the scratchpad copy of the old sync, a `cd` into the upstream clone (run with `--manifest-path`), a pipeline tool's output piped into `sed`.
- Sweep hazard: a test target with no tracing subscriber and blitz-dom's `tracing` feature off prints nothing under `RUST_LOG=trace`; a failure inside the engine is located with temporary prints, restored by hash.
