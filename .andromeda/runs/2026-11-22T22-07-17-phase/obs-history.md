@history_item n=1 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (per-leg CI failure logs; WPT and publish telemetry upstream-only) — added the per-leg `ci-log-{job id}` artifact (failure only, 7 days, from `target/ci-logs/` alone) and marked the WPT report, `wptscores.json` and publish-build logging upstream `DioxusLabs/blitz` only, repository-guarded. This take carries upstream's `165d698c3` (CI Pages artifact per run attempt) and WPT tooling commits `120520960`/`cec303355`: those jobs must stay repo-guarded and the fork's artifact set must not grow (rows decision/wpt-scores-upstream-only, decision/wpt-publishing-upstream-only, decision/ci-log-artifact-on-failure).

@history_item n=2 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (file:line citations re-pointed after the CI files moved) — re-pointed `wpt.yml` and `publish-browser.yml` citations when the CI files moved. Precedent: an upstream edit to those workflows moves the cited lines and needs a re-point pass.

@history_item n=3 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — added the `coverage-report` artifact (success only, 7 days), recorded as a boundary widening (a new upload out of fork CI), later ratified. That gives the two-artifact rule (decision/fork-ci-two-artifacts, decision/coverage-artifact-on-success). Any upload the upstream CI commit would add to fork CI is a widening and goes to the founder-forks file.

@history_item n=4 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — built escher-telemetry's stderr sink, the `LogTracer` bridge and the allowlist scrub: engine prefixes print only the seven safe fields; tracing is compiled only under the `tracing` feature in engine/upstream crates; upstream apps' `fmt::init()` stays unscrubbed. Origin of rows decision/engine-target-safe-fields, decision/engine-tracing-feature-gated, contract/telemetry-sink/log-records and decision/scrub-sink-reach. A merged upstream log site is judged against these sets.

@history_item n=5 marker=2026-10-06-founder-rulings
**2026-10-06-founder-rulings** — the founder ruled no OTel export (no OTel crate, no egress, no credential path) and ratified the coverage-report upload. Origin of decision/no-otel-export: a take that brings an OTel crate or exporter crosses this ruling.

@history_item n=6 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the OTel export residual's status) — the otel-sdk-install owner now names residual `res-fzitzzw2`, re-carried, target next. That is the open residual behind decision/no-otel-export.

@history_item n=7 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** (file:line citations re-pointed after the upstream merge) — the first upstream merge re-pointed 20 citations by the merge's measured line map and recorded that the merge added no log, print or tracing site. Precedent for this sync's citation pass and its log-site census.

@history_item n=8 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** (root-manifest citations re-pointed after the line-61 insert) — fixed root `Cargo.toml` citations an earlier wrap had failed to re-point. Trap: a manifest or line insert the merge makes leaves stale citations unless re-pointed in the same pass.

@history_item n=9 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — added `escher-session` as the second sink installer and scoped the scrub's reach to what was measured. Trap recorded: a zero read from a host that installs no sink says nothing about one that does. This is why the sink re-check must read the sink-installing host and mcp role (decision/sink-host-read-at-trace, decision/session-logs-carry-no-page-content).

