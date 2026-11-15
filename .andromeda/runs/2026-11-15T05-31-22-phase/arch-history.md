@history_item n=1 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — (fork CI through one leg script) every linux CI job runs `bash .github/scripts/ci-leg.sh {leg}`, `fast` the local pre-push gate, rust-cache keyed per job; matters because the take's proofs are those legs (decision/one-ci-leg-runner, decision/local-pre-push-gate) and a lockfile change re-mints every cache key.

@history_item n=2 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — (audit, a11y, coverage legs) added the `audit` (cargo-deny), `a11y` and `coverage` legs and SHA-pinned actions, and recorded the trap that a `Cargo.lock` change re-mints every rust-cache key and starves the legs that restore last; the scope must hold the audit and a11y legs green after the take.

@history_item n=3 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** — the first upstream merge: recorded the `Upstream sync:` line as the next sync's merge base and re-pointed citations into merged files by a measured line map; precedent for how this sync records its pin and moves citations.

@history_item n=4 marker=2026-10-06-project-readme
**2026-10-06-project-readme** — README is escher's front page; at an upstream sync an upstream change to `README.md` resolves to escher's version (decision/readme-is-escher-own) — the rule for any README conflict in this take.

@history_item n=5 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — the sink admits only engine and `escher_` target prefixes, dropping every other target at every level (decision/telemetry-target-prefix-drop); the scope's sink-allowlist re-check reads every new upstream `tracing`/`log` site against this rule.

@history_item n=6 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (manifest citations re-pointed) — trap: any insert into the root manifest's members list or dependency block shifts every citation below it in all masters; a take that edits `Cargo.toml` owes the re-point.

@history_item n=7 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (founder's rulings) — made "Provisional discharge" a playbook rule: PROVISIONAL items are discharged in one batch at each epoch boundary; this chunk, run with the founder away, marks each fork answer PROVISIONAL under it.

@history_item n=8 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (merge base, Parley pin, merged dependency set) — the second sync took upstream's dependency set whole with no `cargo update`, kept the coupled parley/skrifa/vello rule (decision/skrifa-matches-parley-vello) and moved the sync line; precedent for "a take carries upstream's pins as one set or none".

@history_item n=9 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (blitz-tests one target per file) — upstream's `autotests = false` / single `all` target was kept unbuilt (`test = false`) and pinned by `test_blitz_tests_targets.py`; a sync that drops that line turns the pin red — the rule the scope applies to upstream's new and edited test files (decision/blitz-tests-target-per-file).

@history_item n=10 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (Taffy traits, bounds reader renamed) — escher's one edit to upstream-owned code moved `visible_region` onto the renamed `physical_unrounded_geometry`; precedent for a resolution that keeps escher's reader when upstream renames under it.

@history_item n=11 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — the scrolled-box bounds and the hit stopping at an overflow-clipping padding box, direct engine changes flagged upstreamable (decision/client-rect-own-box, decision/hit-stops-at-paint-clip, decision/scrolled-box-bounds); conflicts in `hit_inner` or the bounds readers must keep both.

@history_item n=12 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (accessibility builder state reads) — the builder's closed-set, lower-case ARIA state reads are a direct engine change (decision/aria-state-values-exact-lowercase, decision/a11y-aria-states); a take touching `accessibility.rs` must keep them.

@history_item n=13 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the password mask) — the password paint mask is a direct engine change with merge surface `render.rs` and `document.rs` (decision/password-input-paints-masked); the take's SVG image paint clipping and isolated-layer commits (`d176ac4e6`, `beaa796f4`) land in the same paint code.

@history_item n=14 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (taffy from the registry) — retired the git Taffy for registry "0.15", leaving no git dependency (decision/no-git-dependencies); its trap — taffy now moves with the registry — is exactly the Taffy-main question this chunk measures and routes to the founder.

@history_item n=15 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (bridge drop, default sheet, 28-name falsy predicate) — `mutation_writer.rs` became byte-identical to upstream and the default sheet arrives through `DocumentConfig::default()` (decision/ua-stylesheets-used-as-given, decision/falsy-boolean-attribute-cleared); a take must keep that file upstream's.

