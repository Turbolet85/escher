@history_item n=1 marker=2026-11-22-upstream-sync-ahead-of-the-canvas
**2026-11-22-upstream-sync-ahead-of-the-canvas** (Standard Contracts; Established Decisions ×2) — the last sync, whose base `165d698c3` is this chunk's merge base: `contract/layout/stylo-taffy-exports` gained the ltr flag on `convert::float`/`clear`, and `decision/logical-float-clear-by-direction`, `decision/block-containment-through-display-contents` and `decision/textarea-text-laid-out-once-width-known` landed as upstream layout behaviour ratified for that take. This is the direct precedent for how a take's upstream behaviour changes enter as rows. `f559f3fe2`'s `<br>` change in `construct.rs` and `2e9fd4af1`'s `stylo_taffy/src/convert.rs` edits fall on these same surfaces.

@history_item n=2 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (Project Intent; Stack; Established Decisions; Cross-cutting; Occupied Resources; Inherited Defaults; Conventions) — `decision/upstream-sync-by-two-parent-merge` replaced the cherry-pick rule, so each left-out commit is named with its reason. `decision/dioxus-0-8-move-held` became heir to the Stylo/Dioxus deferral. Also landed: Stylo 0.24, parley 0.13, winit `=0.31.0-beta.4`, reqwest on rustls, and the setup-ndk exception on `decision/one-apt-install-script`. `decision/events-queued-on-document` replaced the dispatch callback, which bears on the new drag events in blitz-dom's `events/driver.rs`. The opt-in devtools CDP listener became an `except` on `decision/one-local-unix-listener`, `decision/session-socket-one-listener` and `absence/served-network-port`. The rule, the hold and the widening rows this chunk must re-read come from here.

@history_item n=3 marker=2026-11-21-dependency-and-ci-debts
**2026-11-21-dependency-and-ci-debts** (Standard Contracts; Occupied Resources; Infrastructure Patterns; ratification entries) — `decision/skia-legs-verify-archive-first`, `decision/cargo-under-verified-skia-url` and `SKIA_BINARIES_URL` came in here. Every skia-building cargo call must run under the verified URL, including the `--locked` workspace build on the merged tree. The founder's own word «Подтверждаю все 24» ratified the 2026-11-19 sync's card operations: the CDP listener exceptions, `decision/blitz-net-reqwest-fetching` and `contract/event-dispatch/focus-change`. This is the precedent for how a sync's delegate-confirmed rows are ratified later.

