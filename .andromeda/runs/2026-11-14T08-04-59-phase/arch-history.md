@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — (headless stand surface, HarnessOptions fields) added `HarnessOptions::font_ctx` → `DocumentConfig.font_ctx` with `Default` leaving it `None`, the stand's `font_ctx()` (bundled DejaVu Sans, system fonts off), the crate-root `DEJAVU_SANS` shared with the wasm entry and compiled on native too, and seven_guis' native `dioxus-native` features `system-fonts` + `woff`. This is the exact surface the chunk extends: a harness-built document that names no font context still falls to `None` → system collection, the gap the scope says P3 must measure (decision/stand-uses-bundled-font, decision/woff-default-in-dioxus-native, contract/headless-stand/boot).

@history_item n=2 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** — (the stand's typography in the Headless stand contract, PROVISIONAL) stated the stand's typography as DejaVu Sans, its one bundled font, in the Headless stand contract and not on design-system's product surface: `font_ctx()` registers it for every generic family with system fonts off, every stand boot holds it through `HarnessOptions.font_ctx` (`boot_cells`, `boot_load` included), seven_guis' native `woff` decodes it. The chunk's "generic families reach the bundled set" claim rests on this row (decision/stand-typography-dejavu-sans); widening it beyond the stand changes where the typography is stated.

@history_item n=3 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (the stand's typography, F2 ratified) dropped the OPEN marker and ratified by the founder's own word that the stand's typography lives in the Headless stand contract with DejaVu Sans registered for every generic family. Any move of the bundled set off the stand (into the document's font context for every run path, or for a future canvas reader) amends a founder-ratified row, so it needs his word, not a delegate's.

@history_item n=4 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — (the headless stand serves Cells and a load page) widened `seven_guis::stand` to boot Cells (`boot_cells`) and the load page (`boot_load`) in the pinned 800x600 Light viewport "with one bundled font", and made `escher-session` serve six tasks through the same boot. These are the run paths (stand boot, served tasks, `mcp` role, coverage role) the chunk must prove all read the bundled set; `boot_load` swaps `net_provider` for `LoadNet`, a precedent for a stand option overridden per boot (contract/headless-stand/boot, decision/stand-load-page-in-process-url).

@history_item n=5 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — (the `screenshot` verb, `fonts-unavailable`) introduced the `fonts-unavailable` cause: a font context with no `sans-serif` family (`can_paint_text` false) is refused before anything is painted. The scope keeps this cause and its text unchanged; a bundled set registered for every generic family must still leave `can_paint_text` meaningful for an app with no decodable font (contract/refusal-causes/fonts-unavailable, contract/target-check/screenshot).

@history_item n=6 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — (the harness's screenshot, `can_paint_text`) added `ScreenshotError::FontsUnavailable` to the harness and the public read-only `BaseDocument::can_paint_text` (a `sans-serif` family resolves) and `layout_as_text_input`. The probe the chunk's `fonts-unavailable` guarantee rests on; changing how a generic family resolves (fontique mapping onto a lone registered font) changes what this reader answers (contract/base-document/can-paint-text, contract/screenshot-error/fonts-unavailable, contract/document-core/can-paint-text).

@history_item n=7 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — (scopes and tests) registered `stand_screenshot_repeat`: one state's shot byte-identical across two child runs and the parent — on one host. The precedent for the chunk's repeatability proof and the reason the cross-OS proof cannot reuse it.

@history_item n=8 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — (the replay API and the shot's same-host limit) restated `stand_screenshot_repeat`'s byte-identity as one-host only: a shot's PNG bytes differ between linux x86_64 and macOS aarch64 (CI#39225375577), so a check compared byte for byte across OSes holds no `screenshot` step; `Check::replay` compares each step's live answer to the kept one. The direct constraint behind the scope's "never compared by shot bytes" and a candidate mechanism (a kept check replayed per OS) for holding the cross-OS comparison in CI (decision/no-screenshot-in-cross-os-byte-check, contract/check-replay/*).

@history_item n=9 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — (each kept answer whole at rest, the committed checks) committed `tests/blitz-tests/checks/counter-cli.json` and `counter-mcp.json` with whole answers (snapshot text, diffs, bounds) and no `screenshot` step, kept fresh by seven_guis `recording` and read by `stand_replay`. The existing cross-host reading carrier: if the chunk holds its proof as a committed reading every CI OS replays, it extends this owner-only exception (decision/committed-stand-checks, decision/kept-checks-owner-only), and any new committed check is a widening of that exception.

@history_item n=10 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — (the kept check) established the kept check as an owner-only, unix-only, std-only replayable record, its answers masked and never logged. Constrains how a cross-OS comparison can be kept: Windows has no recording (decision/cross-process-session-unsupported-on-windows), so a Windows leg of the proof cannot rest on a recorded session (decision/recorded-session-kept-check, contract/kept-check/*).

@history_item n=11 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — (Established Decisions: 4 rows) added `decision/default-font-fallback`: when none of an element's listed families is available the default font is used, for text and for `ch`/`ex` metrics, an upstream behaviour change ratified for its take. This is the engine path an unknown family or `serif`/`monospace` hits when only DejaVu Sans is registered; the chunk's generic-family claim must be measured against it.

@history_item n=12 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — (Parley's pin form and the merged dependency set) moved parley to the registry "0.12" and kept the [Dependency pinning] coupling: parley, skrifa and vello move together. Under the scope's "no new dependency crate" boundary, any font handling must stay inside parley/fontique/skrifa/wuff as pinned (decision/parley-text-layout, decision/font-deps-match-renderers, decision/skrifa-matches-parley-vello).

@history_item n=13 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — (`text-transform-icu` in four default lists) restated the per-crate default feature lists — blitz-dom's svg, woff, accessibility, system-fonts, … — so `system-fonts` and `woff` are defaults set per crate. Turning system fonts off for a run is a font-context choice, not a feature-list change, unless the chunk takes a fork on a crate's defaults (decision/per-crate-default-features, decision/skipped-font-formats, decision/woff-default-in-dioxus-native).

@history_item n=14 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — (Document core: the document's clock) added `DocumentConfig.clock`, `None` in `Default`, a document falling back to `WallClock` — an API addition whose default kept every windowed and embedder-built document's behaviour, flagged upstreamable, ratified at the epoch close. The closest precedent for pushing a determinism input through `DocumentConfig` (contract/document-core/documentconfig) without changing remote pages or the browser app, which the scope's boundary requires.

@history_item n=15 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — (Test harness: the harness's time is the documents' clock) made `HarnessOptions::into_config(self, &VirtualClock)` inject the harness's clock into every document `from_html_with`/`from_vdom` build, while `wrap(doc)` keeps the wrapped document's own config. The pattern (and the `wrap` exception) for making a harness-built document that names no font context read the bundled set rather than the host's.

@history_item n=16 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — (Driver session: eight causes) closed `Cause` at eight with `fonts-unavailable` the eighth, and restated `screenshot`'s help: two shots of one state with no `advance` between them read the same pixels. Keeping `fonts-unavailable` keeps the set at eight; no cause may be added (decision/refusal-causes-closed-at-eight).

@history_item n=17 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — (Standard Contracts: 3 rows) the sibling v030 determinism chunk added `contract/test-harness/run-seed` (the harness supplies a run seed to every Dioxus document it builds) beside `SeededRandom`/`use_seeded_random`. Direct precedent for "the harness supplies the run's determinism input to every document it builds", the fonts analogue this chunk lands.

@history_item n=18 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — (Standard Contracts: no seed on the carriers) recorded `decision/no-seed-on-carriers`, PROVISIONAL: no seed crosses any verb, argument, flag, MCP field, kept-check key or span field, a carrier field being a session-boundary widening. Same shape as this chunk's boundary (no new carrier, verb, argument, field, cause, log field or span): a font choice on a carrier would be a widening and a founder fork.

@history_item n=19 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — (Stack and Technologies: no PRNG crate) recorded `decision/no-prng-crate`: a crate was deliberately not taken for the determinism capability. Precedent for the scope's "no new dependency crate; the font asset is a file already in the tree".

@history_item n=20 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — (the driver session registered) stated that the session library takes the app's boot from its caller and depends on no app. The bundled font therefore cannot be supplied by escher-driver itself; it has to come through the caller's boot (the stand) or the harness/document defaults (decision/session-library-depends-on-no-app, decision/driver-library-side-effect-free).

@history_item n=21 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — (the third blitz-tests shared module) created `tests/blitz-tests/tests/fixtures/mod.rs`, the one statement of the fixture boot and hold and of the shared fixture pages. One of the run paths the scope names (the shared fixture pages already build `build_single_font_ctx(DEJAVU_SANS)`); any change to the fixture boot's fonts lands here, once (decision/fixtures-test-module, decision/shared-fixtures-stated-once).

@history_item n=22 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — (`Parallel` style traversals serialised) recorded that no headless document — harness, stand or driver — sets `style_threading`, while the windowed launch and the browser do. Precedent for a rule held across every headless document path, and for keeping the windowed launch and browser on their own settings (decision/headless-documents-default-threading, decision/headless-documents-sequential-style).

@history_item n=23 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — (the default sheet from the config) made `DocumentConfig::default()` the one source of the default user-agent sheet, a config that sets it used as given. Precedent for a document default living in `DocumentConfig`'s hand-written `Default` (decision/ua-stylesheets-used-as-given), relevant if the bundled set becomes a config default rather than a stand-local constant.

@history_item n=24 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — (the in-process visual lint and the text extent reader) made `Session::lint` refuse `fonts-unavailable` before reading and added `BaseDocument::text_extent` with its limits (RTL and vertical text as LTR lines). Text extents and lint findings are among the readings the chunk compares across OSes; both are font-metric driven (contract/basedocument-readers/text-extent, contract/lint-verb/fonts-unavailable).

@history_item n=25 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — (the gated part: `lint` the twelfth verb) put lint findings on every carrier and in kept checks, refused `fonts-unavailable` like `screenshot`, with no new carrier, exit, cause, field or dependency. Lint findings are a cross-OS reading the scope names, already able to cross through a kept check (decision/lint-findings-on-carriers, contract/lint-verb/wire-form).

@history_item n=26 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — (the snapshot's text form and its size budget) fixed the snapshot text line as role, name, id, state tokens and `@x,y wxh` in `f64` Display form, with `SNAPSHOT_TEXT_BUDGET` 10,000 bytes. The snapshot text with its bounds is the primary reading compared across OSes; any font-metric difference shows up in those numbers (contract/snapshot-text/line-fields, decision/snapshot-text-budget).

@history_item n=27 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — (fork CI through one leg script) set that the matrix tests windows and macos (and only builds ios and android) while linux is tested by the fast `test` leg. The scope's "every CI OS" proof spans these two job shapes; a cross-OS check must run in both the `test` leg and the matrix (decision/linux-tested-by-fast-leg, decision/cargo-legs-locked).

@history_item n=28 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — (legs, the new job) added `coverage_floor.py` floors (escher-driver 97, dioxus-native-dom 83, blitz-test-harness 90 %) and counted nine `libfontconfig1-dev` jobs. A harness change to default fonts lands under the 90 % blitz-test-harness floor; `libfontconfig1-dev` is the CI install behind system-font discovery the chunk turns off for runs (decision/per-crate-coverage-floors).

@history_item n=29 marker=2026-10-09-audit-corrections-agent-surfaces
**2026-10-09-audit-corrections-agent-surfaces** — (fork CI's package install) made the bounded `apt-install.sh` the one path installing `libfontconfig1-dev` on CI. Relevant if the chunk's proof removes or keeps the system-font dependency on CI runners.

@history_item n=30 marker=2026-10-05-as-built-baseline
**2026-10-05-as-built-baseline** — (measured dev-host build environment) recorded the dev host's Arch `fontconfig` 2.18.3 standing in for CI's `libfontconfig1-dev`. The host-font baseline that differs between the dev host and the CI runners, the variance this chunk removes from a run.

@history_item n=31 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (no local gate builds `font-embolden`) recorded that only CI's macOS leg built `blitz-paint/font-embolden` after a local block read green over a macOS-only paint defect (a synthesized bold). A known per-OS font-paint divergence the cross-OS reading proof must account for.

@history_item n=32 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the `embolden` step of `fast`) made the `embolden` step the local witness (`--features blitz-paint/font-embolden --test select_popup_painted`) reproducing macOS's two readings on linux. Precedent for reproducing an OS-specific font reading on another host without a CI job (decision/font-embolden-local-witness).

@history_item n=33 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — (the capability statement) made `docs/capabilities.md` escher's statement of what it can do and its limits, its names and five counts pinned by `test_capability_statement.py`. If v030-14 adds a capability line or states a font limit (canvas text as a future reader), it goes here; its counts must not move since no verb or cause is added (decision/capability-statement).
