@history_item n=1 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — (upstream-only signing, WPT and dispatch) every job reaching the Signed Builds / WPT environments, Pages deploy and dispatch carries `github.repository == 'DioxusLabs/blitz'`, removing the workflows rejected to keep future upstream merges small. Precedent for this sync's CI Pages-artifact commit `165d698c3` and the scope's rule that upstream workflow files stay repository-guarded (decision/upstream-ci-resources-upstream-only, decision/upstream-only-workflows).

@history_item n=2 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — every ci.yml `uses:` pinned to a 40-hex SHA with its ref as trailing comment, workflow-level `permissions: contents: read`, the audit/a11y/coverage legs. The constraint any workflow edit the merge carries must keep (`WorkflowPinTest`, decision/actions-pinned-to-commit-sha, decision/ci-read-only-permissions).

@history_item n=3 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** — the first upstream sync (merge of `23354585`, conflict-free), recorded the `Upstream sync:` line as the next sync's merge base and re-pointed 113 citations by the merge's line map. Origin of the sync-line convention and the post-merge citation re-point this chunk will repeat.

@history_item n=4 marker=2026-10-06-project-readme
**2026-10-06-project-readme** — README is escher's own front page; at an Upstream sync an upstream change to `README.md` resolves to escher's version. Standing conflict rule if the merge touches README.

@history_item n=5 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (the merge base, Parley's pin form, the merged dependency set) — second sync took upstream's dependency set whole with no `cargo update`; parley/skrifa/vello coupling rule; `ci-scripts` needs Python 3.11 for `test_blitz_tests_targets.py` (decision/ci-scripts-python-floor). Precedent for the coupled-pins boundary.

@history_item n=6 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (blitz-tests keeps one test target per file) — upstream's single-binary `tests/all.rs` taken whole but held at `test = false`, no `autotests = false`, pinned by `BlitzTestsTargetsTest`; a sync that drops the line or takes `autotests = false` turns the pin red. Governs upstream's new `display_contents.rs`, `ime_preedit_underline.rs` and the new `mod` line in `all.rs` (decision/blitz-tests-target-per-file, decision/blitz-tests-single-test-target).

@history_item n=7 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (Taffy's tree traits on `LayoutPassState`; upstream's script and layout additions) — `stylo_taffy` export list registered (contract/layout/stylo-taffy-exports) and Selection/innerText recorded as realizations on the registered script-to-DOM crossing, not widenings. Precedent for reading this take's stylo_taffy float/clear change and `DOMTokenList.replace` as realizations rather than crossings.

@history_item n=8 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — the scrolled-box bounds and the hit stop at a clipping box made direct engine changes for every document, flagged upstreamable against upstream `main`. Two of the surfaces the scope says conflicts must keep (decision/client-rect-own-box, decision/hit-stops-at-paint-clip).

@history_item n=9 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the harness's screenshot, the password mask) — the password mask is a direct engine change, merge surface `render.rs` and `document.rs`; `layout_as_text_input` added. Upstream's IME preedit underline (blitz-paint) and textarea text-layout commits land on these surfaces (decision/password-input-paints-masked, contract/base-document/layout-as-text-input).

@history_item n=10 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (taffy from the registry, the beta.3 pins) — the merge took upstream's pin set whole as one coupled move; trap noted that the coupled-pins warning wording changes as taffy's source moves. Precedent for the scope's coupled-pin boundary and a lock move measured by a `--locked` build.

@history_item n=11 marker=2026-10-13-range-input-interaction
**2026-10-13-range-input-interaction** — the range model, a direct engine change, merge surface seven upstream files plus `node/range.rs`, owned by the upstream-sync CARRY. One of the models the scope lists conflicts must keep (decision/range-current-value and siblings).

@history_item n=12 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — keyboard activation and a click focusing a button, direct engine changes; merge surface `events/keyboard.rs`, `events/mod.rs`, `events/pointer.rs`, `node/element.rs`. Conflicts there keep both sides (decision/keyboard-activation-enter, decision/keyboard-activation-space).

@history_item n=13 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** (legs) — `audit` over the whole workspace with licences, upstream's blitz-dom feature-subset clippy line, coverage floors, `stand-repeat` zero-failure budget. The proofs this chunk must keep green after the merge (decision/blitz-dom-feature-subset-clippy, decision/per-crate-coverage-floors, decision/stand-repeat-zero-failure-budget).

@history_item n=14 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** (the merge surface read) — range and keyboard merge surfaces read at upstream `e72650a6` as untouched; upstream read past the base but not merged. The per-sync practice of re-reading each engine change's merge surface against the new upstream tip.

@history_item n=15 marker=2026-10-16-handler-splits-and-readme
**2026-10-16-handler-splits-and-readme** (the range and button arms as private named functions) — escher's arms moved into private functions, parents left at their upstream floor "so the next upstream sync's diff stays out of arms escher does not own". Forerunner of decision/fork-additions-in-own-functions.

@history_item n=16 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the sync at `23b9b26a9`) — merge commit with two parents, pin chosen at a P4 fork; workspace beta.4, keyboard-types 0.8 with Dioxus 0.7.3 carrying its own 0.7; four Taffy-main commits left to wait. Merge-base and left-out-commit precedent.

