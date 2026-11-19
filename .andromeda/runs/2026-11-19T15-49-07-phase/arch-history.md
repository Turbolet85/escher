@history_item n=1 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (upstream-only signing, WPT and dispatch) — guarded every upstream-only job (`release-cli`, `wpt`, `trigger-archive`, `post-results`) by `github.repository == 'DioxusLabs/blitz'` so no fork ref reaches them; the cache budget showed that a lockfile change re-mints every rust-cache key. Matters here because the merge touches upstream workflows and moves `Cargo.lock` heavily: the guards must survive, and the cold-cache CI run should be expected (decision/upstream-only-workflows, decision/upstream-ci-resources-upstream-only).

@history_item n=2 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — SHA-pinned every `uses:` with the ref as a trailing comment, set `permissions: contents: read`, and added the audit, a11y, coverage and real rustdoc legs. These are the gates the merged tree must pass. Any CI workflow edits in the take must keep the pins and the read-only token (decision/actions-pinned-to-commit-sha, decision/ci-read-only-permissions).

@history_item n=3 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** — the first upstream sync. It was a real merge of upstream `main` at `23354585`, conflict-free, with our changes additive. It added the `Upstream sync:` line as the next sync's merge base, took upstream's `normal` alignment keyword, and re-pointed 113 citations by the merge's line map. This is the 0.1.0 precedent for this chunk's two-parent merge and for re-pointing citations after a merge.

@history_item n=4 marker=2026-10-06-project-readme
**2026-10-06-project-readme** — made `README.md` escher's own front page and set the rule that at an upstream sync, an upstream change to `README.md` resolves to escher's version. Applies directly to the merge's conflict resolution (decision/readme-is-escher-own).

@history_item n=5 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — the bridge's falsy boolean clear started as a copied list of 27 names, with the trap "a Dioxus upgrade that changes `isBoolAttr` does not move it". Later replaced by upstream's predicate (n=13). With Dioxus 0.8 in the take, re-read which predicate the merged `mutation_writer.rs` holds.

@history_item n=6 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — the stable-id grammar was built against dioxus-core 0.7.10 (four framework scopes, `VComponent.name`, ElementId reuse). The Dioxus 0.7.3 → 0.8.0-alpha.3 move in the merge must keep the id contract (the stable ids are named in the scope) and re-measure it, not assume it holds.

@history_item n=7 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (manifest citations re-pointed) — set a trap: any insert into the root manifest's members list or dependency block shifts every citation below it in all seven masters. The merge adds `packages/blitz-vibey-cdp-server` and `packages/parley_code_editor` and moves pins in the root `Cargo.toml`.

@history_item n=8 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (the driver session registered) — registered the one local listener, a Unix-domain socket, as a boundary widening ratified by the founder, with no TCP or UDP port. This is the precedent for treating upstream's CDP listener (`127.0.0.1` via `BLITZ_CDP_PORT`) as a boundary widening and a founder fork (absence/served-network-port, decision/one-local-unix-listener).

@history_item n=9 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — the telemetry sink admits only `ENGINE_TARGET_PREFIXES` and `escher_` targets and drops every other target whole. This is the sink allowlist the scope re-checks against the merged delta's new `tracing` and `log` call sites and targets (decision/telemetry-target-prefix-drop).

