@history_item n=1 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (signing and PR-token jobs repository-guarded; CI legs locked) — put `github.repository == 'DioxusLabs/blitz'` guards on release-cli, post-results and the WPT jobs, and `--locked` on every ci.yml cargo leg (row decision/wpt-post-results-trusted-scripts, decision/locked-builds). This take carries four WPT-only commits (`ea07ee716`, `e7cca1d37`, `c30096724`, `d2ee20b98`). A guarded upstream workflow edit must keep the repository guard, or it re-arms signing on fork refs.

@history_item n=2 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — brought in the cargo-deny 0.20.2 audit leg, per-ID ignores with a written reason, SHA-pinned ci.yml `uses:` and a read-only ci.yml token (rows decision/cargo-deny-audit-tool, decision/advisory-ignores-by-id, decision/dependency-audit-ci-gate). The sync's "dependency audit green" gate runs against these rules.

@history_item n=3 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — wired escher's stderr sink: engine targets (`blitz`, `dioxus_native`, `stylo_taffy`, `accesskit_xplat`, `debug_timer`, `js_console`) print only seven safe fields, and eleven content-named fields are redacted (rows decision/engine-target-safe-fields, decision/escher-target-content-scrub, decision/stderr-only-telemetry-sink). The scope's "sink allowlist re-checked" step reads each new upstream `tracing`/`log` target against this list.

@history_item n=4 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** — the first merge precedent. An upstream change to Taffy's alignment fallback was restated with the no-panic guarantee kept. 40 citations were re-pointed by the merge's line map. Script accessors that arrived by merge were recorded as realizations on the script crossing, not widenings. The same pattern applies to this take's re-points and to any script surface it brings.

@history_item n=5 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — the stable id became the AccessKit `author_id` on element nodes and crosses only through the adapter. Accessible-name reads were recorded (rows decision/author-id-on-accessibility-nodes, decision/stable-id-exits). An upstream accessibility edit in the take must keep this resolution.

@history_item n=6 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — set the anchored `{key}//{segment}` tier and the three-way uniqueness argument (rows decision/anchored-id-tier, contract/stable-element-id/anchored-path, contract/stable-element-id/no-collision). These are the stable-id rules a conflict resolution in dioxus-native-dom must keep.

@history_item n=7 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — masked a password's snapshot value as `MASKED_VALUE` and recorded the falsy boolean-attribute clear (rows decision/masked-input-values, decision/falsy-boolean-attribute-removed, contract/snapshot-value/masked-value). It warned that the mask covers the snapshot only. Both are snapshot behaviours the take must not regress.

@history_item n=8 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — registered the session's Unix-domain socket as the one listener (rows decision/session-socket-sole-listener, decision/driver-session-unix-socket-only) and measured what a sink-installing host logs by level. Trap: a "nothing in the logs" check over a process that installs no sink passes vacuously. This bears on the sink re-check and on the CDP-listener carry (`a1670ef1b`, a second listener that needs a security-plan amendment first).

@history_item n=9 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — made outside-target records drop whole at every level, WARN and ERROR included, and measured id and name needles at 0 (rows decision/outside-target-dropped, decision/other-log-targets-dropped, decision/telemetry-sink-target-allowlist). Trap: the check has to run against a sink-installing host (`host_log`). A new upstream log target in the take is either admitted by the allowlist or dropped silently.

@history_item n=10 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — ratified the sink's second record class, one line per closed span, and the diff's three sizes in a log (rows decision/closed-span-lines, decision/diff-counts-only-in-logs). A new upstream span target or field in the take falls under this rule.

@history_item n=11 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — the precedent for a boundary widening that arrives by upstream merge. Upstream #1109's hint-less `@font-face` source fetch was recorded PROVISIONAL, because a widening is never routine even by merge. Input rows were re-read on the merged engine (the `file:` read, `autofocus`, Selection). The sync's dependency delta was reviewed by hand: entering crates, the licence table, a second `unsafe`. This take's new attribute reads (SVG presentation attributes, `hr` `color`/`noshade`/`size`, inline SVG `transform`) and `f5ea1caf1`'s `@font-face` `font-stretch` follow the same path. With the founder away, each is marked PROVISIONAL.

