@history_item n=1 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** — the first upstream merge's wrap re-pointed 20 `file:line` citations into merged upstream files by the merge's measured line map, the merge adding no log, print or tracing site; a sibling entry re-pointed root `Cargo.toml` citations past an insert a prior wrap missed. Precedent for this two-parent merge: read the merged delta for log sites and record "no site added" or name each one; the root manifest moves here too (Stylo, Dioxus, parley, new members).

@history_item n=2 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — the `434e0661` merge's five upstream integration targets joined the no-subscriber census (no subscriber, env read or print), with `host_log` and `mcp_log` passing under both builds after the merge and a fixed-sha census reading 0. Sets the pattern this sync extends: every upstream test target the merge brings is read silent and named, and the host/role trace reads are re-run after the merge (decision/checks-silent, decision/sink-host-read-at-trace).

@history_item n=3 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — the ten-commit take read silent at `f9cbb2756` (no tracing site over `packages/`, no wall-clock read in blitz-dom outside `clock.rs`, escher-telemetry unchanged, `host_log`/`mcp_log` green both builds) and the sink allowlist re-check held; `multi_doc`'s `println!` timings recorded as example output outside the sink. Stated rule: "each upstream sync is read for log sites and the sink re-checked" — the scope's «Sink allowlist re-checked» bullet is this rule.

@history_item n=4 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the take read at `ebacd68bd`; citations re-pointed; slice absences after the module split; the `file:` refusal; the test-crate slice) — replaced the census line rather than adding a sibling (body holds current truth, history in the sidecar); named the two fixed-sha census hits (the `#[cfg(feature = "tracing")]` uncaught-JS-error site at engine target `blitz_vibey_script`, a test-only `TMPDIR` read); re-stated the slice absences after upstream's `runtime/` split; recorded upstream's `file:` refusal in §7. Precedent for this merge's census, for re-reading slice absences (metric/span/pii) when upstream moves code, and for decision/no-file-loads-from-non-file-documents, which the merge's resolutions must keep.

@history_item n=5 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (failed load to its handler; take read at `e42ee1f25` silent; `URLSearchParams`; citations re-pointed) — a taken commit put `NetHandler::error` on a public trait, falsifying a §7 claim; test-only loopback `TcpListener` binds recorded as served by no binary; the tenth bootstrap script moved the slice counts; earlier take lines kept beside the new. Standing trap recorded: "a trait default can hide a take's fix … sweep the implementers of a trait a take extends." Directly relevant to a 212-commit take that adds a CDP listener crate and moves trait-bearing engine crates.

@history_item n=6 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — upstream's own targets `focus_within` and `focus_redraw` recorded silent; the engine changes they pin add no log site; the `Focussed node` INFO line stands after the gate; `host_log`/`mcp_log` pass both builds. Precedent: upstream's new test targets go into the silence census.

@history_item n=7 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — the coverage report's lcov excludes `keyboard_compat.rs` by filename (decision/coverage-artifact-on-success). An upstream-sync-era exclusion the merge's CI edits and coverage leg must keep.

@history_item n=8 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — outside-target records dropped whole at every level; `ENGINE_TARGET_PREFIXES` (seven safe fields) and `ESCHER_TARGET_PREFIXES`; cost: a third-party WARN/ERROR never prints. Rule for later chunks: "a target is admitted only together with a scrub rule for it." Binds decision/sink-target-allowlist and decision/engine-target-safe-fields: any new crate or target the merge brings (`blitz-vibey-cdp-server`, `parley_code_editor`) stays dropped unless admitted with a scrub rule — and admitting one is a boundary widening for the founder.

@history_item n=9 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — escher-telemetry's stderr sink, `service.name`/`service.version` from `CARGO_PKG_*`, the chaining panic hook, the allowlist scrub and its reach (escher's sink only; upstream apps' `fmt::init()` stdout subscribers unscrubbed); engine/upstream tracing compiled only with the `tracing` feature while escher-telemetry is ungated. Origin of decision/telemetry-via-tracing, decision/engine-tracing-feature-gated, decision/scrub-sink-reach and the sink's line contracts; the merge must keep the feature gating on new upstream tracing sites, and upstream's new apps' own subscribers fall under the scrub-reach exception.