@history_item n=4 marker=2026-11-16-taffy-past-1372
**2026-11-16-taffy-past-1372** (Established Decisions; Standard Contracts; Project Intent) — Taffy was moved to git `a3997d55`, the rev `2e9fd4af1` now replaces with registry 0.16.0. Added `decision/reversed-scroll-origins` and `decision/unstyled-elements-hit-tested` as upstream behaviour changes ratified for that take. Amended `decision/client-rect-own-box` (upstream's scroll-shifted geometry kept out), `decision/programmatic-scroll-one-box`, `contract/scroll-hit/hit-clip` and `contract/scroll-hit/scroll-into-view`. Retired `decision/deferred-upstream-dependency-bumps` for `decision/deferred-stylo-dioxus-bumps`. All of these sit on the Taffy surface the 0.16 move touches.

@history_item n=5 marker=2026-11-15-taffy-from-upstream-git
**2026-11-15-taffy-from-upstream-git** (Stack; Established Decisions; Project Intent; take ratification) — `decision/taffy-layout` became "git source, pinned to one revision" and `decision/no-git-dependencies` was reworded with Taffy as the lock's one git source. The eight commits of that git-Taffy take were ratified "for this take alone: the next Taffy take asks its own". The Taffy 0.16 registry move must amend `decision/taffy-layout` back to the registry and retire or moot the Taffy git exception. It must also ask its own ratification of any layout change Taffy 0.16 brings.

@history_item n=6 marker=2026-11-15-upstream-sync-errors-and-explanation
**2026-11-15-upstream-sync-errors-and-explanation** (Established Decisions; Project Intent; Stack) — added the `except` on `decision/no-git-dependencies` ("The taffy crate alone may take a git source … a git Taffy still passes the lock, audit and MSRV legs"). The scope says the Taffy move makes this exception moot, and the founder's T1 A retires it. That `except` is the text to remove, along with its security-plan twins. The same entry added `decision/mutation-tier-excludes-demo-shells`.

@history_item n=7 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — the earlier Taffy move to registry "0.15" retired the git pin under [Dependency pinning]. Its trap: CLAUDE.md's coupled-pins warning no longer names a Taffy git rev, so Taffy moves with the registry like parley. The entry also added `UiEvent::PointerLeave` and the `pointer_leave` check, which carries a hand-written `ShellProvider` and the `cursor-icon` dev-dependency (`22c18e38f` extends `pointer_leave.rs`). It also recorded the bridge dropping removed nodes and the 28-name falsy predicate, with `mutation_writer.rs` byte-identical to upstream.

@history_item n=8 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (keyboard-types 0.8 surfaces; pin) — added the crate-private `keyboard_compat` (upstream's 0.8-to-0.7 bridge for `events.rs`) and its coverage exclusion (`--ignore-filename-regex 'keyboard_compat\.rs$'`, pinned by `test_coverage_leg_reaches_the_floor_on_each_crate`). It also set the `::keyboard_types` absolute-path rule, with two keyboard-types versions in the graph under Dioxus 0.7.3. The drag event's `convert_modifiers` adaptation in `packages/dioxus-native-dom/src/events.rs` runs through this bridge under the Dioxus hold. The exclusion is one of the CARRY's re-read items.

@history_item n=9 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (merge surfaces, coverage exclusion clause; take at `f9cbb2756`) — the `keyboard_compat` exclusion's clause was restated to "until escher takes upstream's retirement of it" (upstream retires it in `286874cc1`, not taken). The engine-debts merge surfaces were moved to a residual re-read at each sync. The entry also recorded the `construct.rs` import-list conflict, resolved to upstream's list plus `RangeInputData`. That is a precedent for `construct.rs` conflicts, which `f559f3fe2` now touches again.

@history_item n=10 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (behaviour changes ratified; merge surfaces at `ebacd68bd`) — introduced the standing pattern behind `decision/upstream-behaviour-changes-ratified-per-take`: upstream behaviour changes for every document are ratified for their take and not counted among escher's direct engine changes. A narrowing (the `file:` script refusal) and a realization on a registered crossing are not widenings. The entry also recorded the `mutator.rs` `set_attribute` conflict resolution. The six commits' preventDefault, overflow-clip and flex-br changes need this classification.

@history_item n=11 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (NetHandler::error; AGENTS.md; twelve commits; merge surfaces; test-only binds; embolden) — standing trap: a trait default method can hide a take's fix. `CountedHandler` silently dropped `NetHandler::error`, so a take that extends a trait sweeps that trait's implementers. This applies to the new `DataTransfer` and drag API in blitz-traits and blitz-dom's driver. The entry also set `decision/upstream-agents-md-excluded`, which is in the left-out set to re-read, and registered blitz-net's test-only TCP binds as no crossing. It made `embolden` the last step of `fast`, and moved the merge surfaces' owner to the next sync with the `hit_inner` conflict against `0980cce5f` named.

@history_item n=12 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — updated `decision/upstream-take-engine-changes` to that take's commits. It added an `except` to `decision/both-layout-modes`: upstream-taken blitz-tests targets are kept as taken even where they run one layout mode, and escher's own targets run both. That governs `flex_br.rs` and `drag_and_drop.rs`. `decision/pinned-actions` was widened to every workflow's `uses:`, with the ref as a trailing comment.

@history_item n=13 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (blitz-tests targets; dependency set) — set `decision/blitz-tests-target-per-file`: one target per file under `tests/`, no `autotests`, and upstream's `tests/all.rs` held at `test = false`, pinned by `test_blitz_tests_targets.py`. A sync that takes `autotests = false` or drops that line turns the pin red. Upstream's new `flex_br.rs`, `drag_and_drop.rs` and the `all.rs` `mod` lines meet this rule. The same take took upstream's dependency set whole with no `cargo update`, and the parley/skrifa/vello coupling rule stood.

@history_item n=14 marker=2026-11-17-upstream-sync-ahead-of-polish-and-ship
**2026-11-17-upstream-sync-ahead-of-polish-and-ship** — AnyRender 0.15 (vello-gpu) was taken, `decision/skrifa-matches-parley` replaced two retired rows after a second skrifa entered the graph, and `decision/scroll-offsets-clamped-after-layout` was added as an upstream behaviour change. The SVG and feImage href kinds were added to `decision/sub-resource-requests-carry-initiator`, which is the precedent that a new upstream fetch path must go through the stamping constructor. Two free-disk rows were retired. This is a recent example of a sync's lock and coupled-pin movement entering the plan as amended rows.

@history_item n=15 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** (Standard Contracts; Established Decisions) — registered `contract/base-document/layout-reason` and the closed sets `contract/layout-reason/{placed-by,shift-by,size-cause,length}` (`calc` among the length forms). It also amended `decision/driver-spans-only`. Escher's `layout_reason.rs` calls `stylo_taffy::convert::resolve_calc_value`, which under Taffy 0.16 takes `taffy::CalcHandle`. That makes this contract, and `Session::layout` and `stand_layout`, a surface of the take's T1 adaptation.

@history_item n=16 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — the `layout` verb carries the layout read on every carrier. It added `decision/layout-answer-crossing`, `contract/driver-verbs/layout`, `contract/answer-json/layout`, `-length` (`{"calc":n}`), `-fallback` ("a later member of an engine set is written unexplained … never a panic") and `contract/driver-session/layout-unexplained`. The `CalcHandle` adaptation must keep the `calc` length and the fallback words, so a reading that moves is re-measured and never loosened.

@history_item n=17 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — set the hit stop at a clipping box's padding box and recorded the one-axis reading: a one-axis clip stops the hit at the whole padding box while `visible_region` narrows the clipped axis only. `f65baeca5` changes paint for a one-axis `overflow: clip`. Whether the merge moves the hit, the paint alone, or opens a hit/paint gap is measured against this record.

@history_item n=18 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** (hit readings) — measured and kept the one-axis reading, which is the basis of `decision/one-axis-clip-readings-kept` (guard `stand_act_obstructed`). A driver click beside a box clipped on one axis is refused `covered`, and neither reader moves (founder C5 «Ничего, замерить и записать»). `covered`'s fixed text states this limit. The stated limit re-read for `f65baeca5` rests on this, and any change to it would be a new founder question.

@history_item n=19 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (geometry readers) — the hit walk also stops at a `contain: paint` box, using paint's own clipping predicate so hit and paint agree; this underlies `decision/hit-stops-at-paint-clip` and `decision/hit-stops-at-clip-padding-box`. `f65baeca5` edits paint's clip in `blitz-paint/src/render.rs`. If the hit is meant to share paint's predicate, it must be checked whether upstream's one-axis paint clip now reads differently from the hit's whole-padding-box stop.

@history_item n=20 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the eight engine changes) — eight direct engine changes, flagged upstreamable, including `visible_region` through transforms and `hit_inner`'s no-hit on a non-invertible transform, which conflicts textually with upstream `0980cce5f`. Their merge surface is a line owned by each upstream sync, and these are the surfaces "conflicts keep both sides' intent" must protect. The entry also changed the checkbox/radio `change` to come from the engine, which `22c18e38f`'s non-cancelable `preventDefault` in `events/driver.rs` sits beside.

@history_item n=21 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — the bridge's buffered render (`decision/bridge-buffered-render`) was ported from upstream `286874cc1` onto the Dioxus 0.7 bridge and flagged not upstreamable. Its merge surface is `dioxus_document.rs`, `mutation_buffer.rs` (replaced whole by upstream's 0.8 file at the 0.8 move) and `lib.rs`, re-read at each sync. That keeps the left-out `286874cc1` and the Dioxus hold coupled. Upstream's dioxus-native-dom drag/file changes land in the same crate.