@history_item n=12 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the hint-less `@font-face` source ratified) — the founder ratified the widening as upstream's behaviour with a check owed (row decision/hintless-font-face-source-sniffed, guard `font_face_sniffed_source`). The rule that no agent URL is routed through the font path stands (row decision/no-agent-urls-through-font-path). `f5ea1caf1` edits this font-face path.

@history_item n=13 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — added three closed state readings and set the closed-field rule: a new field of a closed value set inside a ratified carrier is no widening; authored text, an open set, a new carrier or a new exit still is (rows decision/boundary-widening-rule, decision/closed-value-field-no-widening, decision/snapshot-state-readings). This is the test for deciding which of the take's new reads are widenings.

@history_item n=14 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — ratified the MCP stdio crossing and recorded `serde_json` as read-only, with no `preserve_order` (rows decision/mcp-server-stdio-only, decision/mcp-stdio-no-auth, decision/serde-json-read-only). An upstream dependency set that moves serde_json's features would touch this row.

@history_item n=15 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — painted the password masked in the engine and took `png` with no `base64` crate (rows decision/password-painted-masked, decision/no-base64-crate, decision/screenshot-in-band). The take's paint changes (SVG image clipping and an isolated layer, `d176ac4e6` and `beaa796f4`) must keep the masked password paint, and the dependency set must still enter no base64 crate.

@history_item n=16 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — removed the last git dependency (Taffy moved to the crates.io "0.15") and adopted upstream's 28-name falsy predicate. The founder's rule of 2026-10-13: no git dependency in the root manifest (rows decision/no-git-dependencies, decision/no-git-dependency). This is the rule that keeps the four Taffy-main commits out unless a registry release carries them.

@history_item n=17 marker=2026-10-13-stand-contrast-harness
**2026-10-13-stand-contrast-harness** — blitz-test-harness takes `color` already in the graph and no `tracing` or `style` (rows decision/color-crate-luminance-source, decision/test-harness-no-tracing-no-style). An upstream manifest change to blitz-dom's `color`, or a new harness dependency, would touch these rows.

@history_item n=18 marker=2026-10-13-range-input-interaction
**2026-10-13-range-input-interaction** — the ratified range-input model: a current value kept apart from the attribute, sanitized, with its own key and pointer arms (rows decision/range-current-value-apart, decision/range-value-sanitized, decision/range-value-exits). The scope names the range model among the resolutions a conflict must keep.

@history_item n=19 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — ratified Enter and Space activation and a button click's focus as engine changes for every document (rows decision/enter-activates-focused-control, decision/space-keyup-activates-focused-control, decision/button-click-focuses). This is part of the keyboard model the take must keep, and `handle_click`'s arms are now private `click_*` functions.

@history_item n=20 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — moved to the whole-workspace `--all-features` audit with the licence gate on every push. The paste ignore and memmap2's `unsound` limit were recorded (rows decision/dependency-audit-cargo-deny, decision/licence-gate, decision/audit-ignores-per-id-with-reason). A take whose dependencies enter or move a package must pass the licence half too.

@history_item n=21 marker=2026-10-16-fast-failing-mutants
**2026-10-16-fast-failing-mutants** — three zip 2.4.2 advisory ignores, reached only through `anyrender_serialize` ← browser, dropped when that chain reaches zip 9.0. The four AnyRender git-pin bumps in this take stay out, so the chain and its ignores stand; a moved AnyRender would re-open them.

@history_item n=22 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — a sync precedent. Merged ARIA reads were recorded as a boundary widening by upstream merge, ratified as upstream's behaviour (rows decision/aria-labelledby-outranks-label, decision/descriptions-adapter-only, decision/merged-aria-states-closed-parse). The zbus-family ignores were added under external decay. The sync record took upstream's set whole with Taffy still the registry "0.15" (row decision/upstream-sync-dependency-set). This is the model for listing this take's new attribute reads at P4 and for keeping one lock record.