@history_item n=10 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (merge base, Parley's pin form, the merged dependency set) — the second sync merged 61 commits at `7832c177` and took upstream's dependency set whole, with no `cargo update`. The coupling rule held: parley, skrifa and vello move together. This is the precedent for this take's coupled pin set (Stylo 0.24, parley 0.13, skrifa 0.49, winit beta.4).

@history_item n=11 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (Taffy's tree traits, the bounds reader renamed) — upstream moved the tree traits off the document onto `LayoutPassState` and removed a reader. Escher's one edit to upstream-owned code moved `visible_region` onto the new reader with its signature kept. This is the precedent for a resolution that edits upstream code instead of contorting ours (contract/layout/taffy-traits).

@history_item n=12 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (one test target per file; upstream's `all` stands unbuilt) — upstream moved to one test binary. Escher kept one target per file and holds upstream's `tests/all.rs` at `test = false`, pinned by `test_blitz_tests_targets.py`. Trap: a sync that drops `test = false` or takes `autotests = false` turns the pin red. The scope runs upstream's new tests under this rule (decision/blitz-tests-target-per-file, decision/blitz-tests-single-test-target).

@history_item n=13 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (the bridge drops a removed node; the falsy predicate) — the bridge clears removed nodes (upstream #1185), and the falsy clear became upstream's 28-name `is_bool_attr`, so `mutation_writer.rs` was byte-identical to upstream's. The default UA sheet now comes from `DocumentConfig`. These are bridge surfaces the Dioxus 0.8 merge meets (decision/bridge-removal-drops-subtree, decision/falsy-boolean-attribute-cleared).

@history_item n=14 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (taffy from the registry, the beta.3 pins) — the merge took upstream's pin set whole. Trap for the next sync: the coupled-pins warning no longer names a taffy git rev. Taffy was later moved back to git (n=46).

@history_item n=15 marker=2026-10-13-range-input-interaction
**2026-10-13-range-input-interaction** — the range model is a direct engine change for every document, flagged upstreamable. Its merge surface is seven upstream files plus `node/range.rs`, owned by the upstream-sync carry. The merge must keep the range model (decision/range-current-value, decision/range-keyboard).

@history_item n=16 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — added keyboard activation (Enter and Space) and focus on button click as engine changes for every document. Merge surface: `events/keyboard.rs`, `events/mod.rs`, `events/pointer.rs` and `node/element.rs`. The merge keeps the keyboard model (decision/keyboard-activation-enter, decision/keyboard-activation-space).

@history_item n=17 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — added `stand-repeat` (a zero-failure budget), the whole-workspace `audit` with licences, upstream's blitz-dom feature-subset clippy line, and per-crate coverage floors. The merged tree must pass every one, and any new crate passes through the licence check (decision/stand-repeat-zero-failure-budget, decision/blitz-dom-feature-subset-clippy, decision/per-crate-coverage-floors).

@history_item n=18 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the sync at `23b9b26a9`) — this is the last real merge. Merge commit `3eda07bc` has parents `143b6013` and `23b9b26a9`, which is this chunk's merge base. It brought keyboard-types 0.8 while Dioxus 0.7.3 carries 0.7, so two versions stand in the graph and modules that glob the Dioxus prelude name `::keyboard_types`. The Dioxus 0.8 move may retire that split (decision/keyboard-types-named-by-absolute-path).

@history_item n=19 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the builder's merged ARIA reads) — upstream's `aria-labelledby`, `aria-describedby` and `aria-description` reads were merged as written. The descriptions class on the adapter crossing was a boundary widening by upstream merge, ratified by the founder. Upstream's duplicate `aria-selected` and `aria-label` reads were dropped. This is the precedent for widenings that arrive by merge being the founder's fork (decision/each-aria-attribute-read-once, decision/adapter-crossing-carries-descriptions).

@history_item n=20 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (`keyboard_compat`, the coverage exclusion) — dioxus-native-dom gained upstream's crate-private `keyboard_compat` bridge from keyboard-types 0.8 to Dioxus 0.7. Both coverage lines exclude it until escher takes upstream's retirement of it. The Dioxus 0.8 take reaches that retirement, so the exclusion and its pin need re-reading (decision/coverage-excludes-keyboard-compat, decision/keyboard-compat-coverage-exclusion).

@history_item n=21 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the bump crate leaves the workspace) — upstream deleted `bump`, and blitz packages are now versioned together only through the workspace version and the `=` pins. This is the precedent for an upstream crate deletion taken by sync (absence/in-repo-release-tool, decision/lockstep-package-versions).

@history_item n=22 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — seven direct engine changes for every document (`inert`, `disabled` on form controls, toggle per role, the `contain: paint` hit stop, `absolute_position`, the client rect under transforms, the password caret), later ratified. Their merge surface is owned by the upstream-sync carry. The merge must keep each of them (decision/inert-honoured, decision/disabled-read-on-form-control-only, decision/hit-stops-at-paint-clip).

@history_item n=23 marker=2026-10-18-upstream-sync-determinism-and-traces
**2026-10-18-upstream-sync-determinism-and-traces** — the first partial take: a no-commit cherry-pick of `fb8d6c15e` with the merge base unmoved, because the Taffy-main commits would have put a git Taffy in the manifest. This started the cherry-pick series this chunk now replaces with a real merge. `git cherry` reads those takes as `+` (decision/upstream-sync-merge-base, decision/upstream-taken-by-cherry-pick).

@history_item n=24 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the document clock (`DocumentConfig.clock`; no `Instant::now()` in blitz-dom outside `clock.rs`) is an API addition flagged upstreamable. Its merge surface (`config`, `clock`, `document`, `resolve`, `iframe`, `scrolling`, `events/pointer.rs`) is owned by the sync carry. The scope names the clock invariant as a resolution to keep (decision/document-clock, decision/no-wall-time-outside-clock).

@history_item n=25 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — cherry-picked upstream's `:focus-within`, the not-in-document focus refusal and the focus redraw (#1226, #1228), ratified. These are upstream content already in the tree, so the merge meets them as duplicate edits.

@history_item n=26 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — the select model (keys, typeahead, `apply_select_choice`, `CHECKED` choice) is an engine change for every document. Its merge surface by file is owned by the sync carry. The scope names the select model as a resolution to keep (decision/select-choice-single-path, decision/select-keyboard-model).

@history_item n=27 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (the take read at `f9cbb2756`) — ten commits were cherry-picked. Git Stylo 0.23-pre, the Taffy-main commits and Dioxus 0.8.0-alpha.3 were left to wait for the next version and owned by a residual. That residual is part of the left-out table this chunk re-reads. The flake rust-bin pin moved to 1.91.0 to match `rust-version` (decision/flake-rust-pin-matches-rust-version).

@history_item n=28 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (Parallel traversals serialised, mimalloc, `multi_doc` env reads) — `PARALLEL_TRAVERSAL_LOCK` and mimalloc were registered as process-wide state, and the `MULTI_DOC_*` env reads as occupied names. This is the precedent for registering an upstream example's env reads and statics, which applies to `BLITZ_CDP_PORT` (decision/parallel-traversals-serialised).

@history_item n=29 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (the merge surfaces and their owners; the coverage exclusion's clause) — recorded the range and keyboard merge surfaces' conflicts. The `keyboard_compat` exclusion now reads "until escher takes upstream's retirement of it", since upstream retires the file in `286874cc1`, which is in this take.

@history_item n=30 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (six upstream behaviour changes ratified for this take) — set the pattern: an upstream behaviour change for every document is taken only when ratified for its take, and it is not counted among escher's direct engine changes. The take of 212 commits falls under this rule (decision/upstream-behaviour-changes-ratified-per-take).

@history_item n=31 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (`document_may_load`) — upstream's `file:` script-load refusal was taken as a narrowing, not a widening. It is part of the `file:` refusal the scope names as a resolution to keep (decision/file-script-load-refused).

@history_item n=32 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the take read at `ebacd68bd`; the merge surfaces read) — 24 commits were taken and 16 left out, each with a measured reason. It recorded the `mutator.rs` `set_attribute` conflict resolution (upstream's `open` arm after the range re-sanitize arm) and moved the merge-surface owners on. The runtime/bootstrap split, the `::before`/`::after` flow vote, IME on editor construction, the redraw route, `:open` and the publish workflow's dioxus-cli `0.8.0-alpha.3` all came in this take. Content already taken meets the merge as duplicate edits.

@history_item n=33 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (`TMPDIR` at a second test-only site) — registered upstream test helpers' temp directories as test-only occupied resources. This is the precedent for registering read and write sites in upstream tests that the merge brings.

@history_item n=34 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the eight engine changes) — eight more direct engine changes, flagged upstreamable. Upstream `0980cce5f`'s `hit_inner` hunk is a textual conflict with the non-invertible-transform answer, and its merge surface is owned by the sync. This is a known conflict this merge must resolve with both intents kept (decision/non-invertible-transform-takes-no-hit, decision/visible-region-through-transforms).

@history_item n=35 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (upstream's both-modes exception ended) — upstream's `pseudo_element_flow_classification` was made to run both layout modes. Later the rule was relaxed for upstream-taken targets (n=44). This matters for upstream's new test files.

@history_item n=36 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list recorded) — the select list is a direct engine change for every document. Its merge surface is sixteen files, owned by the upstream sync. The scope names "the select model and its list" as a resolution to keep (decision/select-list-engine-change, decision/select-list-painted-in-scene).

@history_item n=37 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a failed stylesheet through `NetHandler::error`) — trap: a trait default method can hide a take's fix. The do-nothing default compiled silently through `CountedHandler`, so a later take must sweep the implementers of any trait it extends (contract/net-handler/error).

@history_item n=38 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the queued `mounted` event's guard) — upstream's test `element_removed_before_its_mounted_event_is_flushed` was left out because Dioxus 0.7 delivers one mounted event. It was carried to return with `286874cc1`, so the Dioxus 0.8 take re-reads it (decision/stale-mounted-event-skipped).

@history_item n=39 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the Stylo prefs behind a `Once`; `background-clip: text` left out) — `init_stylo_prefs` uses the Stylo 0.22 pref names instead of upstream's 0.23 `set_pref!` names. `ac6f9f582` was left out because `BackgroundClip::Text` is gecko-only on Stylo 0.22. The Stylo 0.24 move ends both reasons, so both need re-reading (decision/stylo-prefs-set-once, absence/background-clip-text).

