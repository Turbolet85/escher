@history_item n=1 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** — (sync base recorded) first upstream merge (23354585, conflict-free, additive) added the `Upstream sync:` line to §Project Intent as the next sync's merge base, and re-pointed 113 citations by the merge's line map; the precedent for the sync line and the citation sweep this take must redo.

@history_item n=2 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — (merge base, dependency set) second sync took upstream's dependency set whole with no `cargo update`; the coupling rule (parley, skrifa, vello move together) stated unchanged — the coupled-pins rule this take's AnyRender 0.15 move must honour (decision/font-deps-match-renderers, decision/skrifa-matches-parley-vello).

@history_item n=3 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — (blitz-tests one target per file) kept every blitz-tests file its own target; upstream's `all` target held at `test = false`, pinned by `BlitzTestsTargetsTest`; a sync that takes `autotests = false` or drops the line turns the pin red — the rule upstream's new test files in this take run under (decision/blitz-tests-target-per-file, decision/blitz-tests-single-test-target).

@history_item n=4 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — (LayoutPassState, bounds reader renamed) upstream moved Taffy's tree traits off the document and removed `unrounded_absolute_position`; escher's one edit moved `visible_region` onto the new reader — the precedent for re-applying escher's readers when an upstream layout/scroll commit (here `e5305a4c1`) reshapes geometry code.

@history_item n=5 marker=2026-10-06-project-readme
**2026-10-06-project-readme** — README is escher's own front page; at an Upstream sync an upstream change to `README.md` resolves to escher's version — a standing conflict rule for this take.

@history_item n=6 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — (taffy from the registry, pins) the earlier sync of the same epoch name took upstream's pin set whole, workspace version and in-repo `=` pins moved together; trap recorded: the coupled-pins warning moves with the registry — precedent for "a take carries upstream's pins as one set or none" (decision/shared-workspace-package, decision/lockstep-package-versions).

@history_item n=7 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — (bridge drops a removed node, 28-name falsy predicate) `mutation_writer.rs` made byte-identical to upstream; a bridge surface later syncs must keep equal (later re-asserted at the ui-graph chunk).

@history_item n=8 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — (keyboard-types 0.8, `keyboard_compat`, coverage exclusion) the coverage leg excludes `keyboard_compat.rs` until escher takes upstream's retirement of it (in `286874cc1`, Dioxus 0.8, still out) — a take touching the bridge must keep the exclusion clause true.

