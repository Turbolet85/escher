@history_item n=1 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** (cargo-deny audit leg, SHA-pinned ci.yml, a read-only ci.yml token) — made ci.yml declare `permissions: contents: read` with no job-level grant, pinned every ci.yml `uses:` to a 40-hex SHA, and added the cargo-deny 0.20.2 audit leg with per-ID ignores only. Why it matters: a free-disk step added to `Test [default features]` is a CI workflow edit, so it must keep the SHA pin and the read-only token (rows `decision/ci-token-read-only`, `decision/ci-actions-pinned-to-sha`, `decision/cargo-deny-audit-tool`).

@history_item n=2 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** (unknown alignment flags; merge citation re-point) — the first upstream merge. It re-pointed 40 citations by the merge's line map and treated the script accessors the merge brought as realizations on the registered script-to-DOM crossing, not a boundary widening. Why it matters: this is the precedent for how a take's new script or DOM surface is read and how citations follow moved lines.

@history_item n=3 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — set the sink's three outcomes: engine targets print only the safe fields, `escher_` targets redact the content-named fields, and any other target is dropped whole at every level, with the `host_log` check standing as the real-host proof. Why it matters: the scope re-checks the sink allowlist against every new `tracing`/`log` call site and target the delta brings (rows `decision/outside-target-dropped`, `decision/other-log-targets-dropped`, `decision/engine-target-safe-fields`, `decision/telemetry-sink-target-allowlist`). Trap: a "nothing in the logs" check over a host that installs no sink passes vacuously.

@history_item n=4 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (hint-less `@font-face` fetched, PROVISIONAL; input rows re-read; dependency security after the sync) — recorded upstream #1109's new font fetch as a boundary widening by upstream merge that only the founder may settle. Re-read the `file:` read on the merged engine. Recorded the entering crates, each reviewed by hand. Why it matters: this is the standing precedent that a widening arriving by an upstream merge (here `7b8712cba`'s external SVG image loads on every document) is never settled silently, and that entering lock names are reviewed one by one.

@history_item n=5 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the hint-less `@font-face` source: the PROVISIONAL mark comes off) — the founder ratified the merged font-fetch widening as upstream's behaviour, with a check of ours owed. Escher still routes no agent- or user-supplied URL through blitz-net. Why it matters: this is the ratification shape for an upstream-merge widening that the SVG external-image fetch will follow at P4.

