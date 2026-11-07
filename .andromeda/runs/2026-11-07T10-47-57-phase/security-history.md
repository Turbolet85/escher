@history_item n=1 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the `MountedData` panic from a spawned task) first recorded in §Error Handling that a task spawned from a key handler that writes a signal is first polled inside `render_immediate`, which holds the document's `RefCell`, so a `MountedData` call there panics `RefCell already borrowed`, any app able to meet it; Cells calls its handles synchronously to avoid it. This is the record the chunk closes, and the reason the fix is an engine change for every Dioxus document.

@history_item n=2 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (the MountedData hazard re-measured and pinned) restated the error-handling record at HEAD: `poll` holds `inner.borrow_mut()` across `vdom.render_immediate`, dioxus-core 0.7.10 polls the task there (`handle_task_wakeup`), `set_focus` borrows through `doc_mut` and panics; measured in both layout modes, pinned by `dioxus_runtime_probe`'s two `should_panic` tests, the fix routed to «MountedData buffered render» on the founder's word (port upstream's buffered render onto the 0.7 bridge to invert the pin). The chunk inverts exactly these two pins and must rewrite this record; the windowed path stays read, not run; `get_scroll_offset` / `get_scroll_size` take a shared borrow through `doc`.

@history_item n=3 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the mounted event of a removed element) added the §Error Handling row: `flush_queued_mounted_events` skips an element removed since its event was queued, through `try_element_to_node_id`, an engine behaviour for every document; upstream's `element_removed_before_its_mounted_event_is_flushed` left out (reads 1 on Dioxus 0.7, asserts 0) and "returns with `286874cc1`". The buffered render must keep the flush after the replay and this skip; whether the left-out test now passes under the port is a question this chunk meets.

@history_item n=4 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — (the 28-name falsy predicate) retired the bridge's `BOOLEAN_ATTRIBUTES` for upstream's private 28-name `is_bool_attr`, leaving `mutation_writer.rs` byte-identical to upstream's at `434e0661`, an in-process mutation path with no new input class. The chunk's boundary keeps that file byte-identical; the replay must reach the falsy clear on both attribute paths unchanged, and any edit there is named with its reason.

@history_item n=5 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — added the Dioxus-mutations row (falsy boolean attributes removed, not written, on dynamic and static template paths) and the password `MASKED_VALUE` row, and re-pointed the `dangerous_inner_html` → `set_inner_html` citation. These are the §Input Validation Dioxus-mutation checks and the mask the replayed buffer must pass through in the same order; trap recorded there: the mask covers the snapshot only, the buffer holds the raw value.

@history_item n=6 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — extended the mask to an `input` of type `file` (`MASKED_VALUE` non-empty, `""` empty, `None` with no `value`), DOM untouched so the attribute still holds the host path. The buffer will carry that unmasked attribute value; the snapshot and diff checks pinning the mask for both types must still pass unchanged.

@history_item n=7 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — added the stable-element-id reader of the HTML `id`: a non-element, stale or detached node reads `None`, no path panics, computed on demand. The chunk's "element ids unchanged" boundary rests on this reader yielding the same ids from the replayed document.

@history_item n=8 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — the id persists across re-render, remount and a second process; a dropped pre-remount `NodeId` reads `None` with no panic. The buffered re-render must keep this persistence and stale-id degradation (witnessed alongside `stale_node_mapping`).

@history_item n=9 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — the `id` row became "computed on demand and written to no log, DOM or vdom", with the standing rule that any further consumer of the id beyond its ratified exits re-opens the row. The buffer holds ids in process only; it must not become a new consumer that writes or exposes them.

