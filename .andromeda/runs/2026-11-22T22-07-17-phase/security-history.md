@history_item n=1 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — signing and PR-token jobs repository-guarded; CI legs locked: release-cli, post-results and the WPT jobs gained `github.repository == 'DioxusLabs/blitz'` (a ref-only gate re-arms signing on the fork) and `--locked` reached every ci.yml leg. Origin of decision/publish-job-repository-guard, decision/publish-secrets-upstream-main-only, decision/wpt-post-results-trusted-scripts and decision/locked-builds; upstream's `165d698c3` (CI Pages artifact) and its WPT tooling commits touch these workflows, which must stay guarded by `github.repository`.

@history_item n=2 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — cargo-deny audit leg, SHA-pinned ci.yml, read-only ci.yml token: ci.yml declares `permissions: contents: read`, every `uses:` pinned to a 40-hex SHA, the first ignore RUSTSEC-2026-0192 (ttf-parser via the winit pin) written with its reason; upstream-only guarded workflows kept `@latest` because a ref-only edit re-arms them. Origin of decision/ci-token-read-only, decision/ci-actions-pinned-to-sha, decision/cargo-deny-audit-tool and decision/audit-ignores-per-id-with-reason; the CI workflow edits this merge brings must keep both, and the CARRY re-points RUSTSEC-2026-0192's reason.

@history_item n=3 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — escher's log sink scrubs: engine targets print only the safe fields, content-named fields redacted, reach limited to escher's sink (upstream `fmt::init` subscribers and the WPT `env_logger` unscrubbed). Origin of decision/engine-target-safe-fields, decision/escher-target-content-scrub and decision/stderr-only-telemetry-sink; the scope's sink allowlist re-check reads the merged delta against these.

@history_item n=4 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** — the first upstream merge: 40 citations into merged upstream files re-pointed by the merge's line map, and the three script accessors the merge brought recorded as realizations on the registered script-to-DOM crossing, not a widening. Precedent for this merge: upstream's `DOMTokenList.replace` (`6fd5a3ade`) reads the same way, and merged files' citations follow the line map.

@history_item n=5 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — the HTML `id` gained the stable-element-id reader: an author key only when non-empty, `/`-free and first in pre-order; a non-element, stale or detached node reads none, no path panics. Origin of contract/stable-element-id/author-key, derivation and unresolved-node, which the merge's `display: contents` pseudo-element and layout changes must leave intact.

@history_item n=6 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — the stable id reaches the platform accessibility API as AccessKit `author_id` (never on a text run, root or window), and the `aria-label` / `<label for>` read was added. Origin of decision/author-id-on-accessibility-nodes and the names crossing; standing rule that any further consumer of the id re-opens the row.

@history_item n=7 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — the `id` row states four tiers (author key, anchored `{key}//{segment}`, component, document) with the three-way uniqueness argument; the anchored tier ratified by the founder. Origin of decision/anchored-id-tier and contract/stable-element-id/anchored-path and no-collision.

@history_item n=8 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — a password's snapshot value masked as `MASKED_VALUE`; the falsy boolean-attribute clear recorded; `disabled` read by presence for the snapshot. Origin of decision/masked-input-values, decision/falsy-boolean-attribute-removed and contract/snapshot-value/masked-value; trap noted then that the engine still painted a password.

@history_item n=9 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — the snapshot's one text form, ids and names in `str`'s Debug form, no JSON or Display form; the file input's value masked too, the DOM keeping the host path. Origin of contract/snapshot-text/id-field, name-field and decision/password-file-value-masked's except.

@history_item n=10 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — the platform tree delivered on change as well as on request (a boundary widening, later ratified) and the diff as an in-process reader. Origin of decision/accessibility-tree-refresh-on-change, whose rule the last sync moved to the redraw.

@history_item n=11 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — the refused Dioxus key: a keyed list row's segment `{tag}[{key}]`, an empty or `/`-bearing key refused for the positional `{tag}:{n}`, each witnessed. Origin of contract/stable-element-id/keyed-list-segment.

@history_item n=12 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the driver session socket (one Unix-domain socket, `0700` directory, `0600` socket, no TCP or UDP port) and, in a sibling entry, what a sink-installing host logs (author-key ids and names reaching stderr through third-party targets). Origin of decision/driver-session-unix-socket-only and decision/session-socket-sole-listener; trap: a "nothing in the logs" check over a host with no sink passes vacuously.