@history_item n=9 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — (the builder's merged ARIA reads) upstream's ARIA reads merged with escher's: each attribute read once, upstream's bool-parsed `aria-selected` and unfiltered `aria-label` dropped; descriptions to the adapter only — the accessibility state reads a resolution must keep (decision/each-aria-attribute-read-once, decision/adapter-crossing-carries-descriptions).

@history_item n=10 marker=2026-10-18-upstream-sync-determinism-and-traces
**2026-10-18-upstream-sync-determinism-and-traces** — first partial take: a no-commit cherry-pick with the merge base unmoved, left-out commits keeping their measured reasons; the shape this chunk's scope follows.

@history_item n=11 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — `:focus-within`, focus outside the document and the focus redraw taken as upstream engine behaviour for every document; first ratified-per-take pattern; FOCUS_WITHIN writes mark no changed-set entry — the changed-set contract a resolution keeps.

@history_item n=12 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — (take at `f9cbb2756`) ten commits taken by founder's word; Stylo 0.22, Taffy, Dioxus 0.7.3 deferrals and the left-out delta's owner; the flake rust-bin pin aligned to 1.91.0 (decision/flake-rust-pin-matches-rust-version); mimalloc registered.

@history_item n=13 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — (dropped-node hit skip, replaced element's natural height) upstream engine changes for every document ratified for the take; decision/hit-and-paint-skip-dropped-nodes originates here, beside escher's hit stops.

@history_item n=14 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — (merge surfaces, the `inline` paths) range and keyboard merge surfaces read at the take, one import-list conflict resolved to upstream's list plus escher's; `layout/inline.rs` became a directory — precedent for reading each merge surface and its owner at every take.

@history_item n=15 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (six upstream behaviour changes ratified for this take) upstream behaviour changes for every document are ratified per take and not counted among escher's direct engine changes; a narrowing and a realization are named as such — the pattern behind decision/upstream-behaviour-changes-ratified-per-take this take's background, shear, SVG-image and scroll-clamp commits fall under.

@history_item n=16 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (`document_may_load`) `file:` script loads only for a `file:` document, recorded as a narrowing — the origin of decision/file-script-load-refused; the take's external SVG images (`7b8712cba`) are read against it.

@history_item n=17 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (merge surfaces at `ebacd68bd`) the one conflict `mutator.rs`'s `set_attribute`, resolved by placing upstream's arm after escher's range re-sanitize arm as an independent `if`; keyboard activation's events files untouched — the resolution pattern and surfaces to re-read.

@history_item n=18 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (`::before`/`::after` flow, one-mode test) an upstream test file asserting one layout mode was named an exception to the both-modes convention (later ended, then re-stated as the upstream-taken exception) — precedent for upstream's new tests in this take (decision/both-layout-modes).

@history_item n=19 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (the upstream take read at `ebacd68bd`, publish workflow's dioxus-cli, citations re-pointed) 24 taken / 16 left out with measured reasons in `evidence/lock.md`, manifests and lock byte-identical; publish workflow upstream-only and repository-guarded; citation sweep after the take's added lines.

@history_item n=20 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the eight engine changes) eight more ratified direct engine changes (visible_region through transforms, non-invertible no hit, focus fixup, tree focus fallback, native toggled, multiple select, optgroup, checkbox/radio change), each upstreamable, their merge surface a line a sync owns; `hit_inner` a textual conflict with upstream `0980cce5f` — merge surfaces this take must keep (decision/hit-focus-form-engine-changes).

@history_item n=21 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the free-disk steps and what they show) `Stand checks repeated` got a pinned `jlumbroso/free-disk-space` step; the record states it is NOT shown to cure the out-of-disk red (CI#41190771843, `Free space left: 5 MB`), the red job alone stood on image `20261018.96.1` whose free disk is unread — the exact hypothesis this chunk's disk line must measure.

@history_item n=22 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (`matrix_test`'s free-disk step) the matrix step was retired and `test_only_stand_repeat_frees_disk` pins that ONLY the `stand-repeat` job frees disk (decision/only-stand-repeat-frees-disk) — a free-disk step for `Test [default features]` must amend this pin and its row, not slip past it.

@history_item n=23 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (a failed stylesheet through `NetHandler::error`; a failed load ending the wait) upstream's trait default method hid the fix behind escher's wrapping `CountedHandler`; trap recorded: a take sweeps every implementer of a trait it extends (contract/net-handler/error, decision/harness-counts-loads-at-net-provider).

@history_item n=24 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the queued `mounted` event's guard) upstream's test `element_removed_before_its_mounted_event_is_flushed` left out, reading 1 event on Dioxus 0.7 where it asserts 0 (decision/stale-mounted-event-skipped); carried, returns only with the Dioxus 0.8 move.

@history_item n=25 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the inline box's paint skip; blending painted; the culling bounds' port; `background-clip: text` left out) paint commits taken with no frame witness, the frame gap given an owner; `e42ee1f25` ported because upstream's version stood on the untaken `ac6f9f582`; blend layers pushed so the open select list still paints last — precedent for this take's paint commits (root/canvas backgrounds, shear) and the AnyRender chain left out.

@history_item n=26 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the key-press panic's fix) `deallocate_anonymous_block` clears children's `layout_parent`, a ratified direct engine change, upstreamable, merge surface owned by the next sync; trap: a panic probe reads green when its input never reaches the path (decision/anonymous-block-dealloc-detaches, decision/anonymous-block-drop-clears-layout-parent).

@history_item n=27 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the twelve commits ratified; the merge surfaces' reading and owner; `AGENTS.md` stays out; the upstream line at `e42ee1f25`) twelve taken ratified per take by commit; merge surfaces re-read row by row to `evidence/readings.md`; upstream `AGENTS.md` kept out (decision/upstream-agents-md-excluded); 36 taken / 21 left out.

@history_item n=28 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the `embolden` step of `fast`) `fast` ends on the font-embolden witness, pinned by `test_fast_leg_ends_on_the_font_embolden_witness` (decision/font-embolden-local-witness) — the local witness for this take's paint changes (shear, embolden) that macOS alone exercises in CI.

@history_item n=29 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the test-only binds and no served port; the Stylo prefs behind a `Once`) blitz-net's loopback TCP test binds registered as test-only; process-wide statics registered — the resource registers a take's new tests or statics must join.