@history_item n=6 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (pixels in band; the engine's password mask, `png`, no `base64`) — screenshots cross in band. A password is painted masked through blitz-paint in every renderer. `png` is the encoder and no `base64` crate is taken. Why it matters: the AnyRender 0.15, canvas-background and shear changes move paint, so the screenshot readings and the password-painted-masked guard must be re-measured, never loosened (rows `decision/screenshot-in-band`, `decision/password-painted-masked`, `decision/no-base64-crate`, `decision/png-encodes-screenshots`).

@history_item n=7 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (the 28-name falsy predicate; no git dependency; the sync's dependency delta) — the last merge-shaped take. It took upstream's dependency set whole and moved Taffy to the registry with no git dependency, the lock's set matching upstream's apart from the `escher-*` entries. Why it matters: this is the precedent for a take that adopts upstream's registry set whole, which an AnyRender 0.15 registry move would follow (row `decision/upstream-sync-dependency-set`).

@history_item n=8 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — made the audit run `cargo deny --locked --workspace --all-features check advisories licenses` over the whole workspace graph, so the fork gates on licences; paste and memmap2 limits recorded. Why it matters: new or removed packages from an AnyRender move pass the licence gate and the whole-graph audit (rows `decision/licence-gate`, `decision/dependency-audit-cargo-deny`).

@history_item n=9 marker=2026-10-16-fast-failing-mutants
**2026-10-16-fast-failing-mutants** (five advisory ignores: zip 2.4.2's three) — ignored zip 2.4.2's three RUSTSEC IDs by ID because zip reaches the graph only through `anyrender_serialize 0.8.0` ← browser, with no fix reachable. The ignores drop when that chain reaches zip 9.0 or leaves zip. Why it matters: upstream now names `anyrender_serialize = "0.9.0"`, so the take may release or move these ignores. The audit's per-ID ignores must be re-read rather than left stale (rows `decision/advisory-ignores-by-id`, `decision/audit-ignores-per-id-with-reason`).

@history_item n=10 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the merged ARIA reads; the bump CLI's rows; eight advisory ignores and the sync record) — ratified upstream's `aria-labelledby` names, descriptions sent to the adapter only, and three closed values as upstream's behaviour by the founder's word. Added the zbus-family ignores and the per-sync lock record. Why it matters: this is the precedent for ARIA reads and attribute reads arriving by merge (rows `decision/aria-labelledby-outranks-label`, `decision/descriptions-adapter-only`, `decision/merged-aria-states-closed-parse`, `decision/aria-label-trimmed-once`), and each sync keeps one lock record.

@history_item n=11 marker=2026-10-18-upstream-sync-determinism-and-traces
**2026-10-18-upstream-sync-determinism-and-traces** (the 2026-10-18 sync record) — a partial no-commit cherry-pick take with no dependency moved. Upstream's Taffy-main commits on a git Taffy were left out under the no-git rule. Why it matters: this is the start of the left-out table whose reasons this take re-reads against `e5305a4c1`.

@history_item n=12 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** (the per-sync lock record of the second partial take) — a cherry-pick take with the lock, manifest and `deny.toml` byte-identical and the git-Taffy commits again not taken. Why it matters: this is the per-sync lock record shape this take writes.

@history_item n=13 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (the ten-commit take's lock record and tokio past RUSTSEC-2026-0386; `multi_doc`'s inputs) — the cherry-pick entered no package but moved lock edges (one `mimalloc` edge added, twelve re-pointed), and an advisory was fixed by a lock-only tokio update. New upstream example env reads and inputs were registered in the examples' validated shape. Why it matters: this is the precedent for lock edges moving under a cherry-pick and for registering any new env read or input the delta brings (scope: "No new crate, port, env var or listener of ours").

@history_item n=14 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (module-split citations; the eleven-commit lock record; line-shift citations; the script fetch's `file:` refusal; `open` by presence; CSS accessors; dioxus-cli in publish; `TMPDIR`) — took `cee6b8731`'s `file:` refusal by loading document as a narrowing, not a widening. It also recorded `open`/`:open` as an engine change for every document, the upstream-only publish workflow's dioxus-cli move, and a test-only env read. Why it matters: these are the root of `decision/default-script-fetcher-local-only`, `decision/file-url-only-for-file-documents`, `decision/file-urls-only-from-file-documents` and `decision/open-pseudo-class-by-open-attribute`. Upstream-only workflow changes keep their repository guard.

@history_item n=15 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — moved the overlay scrollbar fade onto the document's clock, so two shots of one state read the same pixels until `advance` moves the fade. Settle waits on and moves no clock. Why it matters: `e5305a4c1` clamps scroll offsets at layout rounding, on the scrolling surface this determinism rests on (rows `decision/scrollbar-fade-on-document-clock`, `decision/time-moves-only-on-advance`).

@history_item n=16 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the free-disk action at its two CI sites) — added `jlumbroso/free-disk-space` pinned by SHA to `Stand checks repeated` after a red run with `Free space left: 5 MB`. The step was recorded as not shown to be the cure, the failed job having run on an older runner image whose free disk was unread. Why it matters: this is the direct precedent for the operator's disk-red directive. A free-disk step for `Test [default features]` stays a pinned CI action site, and causation must be measured from the logs (row `decision/free-disk-only-on-stand-repeat`).

@history_item n=17 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`visible_region` through transforms; a non-invertible transform takes no hit; inert focus; native checkable; select keys and clicks; optgroup) — engine changes for every document, each ratified on the founder's forks. Why it matters: a take touching scrolling, hit testing or the visible region (`e5305a4c1`) must keep these rows intact under conflict resolution (rows `decision/visible-region-through-transforms`, `decision/non-invertible-transform-no-hit`, `decision/inert-focus-clear-silent`, `decision/accessibility-focus-built-nodes-only`, `decision/multiple-select-keys-and-clicks`, `decision/optgroup-options-and-label`, `decision/engine-change-event-checkable`).

@history_item n=18 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list's state, keys, pointer, close, paint and scroll; readers; labels as pixels; log silence) — built the engine-painted select list on every document. Its scroll writes its own offset and moves no scroll chain or viewport, its visible region answers the list's scrollport, it is painted after the root's render, and its labels are never logged. Why it matters: the scope names the select model and list as a merge surface. The canvas-background and scroll-clamp commits touch paint order and scrolling these rows rest on (rows `decision/select-list-scroll-own-offset`, `decision/visible-region-open-list-row`, `decision/select-list-painted-over-page`, `decision/select-list-closing`, `decision/open-list-shot-pixels`, `decision/select-list-label-never-logged`).

@history_item n=19 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (no served port, two test-only TCP binds; a failed load ends the wait; the twelve-commit lock record) — registered blitz-net's loopback test binds as test-only, with the Unix socket kept as the one listener. Recorded the `NetHandler::error` do-nothing default. Wrote the per-sync lock record. Why it matters: the CDP listener (`a1670ef1b`) would be a served port and needs a security-plan amendment first (rows `decision/session-socket-sole-listener`, `absence/served-network-api`). The recorded trap applies to any take extending a trait: sweep the implementers of a trait a take extends, because a do-nothing default can hide a fix behind a wrapper (row `decision/net-handler-error-default-noop`).

@history_item n=20 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the `matrix_test` free-disk step retired; the `embolden` leg, locked) — removed the always-skipped free-disk step from `matrix_test` and pinned the action to `Stand checks repeated` by `test_only_stand_repeat_frees_disk`. Added the locked `embolden` fast leg. Why it matters: adding a free-disk step to `Test [default features]` must amend `decision/free-disk-only-on-stand-repeat` and its guard. The embolden witness is a proof this take must keep green (row `decision/embolden-locked-fast-leg`).

@history_item n=21 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the key-press panic retired; the mounted event of a removed element; blending, culling, `URLSearchParams`) — fixed the panic in `deallocate_anonymous_block` (a merge surface flagged upstreamable) and left upstream's mounted-event test out on Dioxus 0.7. Recorded paint and script changes by merge as engine changes for every document. Why it matters: the scope re-reads both carries (`cry-blhcfqeb`, `cry-o26emj4y`). Paint-moving commits follow this entry's engine-change recording (rows `decision/dropped-anonymous-block-clears-parent`, `decision/skip-mounted-event-of-removed-element`).