@history_item n=40 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the key-press panic's fix; the twelve ratified; `AGENTS.md` stays out; merge-surface owners) — `deallocate_anonymous_block` is a direct engine change. Twelve upstream commits were ratified for their take. Upstream's `AGENTS.md` stays out of the tree. The merge-surface owner moved to «Upstream sync ahead of determinism and exploration». Each of these is a left-out reason or kept change that this chunk re-reads (decision/anonymous-block-dealloc-detaches, decision/upstream-agents-md-excluded).

@history_item n=41 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the test-only binds and no served port; the upstream line at `e42ee1f25`; the `embolden` step) — the TCP loopback binds in blitz-net's tests were registered as test-only, with no served port. Cells' stale reading taught a trap: re-read a flagged-old reading. The `embolden` step joined `fast`. The CDP server is a served listener, unlike a test bind (absence/served-network-port).

@history_item n=42 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — pulldown-cmark moved to 0.13.6 and `deny.toml` gained the RUSTSEC-2026-0429 ignore. Reading a node's component and a signal's readers depends on Dioxus 0.7.10's public API, while hook-local state is `pub(crate)`. The scope re-reads `deny.toml`'s ignores, and the Dioxus 0.8 move must keep these readings (decision/owning-component-via-public-api, decision/graph-state-by-registration).