@history_item n=30 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — (the hazard closed as an engine change) `286874cc1`'s buffered render ported onto the Dioxus 0.7 bridge, flagged not upstreamable; merge surface `dioxus_document.rs`, `mutation_buffer.rs` (replaced whole at the 0.8 move), `lib.rs` — the carried merge surface this chunk re-reads (decision/bridge-buffered-render, decision/render-with-document-unborrowed).

@history_item n=31 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — (Established Decisions: 4 rows) the last-but-two take: 29 commits at `563d3db73` ratified; added decision/unresolvable-url-skipped and decision/default-font-fallback — the row this take's commit-count rule (decision/upstream-take-engine-changes) re-states.

@history_item n=32 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — (Infrastructure Patterns; Conventions) pinned-actions widened to every workflow with the ref as trailing comment (decision/actions-pinned-to-commit-sha, `WorkflowPinTest`); both-layout-modes gained the upstream-taken exception — rules a new CI step and upstream's new tests obey.

@history_item n=33 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — (Established Decisions; Standard Contracts; Conventions) every engine sub-resource request carries its document as initiator; blitz-net refuses `file:` with FileForbidden; blitz-dom builds requests only through the stamping constructor, `Request::get` banned by clippy — the ban and refusal `7b8712cba`'s external SVG image loads must route through (decision/sub-resource-requests-carry-initiator, decision/blitz-dom-stamped-requests, decision/url-scheme-routing).

@history_item n=34 marker=2026-11-14-remote-pages-local-files
**2026-11-14-remote-pages-local-files** — (Established Decisions; Standard Contracts) the initiator rule and the `file:` load rule re-stated (navigation a non-`file:` page starts is none) — the current wording of decision/file-url-load-rule a new SVG image load is checked against.

@history_item n=35 marker=2026-11-15-upstream-sync-errors-and-explanation
**2026-11-15-upstream-sync-errors-and-explanation** — (no-git-dependencies except; deferred upstream dependency bumps; mutation tier) the taffy-only git exception added to decision/no-git-dependencies; the Stylo/Dioxus deferrals restated; the immediately preceding take, whose left-out table this chunk re-reads.

@history_item n=36 marker=2026-11-15-taffy-from-upstream-git
**2026-11-15-taffy-from-upstream-git** — (Stack; Established Decisions; Project Intent; layout changes ratified) Taffy moved to upstream's git source pinned at one rev (decision/taffy-layout), the no-git rule reworded under its Taffy-only except; eight Taffy-bump commits ratified for that take alone — "the next Taffy take asks its own".

@history_item n=37 marker=2026-11-16-taffy-past-1372
**2026-11-16-taffy-past-1372** — (Established Decisions: 5 rows; Standard Contracts: 3 rows; Project Intent) Taffy pinned at `a3997d55` (the rev upstream still pins); reversed scroll origins, signed scroll clamp, scroll_into_view skipping zero-range boxes, hit-clip content-box frame; client-rect-own-box keeps upstream's scroll-shifted geometry out; deferred-stylo-dioxus-bumps the heir row — the scrolling surface `e5305a4c1` (scroll offsets clamped at layout rounding) touches.

@history_item n=38 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — the client rect subtracts every containing-block scroll but the node's own, and the hit stops at an overflow-clipping padding box — direct engine changes, upstreamable; the scrolled-box bounds a resolution of `e5305a4c1` must keep (decision/client-rect-own-box, decision/hit-stops-at-paint-clip, decision/scrolled-box-bounds).

@history_item n=39 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — `scroll_into_view` widened in the engine to write every box on the containing-block chain, nested instantly; visibility read from geometry, never a hit — escher's into-view readings a scroll-clamping take must preserve (decision/scroll-into-view-whole-chain, decision/visibility-from-geometry-not-hit).

@history_item n=40 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (seven engine changes; the geometry readers) `inert`, `disabled` on form controls, toggle per role, hit at `contain: paint`, `absolute_position` less ancestors' scroll, client rect under transforms, password caret — the "seven" whose merge surface is carried sync to sync and re-read at this take.

@history_item n=41 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — (Document core and [Scrolling and input]) every engine time read goes through the document's clock, no `Instant::now()` in blitz-dom outside `clock.rs`; merge surface carried — the clock invariant a taken commit with a time read must keep (decision/document-clock).

