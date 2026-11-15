@history_item n=1 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — set escher's sink and its allowlist scrub: engine prefixes (`stylo_taffy` among them) print only the seven safe fields, and tracing in the engine and upstream crates is compiled only with the `tracing` feature; a git Taffy that brings new log sites under `stylo_taffy` or a new target must be read against this scrub, not assumed covered.

@history_item n=2 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — origin of `decision/headless-stand-no-sink`: the headless stand and its in-process checks install no subscriber and read no env var; the Taffy move must add no sink, print or env read to any stand check.

@history_item n=3 marker=2026-10-06-founder-rulings
**2026-10-06-founder-rulings** — the founder's ruling behind `decision/no-otel-export`: no OTel crate, egress or credential path; a Taffy lock move must not pull any OTel crate in.

@history_item n=4 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — `escher-session` became a second sink installer and "no sink" was scoped to in-process boots; trap kept for this chunk's readings: a zero read from a host that installs no sink says nothing about one that does, so `host_log` / `mcp_log` must be re-run, not inferred.

@history_item n=5 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — every target outside the engine and escher prefix sets is dropped whole at every level; rule: a target is admitted only together with a scrub rule for it — if the git Taffy brings a `taffy` tracing target, it stays dropped unless a rule is added (a boundary question, not a silent change).

@history_item n=6 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — rule behind `decision/command-span-no-call-content`: never field a `Command`, `Call`, `ArgValue` with `Debug` nor an argument by its schema name; the Taffy move must leave the span's fields untouched.

@history_item n=7 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the span as built) — the `command` span and the `stand_act_spans` re-run children exception in `decision/headless-stand-no-sink`; also records that a workspace-wide build turns `blitz-dom/tracing` on, so engine events reach a child's sink — why `host_log` / `mcp_log` must be read under both builds after a layout-crate move.

@history_item n=8 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — the session host writes the command span for every hosted call (`decision/hosted-call-through-session-run`) and the host's stderr was first read by level under both builds (package vs workspace: one extra `blitz_dom::document` INFO line); the baseline a Taffy move's re-read is set against.

@history_item n=9 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `tool_call` span ratified with exactly `tool` and `ending` (`decision/tool-call-span-two-closed-fields`), `stand_mcp_tools`' in-process recorder the exception to no-subscriber, and `mcp_log` reading the role at `trace` (guard of `decision/sink-host-read-at-trace`).

@history_item n=10 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — precedent for a dependency/upstream merge: the merge's targets join the silence census and `host_log` and `mcp_log` pass under both builds after it; the same reading is owed after the Taffy move.

@history_item n=11 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — the workspace-built host's engine `focussed node` line was read printing twice (5 → 6 lines) with no log site added; trap: an engine change can move host-log line counts without a new site — record such a move as measured, not as a regression or a fix.

@history_item n=12 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — precedent for an upstream take: the taken commits read for tracing sites over `packages/`, `packages/escher-telemetry/` unchanged, `host_log` / `mcp_log` under both builds, and the sink allowlist re-checked; the Taffy chain's blitz-side commits take the same reading.

@history_item n=13 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the host-sink part read) — `host_log`'s paged part read through the host's own sink, its spans the `command` spans recording `verb` alone — the measurement `decision/hosted-call-through-session-run` rests on; it must stay green after the move.

@history_item n=14 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the take read silent; citations re-pointed) — the take's census line was replaced, not given a sibling (the body holds current truth, the sidecar the history), citations into `layout/construct.rs` and others re-pointed by the take's line map; and the `file:` refusal entry records the founder's word ratifying an engine behaviour change for every document — the precedent for the scope's fork on whether Taffy's layout changes need their own ratification.

@history_item n=15 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — origin of `decision/failed-stylesheet-leaves-pending-set` (`NetHandler::error`, a stylesheet's `Err` dropping it from `pending_critical_resources`, which settle and `paint_scene` read); its take-silence reading at `e42ee1f25` and digits-only citation re-points into blitz-dom layout files; trap: a trait default can hide a take's fix — sweep the implementers of a trait a take extends (Taffy's `LayoutPartialTree`-style traits that blitz-dom implements).

@history_item n=16 marker=2026-11-05-residuals-repoint
**2026-11-05-residuals-repoint** — the OTel export's owner sentence in the bootstrap-phases area re-pointed to residual `res-fzitzzw2`; the area holding `decision/no-otel-export`.

@history_item n=17 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the OTel export residual's status) — `res-fzitzzw2` re-carried at 0.3.0 (target: next) on the otel-sdk-install item; `decision/no-otel-export` stands and the Taffy move must not reopen it.

@history_item n=18 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (bridge dispatch) — origin of `decision/no-state-page-skips-registry`: the bridge sets the `SignalRegistry` dispatch only where a registry context stands; a page without one dispatches as before; the load page's handler-coverage check relies on it.

@history_item n=19 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions read) — origin of `decision/actions-read-changes-nothing`: `Session::actions` adds no pump, tick, dispatch or scroll and changes nothing; a layout change must keep `stand_actions`' unchanged-snapshot pin green.

@history_item n=20 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the reached record) — origin of `decision/reached-record-ids-and-events-only`: the reached record holds stable ids and event names, no `NodeId`, its `Debug` its count alone; also the coverage role's replayed steps each writing a `command` span and `host_log`'s coverage run at 0 needles under both builds.

@history_item n=21 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the obs master turned into rows, each verified against the code; the picked rows' handles and guards (`host_log`, `mcp_log`, `stand_act_spans`, `stand_mcp_tools`) date from here — the rows this chunk must leave standing.

@history_item n=22 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — widened `decision/checks-silent`'s exception to a re-run child printing ids or digests; the neighbour of `decision/headless-stand-no-sink`'s exceptions — no new print or child is in the Taffy scope.

@history_item n=23 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — added `decision/exploration-adds-no-span` (§4) and `decision/exploration-catches-panics` (§7): exploration is instrumented only through the command spans of the calls it drives; a layout panic from a new Taffy would surface through that catch, payload withheld.

@history_item n=24 marker=2026-11-14-remote-pages-local-files
**2026-11-14-remote-pages-local-files** — the current §7 wording of `decision/no-file-loads-from-non-file-documents` (refused adding no log site); the §7 area beside `decision/failed-stylesheet-leaves-pending-set`, which the Taffy move must leave unmoved.

@history_item n=25 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — ratified the coverage role's reach of the sink install and of the `command` span (was PROVISIONAL); the current standing of the span reach `decision/hosted-call-through-session-run` describes.
