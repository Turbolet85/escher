@history_item n=1 marker=amd-dtc2dehg
**amd-dtc2dehg** — (2026-10-27-upstream-sync-engine-debts, the take read at `ebacd68bd` silent) the census line was replaced to read the upstream take at `ebacd68bd`: upstream's new target read 0 env · 0 print · 0 subscriber, the taken commits' `packages/*.rs` lines read by a fixed-sha census, no wall-clock read in blitz-dom outside `clock.rs`, escher-telemetry and escher-driver unchanged, `host_log` / `mcp_log` passing under both builds, and the sink allowlist re-checked. The precedent for this take: every upstream take is read for log sites and the sink is re-checked. This chunk takes `ebacd68bd`'s hit-test panic fix again together with `0980cce5f` and the five Taffy bumps, and the new `inline_box_scrollable_overflow` target and the `mutator.rs` unit test need the same reading.

@history_item n=2 marker=amd-q5gqadnt
**amd-q5gqadnt** — (2026-11-01-upstream-sync-ui-graph, the take read at `e42ee1f25` silent) added a §3 silence line, a §4 "no span, field, event or log line" item naming the engine functions the take touched, §6 no-site lines with absence rows for the new test targets, and a §8 re-read. Kept: earlier take readings stand as history beside the new one, not replaced. This is the template for recording the Taffy take's silence: the engine's scroll and hit seams (`hit_inner`, `physical_layout`, `scroll_into_view`, `is_scroll_container`) open no span.

@history_item n=3 marker=amd-twt66vwo
**amd-twt66vwo** — (2026-10-24-upstream-sync-polish-and-ship) the ten-commit take was read silent at `f9cbb2756`: `hit_test_dropped_nodes` was silent, the dropped-node hit skip added no log site, and the sink allowlist was re-checked. This is an earlier precedent for an upstream take that touches hit testing. Its census line was later replaced by amd-dtc2dehg.

@history_item n=4 marker=amd-upkcm6z6
**amd-upkcm6z6** — (2026-10-13-upstream-sync-polish-and-ship) the merge's five upstream integration targets joined the no-subscriber census, each with no subscriber, no env read and no print, with `host_log` / `mcp_log` passing after the merge. It sets the rule that an upstream take's new targets are named in the silence census, which applies to the new `inline_box_scrollable_overflow` target.

@history_item n=5 marker=amd-pig47ax3
**amd-pig47ax3** — (2026-10-27-upstream-sync-engine-debts) the test-crate slice's span and telemetry absences were amended to hold a new upstream target (`pseudo_element_flow_classification`), so the slice reading still names the whole slice. Trap: a new upstream test file in `tests/blitz-tests/tests/` leaves the slice count stale unless it is named.

@history_item n=6 marker=amd-u7uo5fgf
**amd-u7uo5fgf** — (2026-10-06-upstream-sync-element-identity) 20 `file:line` citations into merged upstream files were re-pointed by the merge's line map; the merge added no log site. This is the oldest precedent that an upstream take moves cited lines.

@history_item n=7 marker=amd-3cr6gxvn
**amd-3cr6gxvn** — (2026-10-27-upstream-sync-engine-debts) citations were re-pointed into code the take moved, blitz-dom `mutator.rs` among the files; a citation whose code left its file was re-pointed by hand at Validate. Trap for this take, which edits `mutator.rs`, `node/node.rs`, `events/pointer.rs` and `layout/*`.

@history_item n=8 marker=amd-qv7ebnld
**amd-qv7ebnld** — (2026-11-01-upstream-sync-ui-graph) digit-only citation re-points, among them blitz-dom `node/node.rs`, `mutator.rs` and `document.rs`. These are files this take changes, including `node.rs`'s `hit_inner` conflict and `document.rs`'s `init_stylo_prefs` conflict, so any remaining line citations into them move again.

@history_item n=9 marker=amd-zihybibd
**amd-zihybibd** — (2026-10-31-engine-debts) citations were re-pointed into blitz-dom `document.rs`, `mutator.rs` and `events/pointer.rs`, all with claim text unchanged. `events/pointer.rs` is in this take's touch list.