@history_item n=10 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the second sink installer (`escher-session`) and the by-level reading. Trap recorded: "a zero read from a host that installs no sink says nothing about one that does" — the post-merge sink check must read the sink-installing host and role, not only in-process checks.

@history_item n=11 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the command span as built; the closed span's line) — escher-driver takes `tracing` ungated; a workspace-wide build turns `blitz-dom/tracing` on so a test child with the sink at `info` receives engine events; the closed-span line class and its scrub rule. Origin of decision/engine-tracing-by-invocation, decision/escher-crates-always-trace and the closed-span-line rows; the Dioxus/Stylo moves change what the workspace build compiles in, so the by-invocation readings must be re-read.

@history_item n=12 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the closed span's line ratified by the founder as built (decision/closed-span-lines), with the ruling that a diff's three sizes may reach a log while its content never does. A ratified boundary the merge must not move; any new record class or span target upstream brings is a fresh widening.

@history_item n=13 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the session host writes the command span; typed text measured in a host's log) — measured the workspace-built host's one `blitz_dom::document` INFO line from `info` up and the typed-sentinel 0 at `trace` both builds. Baseline readings the merge re-reads (decision/engine-tracing-by-invocation, decision/session-logs-carry-no-page-content).

@history_item n=14 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — the workspace-built host's `focussed node` engine line printed twice (6 lines / 978 B), why not established; built-alone unchanged. A by-build engine-line count an engine take can move; record any moved reading, never assume.

@history_item n=15 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`focussed node` re-read beside the focus fixup; the chunk's surfaces, no log site added, and the census's carve-out) — the focus line re-read once per focus move under the workspace build; `blitz-shell/src/event.rs`'s inline `#[cfg(test)]` module held out of the added-lines census so its 0 does not cover it. Trap for the merge's added-lines census over upstream code with inline test modules.

@history_item n=16 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the Focussed node line's count, not settled) — a count through the sink with no base reading settles nothing either way. The merge's re-reads need a pre-merge base to state a reading moved or unmoved.

@history_item n=17 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (§3, §4, §6, §7, §8 rows) — the scoped capture now keeps records' text; capture admits engine targets at warn and above (decision/capture-framework-targets-warn-up); `password` joined the content-named set; decision/scrub-at-sink-not-call-sites replaced by decision/scrub-by-field-name and + decision/engine-sites-fixed-messages (the six interpolating engine and framework sites rewritten to fixed messages with named fields). Trap for this merge: upstream's versions of those sites and any new engine/Dioxus 0.8 warn/error sites may interpolate values again, crossing into caused answers; resolutions must keep the fixed messages.

@history_item n=18 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — ~ contract/telemetry-scrub-set/error and ~ decision/scrub-by-field-name: the capture keeps an `error` field as written (cut at the text bound), so a failed fetch's error text crosses the answer carriers. New upstream fetch/load error sites (CDP server, net changes) would ride this carrier; read them against it.

@history_item n=19 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — ~ decision/headless-stand-no-sink, ~ decision/driver-emits-no-event, ~ contract/telemetry-sink/rust-log, ~ contract/escher-sink-env/rust-log, + decision/capture-framework-targets-warn-up: the scoped capture beside the sink, RUST_LOG filtering the sink layer alone; the capture's Dioxus framework targets (`dioxus*`, `generational_box`, `warnings`) at warn and above because the framework traces on every re-render. The Dioxus 0.7.3 → 0.8.0-alpha.3 move may rename or add framework targets this target set names.

@history_item n=20 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the buffered render's mutation buffer, silent and held in process only) — `MutationBuffer` records dioxus-core 0.7.10's 15 `WriteMutations` calls and replays into `MutationWriter`, re-firing its own `trace!` calls; its `Debug` prints `len` alone (decision/mutation-buffer-memory-only). The Dioxus 0.8 move touches this trait surface; resolutions must keep the buffer silent and in-memory, with `len`-only `Debug`.