@history_item n=17 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the keyboard-types 0.8 surfaces: `keyboard_compat`, the coverage exclusion) — dioxus-native-dom gains upstream's crate-private `keyboard_compat`; both `cargo llvm-cov` lines exclude it by filename until upstream retires it. The exclusion the folded CARRY `cry-ikyyzjzj` asks to re-read at this sync (decision/keyboard-compat-coverage-exclusion, decision/coverage-excludes-keyboard-compat).

@history_item n=18 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the builder's merged ARIA reads; descriptions on the adapter crossing) — upstream's accessibility reads merged with each attribute read once, upstream's bool-parsed `aria-selected` and unfiltered `aria-label` dropped; descriptions ratified as a widening by upstream merge. Precedent for keeping escher's accessibility state reads through a conflict (decision/each-aria-attribute-read-once, decision/adapter-crossing-carries-descriptions).

@history_item n=19 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — seven engine changes for every document (inert, disabled on form controls, toggle per role, contain:paint hit stop, absolute_position, client rect under transforms, password caret) with their merge surface owned by the upstream-sync CARRY. Surfaces conflicts must keep (decision/inert-honoured, decision/disabled-read-on-form-control-only, decision/toggle-read-per-role).

@history_item n=20 marker=2026-10-18-upstream-sync-determinism-and-traces
**2026-10-18-upstream-sync-determinism-and-traces** — partial take by no-commit cherry-pick of one commit, merge base unmoved, Taffy-main commits left out because they put a git Taffy in the manifest. The cherry-pick form later retired by decision/upstream-sync-by-two-parent-merge; decision/upstream-sync-merge-base still states it.