@history_item n=22 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the zip ignores gain RUSTSEC-2026-0429; pulldown-cmark 0.13.6; advisories published after a green audit) — a fourth zip ignore was added, again reached through `anyrender_serialize 0.8.0`. A semver-compatible lock-only fix was made. The rule was set to run the audit leg locally before a push. Why it matters: an AnyRender 0.15 / `anyrender_serialize 0.9.0` move bears directly on the zip ignores, and the audit must be green locally before this take's push (row `decision/local-audit-before-push`).

@history_item n=23 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — ported the buffered render onto the 0.7 bridge, removing the spawned-task `MountedData` panic, with the recorder holding no log site. Why it matters: the scope carries this buffered render's merge surface (`cry-mnnoitsu`). A take must keep `decision/dioxus-render-into-buffer`, and its replacement waits for the Dioxus 0.8 move, which is out of scope.

@history_item n=24 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Dependency Security; Input Validation) — moved the SHA-pin guard into `WorkflowPinTest`, which reads every workflow file. Added the rows on untrusted page and stylesheet bytes decoded without panic, and on presentational hints read by a typed parse, both ratified as widenings from the take. Why it matters: a ci.yml edit for the disk red is gated by `WorkflowPinTest`. This is the latest precedent for landing a take's new readers on page content as rows.