@history_item n=23 marker=2026-10-18-upstream-sync-determinism-and-traces
**2026-10-18-upstream-sync-determinism-and-traces** — the first partial take by cherry-pick. Upstream's four Taffy-main commits (`1a11761bc` · `4a5ab9cc9` · `510b592ab` · `1e5479ae6`, git Taffy) were not taken, so the no-git-dependency rule held, and the lock and `deny.toml` stayed byte-identical. These are the same four commits the scope measures again.

@history_item n=24 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — `inert` read by presence, the toggle read per role, `disabled` read by presence on form controls and the password caret placed by the bullets, all ratified 2026-10-18 with the partial sync take (rows decision/inert-subtree, decision/toggle-read-per-role, decision/disabled-read-by-presence, decision/password-caret-placed-by-bullets). The scope names `inert` and the accessibility state reads among the resolutions to keep.

@history_item n=25 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — `advance` became exact on every session and the scrollbar fade moved onto the document's clock (rows decision/advance-moves-virtual-clock-exactly, decision/scrollbar-fade-on-document-clock, decision/time-moves-only-on-advance). This is the clock invariant the scope says every resolution keeps.

@history_item n=26 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — the second partial take. Two commits were cherry-picked with no dependency movement, and the four Taffy-main commits were again not taken. The lock record stays one per sync, and the take was PROVISIONAL on a delegate's answer, a precedent for this founder-away run.