@history_item n=21 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (the document's clock) — every engine time read through the document's clock, no `Instant::now()`/`SystemTime::now()` in blitz-dom outside `clock.rs`; merge surface (`config`, `clock`, `document`, `resolve`, `iframe`, `scrolling`, `events/pointer.rs`) owned by the upstream-sync CARRY. The clock invariant the scope names among conflicts to keep (decision/document-clock, decision/no-wall-time-outside-clock).

@history_item n=22 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — two cherry-pick takes; upstream's `:focus-within` and focus redraw taken PROVISIONAL as an engine change for every document (later ratified); merge-surface CARRY owner moved to the next sync. Precedent for ratifying upstream behaviour per take.

@history_item n=23 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — the select model, a direct engine change for every document, its merge surface by file owned by the upstream-sync CARRY. Part of the select model the scope says conflicts must keep (decision/select-keyboard-model, decision/select-choice-as-checked-state).

@history_item n=24 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (the upstream take read at `f9cbb2756`; serialised traversals, mimalloc and `multi_doc` env reads) — ten cherry-picks; git Stylo, Taffy-main and Dioxus 0.8.0-alpha.3 left out to a residual; upstream's env reads (`MULTI_DOC_*`) and process-wide statics registered as occupied resources. Precedent for the scope's sink/env-read re-check of the merged delta.

@history_item n=25 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (`multi_doc`, the merge surfaces and their owners, the coverage exclusion's clause) — range model's one conflict (`construct.rs` imports) resolved to upstream's list plus escher's; engine-debts surfaces owned by a residual re-read at the next sync; the keyboard_compat exclusion reworded "until escher takes upstream's retirement of it" since upstream retires it in `286874cc1`, not taken. Direct lead for res-7qgqygtj and the Dioxus-hold CARRY.

@history_item n=26 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (six upstream behaviour changes for every document, ratified for this take) — origin of decision/upstream-behaviour-changes-ratified-per-take: upstream's engine behaviour changes stand ratified per take under the playbook pattern and are not counted among escher's direct changes. This sync's display:contents pseudo-elements, float sizes, IME underline and textarea layout fall under it.

@history_item n=27 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the take read at `ebacd68bd`; merge surfaces; `file:` refusal; IME at editor construction; runtime child modules; `TMPDIR`) — 24 taken, 16 left out each with measured reason; `mutator.rs` `set_attribute` conflict resolved by placing upstream's `open` arm after the range arm; `document_may_load` a narrowing; blitz-vibey-script `runtime` split into child modules (where `DOMTokenList.replace` lands); an upstream test helper's `TMPDIR` read registered. Conflict-resolution and registration precedents.

@history_item n=28 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the eight engine changes of this chunk) — eight more direct engine changes, upstream `0980cce5f`'s `hit_inner` hunk a textual conflict with the non-invertible answer, merge surface owned by the upstream sync. Names a known conflict for any take that brings `0980cce5f` (decision/hit-focus-form-engine-changes, decision/non-invertible-transform-takes-no-hit).

@history_item n=29 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list recorded; its paint; no local gate builds `font-embolden`) — the select list, a direct engine change, merge surface sixteen files owned by the upstream sync; `stroke_text` delegates to `stroke_text_as` with `RunPaint` in blitz-paint's render. Upstream's IME preedit underline touches the same text-paint path; the select list is a surface the scope says conflicts must keep.

@history_item n=30 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a failed stylesheet through `NetHandler::error`; a failed load ending the wait) — standing trap: a trait default method can hide a take's fix (the do-nothing default compiled silently through `CountedHandler`), so a take sweeps the implementers of any trait it extends (contract/net-handler/error).

@history_item n=31 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the twelve ratified for their take; `AGENTS.md` stays out; `background-clip: text` left out; the queued `mounted` guard) — twelve upstream commits ratified per take; `AGENTS.md` from `22f0cd000` kept out (no build reads it, its instructions contradict the fork's); a test asserting 0 mounted events left out on Dioxus 0.7 pending `286874cc1`. Direct precedent for this take's `ca0b97b05` (AGENTS.md and `packages/parley_code_editor/AGENTS.md`) and the standing left-out set (decision/upstream-agents-md-excluded, decision/stale-mounted-event-skipped).

@history_item n=32 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (merge surfaces at `e42ee1f25`; the key-press panic's fix; the `embolden` step; test-only binds) — merge surfaces re-read and owner moved; anonymous-block dealloc fix, flagged upstreamable; `embolden` closes `fast`; blitz-net's loopback test binds registered as test-only, no served port. Practice for re-reading surfaces and registering an upstream test's binds.

@history_item n=33 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the advisory fix, `deny.toml` ignore; the MountedData hazard routed to a port) — `deny.toml` gained an ignore after the audit read red; upstream `286874cc1`'s buffered render to be ported onto Dioxus 0.7, the 0.8 move still waiting; left-out commits owned by a residual row. Leads for the RUSTSEC reason CARRY and the Dioxus hold.

@history_item n=34 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the MountedData hazard closed as an engine change) — the bridge's buffered render, a port of `286874cc1` flagged not upstreamable; merge surface `dioxus_document.rs`, `mutation_buffer.rs` (replaced whole by upstream's 0.8 file at the 0.8 move) and `lib.rs`, owned by `res-3v42legm`, re-read by the upstream-sync CARRY. The buffered render the scope says conflicts must keep (decision/bridge-buffered-render, decision/render-with-document-unborrowed).

