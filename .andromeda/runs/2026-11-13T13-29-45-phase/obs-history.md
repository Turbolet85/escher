@history_item n=1 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (per-leg CI failure logs; WPT and publish telemetry upstream-only) — added the §9 row behind `decision/ci-log-artifact-on-failure` (`target/ci-logs/{leg}.log`, uploaded only on failure as `ci-log-{job id}`, 7 days) and marked the WPT report, `wptscores.json` and publish `CARGO_LOG` upstream-only, repository-guarded (`decision/wpt-publishing-upstream-only`, `decision/wpt-scores-upstream-only`). Matters here: the take may edit `wpt.yml` (`f2065fd48`, `946f7c461`) and the chunk pins wpt/publish workflow actions; the repository guards and the upload confinement to `target/ci-logs/` must survive the take and the pin edits.

@history_item n=2 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — added the `coverage-report` artifact (later ratified, 2026-10-06-founder-rulings) as the one fork-CI artifact outside `target/ci-logs/`. Matters here: a new upload out of fork CI is a boundary widening; the take's CI edits and the per-OS tally re-read must not add one silently.

@history_item n=3 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — built escher-telemetry: stderr-only non-ANSI sink, `RUST_LOG` default `warn`, `LogTracer` bridge, `service.name`/`service.version`, chaining panic hook, the engine-prefix allowlist and the content-named scrub set; engine/upstream crates compile tracing only behind the `tracing` feature while escher-telemetry is ungated. Origin of `contract/telemetry-sink/*`, `contract/escher-log-line/*`, `contract/escher-telemetry-events/*`, `contract/escher-telemetry-panic-hook/previous-hook`, `decision/engine-tracing-feature-gated`, `decision/engine-target-safe-fields`, `decision/escher-target-content-named-set`. Matters here: these are the rules the chunk's "sink allowlist re-checked" re-reads against every new upstream `tracing`/`log` site, target and env read.

@history_item n=4 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the headless stand installs no subscriber and reads no env var; only the binary installs the sink. Origin of `decision/headless-stand-no-sink`. Matters here: upstream's new/edited test targets the take carries join the silence census and must install no subscriber, read no env var and print nothing.

@history_item n=5 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** (two entries: merged-file citations re-pointed by the merge's line map; root-manifest citations past the line-61 insert re-pointed) — precedent for an upstream merge: re-point `file:line` citations by a measured line map with no claim change, and record that the merge added no log, print or tracing site. Trap: an earlier wrap missed root-`Cargo.toml` re-points past an insert. Matters here: Stylo/Parley/manifest moves and engine edits in the take will shift cited lines in `Cargo.toml`, `document.rs`, `mutator.rs`, `layout/damage.rs` and others.

@history_item n=6 marker=2026-10-06-founder-rulings
**2026-10-06-founder-rulings** — the founder ruled no OTel export (no OTel crate, no egress, no `OTEL_EXPORTER_OTLP_HEADERS` credential path), held as residual `res-fzitzzw2`. Origin of `decision/no-otel-export`. Matters here: upstream's DevTools/CDP or any new dependency the take brings must not introduce an OTel crate, egress or credential path.

@history_item n=7 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — `escher-session` became the second sink installer under `service.name=seven_guis`; the host's stderr was read by level and found leaking ids/names at `debug`/`trace` from outside targets. Trap recorded: a zero read from a host that installs no sink says nothing about one that does. Matters here: the sink re-check must read a sink-installing host (`decision/sink-host-read-at-trace`, `host_log`/`mcp_log`), not only in-process checks.

@history_item n=8 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — outside-target records dropped whole at every level, whatever `RUST_LOG` names; two prefix sets `ENGINE_TARGET_PREFIXES` / `ESCHER_TARGET_PREFIXES` (`escher_` with underscore). Rule for later chunks: a target is admitted only together with a scrub rule for it; accepted cost: third-party WARN/ERROR never print. Origin of `decision/sink-target-allowlist`, `contract/escher-log-line/bridged-log-record`. Matters here: any new engine target the take adds (DevTools/CDP, `parley_code_editor`, WPT manifest runner) is dropped unless admitted with a scrub rule; admitting one is a boundary widening for P4.

@history_item n=9 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — rule: never field `Command`, `Call`, `ArgValue` (later also `Outcome`, 2026-10-07-act-by-id) with `Debug`, nor an argument by its schema name. Basis of `decision/debug-types-never-logged`, `decision/command-span-no-call-content`. Matters here: conflict resolutions in escher-driver or dioxus-native-dom must not reintroduce such fielding.

@history_item n=10 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the command span as built; `tracing` ungated in escher-driver) — the `command` span (eight fields), escher-driver taking `tracing` ungated; a workspace-wide build turns `blitz-dom/tracing` on, so a test child with the sink at `info` receives engine events. Origin of `decision/engine-tracing-by-invocation`, `decision/escher-crates-always-trace`. Matters here: the take's new engine `tracing` sites reach the sink only under the workspace build, so the re-check must read both builds.