@history_item n=27 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — the single-choice `select` model (selectedness, the 1 s private typeahead, the choice's exits). The 2026-10-19 partial take's PROVISIONAL mark was ratified (rows decision/single-select-selectedness, decision/select-typeahead-private, decision/select-choice-exits). This is part of the select model a conflict must keep.

@history_item n=28 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — the ten-commit take's lock record. No package entered, edges were re-pointed, mimalloc was reviewed by hand, and tokio moved past RUSTSEC-2026-0386 by a lock-only update. The take's new example inputs and env reads were recorded (`MULTI_DOC_*`). Git Stylo, Taffy-main and Dioxus 0.8 were not taken. This is the record form, and the env-read and input-row sweep, this take repeats.

@history_item n=29 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — the eleven-commit take. Upstream's `cee6b8731` brought the `file:` refusal by document, recorded as a narrowing (rows decision/default-script-fetcher-local-only, decision/file-urls-only-from-file-documents). The `open` presence read and the `CSSStyleDeclaration` accessors were recorded as engine changes and realizations, and the upstream-only publish workflow's dioxus-cli was noted. A test-only `TMPDIR` read and 52 re-pointed citations followed. It is the closest precedent for this take's shape: new engine reads per row, one lock record and a citation re-point.

@history_item n=30 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — `visible_region` read through transforms, a non-invertible transform taking no hit in `hit_inner`, the inert focus clear, the tree's `focus`, the engine's `change` on checkables, the multiple select and `optgroup` (rows decision/visible-region-through-transforms, decision/non-invertible-transform-no-hit, decision/inert-focus-clear-silent, decision/accessibility-focus-built-nodes-only, decision/engine-change-event-checkable, decision/multiple-select-keys-and-clicks, decision/optgroup-options-and-label). `hit_inner`'s non-invertible answer conflicts textually with `0980cce5f`'s hunk (the merge-surfaces carry), and the scope names the hit stops among the resolutions to keep.

@history_item n=31 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — the engine-painted select list: its state, keys, pointer, close, paint over the page, scroll, the 20-row cap, the hostile-markup finiteness, `size` and the pixels in a shot. The key-press panic after a hidden ancestor was recorded with its owner the next upstream sync (rows decision/select-list-*, decision/select-size-hint-cap, decision/open-list-shot-pixels, decision/select-list-crossing). This is the select list the scope says every resolution keeps.

@history_item n=32 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — the twelve-commit take. A failed load reaches `NetHandler::error`, whose default does nothing (row decision/net-handler-error-default-noop). The trap: a trait's do-nothing default can hide a fix through a wrapper no conflict or gate touches, so sweep a trait's implementers. Two test-only loopback TCP binds were registered (no served port). `deallocate_anonymous_block` clears its children's parent, retiring the key-press panic (row decision/dropped-anonymous-block-clears-parent, a flagged merge surface). The removed element's mounted event is skipped, upstream's test left out on Dioxus 0.7 (row decision/skip-mounted-event-of-removed-element). The locked `embolden` leg was added (row decision/embolden-locked-fast-leg). Blend modes and `URLSearchParams` were recorded as engine changes. This is the direct predecessor for the merge surfaces, the left-out mounted test and the embolden witness.

@history_item n=33 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — added the fourth zip ignore and a lock-only pulldown-cmark update, and set the rule to run the audit leg locally before a push, because advisories can turn CI red over an unchanged lock (row decision/local-audit-before-push). This take's push has to read the audit leg locally first.

@history_item n=34 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — the buffered render in `dioxus_document.rs` and `mutation_buffer.rs` removed the `RefCell already borrowed` panic; the recorder holds no panic path or log site (row decision/dioxus-render-into-buffer). It is flagged not upstreamable and is re-read only when the Dioxus 0.8 move is taken, which this chunk keeps out.

@history_item n=35 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — the bridge's dispatch is recorded only where a signal registry stands, and the reserved virtual-node id form was added (rows decision/dispatch-recorded-only-with-signal-registry, contract/stable-element-id/virtual-node-form). This is part of the dioxus-native-dom behaviour (graph, ids) the scope says the take keeps.

@history_item n=36 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — set one actionable rule for both readers and the shared target check (row decision/one-actionable-rule). It is dioxus-native-dom and driver surface that must survive the take unchanged.

@history_item n=37 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — the reached-handler record is written ahead of dispatch with no log site or `NodeId`, deduplicated, and kept past element removal (rows decision/reached-handler-record-without-log, decision/reached-record-deduplicated-survives-removal). The scope names the reached record among the dioxus-native-dom behaviour the take keeps.

@history_item n=38 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — turned the master's text into rows, 504 records into 562 rows, each verified against the code. Every picked row's handle comes from this conversion, so a row the take falsifies is amended by handle (`~`/`+`/`−`).

@history_item n=39 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — the last take added decision/presentational-hints-typed-parse (new presentational hint readers on page content, remote pages included, validated by a typed parse) and decision/untrusted-bytes-decoded-without-panic, ratified. The CI pin guard moved to WorkflowPinTest, which reads every workflow. This take's `0bdfe4fa3` (the presentational-hint helpers refactor) and `fb381be2c` (`hr` `color`/`noshade`/`size`) extend the presentational-hints row. Its WPT workflow edits meet the every-workflow pin test.

@history_item n=40 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — added decision/one-stamping-request-constructor: blitz-dom builds a sub-resource request only through `stamped_request`, enforced by `packages/blitz-dom/clippy.toml`'s ban on `Request::get`, a new upstream merge surface. It widened the `file:` rule to engine loads and added contract/blitz-net-provider-error/file-forbidden. The `Request::get` ban carry says `f5ea1caf1`'s `@font-face` change must route any new request through `stamped_request`, and the clippy leg stays red until it does.

@history_item n=41 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — restated decision/driver-session-unix-socket-only: the session socket exists on unix only, while in-process sessions run on linux, windows and macOS. This is the listener rule the scope's "no new listener" and the CDP-listener carry rest on.

@history_item n=42 marker=2026-11-14-remote-pages-local-files
**2026-11-14-remote-pages-local-files** — extended decision/file-urls-only-from-file-documents and decision/file-url-only-for-file-documents to the browser app's `file:` favicon and link navigation. If the take touches the browser app, which is also where the `expect_used` carry lives, these rules hold.