@history_item n=35 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph registry and the bridge's dispatch) — `DioxusEventHandler::handle_event` gains escher's dispatch recording, inert without a registry, with `mutation_writer.rs` kept byte-equal to upstream `434e0661`. An escher addition inside an upstream bridge body to keep through a conflict.

@history_item n=36 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — the take's reading moved to 29 picks ratified as engine changes; `both-layout-modes` gained the exception that upstream-taken blitz-tests targets stay as taken even when they run one mode; pinned-actions widened to every workflow file with the ref comment. Governs upstream's new test files and any workflow edit (decision/both-layout-modes, decision/pinned-actions).

@history_item n=37 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — blitz-net refuses a `file:` request from a non-`file:` document; every engine sub-resource request stamped with its document's initiator, blitz-dom building requests only through the stamping constructor (clippy disallowed-methods guard). The stamped request and `file:` refusal the scope says conflicts must keep; any new upstream request site must go through the constructor (decision/blitz-dom-stamped-requests, decision/url-scheme-routing).

@history_item n=38 marker=2026-11-15-upstream-sync-errors-and-explanation
**2026-11-15-upstream-sync-errors-and-explanation** — `@font-face` `font-stretch` taken as a behaviour change; no-git-dependencies gains its Taffy-only exception (lock, audit and MSRV legs pass); demo shells outside the mutation tier. The scope's "no git dependency save Taffy's pinned rev" rests on this (decision/no-git-dependencies).

@history_item n=39 marker=2026-11-15-taffy-from-upstream-git
**2026-11-15-taffy-from-upstream-git** — Taffy from upstream git pinned to one rev; the eight taken blitz commits ratified for this take alone ("the next Taffy take asks its own"). Per-take ratification precedent and the decision/taffy-layout row.

@history_item n=40 marker=2026-11-16-taffy-past-1372
**2026-11-16-taffy-past-1372** — Taffy past #1372 taken with reversed scroll origins; client-rect-own-box now states it differs from upstream, upstream's scroll-shifted geometry and self match kept out; deferred-upstream-dependency-bumps retired to deferred-stylo-dioxus-bumps. A standing intentional divergence a merge conflict must preserve (decision/client-rect-own-box, decision/reversed-scroll-origins).

@history_item n=41 marker=2026-11-17-upstream-sync-ahead-of-polish-and-ship
**2026-11-17-upstream-sync-ahead-of-polish-and-ship** — AnyRender 0.15 (vello-hybrid → vello-gpu, renderer feature renamed), skrifa-matches-parley amended for a second skrifa, scroll offsets clamped after layout taken as upstream behaviour, SVG image/feImage hrefs stamped, `only-stand-repeat-frees-disk` retired. Prior sync's row moves and its card-confirmed widening (decision/skrifa-matches-parley, decision/scroll-offsets-clamped-after-layout, decision/sub-resource-requests-carry-initiator).

@history_item n=42 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (unexplained placer) — an element with no layout reason, "such as a display contents group", answers `placed by unexplained`. Upstream's `display: contents` changes (`1fa2034d0`, `719268a9e`) may move this reading (contract/driver-session/layout-unexplained).