@history_item n=11 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the closed span's line; a span's fields under the scrub) — added closed-span lines as a second record class, judged pair by pair by the same allowlist rule, content-named values never stored. Origin of `contract/span-line/*`, `decision/closed-span-lines`, `decision/closed-span-lines-judged-alike`, `decision/content-span-fields-never-stored`. Matters here: any upstream span the take adds is judged by these rules; a tracing-subscriber move must keep `time.busy`/`time.idle` as its own.

@history_item n=12 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the closed span's line ratified as built; a diff's three sizes may reach a log, its content never. Matters here: the ratified record classes are fixed; no new record class arrives with the take without a ruling.

@history_item n=13 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (session host writes the command span; command line writes answers) and (typed text measured in a host's log; host by level under both builds) — stdout carries answers, never log lines; the host read at `trace` under both builds with 0 sentinel/id/name needles, the workspace-built host writing one `blitz_dom::document` INFO line. Origin of `contract/escher-session-stdout/command-answer`, `decision/session-logs-carry-no-page-content`. Matters here: the baseline line counts the sink re-check compares against after the take.

@history_item n=14 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `tool_call` span (two closed fields, `tool`/`ending`) ratified; the `mcp` role writes protocol lines only on stdout. Origin of `decision/tool-call-span-two-closed-fields`, `contract/mcp-role-stdout/json-rpc`, `contract/escher-session-mcp/stdout`. Matters here: guard `stand_mcp_tools` and `mcp_log` must stay green after the take.

@history_item n=15 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — precedent: the merge's five upstream integration targets joined the no-subscriber census, `host_log`/`mcp_log` passing under both builds, the fixed-sha census reading 0. Matters here: the same census treatment for upstream's new test files this take carries.

@history_item n=16 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — re-read the workspace-built host: the `blitz_dom::document` `focussed node` INFO line printing twice (6 lines / 978 B; `mcp` role 11), why not established; dated readings kept as dated, re-reads appended. Matters here: a moved engine line count after the take is recorded beside the prior reading, explained or marked unexplained.

@history_item n=17 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — client stderr, host beyond CRUD, and the workspace-built windowed `seven_guis_native` read by build and level, 28 runs, 0 needles. Matters here: the scope of the by-build reading the sink re-check can re-run.

@history_item n=18 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — the coverage report's lcov excludes `keyboard_compat.rs` by filename (test-plan §10). Matters here: a prior upstream sync that touched the §9 coverage row; the take must keep the exclusion consistent.

@history_item n=19 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — upstream focus checks `focus_within`/`focus_redraw` recorded silent; `focus_redraw`'s `RecordingShell` a counting `ShellProvider`, not a sink; engine take PROVISIONAL (later dropped, 2026-10-20-select-by-id). Matters here: precedent that an upstream test helper recording calls is not a sink; read new upstream test helpers the same way.

@history_item n=20 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — the ten-commit take read silent (no tracing site over `packages/`, no wall-clock read outside `clock.rs`, escher-telemetry unchanged, `host_log`/`mcp_log` pass both builds); sink allowlist re-check held; `multi_doc`'s printed timings recorded in §1/§5/§6. Rule: each upstream sync is read for log sites and the sink re-checked. Matters here: the direct template for this chunk's "sink allowlist re-checked" and the `overview/5-metric-coverage` examples.

@history_item n=21 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the take read at `ebacd68bd` silent, the sink re-check restated) — the census line replaced, not given a sibling (a body holds current truth, history in the sidecar); the uncaught-JS-error `tracing::error!` at `blitz_vibey_script` stays feature-gated, the sink printing its safe fields and the message `[redacted]`; a test-only `TMPDIR` read recorded. Matters here: the form and the replace-vs-beside choice for this take's census line (the 2026-11-01 take kept its readings beside instead).