@history_item n=22 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (registry dispatch) — `DioxusEventHandler::handle_event` sets the `SignalRegistry` dispatch around `runtime.handle_event`, a change on every Dioxus document's event path that is inert without a registry. The entry also made `use_graph_state` and the bridge tests part of the bridge. The conflicted `packages/dioxus-native-dom/src/events.rs` import blocks are on this event path, so a resolution must keep the registry dispatch and its imports.

@history_item n=23 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (reached record) — `handle_event` writes the reached record only where it is switched on, with no borrow across `render_immediate`. The ungated modules became `signal_registry` and `reached`. New drag events dispatched through the bridge pass this code. The `events.rs` conflict must keep both escher's reached and registry arms and upstream's drag arm.

@history_item n=24 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** (reached record resolved before, written after) — reworded `decision/reached-record-before-dispatch` and `decision/reached-recorded-before-handlers`: the record is resolved before the dispatch and written after, and a non-bubbling event's target alone counts. These rows hold for any new event kind upstream adds, so a drag event's propagation must be read the same way.

@history_item n=25 marker=2026-11-22-epoch-2-corrective
**2026-11-22-epoch-2-corrective** — set `decision/fork-additions-in-own-functions`: a fork addition to an upstream function over the complexity ceiling goes into a function of escher's own, with one call in upstream's body. This shapes how conflict resolutions and the two adaptations, `layout_reason.rs`'s `CalcHandle` and `events.rs`'s `convert_modifiers`, are written into upstream-owned functions.

