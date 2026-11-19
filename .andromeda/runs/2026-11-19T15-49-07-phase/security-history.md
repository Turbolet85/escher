@history_item n=1 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (signing and PR-token jobs repository-guarded; CI legs locked) — put `github.repository == 'DioxusLabs/blitz'` guards on release-cli, post-results and the WPT jobs, because a guard that checks only the ref re-arms signing on the fork's own branches. It also made every ci.yml cargo leg `--locked`. When the merge brings in upstream's workflow edits, each file must keep its repository guard (rows decision/publish-job-repository-guard, decision/publish-secrets-upstream-main-only, decision/release-publish-upstream-only, decision/wpt-post-results-trusted-scripts, decision/locked-builds).

@history_item n=2 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — gave ci.yml `permissions: contents: read`, pinned every ci.yml `uses:` to a 40-hex SHA and added the cargo-deny 0.20.2 audit job (ignores per ID only, each with a reason). A guarded upstream workflow was left at its upstream form so it would not re-arm on the fork. These are the CI rules a merge resolution must keep (decision/ci-token-read-only, decision/ci-actions-pinned-to-sha, decision/cargo-deny-audit-tool, decision/dependency-audit-ci-gate).

@history_item n=3 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — wired escher's stderr allowlist sink (engine targets keep only safe fields; content-named fields redacted). Upstream's `fmt::init()` stdout subscribers and the WPT runner's `env_logger` stay unscrubbed. Every new tracing target or log site the merge brings (the CDP server among them) is measured against this sink (decision/stderr-only-telemetry-sink, decision/engine-target-safe-fields, decision/telemetry-sink-target-allowlist).

@history_item n=4 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** (first upstream merge) — precedent for a sync. Upstream's switch to Taffy's `normal` changed the alignment fallback while the no-panic guarantee held. 40 citations into merged files were re-pointed by a measured line map. Script accessors the merge brought were recorded as realizations on the registered script-to-DOM crossing, not as boundary widenings. A later chunk applied the same rule to the merge's CSS accessors and `URLSearchParams`.

@history_item n=5 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — made the stable id the AccessKit `author_id`, so it leaves only through the platform adapter. Standing rule: any further consumer of the id re-opens the row. The merge's accessibility code has to keep this (decision/author-id-on-accessibility-nodes, decision/stable-id-exits).