@history_item n=22 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (citations re-pointed into the code the take moved) — script-error-surface citations moved to `runtime/events.rs`/`runtime/timers.rs` after upstream's split; a citation whose code left its file was fixed by hand. Matters here: `contract/script-error-source-labels/*`, `contract/window-error-event/*`, `contract/window-onerror/arguments` cite code upstream may move again; re-point by hand where a line map cannot follow.

@history_item n=23 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (blitz-vibey-script slice absences after the module split; test crate slice holds `pseudo_element_flow_classification`) — slice absences restated with counts after upstream moved files; a new upstream target extends the test-crate slice. Matters here: slice counts (13 modules, 10 bootstrap JS, 32/61 files) may move with this take's new files and must be restated.

@history_item n=24 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the `file:` refusal in the script error surface) — upstream's `document_may_load` refusal placed in §7 as a typed `FetchError::Forbidden`, ratified as an engine change for every document. Origin of `decision/no-file-loads-from-non-file-documents`. Matters here: precedent for placing an upstream engine behaviour change for every document as a ruled widening; conflicts must keep the refusal.

@history_item n=25 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the chunk's surfaces in §6, the census's carve-out) — `blitz-shell/src/event.rs`'s inline `#[cfg(test)]` module held out of the added-lines census, so that census's 0 does not speak for it. Matters here: the take's added-lines census over `packages/` must name any such carve-out.

@history_item n=26 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`focussed node` re-read beside the focus fixup) and (where a `multiple` select's set, an `optgroup`'s label and the engine's `change` leave) — the focussed-node INFO line once per focus move under the workspace build; the chosen set, optgroup label and checkable `change` exits. Origin of `decision/select-chosen-set-never-logged`, `contract/multiple-select-form-entries/chosen-option`, `contract/snapshot/multiple-select-value`, part of `decision/optgroup-label-crossing`. Matters here: the select model is a conflict surface the take must keep; the AccessKit refresh (`be60597f6`) may touch the optgroup `Role::Group` name.

@history_item n=27 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a page's labels as pixels and the size read, the widening ratified) and (an open row's bounds and `SelectListRow.label`) and (the Focussed node line's count, not settled) — ratified open-list pixels, the `size` read capped at 65534, optgroup headers painted; row bounds crossing, labels never logged; the focussed-node count through the sink read 0 with no base. Origin of `decision/open-list-shot-carries-labels`, `decision/select-size-attribute-read`, `decision/optgroup-label-crossing`, `contract/snapshot-diff/open-list-row-bounds`, `decision/select-list-label-never-logged`. Matters here: the select list is named among the surfaces the take must keep; upstream paint changes (text-shadow, masks) reach the list's pixels.

@history_item n=28 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a failed load reaches its handler, and where its error text stops) — `NetHandler::error` taken from upstream; a failed stylesheet leaves the pending critical set; `url`/`error` scrubbed on `Resource load failed`; report overstated what the sink prints and the body holds the measured read. Standing trap: a trait default can hide a take's fix (it stopped at `CountedHandler`); sweep the implementers of a trait a take extends. Origin of `decision/failed-stylesheet-leaves-pending-set`, `contract/telemetry-scrub-set/error`, `contract/telemetry-scrub-set/url`. Matters here: apply the trait-implementer sweep to this take's trait changes; lossy page decoding and DevTools may add error paths.