@history_item n=26 marker=2026-10-16-handler-splits-and-readme
**2026-10-16-handler-splits-and-readme** (range and button arms) — escher's arms in `handle_click`, `handle_pointerdown` and `handle_key_or_input_event` were moved into private named functions, with the parents left at their upstream floor "so the next upstream sync's diff stays out of arms escher does not own". That is the precedent behind keeping the `events/driver.rs` and `pointer.rs` merge small when `22c18e38f` and `82a6573ee` touch them.

@history_item n=27 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — any engine time read goes through the document's clock, and no `Instant::now()` stands in blitz-dom outside `clock.rs` (`decision/document-clock`, `decision/no-wall-time-outside-clock`). Its merge surface includes `events/pointer.rs`. Upstream's drag handling in blitz-dom's driver must keep this clock invariant, which the scope lists among both sides' intent.

@history_item n=28 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — `decision/url-scheme-routing`, `decision/file-url-load-rule`, `decision/sub-resource-requests-carry-initiator` and `decision/blitz-dom-stamped-requests` were set here, with blitz-net refusing `FileForbidden` before any disk read. `82a6573ee`'s dioxus-native-dom dropped-file reader reads `std::fs` with no path restriction, outside this rule. That is the widening the founder answered (D2 A), to be recorded against these rows.

@history_item n=29 marker=2026-11-14-remote-pages-local-files
**2026-11-14-remote-pages-local-files** — narrowed `decision/file-url-load-rule`: in the browser app, a navigation a non-`file:` page starts is dropped silently, and an embedder's top-level load stays one. The same entry reworded `decision/sub-resource-requests-carry-initiator`. Together with item 28, this is the full `file:` boundary the OS drop input and its file reads are measured against.

@history_item n=30 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (upstream-only workflows) — every job reaching signing, WPT, Pages or dispatch carries `github.repository == 'DioxusLabs/blitz'`, keeping upstream's workflows byte-equal and future merges small. Any upstream workflow edit in the take must keep this guard, as the scope's CI boundary requires (`decision/upstream-only-workflows`, `decision/publish-workflow-upstream-only`).

@history_item n=31 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — SHA-pinned actions, the read-only `contents: read` token, the audit, a11y and coverage legs, and the real rustdoc gate. The take's CI edits must keep every action pinned to a 40-hex SHA and the token unchanged (`decision/actions-pinned-to-commit-sha`), and the doc leg must stay green.

@history_item n=32 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — set `decision/stand-repeat-zero-failure-budget` and the audit over the whole workspace with licences. It also added upstream's blitz-dom feature-subset clippy line (`decision/blitz-dom-feature-subset-clippy`) and the per-crate coverage floors. The merged tree must pass all of these, and the subset clippy line is where blitz-dom's new drag code is linted without default features.

@history_item n=33 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (advisory fix; deny.toml ignore) — pulldown-cmark moved to 0.13.6 in `Cargo.lock` alone, and `deny.toml` gained a reasoned ignore (RUSTSEC-2026-0429). This is the precedent for keeping the audit green when the merged lock brings advisories. The new `url` dependency in blitz-shell and the Taffy 0.16 checksum enter the audit and licence check.

@history_item n=34 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (manifest citations re-pointed) — trap: any insert into the root manifest's members list or dependency block shifts every citation below it in all masters. `29a439e05` edits the root manifest's dioxus-native dev features and `2e9fd4af1` edits the Taffy line, so citations into `Cargo.toml` need re-reading.

@history_item n=35 marker=2026-10-06-project-readme
**2026-10-06-project-readme** — at an upstream sync, an upstream change to `README.md` resolves to escher's version (`decision/readme-is-escher-own`). This applies if the delta touches the README.

@history_item n=36 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** — the first sync recorded the `Upstream sync:` line as the next sync's merge base and re-pointed citations into merged upstream files by the merge's line map. That is the pattern this take repeats: `decision/upstream-sync-merge-base` moves to `29a439e05`, and citations into the merged files are re-read.

@history_item n=37 marker=2026-10-18-upstream-sync-determinism-and-traces
**2026-10-18-upstream-sync-determinism-and-traces** — the first partial take by cherry-pick, which held the Taffy-main commits because they put a git Taffy in the manifest. That history of the Taffy deferral is now closed by `2e9fd4af1`'s registry 0.16. The cherry-pick form itself was replaced by `decision/upstream-sync-by-two-parent-merge`.

