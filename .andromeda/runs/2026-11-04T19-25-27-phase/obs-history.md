@history_item n=1 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — (escher-telemetry: chaining panic hook, allowlist scrub) Added escher-telemetry's chaining native panic hook. It logs one ERROR at `escher_telemetry::panic`, then runs the previous hook, so std still prints the raw message. It also added the allowlist scrub, which redacts `panic.payload` and keeps only safe fields on the engine prefixes (`dioxus_native` among them), and it recorded the raw panic message as residue past the scrub. This is where the extract's §7 Panic hooks and §8 Values logged as-is come from: the reading's account of the MountedData panic (one redacted ERROR plus the raw stderr message in a sink-installing binary) rests on it. The probe must quote neither.

@history_item n=2 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — Named `seven_guis_native` as the one installer. The headless stand `seven_guis::stand` and its in-process checks install no subscriber, read no env and print nothing, so a headless boot has no escher sink. This is the "headless stand row" the extract's silence constraint rests on: the probe runs headlessly on the stand and must keep that row true.

@history_item n=3 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — Made the one `println!` exception: `stand_id_persistence`'s re-executed child prints to stdout that the parent captures and reads only into assertions, never a log. This is the precedent the extract's re-run-child pattern names if the probe has to isolate the panicking `poll` in a child process. The child's output must go only into assertions.

@history_item n=4 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — (bridge log-site citations) Re-pointed dioxus-native-dom `mutation_writer.rs` citations in §6 and §8 Values logged as-is and recorded the bridge's `trace!` count as 13, with no site added by that chunk. These are the unscrubbed text and attribute `trace!` sites the extract forbids the probe to read from or add beside. A change in dioxus-native-dom must keep that count, and it must re-point the citations if lines move.

@history_item n=5 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — Extended the headless-stand census to cover the checks' shared module `tests/blitz-tests/tests/common/mod.rs`. It also recorded that dioxus-native-dom's unit-test file `dioxus_document_tests.rs` is named in no obs-plan sentence, its census of 0 kept in the chunk report only. Precedent: a probe check reading `common` keeps that module in the census, and any probe test placed in dioxus-native-dom has no obs-plan home unless a row is written.

@history_item n=6 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — (second sink installer) Added `escher-session` as a second installer, under the same `service.name=seven_guis`, and scoped "a headless boot has no escher sink" to a boot made in process. §7 Panic hooks names both installers. Trap recorded: a zero read from a host that installs no sink says nothing about one that does. The probe's in-process zero must not be stated as covering the sink-installing binaries, where the MountedData panic would reach the hook.

@history_item n=7 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — Made the sink drop every target outside `ENGINE_TARGET_PREFIXES` and `ESCHER_TARGET_PREFIXES`, whole and at every level, whatever `RUST_LOG` says. Rule: a target is admitted only together with a scrub rule for it. This is behind the extract's anti-pattern that the sink "drops every other target" and so gives no excuse for printing. A probe must not add a target either.

@history_item n=8 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — (the command span as built) Built the `command` span's eight fields with its never-fielded list and made `tracing` ungated in escher-driver. It made §2 Feature wiring say a workspace-wide build turns `blitz-dom/tracing` on, while a package-alone build leaves it off. It also set the census's one exception: `stand_act_spans`' two re-run children, with the sink over an in-memory capture. This is the source of the extract's eight-field binding, the build-dependence constraint and the `stand_act_spans` carve-out for installing the sink.

@history_item n=9 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — (the session host writes the command span) Measured By invocation for the session host: one `blitz_dom::document` INFO engine line under the workspace build, none when built alone. Standing rule: no span or log field added, and the next surface adds none that holds what a call supplied or a screen reads. This backs the extract's "measurements must not depend on engine log output" and its rule that nothing the probe reads may be fielded.

@history_item n=10 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — Three engine checks without a shared module joined the no-subscriber census by name, and no count moved. Precedent: a probe check added to tests/blitz-tests is named in the census, and the reader counts move only if it declares a shared module.

@history_item n=11 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — Ratified the `tool_call` span's two closed-word fields (`tool`, `ending`) and made `stand_mcp_tools`' in-process recorder a second exception to "installs no subscriber". This is the source of the extract's two-field binding and of the hand-written `Recorder` under `with_default`, the only allowed route if a verdict needs a span read.

@history_item n=12 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — Added `stand_screenshot_repeat` as a second re-executed child whose stdout the parent reads into assertions only. This is the extract's second named precedent for isolating a panicking poll in a child.