@history_item n=43 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — ported upstream `286874cc1`'s buffered render onto the Dioxus 0.7 bridge (`MutationBuffer`), flagged not upstreamable. Its merge surface (`dioxus_document.rs`, `mutation_buffer.rs`, `lib.rs`) is to be "replaced whole by upstream's 0.8 file at the 0.8 move". Its witnesses are the probe's `…_spawned_acts_*` tests, because upstream's ported test is insensitive on 0.7. The dioxus-core 0.7.10 first-poll paths are recorded. The Dioxus 0.8 take reaches all of this (decision/bridge-buffered-render, decision/render-with-document-unborrowed).

@history_item n=44 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph registry and the bridge's dispatch; async handler writes) — `handle_event` sets the registry dispatch, `mutation_writer.rs` was byte-equal to upstream `434e0661`, and `use_graph_state` takes dioxus-core types. Async writes landing inside their dispatch, and the watcher not re-subscribing, were measured on dioxus 0.7.10 and need re-measuring on 0.8 (decision/dioxus-event-path-records-opt-in, decision/signal-watcher-no-resubscribe).

@history_item n=45 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the reached record on the bridge's dispatch) — `handle_event` writes the reached record before `runtime.handle_event`, with no borrow across `render_immediate`. This is another escher edit on the bridge's event path that the Dioxus 0.8 merge must keep (decision/reached-record-before-dispatch).

@history_item n=46 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — the 29-pick take at `563d3db73` was ratified as engine behaviour changes. It added the decisions on unresolvable URLs, lossy decoding and default font fallback. `pinned-actions` widened to every workflow. `both-layout-modes` gained the exception that upstream-taken blitz-tests targets are kept as taken. This is the latest cherry-pick reading before this merge, and the exception applies to upstream's new tests (decision/both-layout-modes, decision/upstream-take-engine-changes).

@history_item n=47 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — engine sub-resource requests carry an initiator, blitz-net refuses `file:` with `FileForbidden`, and blitz-dom builds `Request` only through the stamping constructor (a clippy `disallowed-methods` guard). The scope names "the stamped sub-resource request and the `file:` refusal" as resolutions to keep (decision/sub-resource-requests-carry-initiator, decision/blitz-dom-stamped-requests, decision/url-scheme-routing).