@history_item n=13 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — a sink-installing host logs no id and no name at any level: outside-target records dropped whole, WARN and ERROR included, a bridged record judged by its target. Origin of decision/outside-target-dropped, decision/other-log-targets-dropped, decision/stable-id-not-logged and decision/accessible-names-not-logged; the scope's sink re-check reads any new `tracing`/`log` target the merge adds against this drop.

@history_item n=14 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the sink's second record class (one line per closed span) and a diff's three sizes in a log ratified. Origin of decision/closed-span-lines and decision/diff-counts-only-in-logs; standing rule that a field holding a node, id, name or value is a new crossing question.

@history_item n=15 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — a hint-less `@font-face` source fetched and sniffed, recorded PROVISIONAL as a boundary widening by upstream merge; the `file:` read and the script Selection API re-read on the merged engine; dependency security after the sync (upstream's licence CI job taken and reverted, entering crates reviewed by hand). Precedent: a widening that arrives by upstream merge is still the founder's call, which this chunk's founder-forks file carries for every widening, and upstream's licenses job stays reverted (decision/licence-gate's except).

@history_item n=16 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — the hint-less `@font-face` source's PROVISIONAL mark came off, ratified as upstream's behaviour with a check owed; in sibling entries the socket and command line became carriers of ids, names and values. Shows how an upstream-merge widening is ratified and that escher routes no agent-supplied URL through blitz-net.

@history_item n=17 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — three state readings inside the ratified carriers and the founder's closed-field rule (a new closed-value field in a ratified carrier is no widening; authored text, an open value set, a new carrier or exit still is); the control-states Markup row with its exact lower-case match. Origin of decision/boundary-widening-rule, decision/closed-value-field-no-widening, decision/snapshot-state-readings, decision/aria-state-lowercase-exact-match and decision/control-states-closed-values-only, the test this chunk applies to each widening the merge brings.

@history_item n=18 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the screen's pixels in band on the three carriers (a widening, ratified), the engine's password mask painted as bullets in every document, and `png` taken by escher-driver. Origin of decision/screenshot-in-band, decision/screen-pixels-in-band, decision/password-painted-masked and decision/png-encodes-screenshots; upstream's IME preedit underline (`7beb59176`) in blitz-paint must not re-expose a password's characters in pixels.

@history_item n=19 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — the falsy-attribute predicate taken as upstream's closed 28-name set, Taffy moved to the registry (no git dependency), and the sync's dependency delta recorded. Precedent for the per-sync lock record and decision/falsy-boolean-attribute-removed's current form.

@history_item n=20 marker=2026-10-13-range-input-interaction
**2026-10-13-range-input-interaction** — the range input row, a boundary widening ratified: a current value apart from `value`, sanitized to a finite on-grid value, keys and pointer under `disabled`, exits only as form entry, `input` value and snapshot value. Origin of decision/range-current-value-apart, range-keyboard-actions, range-pointer-actions, range-value-exits and range-value-sanitized, which a conflict in form-control code must keep.

@history_item n=21 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — keyboard activation (Enter keydown, Space keyup) on every document, a boundary widening ratified, and `disabled` by presence. Origin of decision/enter-activates-focused-control, decision/space-keyup-activates-focused-control and decision/disabled-read-by-presence.

@history_item n=22 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — the audit over the whole workspace's graph with `--all-features`, the licence gate on every push, the `coverage_floor.py` input row. Origin of decision/dependency-audit-cargo-deny, decision/licence-gate and decision/coverage-floor-input-validation; the audit leg green after the merge is a scope gate.

@history_item n=23 marker=2026-10-16-fast-failing-mutants
**2026-10-16-fast-failing-mutants** — zip 2.4.2's three advisories ignored by ID with a written reason and an owner, after external decay turned the audit red over an unchanged lock. Precedent for decision/advisory-ignores-by-id and the audit-red-on-unchanged-lock case this chunk's audit may meet.

@history_item n=24 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — the merged ARIA reads (`aria-labelledby` outranking label and `aria-label`, descriptions to the adapter only, three closed values) ratified as a boundary widening by upstream merge; the zbus family's three ignores. Origin of decision/aria-labelledby-outranks-label, decision/aria-label-trimmed-once, decision/descriptions-adapter-only, decision/merged-aria-states-closed-parse and decision/accessibility-adapter-content-classes; precedent that upstream code reaching a carrier is ratified as upstream's behaviour by the founder's own word.

@history_item n=25 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — the `inert` read by presence on an element or ancestor, the toggle read per role, `disabled` on form controls alone, the password caret by the bullets; ratified later by the founder. Origin of decision/inert-subtree, decision/toggle-read-per-role and decision/password-caret-placed-by-bullets.

@history_item n=26 marker=2026-10-18-upstream-sync-determinism-and-traces
**2026-10-18-upstream-sync-determinism-and-traces** — a partial take (one commit by no-commit cherry-pick), upstream's Taffy-main commits not taken, PROVISIONAL as a delegate's answer. Precedent: the take's shape and every left-out commit are the founder's fork, as this chunk's directive requires.

@history_item n=27 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — `advance` accepted on every session and moving the virtual clock by exactly `ms`; the overlay scrollbar fade reads the document's clock. Origin of decision/advance-moves-virtual-clock-exactly and decision/scrollbar-fade-on-document-clock, part of the clock invariant the scope's conflicts must keep.

@history_item n=28 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified by the founder, Epoch 2 close) — eleven PROVISIONAL marks across engine-debts, one-virtual-clock and the cursor-fix partial take ratified. Shows the epoch-boundary discharge path for any delegate-answered fork of this chunk.

@history_item n=29 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — the per-sync lock record of the second partial take, Taffy-main again not taken, PROVISIONAL. Precedent for the one lock record every sync keeps.

@history_item n=30 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — the `select` (single-choice) row, a boundary widening and an engine change for every document: choice as `CHECKED`, the selectedness setting algorithm, the 1 s typeahead kept private, the choice's exits. Origin of decision/single-select-selectedness, decision/select-choice-exits and decision/select-typeahead-private.

@history_item n=31 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — the ten-commit take's lock record (lock edges moved, a lock-only `tokio` update past RUSTSEC-2026-0386, mimalloc reviewed by hand) and `multi_doc`'s inputs and env reads. Precedent for reviewing entering or moved packages by hand and for registering an upstream example's env reads.

@history_item n=32 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — the eleven-commit take: the script runtime's module split with 17 citations re-pointed, the lock record, the script fetch's `file:` refusal by document (a narrowing), `:open` by presence, CSS property accessors as a realization on the script crossing, dioxus-cli `0.8.0-alpha.3` in the upstream-only publish workflow, `TMPDIR` at a second test site. Origin of decision/file-url-only-for-file-documents, decision/default-script-fetcher-local-only and decision/css-property-named-accessors; the `DOMTokenList.replace` commit lands in the same blitz-vibey-script surface.

@history_item n=33 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — `visible_region` through transforms, a non-invertible transform takes no hit, a `multiple` select's keys, clicks and set, `optgroup` and its label, the engine's `change` on checkables, the native checkable in the tree, the inert focus clear and the tree's `focus`, Cells' grid keys, the `MountedData` hazard, RUSTSEC-2026-0414 by a lock update. Origin of decision/visible-region-through-transforms, decision/non-invertible-transform-no-hit, decision/multiple-select-keys-and-clicks, decision/multiple-select-set-exits, decision/optgroup-options-and-label, decision/option-disabled-one-rule, decision/engine-change-event-checkable, decision/accessibility-focus-built-nodes-only and decision/cells-grid-keyboard.

@history_item n=34 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — the engine-painted select list: which select opens one, its keys, pointer, wheel, close, twenty rows, engine colours, own-offset scroll, painted labels, hostile markup kept finite, its public readers and one writer, labels never logged, a shot of an open list carrying labels as pixels, the list's crossing as bounds and pixels only, `size` read and capped — each an engine change for every document and a boundary widening ratified, his fork words gathered. Origin of every decision/select-list-* row, decision/open-list-shot-pixels, decision/select-list-crossing and decision/select-size-hint-cap; conflicts in blitz-dom events and blitz-paint (`7beb59176`, `6636442d3`) must keep this model.

@history_item n=35 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — the twelve-commit take: a failed load reaching the handler (trap: a trait's do-nothing default hides a fix through a wrapper — sweep implementers of a trait a take extends), the lock record, the key-press panic after a lost anonymous block retired by escher's own fix, the mounted event of a removed element with upstream's Dioxus-0.8 test left out, `URLSearchParams` with the native helper deleted before page script, two test-only TCP binds. Origin of decision/dropped-anonymous-block-clears-parent, decision/skip-mounted-event-of-removed-element, decision/url-search-params-bootstrap and decision/search-params-native-parser-hidden; upstream's `display: contents` commits (`1fa2034d0`, `719268a9e`) touch the anonymous-block and box-construction paths this fix rests on.

@history_item n=36 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — advisories published after a green audit turn CI red over an unchanged lock, so the audit leg runs locally before a push; zip's fourth ignore; the audit's residual owners re-pointed to store rows. Origin of decision/local-audit-before-push; bears on the CARRY re-pointing RUSTSEC-2026-0192's reason to its residual row.

@history_item n=37 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — the spawned-task `MountedData` panic removed by the buffered render, an engine change for every Dioxus document, with its limit for a task polled during render. Origin of decision/dioxus-render-into-buffer, a conflict surface the scope names ("the buffered render") and one the Dioxus hold keeps on 0.7.

@history_item n=38 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — the snapshot's `clickable` reading (a closed bool, no widening) and the reserved virtual-node id form `{owner}///{key}`, a stable-id contract change ratified. Origin of contract/snapshot-text/clickable and contract/stable-element-id/virtual-node-form.

@history_item n=39 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — the `actions` verb: one shared target check, the read changing nothing, the unpaged answer under the answer bound. Origin of decision/actions-read-shares-target-check, decision/actions-read-changes-nothing and decision/actions-answer-unpaged.

@history_item n=40 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the masters became rows (504 records to 562 rows), each verified against the code. Explains the handles in the picked rows: later entries name what they change by a row's handle.

@history_item n=41 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — the SHA-pin check moved into `WorkflowPinTest`, reading every workflow file; untrusted page and stylesheet bytes decoded without panic. Sets decision/ci-actions-pinned-to-sha's guard, which any workflow file the merge touches (`165d698c3`, the WPT tooling) must pass.

@history_item n=42 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — the `file:` refusal widened to every engine sub-resource load, the one stamping request constructor enforced by a clippy rule, the `FileForbidden` contract. Origin of decision/one-stamping-request-constructor and contract/blitz-net-provider-error/file-forbidden; an upstream file that builds a request in blitz-dom meets the lint as a merge surface.

@history_item n=43 marker=2026-11-14-remote-pages-local-files
**2026-11-14-remote-pages-local-files** — the browser app's favicon and link navigation joined the `file:` rule in both its Input Validation and Threat Model rows. Current form of decision/file-urls-only-from-file-documents and decision/file-url-only-for-file-documents.

@history_item n=44 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — the driver session socket exists on unix only while an in-process session runs on linux, windows and macOS. Current form of decision/driver-session-unix-socket-only.

@history_item n=45 marker=2026-11-15-upstream-sync-errors-and-explanation
**2026-11-15-upstream-sync-errors-and-explanation** — the Taffy-only git exception added to the no-git-dependency rows and the upstream-sync dependency rule widened to a lock-only semver-compatible update. History of decision/no-git-dependency, decision/no-git-dependencies and decision/upstream-sync-dependency-set.

@history_item n=46 marker=2026-11-15-taffy-from-upstream-git
**2026-11-15-taffy-from-upstream-git** — Taffy moved to upstream's git at a full commit rev, no patch section, integrity resting on the rev. Current except of decision/no-git-dependency; the scope's "no git dependency save Taffy's pinned rev" rests on it.

@history_item n=47 marker=2026-11-16-taffy-past-1372
**2026-11-16-taffy-past-1372** — the Taffy git rev moved to `a3997d55`. The pin decision/engine-dependency-pins names today; upstream's float and `display: contents` layout commits must build against it.

@history_item n=48 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — an acting step runs under escher-telemetry's scoped log capture, added as the except of the driver no-subscriber rows. Current form of decision/driver-installs-no-subscriber and decision/driver-no-subscriber-no-env.

@history_item n=49 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — the layout answer added to the list of screen content kept out of logs. History of decision/no-screen-content-in-logs.

@history_item n=50 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — `password` joined the content-named field set (twelve), the caused text added to what reaches no log, the mask's except for a caused item. Current form of decision/escher-target-content-scrub, decision/escher-target-content-fields-withheld and decision/no-screen-content-in-logs, the sets the scope's sink re-check reads.

@history_item n=51 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — a caused item's `error` field kept as written; the sink's scrubbing reaches the sink only, the scoped capture hiding its own field set. Current except of decision/mask-holds-on-every-carrier and decision/escher-telemetry-allowlist-sink.

@history_item n=52 marker=2026-11-17-upstream-sync-ahead-of-polish-and-ship
**2026-11-17-upstream-sync-ahead-of-polish-and-ship** — the dependency-set rule amended for a coupled registry set upstream moved with packages entering, SVG image href fetched as a sub-resource (a widening by merge, ratified), the free-disk row retired to test-plan. Current form of decision/upstream-sync-dependency-set and origin of decision/svg-image-href-fetch; the scope's coupled-pins boundary rests on it.

@history_item n=53 marker=2026-11-17-stand-requirement-sweep
**2026-11-17-stand-requirement-sweep** — `stand_masks_on_new_answers` added to decision/mask-holds-on-every-carrier's guard. A stand check the merge must leave green.

@history_item n=54 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — the select list also crosses as a layout answer's bounds and its select-list placing word. History of decision/select-list-crossing.

@history_item n=55 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — an advisory ignore's written reason must name its owner as a residual row by its id, and no product file names the residuals file. Current form of decision/advisory-ignores-by-id; the CARRY `cry-rqochxka` re-points RUSTSEC-2026-0192's reason to `res-y4gizreq` under this rule.

@history_item n=56 marker=2026-11-19-build-directory-held
**2026-11-19-build-directory-held** — every cargo run of the build-directory sweep passes `--locked`; `CI` added as an env read. Current form of decision/locked-builds and origin of contract/environment-variables/ci; the lock-move measurement by a `--locked` workspace build rests on it.

@history_item n=57 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (API Security: 2 rows; API Security absence and overview) — the last sync's merge brought the opt-in devtools CDP server on loopback, so the session socket is the one listener of every default build only. Current form of decision/session-socket-sole-listener, absence/served-network-api and overview/api-security; the scope's "no new port or listener of ours" reads against it.

@history_item n=58 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (Input Validation: 5 rows) — the refresh moved to the redraw, isTrusted by dispatch source, programmatic focus events, a cancelled pointerup still clicks, the SVG mask URL fetch through the stamping constructor; each answered by the founder's W-forks. The immediate precedent for this sync's founder-forks file: every engine change or widening the merge brings is put to him, here at merge base `547d7514b`.

@history_item n=59 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (Dependency Security: 2 rows) — Stylo 0.24 and winit `=0.31.0-beta.4` taken, Dioxus held at 0.7.3, Taffy unchanged; Skia a default renderer with a build-time fetch. Current decision/engine-dependency-pins (the Dioxus hold the CARRY `cry-ikyyzjzj` re-reads) and origin of decision/skia-default-build-fetch.

@history_item n=60 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (Data Protection: 4 rows; Secret Management: 2 rows) — TLS moved to rustls and the clipboard to winit, replacing native-tls and arboard rows; `BLITZ_CDP_PORT` and `BLITZ_DEVTOOLS_FRONTEND_DIR` registered. Origin of decision/clipboard-via-winit; the sync's env-read census is the scope's "no new env var of ours" baseline.

@history_item n=61 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** — the highlighted reading added to the snapshot and diff and to the select list's crossing; the Cells grid's current cell states `aria-selected`. Current form of decision/snapshot-state-readings, decision/select-list-crossing, decision/cells-grid-keyboard and origin of contract/snapshot-text/highlighted.

@history_item n=62 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** — the graph answer paged by a bounded part argument, replacing the unpaged row. Origin of decision/graph-answer-paged.

@history_item n=63 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** — a focused node gives focus up silently when it comes under `inert` and, after each layout, when it no longer shows, a `display: contents` control keeping focus; the programmatic-focus-events except and the dropped-anonymous-block rule moved to a re-focused control. Current form of decision/inert-focus-clear-silent, decision/programmatic-focus-events and decision/dropped-anonymous-block-clears-parent; upstream's `is_or_contains_block` through `display: contents` (`1fa2034d0`) and `display: contents` pseudo-elements (`719268a9e`) meet these rules directly.

@history_item n=64 marker=2026-11-21-single-select-sized-as-a-list-box
**2026-11-21-single-select-sized-as-a-list-box** — a single select's `size` read, a sized single select a list box that opens no list and may stand with nothing chosen. Current form of decision/single-select-selectedness, decision/select-size-hint-cap, decision/select-list-open-gate, decision/select-list-keys-and-painted-labels, decision/select-expanded-from-engine-state, decision/select-readers-resolve-node-closed-select-none and decision/select-list-closes-when-select-unfit.

@history_item n=65 marker=2026-11-21-dependency-and-ci-debts
**2026-11-21-dependency-and-ci-debts** — Skia's archive verified against a committed sha256, the source-build fallback stated as a limit, no forced download, `SKIA_BINARIES_URL` registered, memmap2's except naming the cacache wait; and the founder's ratification of all 24 card operations of the last sync (Input Validation, Secret Management, Data Protection, Dependency Security, API Security). Current form of decision/skia-default-build-fetch, decision/skia-archive-fallback-source-build, decision/no-force-skia-binaries-download, contract/environment-variables/skia-binaries-url and decision/dependency-audit-cargo-deny; the last sync's widenings now stand ratified, the base this merge builds on.