@history_item n=13 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — Moved the `common` reader count to 19. The silence probe was re-run at wrap over the shared module whose helpers the check reads, because the report's probe had read the check file alone. Trap: the probe's census must cover the shared modules it reads, not only the new file, as the extract's acceptance states.

@history_item n=14 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — The silence census over the extended checks and the shared module read 0 (no subscriber, env read, print or listener). This is the "stand-a11y-assertions probe row" the extract cites as the census pattern for a new check file.

@history_item n=15 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — (§2 Feature wiring re-read) Re-read the workspace-built host: the `blitz_dom::document` `focussed node` INFO line prints twice (6 lines / 978 B), while the host built alone is unchanged. Recorded, not explained. This shows engine output moving between builds, which is why the extract says no probe verdict may rest on engine log output.

@history_item n=16 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — (the stderr readings) Read the workspace-built windowed `seven_guis_native` by build and level, with 0 needles, and drove `busy` on a real `command` line in `stand_act_spans`. This bears on the reading's windowed-path statement (read, not run, for the panic) and on §2 Feature wiring by build.

@history_item n=17 marker=2026-10-19-test-splits
**2026-10-19-test-splits** — The seven_guis shared module `tests/common/mod.rs` states, for `host_log` and `mcp_log`, assertion messages that name a kind and a count, never a needle or a captured line. escher-driver keeps 2 `info_span!` sites. This is the extract's "a failing check's message names a kind and a count" pattern and its 2-`info_span!` acceptance.

@history_item n=18 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — Put the third shared module, `tests/blitz-tests/tests/fixtures/mod.rs`, in the silence census. Its readers hold the 4 print or sink sites of `stand_act_spans`' re-run children, and none was added. A probe reading `fixtures` extends its reader count and must keep those 4 sites the only ones.

@history_item n=19 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — Recorded that dioxus-native-dom's `Snapshot::text_parts` opens no span, emits no event and records no field. Precedent for an escher-side reader added inside dioxus-native-dom (the probe's "exposed with a cost" class): it is recorded as opening no span and adding no field.

@history_item n=20 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — (the served stand's silence) Recorded the served stand as silent: `boot_cells`, `boot_load` and `LoadNet` hold no print, `tracing` call, env read or listener, with the census 0 over `stand.rs` and the blitz-tests `common` and `fixtures` modules. The probe's served task must keep that census at 0.

@history_item n=21 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (four entries) The chunk where the MountedData hazard was measured. (a) Its new checks' silence went to a §6 `observed absent` row, and §3's silence block got no row. (b) The added-lines census over `packages/` holds out inline `#[cfg(test)]` modules, so its 0 does not speak for them. dioxus-native-dom's `closed_select_of` and `is_multiple_select` are readers recorded instrumentation n/a. (c) The `fixtures` readers moved 24 to 28. `session_common` was left at twenty-nine though the census read 31. Precedents: where the probe's silence row goes, the carve-out for its added-lines census, and the counts it extends.

@history_item n=22 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the select list adds no log site; its checks are silent) Added `is_select` to dioxus-native-dom's silent readers, instrumentation n/a. It moved `session_common` readers to thirty-two and `stand_act_*` to sixteen, and wrote the §6 observed-absent row together with the §3 row for the new checks. These are the current counts and the row forms a probe check in tests/blitz-tests extends.

@history_item n=23 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (the test crate slice's absences) Made the 61-file test crate slice's absences (spans or traces, telemetry or logging crates) note a new target added to the slice. The cascade binds the probe's tests/blitz-tests file to these same 61-file absences. A new probe file needs the same note, and the slice count must not be stated as covering a file it never read.

@history_item n=24 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (two entries) (a) Read the take at `e42ee1f25` silent: the `flush_queued_mounted_events` guard and the fork's fixes add no env read, print, `tracing` site or span. escher-driver holds 2 `info_span!` sites, `host_log` and `mcp_log` read 0 needles at `trace` under both builds, and `packages/escher-telemetry/` and `packages/escher-driver/` are unchanged. This is the baseline the extract and the cascade (fct-7dblu2wz) cite for re-verifying `events.rs:162` and `poll` at HEAD. (b) Re-pointed citations, telemetry-strategy.md's By invocation line among them, which still names `ci-leg.sh fast` as a workspace build though `fast` now ends on a `cargo test -p blitz-tests` step. Trap: which build a runner makes decides whether `blitz-dom/tracing` is on.