@history_item n=38 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — a take of upstream's `:focus-within` and focus redraw, held PROVISIONAL until the epoch batch. This is the precedent that an engine behaviour change answered by a delegate is PROVISIONAL, whereas this chunk's forks were answered by the founder's own word (T1, T2, D1, D2, C1, B1, P1 all A).

@history_item n=39 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — a log target is admitted only by `ENGINE_TARGET_PREFIXES` or `escher_`, and every outside target is dropped whole at every level (`contract/telemetry/target-filter`). The scope re-checks the sink allowlist, so the merged delta's new `tracing`/`log` sites (blitz-shell `dnd.rs`, drag events) are read against these prefix sets.

@history_item n=40 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (capture) — escher-telemetry's per-step capture counts records of outside targets, and `contract/telemetry/target-filter` became the sink's drop. A new upstream log site in the drag path can surface in an acting step's `caused` on the stand, though no stand task reaches the OS drop input.

@history_item n=41 marker=2026-11-23-circle-drawer-on-a-canvas
**2026-11-23-circle-drawer-on-a-canvas** — added `decision/widget-click-synthesis` (the engine synthesises click, contextmenu and dblclick on a custom-widget target after the widget's own handling, with no click walk and no focus clear) and `decision/widget-click-point-content-box`, and amended `decision/click-focuses-button`. These are fresh engine changes on the pointer path, which `22c18e38f` (preventDefault on non-cancelable events in `events/driver.rs`) and `82a6573ee`'s drag events touch. "Custom-widget pointer frame and click synthesis" is a named intent the conflicts must keep.

@history_item n=42 marker=2026-11-23-canvas-placement-and-text
**2026-11-23-canvas-placement-and-text** — `decision/custom-widget-content-box-paint` (a direct engine change answering upstream issue 1578) and `contract/nodes-widgets/widget-fonts` / `contract/base-document/font-ctx` were added. These are escher's widget paint changes in `blitz-paint`'s render path, which `f65baeca5` also edits, so the canvas paint must survive the merge.

@history_item n=43 marker=2026-11-23-canvas-in-a-headless-shot
**2026-11-23-canvas-in-a-headless-shot** — added `decision/canvas-crosses-as-pixels` and `decision/harness-paints-custom-widgets` (blitz-test-harness names blitz-paint's custom-widget feature, guard `stand_canvas_shot`). The screenshot readings re-read after the merge include canvas shots that depend on this feature wiring.

@history_item n=44 marker=2026-11-23-canvas-shot-across-hosts
**2026-11-23-canvas-shot-across-hosts** — `decision/no-screenshot-in-cross-os-byte-check` gained the canvas-only exception, a committed digest table held byte-equal on three CI hosts. `decision/committed-stand-checks`' reason was also scoped. The WATCH on `stand_canvas_cross_os` (Circle Drawer's canvas `SAMPLE` digest) reads this chunk's CI runs, and a moved reading is re-measured, never loosened.

@history_item n=45 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — `decision/stand-readings-same-on-every-ci-os` (guard `stand_cross_os_readings`) and the DejaVu-only stand typography rule (guard `stand_bundled_font_families`). These are among the screenshot and cross-OS readings the merged tree must keep green on linux, windows and macOS.

@history_item n=46 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — widened `decision/stand-readings-same-on-every-ci-os` to seven served tasks and named the order page in `contract/headless-stand/boot`. It also added `decision/app-crate-takes-own-tracing` (seven_guis takes tracing itself, since dioxus-native's prelude re-exports none). This is the current form of the cross-OS rule the stand checks are held to after the merge.

@history_item n=47 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** — added the post-layout focus fixup and Tab skipping, extending `decision/focus-fixup-without-events` and `decision/hit-focus-form-engine-changes`. The `except` on `decision/bridge-buffered-render` (the busy-yield livelock) and the boundary-marks pruning came from here too. These are recent direct engine changes in `events` and the bridge that the merged `events/driver.rs` and dioxus-native-dom must keep.

@history_item n=48 marker=2026-11-19-build-directory-held
**2026-11-19-build-directory-held** — the dev-host target sweep, which removes stale debug build units under `target/` and runs at the end of a green `fast`. Together with `decision/run-state-areas-local-only`, this is why the probe or trial worktree stands OUTSIDE `target/` per the directive.

@history_item n=49 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (password mask) — a password input paints its value as bullets in every document, a direct engine change whose merge surface is `render.rs` and `document.rs` (`decision/password-input-painted-masked`). `f65baeca5` edits `blitz-paint/src/render.rs`, so the mask must survive the merge.