@history_item n=10 marker=amd-z2cdlvn4
**amd-z2cdlvn4** — (2026-10-31-select-popup) re-pointed citations into blitz-dom `document.rs` and `events/pointer.rs` after the select list's added lines. This is a further precedent for the files this take touches.

@history_item n=11 marker=amd-ctrpbs2a
**amd-ctrpbs2a** — (2026-10-10-scrolling-box-bounds-and-hit) the engine's scrolled-box checks `scrolled_box_client_rect`, `hit_clipped_at_scrolling_box` and `scrolled_box_absolute_position` joined the no-subscriber census; the chunk added no log site and the command span kept its eight fields. These are the checks of the own-scroll rule, the hit stop and the `absolute_position` reading that this take must keep. They must stay silent after the reconcile.

@history_item n=12 marker=amd-7xw4i5on
**amd-7xw4i5on** — (2026-10-18-engine-debts) the rewritten `scrolled_box_absolute_position` and `hit_clipped_at_scrolling_box` were recorded silent, together with `transformed_client_rect` and `stand_act_transformed`. Engine read changes were recorded as adding no log site, span or field. This is the precedent for re-reading those checks' silence when the scroll and hit readings change.

@history_item n=13 marker=amd-7g47dwps
**amd-7g47dwps** — (2026-10-31-select-popup) `select_popup`, `select_popup_painted` and `stand_act_popup` were recorded silent; the select list adds no log site or span. This take's seam fix `is_scroll_container` exists because upstream's overflow read panicked in `select_popup`'s inert/disabled/hidden/removed test under workspace features. That check's silence row stands on the code path this take changes.

@history_item n=14 marker=amd-twhohqtg
**amd-twhohqtg** — (2026-10-31-select-popup) ratified widenings: a shot of an open list carries option and group labels as pixels, and an `optgroup` label is painted as a group header on no log line. These are the source of the picked rows `decision/open-list-shot-carries-labels` and `decision/optgroup-label-crossing`. The take's scroll and hit changes must not move these crossings.

@history_item n=15 marker=amd-tjocwgx3
**amd-tjocwgx3** — (2026-10-31-select-popup) an open list's row crosses the snapshot and the diff by its own `bounds`, within a closed `differs` set, and `SelectListRow.label` is never logged. These are the source of the picked rows `contract/snapshot-diff/open-list-row-bounds` and `decision/select-list-label-never-logged`. A scroll-origin change can move row `bounds` readings but must add no field.

@history_item n=16 marker=amd-2rlbtdpf
**amd-2rlbtdpf** — (2026-10-31-engine-debts) recorded where a `multiple` select's set, an `optgroup`'s label and the engine's `change` leave, each with no log site on its added lines. This is the first form of the optgroup label crossing that amd-twhohqtg later widened to painting.

@history_item n=17 marker=amd-oenf6egf
**amd-oenf6egf** — (2026-11-01-upstream-sync-ui-graph) a failed stylesheet reaches its handler through `NetHandler::error` and leaves the pending critical set. This is the source of the picked row `decision/failed-stylesheet-leaves-pending-set`. Standing trap from this take: a trait default can hide a take's fix, so sweep the implementers of any trait the take extends. That applies here to Taffy's trait surface (for example `cache_get` taking `&self`, and `ScrollableOverflow`).

@history_item n=18 marker=amd-cschmkve
**amd-cschmkve** — (2026-10-07-driver-command-spans) the command span as built: eight fields, `added` · `removed` · `changed` among them. A workspace-wide build turns `blitz-dom/tracing` on, so a test child with the sink at `info` also receives engine events. This is the source of `decision/engine-tracing-by-invocation` and the `command-span` added/changed/removed rows. The scope says the driver's answers change only where engine readings change.