@history_item n=16 marker=2026-10-13-range-input-interaction
**2026-10-13-range-input-interaction** — the range model, a direct engine change whose merge surface is seven upstream files plus `node/range.rs` (decision/range-*); the take's `hr` presentational-hint and attribute refactor commits may meet `node/element.rs`, `construct.rs` and `mutator.rs`.

@history_item n=17 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — keyboard activation and a click focusing a button, direct engine changes with merge surface `events/keyboard.rs`, `events/mod.rs`, `events/pointer.rs`, `node/element.rs` (decision/keyboard-activation-enter/space, decision/click-focuses-button); conflicts there keep both sides.

@history_item n=18 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — added `stand-repeat` with a zero-failure budget, the licence audit over the whole workspace, upstream's blitz-dom feature-subset clippy line and the coverage floors (decision/stand-repeat-zero-failure-budget, decision/blitz-dom-feature-subset-clippy, decision/per-crate-coverage-floors); every one is a gate this take must keep green.

@history_item n=19 marker=2026-10-16-handler-splits-and-readme
**2026-10-16-handler-splits-and-readme** (range and button arms as private functions) — `handle_click`'s escher arms became private named functions so the next sync's diff stays out of arms escher does not own; ground of the `cry-lrzp6e2g` carry that an upstream `handle_click` arm edit is re-applied by hand.

@history_item n=20 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the sync at `23b9b26a9`) — the last whole merge base; the four Taffy-main commits (`1a11761bc` · `4a5ab9cc9` · `510b592ab` · `1e5479ae6`) first left to wait because they put a git Taffy in the root manifest — the same four this chunk measures; keyboard-types 0.8 with Dioxus's 0.7 beside it (decision/keyboard-types-named-by-absolute-path).

@history_item n=21 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (keyboard-types 0.8 surfaces) — the coverage leg excludes `keyboard_compat.rs` until escher takes upstream's retirement of it (decision/coverage-excludes-keyboard-compat); a take of `286874cc1` or the 0.8 move would move that clause.

@history_item n=22 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the builder's merged ARIA reads) — upstream's ARIA reads were merged with each attribute read once, upstream's bool-parsed `aria-selected` and unfiltered `aria-label` dropped (decision/each-aria-attribute-read-once, decision/adapter-crossing-carries-descriptions); a boundary widening by upstream merge needs the founder's word.

@history_item n=23 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (seven engine changes) — `inert`, `disabled` on form controls, the toggle per role, the `contain: paint` stop and `absolute_position`, each a direct engine change whose merge surface is carried to the next sync; the scope names `inert`, the hit stops and the accessibility state reads among what each resolution keeps.

@history_item n=24 marker=2026-10-18-upstream-sync-determinism-and-traces
**2026-10-18-upstream-sync-determinism-and-traces** — the first partial take: a no-commit cherry-pick of `fb8d6c15e` alone, the merge base unmoved, Taffy staying the registry because the four Taffy-main commits put a git Taffy in the manifest (decision/upstream-sync-merge-base, decision/upstream-taken-by-cherry-pick); the shape this take follows.

@history_item n=25 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the document's clock: no `Instant::now()` / `SystemTime::now()` in blitz-dom outside `clock.rs` (decision/document-clock, decision/no-wall-time-outside-clock); the scope's "clock invariant" — any upstream time read the take brings goes through the clock.

@history_item n=26 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified) — three delegate-answered PROVISIONAL batches discharged by the founder at the epoch boundary; precedent for this chunk's PROVISIONAL marks with the founder away.

@history_item n=27 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — a cherry-picked upstream engine change for every document (`:focus-within`) taken PROVISIONAL on a delegate answer and discharged later; the merge base unmoved, the Taffy-main commits still waiting.

@history_item n=28 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — the select model, a direct engine change for every document whose merge surface by file rides the upstream-sync CARRY (decision/select-model-engine-change); the scope keeps the select model through every resolution.

@history_item n=29 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (the take at `f9cbb2756`) — thirteen commits by cherry-pick; git Stylo 0.23-pre, the Taffy-main commits and Dioxus 0.8 deferred to the next version (decision/deferred-upstream-dependency-bumps); the WPT runner's direct `mimalloc` the one exception to workspace dependencies (decision/workspace-dependencies).