@history_item n=25 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** (Threat Model Summary; Input Validation; Error Handling) — widened the `file:` refusal to every engine sub-resource load (stylesheet, @import, font, image, iframe) refused by blitz-net before a disk read. Added `decision/one-stamping-request-constructor` (the blitz-dom clippy ban on `Request::get`) and the path-free `FileForbidden` contract. Why it matters: `7b8712cba` adds external image loads in SVG on every document. Each must go through the stamping constructor so blitz-net's `file:` refusal applies, or the clippy leg goes red (carry `cry-fdvxodkv`).

@history_item n=26 marker=2026-11-14-remote-pages-local-files
**2026-11-14-remote-pages-local-files** (Input Validation; Threat Model Summary) — extended both `file:` rows to the browser app's favicon and link navigation. Why it matters: these are the current wording of `decision/file-urls-only-from-file-documents` and `decision/file-url-only-for-file-documents`, which a new SVG image load must satisfy and which a new loaded kind may need named.

@history_item n=27 marker=2026-11-15-upstream-sync-errors-and-explanation
**2026-11-15-upstream-sync-errors-and-explanation** (Dependency Security: 3 rows) — gave the no-git-dependency rule its Taffy-only exception. Allowed a sync to take git-pinned content only by a lock-only semver-compatible update of a pinned registry crate. Why it matters: this is the last sync's rule shape. A git AnyRender is barred, and a registry AnyRender 0.15 move is the shape the take must use (rows `decision/no-git-dependency`, `decision/no-git-dependencies`, `decision/upstream-sync-dependency-set`).

@history_item n=28 marker=2026-11-15-taffy-from-upstream-git
**2026-11-15-taffy-from-upstream-git** (Dependency Security: 4 rows) — moved Taffy to upstream's git rev, its integrity resting on the rev with no lock checksum. Extended the sync rule to Taffy's own git pin. Why it matters: carry `cry-hbkcvbxr` re-reads this merge surface against upstream's pin at `e5305a4c1` (rows `decision/engine-dependency-pins`, `decision/no-git-dependency`).

@history_item n=29 marker=2026-11-16-taffy-past-1372
**2026-11-16-taffy-past-1372** (Dependency Security: 1 row) — moved the Taffy pin to `a3997d55`, the rev upstream's manifest pins at `e5305a4c1`. Why it matters: no Taffy move is expected in this take, and `decision/engine-dependency-pins` must still read true after it.

@history_item n=30 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (Logging & Monitoring: 3 rows and 1 row) — added `password` to the twelve content-named fields and put the step's caused text on the no-log list. Why it matters: the sink-allowlist re-check reads new call sites against this field set and list (rows `decision/escher-target-content-scrub`, `decision/escher-target-content-fields-withheld`, `decision/no-screen-content-in-logs`).

@history_item n=31 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** (Logging & Monitoring: 1 row) — restated the sink exception: its scrub reaches that sink only, escher's scoped capture hides its own field set, and upstream's subscribers, env_logger and the panic hook stay unscrubbed. Why it matters: new upstream subscribers or prints in the delta fall under this exception's reach and must be named (row `decision/escher-telemetry-allowlist-sink`).

@history_item n=32 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (Input Validation: 4 rows; Threat Model Summary: 2 rows; Data Protection: 1 row) — put the layout read on every ratified carrier and added the select-list placing word to the select-list crossing. Why it matters: a scroll-offset clamp at layout rounding can move layout answers and select-list row bounds these rows carry, so any moved reading is re-measured (rows `decision/session-layout-read-in-process`, `decision/select-list-crossing`, `contract/driver-verbs/layout`, `decision/kept-check-layout-answer`).