@history_item n=29 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the upstream take read at `e42ee1f25` is silent) — the last take's sink reading: no env read, print, `tracing` site or wall-clock read; the only binds test-only loopback `TcpListener`s in blitz-net's `#[cfg(test)]`; escher-driver 2 `info_span!`, `stand_act_spans` eight fields, `host_log`/`mcp_log` 0 needles at `trace` both builds; no scrub set moves; the `ebacd68bd` lines kept beside as history. Matters here: the immediate precedent and baseline for this chunk's re-check, including how to treat test-only listeners (DevTools/CDP may open real ones).

@history_item n=30 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (citations re-pointed past the take's line moves) and (`URLSearchParams` and the tenth bootstrap script) — digits-only re-points across §4–§9 and telemetry-strategy; bootstrap JS count 9 → 10, a new script API read as a realization on the registered script crossing. Matters here: the same re-point pass after this take, and any new script API the take adds is read for what it carries out.

@history_item n=31 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the probe's read values stay in the test) — the probe's failure messages and reading carry no read value. Origin of `decision/runtime-probe-leaks-no-read-value`. Matters here: the probe is a check a Dioxus 0.8 shape would re-run and must still leak nothing.

@history_item n=32 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the buffered render's mutation buffer, silent and held in process only) and (the OTel export residual's status) — `MutationBuffer` holds page content in memory only, `Debug` prints `len`, the replay re-fires `MutationWriter`'s `trace!` calls; `res-fzitzzw2` re-carried at 0.3.0. Origin of `decision/mutation-buffer-memory-only`. Matters here: the buffered render's merge surface is freight (`cry-246ouqff`); upstream 0.8's `mutation_buffer.rs` would replace it whole, and this row must hold for whatever replaces it.

@history_item n=33 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the snapshot's clickable reading carries no page text) — `clickable` a closed bool after `focused`. Origin of `contract/snapshot-node/clickable`, `contract/snapshot-text/clickable`. Matters here: the snapshot is a named conflict surface; listener or disabled-control changes upstream must keep the bool's meaning.

@history_item n=34 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions read opens no span and adds no site) — `Session::actions` changes nothing in the document; `actionable_nodes` read-only. Origin of `decision/actions-read-changes-nothing`. Matters here: a guarded behaviour that upstream mutation or AccessKit changes must not break.

@history_item n=35 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` restates the driver's spans and the scrub's limits; its pin reads none of them, so a change is made there by hand. Origin of `decision/capability-statement-updated-by-hand`. Matters here: if the take moves the scrub's limits or span set (e.g. a new admitted target), the statement is edited by hand.

@history_item n=36 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the host's six slugs, the served stand's silence and the host-sink part read measured) — the hosted part read measured through the host's own sink, its spans the `command` spans recording `verb` alone. Basis of `decision/hosted-call-through-session-run`. Matters here: `host_log`'s part-read test is among the standing sink checks to re-run after the take.

@history_item n=37 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (escher-session's coverage role, a further reach of the same sink install) and (the coverage line's carried fields) — the coverage line on stdout as an answer, never a log line; PROVISIONAL (delegate overseer) for the founder's epoch-boundary batch. Origin of `contract/coverage-role-stdout/coverage-line`. Matters here: this chunk opens Epoch 4, so these PROVISIONAL marks may be due in the founder's batch and must not be discharged on the chunk's own judgement.

@history_item n=38 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the masters became rows (315 records → 281 rows), each verified against the code at `edd0cfaea7f0` and re-read at `f84ccd9f58bc`; 31 divergences recorded as the code shows, 2 as written owed by the route. Matters here: every picked row's text is the 2026-11-12 reading; a row the take falsifies lands as a row amendment (`~ {handle}`) at the wrap, as the changed-set qualifier freight does.