@history_item n=42 marker=2026-10-16-handler-splits-and-readme
**2026-10-16-handler-splits-and-readme** — `handle_click`'s range and button arms split into private named functions, parents kept at upstream's floor so the sync diff stays out of escher's arms — why an upstream edit to a `handle_click` arm is re-applied by hand (carried as `cry-hvl3d4gp`).

@history_item n=43 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the list's paint; the list recorded; open list's exceptions) the select list is painted last inside `paint_scene`, past clips, its merge surface sixteen files owned by a sync; the hit answers the list first — the paint order this take's root/canvas background commits must not displace (decision/select-list-painted-in-scene, decision/paint-to-paint-scene).

@history_item n=44 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — (the harness's screenshot, the password mask) the harness paints through `anyrender` + `anyrender_vello_cpu` and composites over white after paint; the password mask a direct engine change in `render.rs` — the AnyRender 0.14 → 0.15 move reaches both, and moved screenshot readings are re-measured (decision/headless-screenshot-cpu-renderer, decision/harness-screenshot-white-after-paint, decision/password-input-painted-masked).

@history_item n=45 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — (the scopes, the tests and the stack) `png` taken with no package entering the lock; `stand_screenshot_repeat` byte-identity — a pixel reading the take's paint changes may move (decision/png-shot-encoder).

@history_item n=46 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — (the shot's same-host limit) a shot's PNG bytes differ between linux and macOS, so a cross-OS byte comparison holds no `screenshot` step — bounds how moved screenshot readings are compared per OS.

@history_item n=47 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — (Standard Contracts: 2 rows) stand readings identical on every CI OS (snapshot, lint, actions digests, no screenshot) and the DejaVu Sans fallback guarded by `stand_bundled_font_families` — the per-OS readings this take's shear/paint commits must leave equal (decision/stand-readings-same-on-every-ci-os, decision/stand-typography-dejavu-sans).

@history_item n=48 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — every ci.yml `uses:` pinned to a 40-hex SHA with its ref as a trailing comment, workflow-level `permissions: contents: read`, audit via cargo-deny over `deny.toml` — the constraints a new free-disk step in `Test [default features]` must meet (decision/actions-pinned-to-commit-sha, decision/cargo-deny-and-llvm-cov-tools, decision/dependency-and-licence-audit).

@history_item n=49 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — (shared test cache, cache budget re-measured over) a `Cargo.lock` change re-mints every rust-cache key and evicts over the 10 GB budget — an AnyRender lock move cold-starts every leg (decision/rust-cache-on-compiling-jobs).

@history_item n=50 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — one leg script for CI and host, `fast` the local pre-push gate; the dead `ubuntu-24.04` Free Disk Space condition kept unchanged — the earliest record of the free-disk steps the disk line revisits (decision/local-pre-push-gate, decision/one-ci-leg-runner).

@history_item n=51 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — (legs, the new job) `stand-repeat` zero-failure three rounds, upstream's blitz-dom feature-subset clippy line, audit with licences over the whole workspace — gates the take must pass (decision/stand-repeat-zero-failure-budget, decision/blitz-dom-feature-subset-clippy).

@history_item n=52 marker=2026-10-09-audit-corrections-agent-surfaces
**2026-10-09-audit-corrections-agent-surfaces** — fork CI's package install is one bounded, retried `apt-install.sh`, pinned by `InstallScriptTest`; a CI edit keeps every install through it.

@history_item n=53 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the `a11y` leg runs fourteen named targets on one line, pinned by `test_ci_workflows.py` — the a11y leg the scope requires green after the take (decision/a11y-leg-widens-without-new-job).

@history_item n=54 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — the sink admits only engine and `escher_` target prefixes, dropping every other target whole at every level — the allowlist this chunk re-checks for any new `tracing`/`log` call site in the taken delta.

@history_item n=55 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — tracing gated per call site with a `cfg(not(feature = "tracing"))` fallback in the engine and upstream crates, escher's own crates ungated — the form a new upstream call site must take (decision/tracing-gated-per-call-site).

@history_item n=56 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — (Standard Contracts: target filter; Conventions) the capture layer counts outside-target records while the sink drops them whole; a capture is opened before a check's own recorder — the standing capture checks (0 needles) a taken log site must keep green (decision/capture-before-own-recorder).

@history_item n=57 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — (`Parallel` style traversals serialised, mimalloc and the `multi_doc` env reads) upstream's new env reads and process-wide statics registered as occupied resources — the sink/env re-check this take's delta needs.