@history_item n=30 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (Parallel traversals, dropped-node hit skip) — the `PARALLEL_TRAVERSAL_LOCK` and the hit skipping dropped nodes in `hit_inner` were taken as upstream engine changes (decision/parallel-traversals-serialised, decision/hit-and-paint-skip-dropped-nodes); `hit_inner` is a recurring merge surface.

@history_item n=31 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (merge surfaces and the coverage exclusion's clause) — the range and keyboard merge surfaces re-read at the take (the one conflict `construct.rs`'s import list) and the `keyboard_compat` exclusion re-worded "until escher takes upstream's retirement" (`286874cc1`); the per-take re-read this chunk repeats.

@history_item n=32 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the `a11y` leg runs fourteen named targets on one locked line, pinned by `test_ci_workflows.py` (contract/ci-leg-legs/a11y); the scope requires this leg green after the take.

@history_item n=33 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** (stand-repeat membership held jointly; complexity reading) — `stand-repeat`'s named list is held jointly by `test_stand_repeat_leg_runs_three_rounds` and the sweep test, and the cognitive-complexity ceiling of 15 is a dev-host reading only (decision/cognitive-complexity-ceiling); an upstream test file added to the leg moves both pins.

@history_item n=34 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (six upstream behaviour changes ratified for this take) — set the pattern: upstream behaviour changes for every document are ratified per take and not counted among escher's direct engine changes (decision/upstream-behaviour-changes-ratified-per-take); the 16 commits' SVG, `hr` and `@font-face` changes need the same treatment, PROVISIONAL with the founder away.

@history_item n=35 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (merge surfaces read at `ebacd68bd`) — the one conflict was `mutator.rs`'s `set_attribute`, resolved by placing upstream's `open` arm after the range re-sanitize arm as a second independent `if`; precedent for resolving attribute-arm conflicts when the presentational-hint refactor (`0bdfe4fa3`) touches attributes.

@history_item n=36 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the take at `ebacd68bd`; browser and script citations) — eleven commits cherry-picked, sixteen left out with measured reasons; the browser `tab.rs` blocks this take moved are where the unjustified `expect_used` allow (`cry-eyudppev`, decision/justified-expect-unwrap) arrived.

@history_item n=37 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (`::before` / `::after` one-mode test) — upstream's test asserting one layout mode was first recorded as an exception to the both-modes rule; precedent for upstream-taken test files under decision/both-layout-modes.

@history_item n=38 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the eight engine changes) — `visible_region` through transforms, non-invertible transform takes no hit, focus fixup, tree focus to the window, native toggled, multiple select, optgroup, change on checkbox and radio — each flagged upstreamable, nothing sent; upstream `0980cce5f`'s `hit_inner` hunk is a textual conflict with the non-invertible answer (`cry-oglnc26y`).

@history_item n=39 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the free-disk steps; pulldown-cmark) — the `Stand checks repeated` job's free-disk step after an out-of-disk red, and an advisory fixed by a lock-only semver move; precedent for keeping CI and the audit green across a take that grows the build.

@history_item n=40 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list recorded) — the select list is a direct engine change for every document with a sixteen-file merge surface owned by the upstream sync; its rejected designs (decision/select-list-rejected-designs) and its paint inside `paint_scene` (decision/select-list-painted-in-scene) must survive the paint commits of this take.

@history_item n=41 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (no local gate builds `font-embolden`) — a paint change read green locally and red on macOS alone, so the `embolden` witness was owed; the scope requires the `embolden` witness after the take's paint commits.

@history_item n=42 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (taffy is no re-export) — taffy is part of blitz-dom's public API through signatures with no `pub use` (decision/taffy-layout); a git Taffy move would change that public surface.

@history_item n=43 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a failed stylesheet through `NetHandler::error`) — the trap: a trait default method can hide a take's fix (the do-nothing default compiled through `CountedHandler`), so a later take sweeps the implementers of any trait it extends (contract/net-handler/error, decision/failed-stylesheet-unblocks).

@history_item n=44 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (Stylo prefs behind a `Once`) — prefs are written with the Stylo 0.22 names, not upstream's 0.23 `set_pref!` names (decision/stylo-prefs-set-once); any upstream pref edit is translated, not taken whole.

@history_item n=45 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (`background-clip: text` left out) — `ac6f9f582` stays out because `BackgroundClip::Text` is gecko-only on Stylo 0.22 (absence/background-clip-text); its reason is among the left-out delta re-read against `ea07ee716`, and the culling port depends on it.