@history_item n=19 marker=amd-u2nw2mep
**amd-u2nw2mep** — (2026-10-09-founder-rulings) the founder ratified the closed span's line, including that the three sizes of a diff may reach a log while its content never does. The `added` / `removed` / `changed` count fields stand on this, so a scroll-reading change that moves diff sizes stays within the ratified fields.

@history_item n=20 marker=amd-6zxxoq74
**amd-6zxxoq74** — (2026-10-07-driver-command-spans) the closed span's line shape and the scrub of a span's fields: escher targets print every pair unless content-named, engine targets print only the safe fields, and other targets print nothing. This is the source of `contract/command-span/line` and `decision/scrub-at-sink-not-call-sites`.

@history_item n=21 marker=amd-3jlpr7el
**amd-3jlpr7el** — (2026-10-10-mcp-surface) the `tool_call` span was ratified with its fields `tool` and `ending`, closed words only. This is the source of the picked row `contract/tool-call-span/line`. The scope's boundary allows no new span or field.

@history_item n=22 marker=amd-2iwce5bg
**amd-2iwce5bg** — (2026-10-31-select-popup) the driver's fixed help and meaning texts add no log site or span field, the `command` span keeping its eight fields. This is the source of `contract/command-span/field-count`. The select-list engine readers were recorded as opening no span.

@history_item n=23 marker=amd-dnjkg5io
**amd-dnjkg5io** — (2026-10-10-driver-cli) measured the session host under both builds: one engine line, `blitz_dom::document` at INFO, from `info` up under the workspace build and none when the host is built alone. This is the engine-tracing-by-invocation reading that an engine change can move.

@history_item n=24 marker=amd-4cbpytxz
**amd-4cbpytxz** — (2026-10-14-stand-requirement-sweep) re-read the workspace host and found the `focussed node` INFO line printed twice (6 lines, 978 B), with the cause recorded but not established. Trap: an engine change can alter engine-event counts under the workspace build, so `host_log` / `mcp_log` line counts are worth re-reading after the take.

@history_item n=25 marker=amd-t7lc5vnn
**amd-t7lc5vnn** — (2026-10-31-engine-debts) re-read the `focussed node` line: once per focus move under the workspace build, holding a node number and no id or name. This is the engine event most likely to move when this take changes the hit and scroll paths.

@history_item n=26 marker=amd-cmorfrzt
**amd-cmorfrzt** — (2026-10-31-select-popup) a count of the `focussed node` line through the sink on the workspace build read 0 with no base reading, so it settles nothing. A take that wants to claim "unmoved" needs a base reading first.

@history_item n=27 marker=amd-xzye3ehn
**amd-xzye3ehn** — (2026-10-14-quality-gates) `busy` is driven on a real line, and the client's stderr and the workspace-built windowed binary were read by build and level, 0 needles. These are the standing workspace-build readings that the engine take must not move.

@history_item n=28 marker=amd-7v2ajbq2
**amd-7v2ajbq2** — (2026-10-06-telemetry-bootstrap) established that tracing compiles only with the `tracing` feature in the engine and upstream crates, plus the allowlist scrub of the engine prefixes, `stylo_taffy` among them. This is the source of `decision/engine-tracing-feature-gated` and `decision/tracing-for-telemetry`. The Taffy bump must keep any new Taffy or `stylo_taffy` log site behind that gate and under the engine scrub.

@history_item n=29 marker=amd-yjzlnn3o
**amd-yjzlnn3o** — (2026-10-07-sink-target-allowlist) records from any target outside the two prefix sets are dropped whole at every level. Rule: a target is admitted only together with a scrub rule for it. If the Taffy bump brings a new crate target with log sites (for example `taffy`), it is dropped, not printed.

@history_item n=30 marker=amd-vm35icer
**amd-vm35icer** — (2026-10-07-driver-session) two sink installers, the seven_guis binaries, and the scrub's reach scoped to the measurement. Trap: a zero read from a host that installs no sink says nothing about one that does. This is the source of `decision/scrub-sink-reach`.