@history_item n=21 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (dioxus-native-dom's graph modules and bridge dispatch; the state registry holds no value; the graph answer never logged) — `graph`, `signal_registry` and the bridge dispatch in `DioxusEventHandler::handle_event` add no log site; `SignalRegistry` holds no value (decision/state-registry-holds-no-values, decision/graph-answer-never-logged, decision/no-state-page-skips-registry). dioxus-native-dom is named in scope as a surface the merge must keep.

@history_item n=22 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage module, the reached record and their check log nothing) — dioxus-native-dom's ungated `reached` module, written in `DioxusEventHandler::handle_event`, holds ids and event names only, `Debug` its count alone (decision/reached-record-ids-and-events-only, decision/coverage-line-never-logged). Another escher surface in the Dioxus bridge the merge's resolutions must keep silent.

@history_item n=23 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (per-leg CI failure logs; WPT and publish telemetry upstream-only) — the per-leg failure log artifact confined to `target/ci-logs/`; WPT report, `wptscores.json` and publish-build trace logging upstream `DioxusLabs/blitz` only, their jobs repository-guarded (decision/wpt-scores-upstream-only, decision/wpt-publishing-upstream-only, decision/ci-log-artifact-on-failure). The merge's upstream workflow edits must keep the `github.repository` guards and the two-artifact limit.

@history_item n=24 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — the coverage-report artifact, the one fork-CI upload outside `target/ci-logs/`, a boundary widening (later ratified). Binds decision/fork-ci-two-artifacts and decision/coverage-artifact-on-success: an upstream workflow change adding an upload is a new widening for the founder.

@history_item n=25 marker=2026-10-09-audit-corrections-agent-surfaces
**2026-10-09-audit-corrections-agent-surfaces** — the package install's output stays in the step log (`apt-install.sh`, run by no `ci-leg.sh` leg); "a new artifact out of fork CI is a boundary widening that waits for the founder". Applies to CI workflow edits the merge brings.

@history_item n=26 marker=2026-10-06-founder-rulings
**2026-10-06-founder-rulings** — no OTel export: no OTel crate, no egress, no credential path (decision/no-otel-export), the coverage upload ratified. The merge's new dependencies (CDP server crate, `anyrender_tiny_skia`) are read against this: an OTel crate or egress path arriving by the merge is a widening, not taken silently.

@history_item n=27 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the OTel export residual's status, at its owner site) — the otel-sdk-install item's owner reads residual `res-fzitzzw2`, re-carried at 0.3.0 (target: next). Context for decision/no-otel-export; not among the scope's re-read residuals, so its status stands unless the merge brings an OTel crate.

@history_item n=28 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the masters became rows (decision · contract · absence · overview), each verified against the code. From here an amendment names rows by handle (`+`/`~`/`−`); this chunk's obs amendments state which rows the merge changes, replaces or leaves, rather than re-pointing line citations as earlier syncs did.

@history_item n=29 marker=2026-11-14-remote-pages-local-files
**2026-11-14-remote-pages-local-files** — ~ decision/no-file-loads-from-non-file-documents: the browser app's favicon and link navigation drop a non-`file:` document's `file:` load silently, adding no log site (after 2026-11-13-local-files-for-local-documents named the engine's fixed-text sub-resource refusal). The scope lists the `file:` refusal and the stamped sub-resource request among intents every resolution keeps; upstream's browser app and net changes must not reopen it.

@history_item n=30 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — ~ decision/no-file-loads-from-non-file-documents: blitz-net refuses a `file:` sub-resource for a non-`file:` document with the fixed-text `FileForbidden` error through the handler's error path, naming no URL or path and adding no log site. A fork change in blitz-net the merge's conflict resolution must keep.

@history_item n=31 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — + decision/layout-read-opens-no-span and ~ decision/checks-silent / ~ decision/headless-stand-no-sink exceptions for `stand_layout`'s recorder. The checks-silent exception list is the register the merge's new upstream targets are judged against.

@history_item n=32 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (3. Observability Harness Contract) — ~ decision/checks-silent except: "the order page logs" — pinned code may add a log site only as a named exception. Any log site the merge brings into code the stand checks pin needs the same naming.

@history_item n=33 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — ~ decision/checks-silent except: a re-run child prints ids or digests its parent reads. Part of the exception register the merge's census reads against.