@history_item n=43 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (Project Intent) — the sync made a real two-parent merge with every left-out commit named with its reason, retiring the cherry-pick rule; Dioxus 0.8 move held at 0.7.3 after Stylo moved to 0.24. The rules this chunk repeats (decision/upstream-sync-by-two-parent-merge, decision/dioxus-0-8-move-held); decision/upstream-sync-merge-base still speaks of cherry-picks.

@history_item n=44 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (Stack, Established Decisions, Conventions, Cross-cutting, Standard Contracts) — Parley 0.13, Stylo 0.24, reqwest on rustls, winit beta.4, setup-ndk action; overscroll-behavior exceptions; events queued on the document replacing the dispatch callback; accessibility refresh moved to redraw; inline-box placeholder fallback family as escher's direct change; `stylo-prefs-set-once` retired to upstream's set. The last merge's row changes this one rests on (decision/parley-text-layout, decision/stylo-css-engine, decision/inline-box-placeholder-fallback-family, decision/accessibility-tree-refresh-on-change).

@history_item n=45 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (Occupied Resources, Inherited Defaults) — the merge brought an opt-in devtools CDP TCP listener, `BLITZ_CDP_PORT`, `BLITZ_DEVTOOLS_FRONTEND_DIR` and the `blitz-vibey-cdp-server` crate; listener and no-port absences narrowed to default builds. Precedent for a widening the merge carries — each now a fork for `evidence/founder-forks.md` (absence/served-network-port, decision/driver-session-unix-socket).

@history_item n=46 marker=2026-11-21-dependency-and-ci-debts
**2026-11-21-dependency-and-ci-debts** (skia archive; `SKIA_BINARIES_URL`; cargo under the verified URL) — skia-building legs verify skia's archive first and every cargo call in them runs under the verified URL, since skia-bindings reruns on the env var. Any lock move measured by a `--locked` workspace build must run under it (decision/skia-legs-verify-archive-first, decision/cargo-under-verified-skia-url).

@history_item n=47 marker=2026-11-21-dependency-and-ci-debts
**2026-11-21-dependency-and-ci-debts** (ratification of the upstream sync's card operations) — the founder ratified all 24 card operations of the last sync (listener exceptions, CDP names, rustls, cancelled pointerup, focus-change contract), replacing the overseer's delegate confirm. The precedent the operator's directive now inverts: this sync's widenings go to the founder as forks, not a delegate card.

@history_item n=48 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** — focus fixup after layout with "a display contents control … keep focus"; `integration-tests-in-blitz-tests` excepts blitz-vibey-script's own `tests/`; buffered render's livelock limit; boundary registry pruned. Upstream's `display: contents` and `DOMTokenList.replace` changes meet these rows (decision/focus-fixup-without-events, decision/integration-tests-in-blitz-tests, decision/bridge-buffered-render).

@history_item n=49 marker=2026-11-21-single-select-sized-as-a-list-box
**2026-11-21-single-select-sized-as-a-list-box** — sized single select as an in-flow list box, a direct engine change for every document; select size read in every document; `size`/`multiple` writes restyle a select's descendants. Latest form of the select model the scope says conflicts keep (decision/single-select-list-box, decision/targeted-restyle-on-mutation).

@history_item n=50 marker=2026-11-22-epoch-2-corrective
**2026-11-22-epoch-2-corrective** — new decision/fork-additions-in-own-functions: a fork addition to an upstream function over the complexity ceiling goes into escher's own function with one call in upstream's body (applied to `build_accessibility_node`, `handle_key_or_input_event`, `compute_presentational_hints`). The form conflicts in those bodies must preserve, and the rule for any new escher edit the merge needs.

@history_item n=51 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — the sink admits only the engine and `escher_` prefix sets, dropping every other target whole at every level. The standing rule behind the scope's sink-allowlist re-check of the merged delta's new `tracing`/`log` targets (decision/telemetry-target-prefix-drop).