@history_item n=10 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — records from outside targets are now dropped whole at every level, whatever `RUST_LOG` names, and the third-party WARN/ERROR cost is accepted. Rule for later chunks: a target is admitted only together with a scrub rule for it. Origin of decision/sink-target-allowlist and contract/escher-log-line/*. This chunk's "sink allowlist re-checked" reads every new target in the merged delta against it.

@history_item n=11 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the driver's command span as built) — recorded that a workspace-wide build turns `blitz-dom/tracing` on, so a test child running the sink at info also receives engine events. Origin of decision/engine-tracing-by-invocation: the merge's engine sites can reach the sink under the workspace build, which is why `host_log`/`mcp_log` run under both builds.

@history_item n=12 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the founder ratified the closed span's line as a second record class (a boundary widening), judged pair by pair by the scrub (rows decision/closed-span-lines, contract/span-line/*, decision/closed-span-lines-judged-alike). An upstream span under an admitted target would write such a line.

@history_item n=13 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — rule: never field a `Command`, `Call` or `ArgValue` with `Debug`, nor an argument by its schema name. This rule is behind decision/debug-types-never-logged, which must hold where conflicts touch the driver.

@history_item n=14 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (typed text measured in a host's log; the session host by level under both builds) — first measured the sink-installing host at every level under the package-alone and workspace builds, with 0 id, name and sentinel needles. That reading is the baseline decision/sink-host-read-at-trace and decision/session-logs-carry-no-page-content re-read after each merge.

@history_item n=15 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — a shot's image never appears in a log: 0 base64 bytes on the host's and mcp role's stderr at trace under both builds (row contract/command-answer/shot). The screenshot readings this chunk re-measures after the paint changes (`7beb59176` IME underline) stay out of logs.

@history_item n=16 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (the merge's five targets join the no-subscriber census) — precedent: the merge's new upstream integration targets were each read silent (no subscriber, env read or print), and `host_log` and `mcp_log` passed under both builds after the merge. Upstream's `display_contents.rs` and `ime_preedit_underline.rs` get the same reading (decision/checks-silent).

@history_item n=17 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — the workspace-built host's `blitz_dom::document` `focussed node` INFO line was seen printing twice, and why is recorded but not established. Trap: an engine change can move the engine lines the sink admits under the workspace build without adding a site.

@history_item n=18 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — measured a client's stderr and the workspace-built windowed `seven_guis_native` by build and level: 28 runs, 0 needles. This extends the by-invocation reading (decision/engine-tracing-by-invocation) that a merge must not break.

@history_item n=19 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — the coverage report's lcov excludes `keyboard_compat.rs` by filename, on the operator's P4 fork 3 answer. The CARRY `cry-ikyyzjzj` asks this sync to re-read that exclusion with the Dioxus 0.7.3 hold; it bears on decision/coverage-artifact-on-success.

@history_item n=20 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — upstream's focus targets were recorded silent, and the engine take was marked PROVISIONAL on the operator's delegate answer (later dropped on the founder's word, per 2026-10-20-select-by-id). Precedent: a take's engine behaviour change is the founder's fork, not a default, which matches this chunk's founder-forks directive.

@history_item n=21 marker=2026-10-19-test-splits
**2026-10-19-test-splits** — the seven_guis shared module states, for `host_log` and `mcp_log` alone, that assertion messages name a kind and a count, never a needle or a captured line (row decision/log-check-failures-name-kind-count). The post-merge log checks rest on these zeros.

@history_item n=22 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — a ten-commit take was read silent: new target `hit_test_dropped_nodes`; no tracing site added over `packages/`; no wall-clock read in blitz-dom outside `clock.rs`; escher-telemetry unchanged; the sink allowlist re-check held. Rule: each upstream sync is read for log sites and the sink re-checked, which is the template for this chunk's "sink allowlist re-checked" bullet.

@history_item n=23 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the take read at `ebacd68bd` silent, and the sink re-check restated) — the census line for an upstream take is replaced, not given a sibling (body holds current truth, history in this sidecar). The fixed-sha census over the taken commits' `packages/*.rs` lines named its hits (the gated `tracing::error!` at `blitz_vibey_script`, a test-only `TMPDIR` read). escher-telemetry and escher-driver were confirmed unchanged. This is the shape of this sync's census.

@history_item n=24 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (citations re-pointed into the code the take moved) — when a take split blitz-vibey-script's runtime into `runtime/` modules, citations whose code had left the file read false and were re-pointed by hand at Validate. Trap for this sync's citation pass over blitz-dom, stylo_taffy and blitz-paint.

@history_item n=25 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the test crate slice's absences hold `pseudo_element_flow_classification`) — an upstream target added as its own file, with its `mod` line in the unbuilt `all.rs`, was read silent and the slice counts updated. This sync adds `display_contents.rs` and `ime_preedit_underline.rs` plus a `tests/all.rs` `mod` line, so the same reading applies.

@history_item n=26 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the blitz-vibey-script slice's absences after the module split) — the slice absences (metrics, scrub, spans) were restated for moved upstream code under the reading's own counts. This sync's `DOMTokenList.replace` (`6fd5a3ade`) lands in that slice.

@history_item n=27 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the `file:` refusal in the script error surface) — upstream's `file:` refusal, taken on the founder's word, was placed in §7 with no log site of its own. Origin of decision/no-file-loads-from-non-file-documents, which the scope lists among the conflict intents to keep.

@history_item n=28 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — `~ decision/no-file-loads-from-non-file-documents`: an engine sub-resource `file:` fetch for a non-`file:` document fails with fixed text naming no URL or path, adding no log site. A merge conflict in blitz-net must keep it.

@history_item n=29 marker=2026-11-14-remote-pages-local-files
**2026-11-14-remote-pages-local-files** — `~ decision/no-file-loads-from-non-file-documents`: the browser app's favicon and link navigation drop a `file:` load silently. This is the current rule text the merge must keep.

@history_item n=30 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the chunk's surfaces in §6, no log site added, and the census's carve-out) — blitz-shell's `event.rs` inline `#[cfg(test)]` module was held out of the added-lines census under `packages/`, so a 0 there does not cover it. Trap for this sync's added-lines census over the merged delta.

@history_item n=31 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (where a `multiple` select's set, an `optgroup`'s label and the engine's `change` leave) — set the exits of a select's chosen set and an optgroup label (founder forks F6, F7, F10), behind decision/select-chosen-set-never-logged and decision/optgroup-label-crossing. The scope names the select model and its list as conflict intents to keep.

@history_item n=32 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a page's labels as pixels and the size read, the widening ratified) — an open list's shot carries option and group labels as pixels, and an optgroup label is painted as a header; ratified boundary widenings. Origin of decision/open-list-shot-carries-labels and decision/optgroup-label-crossing. Paint changes in the merge (`7beb59176`) must keep them.

@history_item n=33 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open row's bounds and SelectListRow.label) — `SelectListRow.label` holds page text in a public read type and is never logged (decision/select-list-label-never-logged).

@history_item n=34 marker=2026-11-21-single-select-sized-as-a-list-box
**2026-11-21-single-select-sized-as-a-list-box** — `~ decision/select-chosen-set-never-logged`: a single list box's chosen row and labels cross only in the snapshot, the diff and the actions answer. This is the current text of the picked row.

@history_item n=35 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a failed load reaches its handler, and where its error text stops) — upstream's `NetHandler::error` made a failed stylesheet drop from the pending critical resources (decision/failed-stylesheet-leaves-pending-set); `url`/`error` are redacted on `Resource load failed`. Standing trap: a trait default can hide a take's fix, so sweep the implementers of a trait a take extends.

@history_item n=36 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the upstream take read at `e42ee1f25` is silent) — the previous sync's silence reading: no env read, print, tracing site or wall-clock read; only test-only loopback binds; escher-driver at 2 `info_span!` sites; `host_log`/`mcp_log` at 0 needles under both builds; sink re-read with no scrub-set move. Kept: earlier take lines stand as history beside the new one. This is the baseline this sync's reading is set against.

@history_item n=37 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (citations re-pointed past the take's line moves) — digits-only re-point after an upstream take across `document.rs`, `construct.rs`, `damage.rs`, `render.rs` and the CI files. This sync touches layout and paint files, so the same pass is owed.

@history_item n=38 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (`URLSearchParams` and the tenth bootstrap script) — an upstream script API was read as a realization on the registered script crossing, with no log site or carrier, and the slice counts were updated. Precedent for `DOMTokenList.replace` in blitz-vibey-script.

@history_item n=39 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the buffered render's mutation buffer, silent and held in process only) — `MutationBuffer` holds page content in memory only; its `Debug` prints the length alone; the replay re-fires `MutationWriter`'s `trace!` calls. Origin of decision/mutation-buffer-memory-only. The buffered render is a conflict intent to keep, and dioxus-core 0.7.10's `WriteMutations` ties it to the Dioxus 0.7.3 hold.

@history_item n=40 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — `~ decision/checks-silent` exception: a re-run child prints ids or digests. This is the history of the picked row's exception list.

@history_item n=41 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — `~ decision/checks-silent` and `~ decision/headless-stand-no-sink`: layout stand checks install a recorder. `+ decision/layout-read-opens-no-span`: added so the absence is not read as drift.

@history_item n=42 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (4. Span / Trace Coverage) — `~ decision/layout-read-opens-no-span`: reached by the layout verb, the read runs inside that call's command span. This is the current text of the picked row; the merge's layout changes must not add a span to it.

@history_item n=43 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (3. Observability Harness Contract) — `~ decision/sink-host-read-at-trace`: the needle set gains the layout answer's keys. This is the current needle set for this chunk's post-merge `host_log`/`mcp_log` reads.

@history_item n=44 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — `~ decision/headless-stand-no-sink`: acting steps run under escher-telemetry's scoped capture. `~ contract/escher-sink-env/rust-log`: the EnvFilter filters the sink layer alone. `+ decision/capture-framework-targets-warn-up`: the operator's ruling on the capture's target set. This is why new Dioxus or engine warn sites from a take reach the caused answer.

@history_item n=45 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (3. Observability Harness Contract) — `~ decision/headless-stand-no-sink` and `~ decision/checks-silent`: the error-text founder check installs escher's sink over an in-memory writer. This is the current exception text of both picked rows.

@history_item n=46 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (6. Log Coverage) — `~ decision/capture-framework-targets-warn-up`: the capture now admits engine targets at warn and above. An upstream engine warn or error site added by the merge would cross into caused answers.

@history_item n=47 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (7. Error Capture & Reporting) — `~ contract/telemetry-scrub-set/error` and `~ contract/telemetry-scrub-set/url`: the capture scrubs by the same field-name set. Behind the picked §7 rows.

@history_item n=48 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (8. PII Scrubbing & Compliance: 1 row(s)) — `~ decision/escher-target-content-named-set`: `password` joined the content-named set. This is the current redaction list.

@history_item n=49 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (8. PII Scrubbing & Compliance: 3 row(s)) — `− decision/scrub-at-sink-not-call-sites → decision/scrub-by-field-name`, `+ decision/engine-sites-fixed-messages`: the six interpolating engine and framework sites were rewritten to fixed messages with named fields. Trap: an upstream merge that touches or adds an interpolating warn or error site can undo this convention, and a conflict must keep escher's fixed-message form.

@history_item n=50 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — `~ contract/telemetry-scrub-set/error`, `~ decision/scrub-by-field-name`, `~ decision/answers-echo-nothing-typed`: the capture judges by the answer's own hidden set without `error`, so a failed fetch's error text crosses the answer carriers (operator's «Открыть error в caused», later ratified by the founder per 2026-11-17-a-component-in-isolation). These are the current texts of the picked rows.

@history_item n=51 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (3. Observability Harness Contract) — `~ decision/checks-silent` exception: the order page logs. This is the current exception list of the picked row.

@history_item n=52 marker=2026-10-09-audit-corrections-agent-surfaces
**2026-10-09-audit-corrections-agent-surfaces** — the package install's output stays in the job's step log. Kept: a new artifact out of fork CI is a boundary widening that waits for the founder. Applies to any CI workflow edit this merge brings (decision/fork-ci-two-artifacts).

@history_item n=53 marker=2026-10-06-stand-test-contract
**2026-10-06-stand-test-contract** — `target/agent-run/` harness state is local only, uploaded by no CI leg. One origin of decision/local-state-never-uploaded, which the merged CI must keep.

@history_item n=54 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — the recording directory row (`0700`, call content at rest), local only, uploaded by no CI leg. Part of decision/local-state-never-uploaded.