@history_item n=6 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — added the anchored `{key}//{segment}` tier and its three-way uniqueness argument, ratified by the founder. A merge that touches dioxus-native-dom ids or the bridge must keep the grammar (decision/anchored-id-tier, contract/stable-element-id/*).

@history_item n=7 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — made the shell's poll rebuild the platform tree on change, a boundary widening ratified at the Epoch 3 boundary. Shell or accessibility code the merge touches must keep the refresh (decision/accessibility-tree-refresh-on-change).

@history_item n=8 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — registered the session's Unix-domain socket as a new listener and input surface. Precedent: "a new listener and a new input surface are a security-plan amendment first", and a TCP or UDP port was ruled out because it needs an auth surface. The same entry recorded third-party targets printing ids and names on a sink-installing host, with the trap that a "nothing in the logs" check over a host with no sink passes vacuously. This is the direct precedent for the CDP server's `127.0.0.1` listener (res-ipcah32k), which needs an amendment before it lands and is the founder's fork (decision/session-socket-sole-listener, absence/served-network-api, decision/driver-session-unix-socket-only).

@history_item n=9 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — drops a record from any target outside the allowlist whole, at every level including WARN and ERROR. The cost is that third-party warnings no longer print. New crates the merge adds (blitz-vibey-cdp-server, parley_code_editor) log under targets that this sink drops or scrubs. The scope's re-check of the sink allowlist has to confirm the capture checks still read 0 needles (decision/outside-target-dropped, decision/other-log-targets-dropped).

@history_item n=10 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — a settle reports a load in flight and never waits on one, so the absent request timeout cannot hang it. The rule for later work: a command that waits on a load with a clock is a new decision. Net or loader changes in the merge must keep this (decision/settle-never-waits-on-loads, absence/request-timeout).

@history_item n=11 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (three entries: the hint-less `@font-face` widening, PROVISIONAL; input rows re-read on the merged engine; dependency security after the sync) — first precedent for a boundary widening that arrives by upstream merge. The operator recorded it as PROVISIONAL because the call is the founder's, also when the widening comes by a merge. That matches this chunk's rule to collect such widenings in one file for the founder. The same sync re-read the `file:` read, `autofocus`, the Selection API and an env read on the merged engine, reviewed entering crates by hand (licence, repository, build script, proc-macro) and recorded a second `unsafe` and the ungated licence table.

@history_item n=12 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the hint-less `@font-face` source ratified) — the founder ratified the merge-borne widening as upstream's behaviour with a check of ours owed. This is the pattern a merge-borne widening such as the CDP listener follows to discharge. The same chunk widened the session socket to carry calls and answers.

@history_item n=13 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — set the founder's rule: a new field of a closed value set inside a ratified carrier is no widening, while authored text, an open value set, a new carrier or a new exit still is. Use it to judge which of the merge's changes go to the founder's widening file (decision/boundary-widening-rule, decision/closed-value-field-no-widening).

@history_item n=14 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — registered the MCP stdio crossing (no listener, no auth, the spawning client is the boundary) and kept the session socket as the one listener. A served TCP port from the CDP crate would break the "one listener" and "no served port" rows (decision/mcp-server-stdio-only, decision/mcp-stdio-no-auth).

@history_item n=15 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (pixels in band; the engine's password mask; `png`) — blitz-paint paints a password as bullets in every document and renderer, and a shot's base64 is written by hand with no `base64` crate. Paint code the merge touches (the parley 0.13 move, the new `anyrender_tiny_skia`) must keep the mask, and no `base64` crate may enter (decision/password-painted-masked, decision/no-base64-crate, decision/png-encodes-screenshots).

@history_item n=16 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (the 28-name falsy predicate; no git dependency; the sync's dependency delta) — took upstream's `is_bool_attr` set, making `mutation_writer.rs` byte-identical to upstream's. It wrote the per-sync lock record (what entered, left or moved; the audit green with `deny.toml` unchanged). This is the record form this sync's lock re-read follows (decision/falsy-boolean-attribute-removed, decision/upstream-sync-dependency-set).

@history_item n=17 marker=2026-10-13-stand-contrast-harness
**2026-10-13-stand-contrast-harness** — blitz-test-harness takes `color` from the graph through blitz-dom and adds no lock package. If the merge moves `color`, this reviewed bullet must be re-read (decision/color-crate-luminance-source).

@history_item n=18 marker=2026-10-13-range-input-interaction
**2026-10-13-range-input-interaction** — the range input's current value, sanitization and exits, a ratified boundary widening and engine change for every document. The merge resolution has to keep the range model (decision/range-*).

@history_item n=19 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — keyboard activation (Enter and Space), a button click's focus and `disabled` read by presence, ratified widenings in every document. The merge must keep these engine arms (decision/enter-activates-focused-control, decision/space-keyup-activates-focused-control, decision/button-click-focuses, decision/disabled-read-by-presence).

@history_item n=20 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — runs the audit `--workspace --all-features` over advisories and licences; the fork gates on licences on the founder's word. It also records that memmap2 is kept out by the `unsound` default, not by pruning. New and moved packages after the merge (Stylo 0.24, Dioxus 0.8 alpha, parley 0.13, skrifa 0.49, anyrender_tiny_skia) face this licence gate and audit (decision/licence-gate, decision/dependency-audit-cargo-deny).

@history_item n=21 marker=2026-10-16-fast-failing-mutants
**2026-10-16-fast-failing-mutants** — zip 2.4.2's ignores, reached only through `anyrender_serialize 0.8.0` ← `browser`, dropped when that chain reaches zip 9.0. The merge's AnyRender and browser changes must re-read whether those ignores and their reasons still hold (decision/audit-ignores-per-id-with-reason, decision/advisory-ignores-by-id).

@history_item n=22 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (merged ARIA reads; the bump CLI's rows leave; eight advisory ignores, the zbus family and the sync record) — upstream's `aria-labelledby`, descriptions and closed `aria-expanded`, `aria-disabled` and `aria-level` reads were ratified as widenings by upstream merge, with upstream's unfiltered `aria-label` read dropped in the merge. The zbus-family ignores rest on accesskit_unix requiring zbus 5. That entry's sync took upstream's set whole at `23b9b26a9`, which is this chunk's merge base. The merge must keep the trimmed `aria-label` read and re-read the zbus ignores (decision/aria-label-trimmed-once, decision/aria-labelledby-outranks-label, decision/descriptions-adapter-only, decision/merged-aria-states-closed-parse).

@history_item n=23 marker=2026-10-18-upstream-sync-determinism-and-traces
**2026-10-18-upstream-sync-determinism-and-traces** — a partial cherry-pick take, recorded PROVISIONAL and later ratified, that left upstream's Taffy-main commits out. Its lock record is one of the earlier left-out decisions res-7qgqygtj holds. This chunk re-reads each against `547d7514b`, and a reason that no longer holds puts the commit back in.

@history_item n=24 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the `inert` read and the toggle per role; `disabled` on form controls and the password caret; the PROVISIONAL batch ratified) — escher-made engine changes for every document. The merge must keep the `inert` subtree, the toggle per role, `disabled` presence on form controls and the password caret laid out by the bullets (decision/inert-subtree, decision/toggle-read-per-role, decision/aria-state-lowercase-exact-match).

@history_item n=25 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — `advance` moves the session's clock by exactly `ms`, and the scrollbar fade reads the document's clock. This is the clock invariant the scope names among the resolutions to keep (decision/advance-moves-virtual-clock-exactly, decision/scrollbar-fade-on-document-clock, decision/time-moves-only-on-advance).

@history_item n=26 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — the second partial cherry-pick take's lock record. git Taffy was again not taken, and the take was PROVISIONAL until ratified. It is part of the left-out history this merge re-reads.

@history_item n=27 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — an acting step waits on a load for at most `LOAD_WAIT`, and a settle still never waits. Net and loader changes in the merge must keep this bound (decision/bounded-load-wait).

@history_item n=28 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (the ten-commit take's lock record and tokio past RUSTSEC-2026-0386; `multi_doc`'s inputs, the traversal lock) — fixed an advisory by a lock-only tokio update, reviewed mimalloc's vendored C by hand and recorded Stylo 0.23-pre and Dioxus 0.8.0-alpha.3 as not taken. That sync also registered upstream's new env reads (`MULTI_DOC_*`) and the parallel-traversal lock. This merge now meets Dioxus 0.8 and Stylo 0.24 and must register any new env read, such as `BLITZ_CDP_PORT` (decision/parallel-style-traversal-serialised).

@history_item n=29 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (citations follow the runtime's module split; the eleven-commit take's lock record; the script fetch's `file:` refusal, a narrowing; `open` by presence; CSS property accessors; dioxus-cli in the publish workflow; `TMPDIR` at a second site) — upstream's `file:` refusal was recorded as a narrowing, not a widening, and the CSS accessors as a realization on the script crossing. dioxus-cli `0.8.0-alpha.3` already sits in the repository-guarded publish workflow. Its sixteen left-out commits feed res-7qgqygtj (decision/file-url-only-for-file-documents, decision/script-fetcher-file-data-only, decision/css-property-named-accessors, decision/open-pseudo-class-by-open-attribute).

@history_item n=30 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (free-disk action; pulldown-cmark past RUSTSEC-2026-0414; `visible_region` through transforms; select readers; the engine's `change`; `inert` focus; native checked state; non-invertible hit; `multiple` select; `optgroup`; Cells keys) — a set of escher engine changes for every document that the merge must keep: the select model, the hit stops, the into-view readings and the accessibility state reads (decision/visible-region-through-transforms, decision/non-invertible-transform-no-hit, decision/engine-change-event-checkable, decision/multiple-select-*, decision/optgroup-options-and-label, decision/accessibility-focus-built-nodes-only, decision/inert-focus-clear-silent, decision/cells-grid-keyboard).

@history_item n=31 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list's readers, state, keys, pointer, paint, scroll, `size`, the open-list shot as pixels, the list's crossing and log silence; the key-press panic owned by "the next upstream sync") — the engine-painted select list on every document, a series of ratified widenings. The merge must keep all of it, including blitz-paint's `paint_select_list` order and the engine colours (decision/select-list-*, decision/open-list-shot-pixels, decision/select-list-crossing, decision/select-size-hint-cap).

@history_item n=32 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (no served port with two test-only TCP binds; a failed load reaches the handler; the twelve-commit lock record; the key-press panic retired; the mounted event of a removed element; a stylesheet that fails blocks no render; blending; URLSearchParams; the `embolden` leg) — the closest prior sync. It registered upstream's loopback `TcpListener::bind` as test-only, served by no binary; a served CDP port goes beyond that. It recorded the trap that a trait's do-nothing default (`NetHandler::error`) can hide a fix behind a wrapper, so sweep the implementers of any trait a take extends. It left out upstream's removed-element mounted test because it reads 1 on Dioxus 0.7; that test is expected back with the Dioxus 0.8 move. It also added the locked `embolden` fast leg (decision/net-handler-error-default-noop, decision/failed-load-ends-wait, decision/failed-stylesheet-blocks-no-render, decision/skip-mounted-event-of-removed-element, decision/dropped-anonymous-block-clears-parent, decision/embolden-locked-fast-leg).

@history_item n=33 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (zip ignores gain RUSTSEC-2026-0429; residual owners as store rows; pulldown-cmark 0.13.6; advisories published after a green audit; the MountedData hazard pinned) — the standing rule is to run the audit leg locally before a push, because CI can go red on new advisories over an unchanged lock. The ignores' owners are residual rows (decision/local-audit-before-push).

@history_item n=34 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — ported upstream's buffered render onto the Dioxus 0.7.10 bridge (`MutationBuffer`, one borrow, then the mounted flush), with no panic path and no log site. With the merge's move to Dioxus 0.8.0-alpha.3, this escher-side port meets upstream's own buffered render. The resolution must keep the guarantee and its `dioxus_runtime_probe` pins (decision/dioxus-render-into-buffer).

@history_item n=35 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph answer's crossing; the bridge's dispatch and signal registry; the `clickable` reading; the reserved virtual-node id) — escher's additions to dioxus-native-dom's `handle_event` and its registry, which take dioxus-core types alone. The Dioxus 0.8 move must keep them inert without a registry and free of any state value (decision/graph-answer-crossing, decision/signal-registry-holds-no-state-value, decision/dispatch-recorded-only-with-signal-registry, contract/stable-element-id/virtual-node-form).

@history_item n=36 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — the actions read, the shared target check and the one actionable rule. Escher reads these in dioxus-native-dom and escher-driver, and the merge must keep them (decision/actions-read-shares-target-check, decision/one-actionable-rule, decision/actions-answer-unpaged).

@history_item n=37 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the reached record and `handler_coverage` as readers of the id) — the reached record is written inside the bridge's `handle_event` after its early returns. The Dioxus 0.8 move must keep it deduplicated, with no log site and no `NodeId` (decision/reached-handler-record-without-log, decision/reached-record-deduplicated-survives-removal).

@history_item n=38 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — moved the SHA pin guard to `WorkflowPinTest`, which reads every workflow file, so upstream workflow files the merge brings must also be SHA-pinned or the test fails. It added rows for upstream's decoding of untrusted page and stylesheet bytes and for typed presentational-hint parsing, both ratified by the founder (decision/ci-actions-pinned-to-sha, decision/untrusted-bytes-decoded-without-panic).

@history_item n=39 marker=2026-11-13-cognitive-load-corrective
**2026-11-13-cognitive-load-corrective** — removed the false "why" from the unresolvable-stylesheet-href row. The resolver answers none rather than panicking, and merged loader code must keep that (decision/unresolvable-stylesheet-href-no-load).

@history_item n=40 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** (Threat Model; Input Validation; Error Handling) — widened the `file:` refusal to every engine sub-resource load in blitz-net. It added the clippy-enforced single request constructor that stamps the loading document, a merge surface upstream's blitz-dom does not have, and the fixed, path-free `FileForbidden` text. The scope names the stamped request and the `file:` refusal among the resolutions to keep (decision/file-urls-only-from-file-documents, decision/one-stamping-request-constructor, contract/blitz-net-provider-error/file-forbidden).

@history_item n=41 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — the seeded source takes its seed from in-process code only and reads no env, file or carrier. The merge must not route an outside seed into it (decision/seeded-source-in-process-seed-only).

@history_item n=42 marker=2026-11-14-remote-pages-local-files
**2026-11-14-remote-pages-local-files** (Input Validation; Threat Model Summary) — the browser app names no `file:` favicon for a non-file document and drops a `file:` navigation silently. Changes the merge makes to `apps/browser` must keep this (decision/file-url-only-for-file-documents, decision/file-urls-only-from-file-documents).

@history_item n=43 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — restated the socket row: the session's Unix-domain socket exists on unix only, and in-process sessions run on every OS. Any listener the merge adds is measured against this row (decision/driver-session-unix-socket-only).

@history_item n=44 marker=2026-11-15-upstream-sync-errors-and-explanation
**2026-11-15-upstream-sync-errors-and-explanation** — added the Taffy-only git exception to both no-git rows and widened the sync dependency rule. Any git content the merge brings beyond Taffy's pin breaks the no-git rows (decision/no-git-dependency, decision/no-git-dependencies, decision/upstream-sync-dependency-set).

@history_item n=45 marker=2026-11-15-taffy-from-upstream-git
**2026-11-15-taffy-from-upstream-git** — moved Taffy to upstream's git pin at a full rev with no patch section; its integrity rests on the rev because the lock entry has no checksum. The scope finds Taffy's rev equal on both sides of the merge (decision/engine-dependency-pins, decision/no-git-dependency).

@history_item n=46 marker=2026-11-16-taffy-past-1372
**2026-11-16-taffy-past-1372** — set the engine pins row to Taffy `a3997d55` with Stylo 0.22.0, Dioxus 0.7.3 and winit `=0.31.0-beta.3`. Taking upstream's Stylo 0.24, Dioxus 0.8.0-alpha.3 and winit beta.4 moves every pin this row states, and holding one is an undo of upstream's change that must be named (decision/engine-dependency-pins).

@history_item n=47 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (Input Validation) — the driver installs no subscriber and reads no env, except that an acting step runs under escher-telemetry's scoped capture. The merge must not add a subscriber or an env read to the driver (decision/driver-installs-no-subscriber, decision/driver-no-subscriber-no-env).

@history_item n=48 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (Logging & Monitoring) — the escher target scrub set is twelve content-named fields, with `password` added. New log fields in merged code under escher targets are judged against this set (decision/escher-target-content-scrub, decision/escher-target-content-fields-withheld).

@history_item n=49 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — the layout verb on every carrier and the select list's `select-list` placing word in a layout answer. These are escher readings over the engine's layout that the merge must keep (decision/session-layout-read-in-process, decision/select-list-crossing, decision/kept-check-layout-answer, contract/driver-verbs/layout).

@history_item n=50 marker=2026-11-17-upstream-sync-ahead-of-polish-and-ship
**2026-11-17-upstream-sync-ahead-of-polish-and-ship** (Dependency Security; Input Validation; the free-disk row retired) — the last sync before this one. The founder amended the sync dependency rule to allow a coupled registry set upstream moved, with packages entering (the AnyRender 0.15 set, seven packages). The SVG `image`/`feImage` href fetch arrived by merge and was recorded as a ratified widening through the stamping constructor and the `file:` refusal. The rule governs which of upstream's coupled moves (Stylo, Dioxus, parley, skrifa) this merge can take (decision/upstream-sync-dependency-set, decision/svg-image-href-fetch).

@history_item n=51 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — each advisory ignore's reason names its owner as a residual row by id, and no product file names the residuals file. Re-reading `deny.toml`'s ignores after the merge must keep this form (decision/advisory-ignores-by-id).

@history_item n=52 marker=2026-11-19-build-directory-held
**2026-11-19-build-directory-held** (Dependency Security; Secret Management) — added the build-directory sweep's cargo runs to the locked-builds rule and the `CI` flag as an env read. The merge must keep both, and the take's new or moved cargo invocations stay `--locked` (decision/locked-builds, contract/environment-variables/ci).