@history_item n=48 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — one run seed is held as a Dioxus root context in the ungated `seed` module, and no PRNG crate is taken. This root-context mechanism must survive the Dioxus 0.8 move (decision/one-run-seed, decision/no-prng-crate).

@history_item n=49 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — stand typography falls back to the bundled DejaVu Sans, and stand readings are identical on every CI OS (`stand_cross_os_readings`). The parley 0.13 and skrifa moves can shift these readings, which the scope says are re-measured, never loosened (decision/stand-typography-dejavu-sans, decision/stand-readings-same-on-every-ci-os).

@history_item n=50 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** (std Result under the Dioxus prelude) — on Android, the Dioxus prelude glob brings manganis's one-parameter `Result`, so prelude-globbing files spell `std::result::Result`. Dioxus 0.8's prelude may change what the glob brings (decision/std-result-under-dioxus-prelude).

@history_item n=51 marker=2026-11-14-remote-pages-local-files
**2026-11-14-remote-pages-local-files** — in the browser app, a navigation a non-`file:` page starts to `file:` is dropped, and the `file-url-load-rule` was amended. An upstream rewrite of the browser's navigation provider must keep this (decision/file-url-load-rule).

@history_item n=52 marker=2026-11-15-upstream-sync-errors-and-explanation
**2026-11-15-upstream-sync-errors-and-explanation** — `@font-face` registers `font-stretch` (ratified behaviour change). `no-git-dependencies` gained the Taffy-only exception. Demo shells are kept outside the mutation tier. The scope's boundary "no git dependency save Taffy's pinned rev" rests on this (decision/no-git-dependencies, decision/font-face-css-descriptors).

@history_item n=53 marker=2026-11-15-taffy-from-upstream-git
**2026-11-15-taffy-from-upstream-git** — Taffy moved to upstream's git source at one rev. Eight blitz commits were ratified for that take alone ("the next Taffy take asks its own"). `0980cce5f` and its fix `ebacd68bd` stayed out because of the reversed-scroll-origin change. The scope reads Taffy's rev as equal on both sides (decision/taffy-layout).

@history_item n=54 marker=2026-11-16-taffy-past-1372
**2026-11-16-taffy-past-1372** — took Taffy past #1372 at `a3997d55` with reversed scroll origins, but kept upstream's scroll-shifted geometry and its self match out to preserve the own-scroll client-rect rule. The deferral row was replaced by `deferred-stylo-dioxus-bumps` (Stylo 0.22, Dioxus 0.7.3 wait). This merge moves exactly those deferred pins, so the row and the kept-out geometry must be re-read (decision/client-rect-own-box, decision/deferred-stylo-dioxus-bumps, decision/reversed-scroll-origins).

@history_item n=55 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — escher-telemetry's capture is a process-wide dispatcher pair, the driver installs no global subscriber, and the sink's target filter drops outside targets. This is part of the telemetry surface the sink-allowlist re-check reads against new upstream log sites (contract/telemetry/target-filter, decision/capture-dispatcher-pair).

@history_item n=56 marker=2026-11-17-upstream-sync-ahead-of-polish-and-ship
**2026-11-17-upstream-sync-ahead-of-polish-and-ship** — the AnyRender 0.15 set was taken, with vello-hybrid renamed vello-gpu (renderer rows and the `seven_guis` feature exception). A second skrifa and read-fonts version came in, so the font rule became "skrifa follows parley". Scroll offsets are clamped after layout. The free-disk rows moved to test-plan. This is the last sync before this chunk: the take now moves skrifa 0.44 → 0.49 and parley 0.12 → 0.13, so the skrifa rule needs re-reading (decision/skrifa-matches-parley, decision/window-renderer-chosen-at-compile-time, decision/scroll-offsets-clamped-after-layout).

@history_item n=57 marker=2026-11-19-build-directory-held
**2026-11-19-build-directory-held** — the `fast` leg ends with the dev-host target sweep, which is skipped under `CI` and never reddens the leg, and `CI` is now an occupied env name. The scope's proof runs `ci-leg.sh fast`, and a lockfile-heavy merge mints new unit hashes under `target/` (decision/local-pre-push-gate, contract/ci-leg-legs/fast).