@history_item n=46 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the merge surfaces' reading and owner at `e42ee1f25`) — moved the merge surfaces and resolutions to a CARRY on the next sync, each flagged upstreamable and nothing sent; the `cry-oglnc26y` carry this chunk re-reads row by row.

@history_item n=47 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the key-press panic's fix) — `deallocate_anonymous_block` clearing children's layout parent, a ratified direct engine change, upstreamable (decision/anonymous-block-dealloc-detaches); a named merge surface, with the trap that a panic probe can read green when its input never reaches the path.

@history_item n=48 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the twelve commits ratified; `AGENTS.md` out; upstream line) — the twelve upstream commits ratified for their take apart from escher's changes, `AGENTS.md` kept out (decision/upstream-agents-md-excluded), and the upstream line at `e42ee1f25`, 36 taken, 21 left out; the left-out list `cry-qrse4pmp` re-reads.

@history_item n=49 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (blending painted; the culling port) — `mix-blend-mode`/`isolation` layers push in `render_element` with the open select list painted after them, and the culling bounds port; the paint code the take's SVG image clipping and isolated-layer commits meet.

@history_item n=50 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the `embolden` step of `fast`; test-only binds) — `embolden` became `fast`'s last step (contract/ci-leg-legs/embolden), and upstream tests binding loopback TCP were registered as test-only, no served port (absence/served-network-port); new upstream test binds in this take register the same way.

@history_item n=51 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the advisory fix and the deny.toml ignore) — a red audit fixed by a lock-only move and one per-ID `deny.toml` ignore with its reason (decision/dependency-and-licence-audit); the shape of an audit fix if the take's dependencies read red.

@history_item n=52 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the MountedData hazard closed) — the buffered render ported from `286874cc1` onto the Dioxus 0.7 bridge, flagged not upstreamable, merge surface `dioxus_document.rs`, `mutation_buffer.rs`, `lib.rs`, re-read at the 0.8 move (decision/bridge-buffered-render, `cry-2hvzi4ns`); Dioxus 0.8 stays out of this take.

@history_item n=53 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph registry and the bridge's dispatch) — the bridge's dispatch change is inert without a registry, with `mutation_writer.rs` byte-equal to upstream `434e0661` (decision/dioxus-event-path-records-opt-in); a take touching the bridge keeps both.

@history_item n=54 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the masters became rows verified against the code; from here a take's amendments name rows by handle (`+`/`~`/`−`), the form this chunk's wrap will write.

@history_item n=55 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Established Decisions) — the previous take: 29 picks at `563d3db73` recorded as engine behaviour changes, with unresolvable URLs skipped, lossy page decoding and the default-font fallback added (decision/unresolvable-url-skipped, decision/default-font-fallback); this take's pin moves on from `563d3db73` and its `@font-face` commit meets the font fallback.

@history_item n=56 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Conventions; Infrastructure Patterns) — upstream-taken blitz-tests targets are kept as taken even where they run one layout mode (decision/both-layout-modes except), and every workflow `uses:` is SHA-pinned with its ref as a trailing comment; both rules apply to this take's test files and any workflow edits.

@history_item n=57 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — blitz-dom builds a `Request` only through the stamping constructor, `Request::get` banned by clippy (decision/blitz-dom-stamped-requests), `StylesheetHandler` carries `document_url` and every sub-resource carries its initiator; `f5ea1caf1`'s `@font-face` loading is read against this ban (`cry-nnm3rzya`).

@history_item n=58 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — the stand loads DejaVu Sans alone and every generic and unavailable family lays out in it by fallback, readings identical across CI OSes (decision/stand-typography-dejavu-sans, decision/stand-readings-same-on-every-ci-os); `f5ea1caf1` passing `font-stretch` to fontique may move the stand's font matching and the per-OS tallies.

@history_item n=59 marker=2026-11-14-remote-pages-local-files
**2026-11-14-remote-pages-local-files** — narrowed decision/justified-expect-unwrap's except to the favicon test module, leaving `tab.rs`'s unjustified allow (`cry-eyudppev`) owed to the take that next touches the browser app; also restated decision/file-url-load-rule and decision/sub-resource-requests-carry-initiator.