@history_item n=10 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — (the engine's password mask) recorded the paint mask as "an engine change for every document", the founder's fork, the class the playbook's engine-change rule then began to cover. Precedent for this chunk's "engine change for every Dioxus document, ratified" record with its witness.

@history_item n=11 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — set the widening rule on the socket row: a new field of a closed value set inside a ratified carrier is no widening, while authored text, an open value set, a new carrier or a new exit still is. The chunk relies on this rule to record itself as crossing nothing.

@history_item n=12 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — (the session socket carries calls and answers) restated the trust boundary's carried set: ids and typed text in; ids, names, control values (password or file masked), snapshot text and diff out, no `NodeId`. The buffered render must leave this ratified set holding exactly, no mutation content reaching an answer.

@history_item n=13 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — (the MCP server's input and its crossing) ratified the MCP crossing and its result's carried set (ids, names, values masked, snapshot text). After the port every Dioxus document renders through the buffer; MCP results must keep carrying only this set, no new result field.

@history_item n=14 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — reconciled the carried sets on socket, MCP, CLI stdout and kept checks to name a lint's findings and the diff. These are the carried-set sentences (`boundary-session-socket`, `boundary-mcp-stdio`) the cascade quotes; they must read unchanged after the chunk.

@history_item n=15 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — wired escher's sink: engine targets (`blitz`, `dioxus_native`, …) print only the safe fields, content-named fields (`text`, `value`, `attrs`, `html`, …) redacted at any target; the std panic hook's raw message stays unscrubbed. Any log site the buffer adds must stay inside this allowlist, and a panic message on the replay path must hold no buffered content.

@history_item n=16 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — (what a sink-installing host logs) measured that at `trace` a name printed through `dioxus_core::diff::node`, with the trap that a "nothing in the logs" check over a process that installs no sink passes vacuously. The buffer sits on dioxus-core's diff output path; any no-leak claim must be measured on a sink-installing host.

@history_item n=17 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — made the sink drop any outside-target record whole at every level, measured 0 id and name needles by level on host and windowed. The buffer module must add no log site that evades or relies only on this scrub; `host_log` is the standing check.

@history_item n=18 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — (the sink's second record class) measured typed text in process: `stand_act_spans`' sink capture holds 0 supplied texts, 0 of 36 id needles, 0 of 8 name needles, both layout modes. `stand_act_spans` is among the `stand_*` witnesses this chunk re-runs to show no mutation content reaches a log.

@history_item n=19 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — committed `counter-cli.json` and `counter-mcp.json` read by `stand_replay`, each kept step re-run through `Session::run` (passing `validate` again) and compared byte for byte. The buffered render must leave every kept answer equal, `stand_replay` one of the chunk's unchanged-behaviour witnesses.

@history_item n=20 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — (eight advisory ignores; the 2026-10-17 sync record) recorded keyboard-types 0.8 taken with Dioxus keeping its own 0.7, two versions in the graph. Upstream `286874cc1`'s keyboard-types 0.8 move and `keyboard_compat.rs` removal must stay unported so this graph stands.

@history_item n=21 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — (the per-sync lock record of the ten-commit take) recorded Dioxus 0.8.0-alpha.3 (`286874cc1`) not taken, the pins staying. This chunk ports only the 0.7-compatible shape of that same commit; the Dioxus "0.7.3" pin and the not-taken record must stand, no package entering `Cargo.lock`.

@history_item n=22 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (this take's lock record; no served port) restated the pins (Stylo "0.22.0", Taffy "0.15", Dioxus "0.7.3", winit `=0.31.0-beta.3`, no git dependency) and the API Security opening row: no served TCP or UDP port, the session socket the one listener. The in-process buffer must leave both as stated.

@history_item n=23 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — set the audit command `cargo deny --locked --workspace --all-features check advisories licenses` over the whole workspace graph, the licence gate on. This is the command the chunk's acceptance re-runs to show the graph unchanged.

@history_item n=24 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (advisories published after a green audit; the zip ignores gain RUSTSEC-2026-0429) recorded that CI's audit can go red over an unchanged lock on newly published advisories, with the standing rule to run `bash .github/scripts/ci-leg.sh audit` locally before a push; nine ignores now stand. The chunk's "audit stays green" acceptance must be read under that external-decay rule.