@history_item n=31 marker=amd-zbauq3ll
**amd-zbauq3ll** — (2026-11-16-layout-explained) `decision/checks-silent` and `decision/headless-stand-no-sink` gained the layout stand check's own recording subscriber for its TRACE capture of the layout read. This is the current exception list that this take's new upstream tests must fit within.

@history_item n=32 marker=amd-vqwvafxb
**amd-vqwvafxb** — (2026-11-16-layout-explained) added `decision/layout-read-opens-no-span`, guarded by stand_layout's `the_read_writes_no_escher_line_at_trace`. The layout read sits on the Taffy layout and scrolled-box readings this take changes, so that guard must stay green after the bump.

@history_item n=33 marker=amd-y7qwmdzh
**amd-y7qwmdzh** — (2026-11-14-seeded-randomness) widened `decision/checks-silent`'s exception to a re-run child printing ids or digests for its parent. This is relevant because the take must leave the committed digests (`stand_cross_os_readings`) and their re-run reading unchanged.

@history_item n=34 marker=amd-b3ck4xhx
**amd-b3ck4xhx** — (2026-11-16-errors-with-the-action, §3) an in-process acting step now runs under escher-telemetry's scoped capture, which prints nothing, and the sink's `RUST_LOG` filter applies to the sink layer alone. This is the latest telemetry change on the base this take stands on (`06c8ed8a2`). An engine panic or event on a hit or scroll path inside an acting step is now counted by the capture.

@history_item n=35 marker=amd-pmdqczdc
**amd-pmdqczdc** — (2026-11-16-errors-with-the-action, §4) `decision/driver-emits-no-event` now names the scoped capture around an acting step, which counts records and prints nothing, while the command span is unchanged. The scope forbids any new span or log field, and the take must keep this row true.

@history_item n=36 marker=amd-cazdlog4
**amd-cazdlog4** — (2026-11-16-errors-with-the-action, §6) the `RUST_LOG` EnvFilter applies to the sink layer alone, and `decision/capture-framework-targets-warn-up` was added: the capture counts framework targets at warn and above. A WARN from engine or Taffy-adjacent code that the take introduces on an acting step's path would surface in an acting answer's `caused`. The panic fixed by the `is_scroll_container` seam (`ebacd68bd`'s class of "no computed styles" panic) would also have reached it.

@history_item n=37 marker=amd-ei5tfcg5
**amd-ei5tfcg5** — (2026-11-08-what-can-i-do-here) the actions read changes nothing in the document and adds no `scroll_into_view` call, opening no span. This is the source of `decision/actions-read-changes-nothing`. The take rewrites `scroll_into_view`'s `scroll_state` reading, which the actions read must still not reach.

@history_item n=38 marker=amd-6x37n7n2
**amd-6x37n7n2** — (2026-10-19-recorded-check-replay) each kept answer is held at rest, and the committed Counter checks `counter-{cli,mcp}.json` are public, with no log line. This is the source of `decision/committed-kept-checks-clean`, `decision/committed-checks-hold-no-private-data` and the answer-digest contract. The measurement reads the committed checks passing unchanged, which the take must keep.

@history_item n=39 marker=amd-bapu2wis
**amd-bapu2wis** — (2026-10-06-founder-rulings) no OTel export in escher: no OTel crate, no egress, no credential path. This is the source of `decision/no-otel-export`. The dependency bump must add no OTel crate (the scope keeps the dependency list unchanged).

@history_item n=40 marker=amd-frvauysc
**amd-frvauysc** — (2026-11-07-mounteddata-buffered-render) the OTel export residual `res-fzitzzw2` re-carried at 0.3.0 (target: next) at the `otel-sdk-install` owner site. This is the current status of the row `decision/no-otel-export` rests on.

@history_item n=41 marker=amd-i4np4ecm
**amd-i4np4ecm** — (2026-11-12T03-36-23-rows) the masters became rows (decision · contract · absence · overview), each verified against the code. The picked rows' handles date from this conversion. Line citations now live in the code rather than the master, so this take's amendments are row edits (`~ handle`) rather than citation re-points.
