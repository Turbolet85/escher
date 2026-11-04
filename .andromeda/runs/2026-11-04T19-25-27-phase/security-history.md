@history_item n=1 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the `MountedData` panic from a spawned task) — added the error-handling record this chunk re-verifies. A task spawned from a key handler that writes a signal is first polled inside `render_immediate`, which holds the document's `RefCell`, so a `MountedData` call in it panics `RefCell already borrowed`, any app able to meet it. It was measured headlessly through the harness's `poll(None)`; the windowed shell runs the same lines, read and not run. Cells calls its handles synchronously (`drop(handle.scroll_to(…))`, `drop(handle.set_focus(true))`). It was recorded because the planned async form panicked and was not used, and no fix was made. This is the mechanism the founder's reading must restate from a run at HEAD, with its fix left as his fork.

@history_item n=2 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (Cells' `cells-grid` keys) — added the Example apps row for the stand's key handler that the hazard's reproduction drives. `cells-grid` takes `tabindex` 0 and an `onkeydown` with a closed key match: arrows move `current`, clamped; Enter opens the editor focused; the editor's Enter commits, stops propagation and returns focus. A click inside a cell's open editor returns early. This was the founder's F13, with the early return on the operator's P2 answer. Probe code on the stand must leave this ratified behaviour holding.

@history_item n=3 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the citations the chunk's edits moved) — re-anchored citations after edits that included `dioxus-native-dom`. It is the origin of the `events.rs` coordinates the hazard record cites, so the chunk must re-check `events.rs:162` at HEAD, not trust the number.

@history_item n=4 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (citations re-pointed to the chunk's tree) — changed 61 lines in their citation digits alone, error-handling.md 2 of them, after the twelve-commit take and the chunk's edits. This is the sync after which the scope asks whether `events.rs:162` and the `dioxus_document.rs` `poll` coordinates still hold: the digits may have moved while the claims did not.

@history_item n=5 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the mounted event of a removed element) — added an error-handling row: `flush_queued_mounted_events` skips an element removed since its event was queued, through `try_element_to_node_id`, an engine behaviour for every document. The upstream test `element_removed_before_its_mounted_event_is_flushed` is left out: it reads 1 mounted event on Dioxus 0.7 where it asserts 0, and it returns with `286874cc1` (Dioxus 0.8). This is the closest prior change to the mounted-event path. It is a precedent for the option-returning bridge read, and a trap: upstream behaviour that needs Dioxus 0.8 is not reachable at the 0.7.3 pin.

@history_item n=6 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the key-press panic retired) — fixed a panic that a remote page could reach, which had been recorded and carried and not fixed (2026-10-31-select-popup). The fix is escher's own, on the founder's answer to fork F1 option A: a direct engine change for every document, ratified by his word. Afterwards the operator's reading of the fork was marked as not the founder's own word. This is the precedent for how the MountedData fix must travel: a measured panic carried to the founder as a fork, then fixed only on his answer.

@history_item n=7 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (this take's lock record) — recorded that `Cargo.toml`, `Cargo.lock`, `deny.toml` and every crate manifest are byte-identical. The pins stay: Stylo "0.22.0", Taffy registry "0.15", Dioxus "0.7.3", winit `=0.31.0-beta.3`, no git dependency. The audit reads `advisories ok`, `licenses ok`. This is the baseline the chunk's acceptance (lock and Dioxus lines unchanged, audit green) is measured against.

@history_item n=8 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (no served port, two test-only TCP binds) — changed the API Security opening from "no TCP or UDP port" to "no served TCP or UDP port". Two `blitz-net` `#[cfg(test)]` loopback binds are registered as test-only, under session rule 2026-10-15. The driver session socket stays the one listener and `mcp` binds nothing. Any listener a probe test opened would owe the same registration; the scope forbids one.

@history_item n=9 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (the per-sync lock record of the ten-commit take) — recorded that git Stylo 0.23-pre, the six Taffy-main commits and Dioxus 0.8.0-alpha.3 are not taken, with the pins staying. This is the source of the cascade's "Dioxus 0.8.0-alpha.3 stays not taken": a link readable only on 0.8 gets the verdict "not exposed without a fork-level change".

@history_item n=10 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (the 28-name falsy predicate; no git dependency) — left no git dependency in the root manifest (taffy moved to crates.io). This is the founder's rule of 2026-10-13 that later lock records cite. A vendored or git Dioxus copy for the probe would break it.

@history_item n=11 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — established cargo-deny 0.20.2 running `cargo deny --locked check advisories` as ci.yml's `audit` job, with per-ID ignores only with a written reason. This is the audit leg (`bash .github/scripts/ci-leg.sh audit`) the chunk's acceptance requires green.

@history_item n=12 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — widened the audit to the whole workspace (`--workspace --all-features check advisories licenses`) and added a licence gate. It also recorded `TMPDIR` as read by escher-driver's client unit test, while the library reads nothing beyond `RUST_LOG`. A new test file under `tests/blitz-tests` comes under that whole-workspace audit, and any env read it makes must be one already registered.

@history_item n=13 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (`TMPDIR` read at a second test-only site) — registered a second test-only `std::env::temp_dir()` read, because the environment-values list names every read. This is the precedent: a probe test's directory must come from an already registered test-time source, never a new env read.

@history_item n=14 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the engine's password mask, nine causes, no image in a log, `png`) — made the paint-time password mask the founder's fork 2. As "an engine change for every document" it falls under the playbook's engine-change rule, which this entry introduced. This is the rule behind the scope's requirement that every MountedData fix option be labelled an engine change for every document awaiting his ratification.

@history_item n=15 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (three state readings inside the ratified carriers; the closed-field rule) — set the socket row's widening rule. A new field of a closed value set inside an already ratified carrier is not a widening; a field holding authored text, an open value set, a new carrier or a new exit still is. A component name, a handler or a signal value is authored or open text, so letting it reach a carrier would be a widening. The probe must add none.

@history_item n=16 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the session socket carries calls and answers) — widened the socket, ratified, to carry calls in and answers out: ids, names, masked values, snapshot text and diffs, and no `NodeId`. The boundary stays the owning user by file mode alone (`0700`/`0600`), with no handshake. This is the ratified crossing the probe must leave exactly as it is.

@history_item n=17 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (ids, names and values leave in an answer; a refusal has a written form) — gave the id its exits in driver answers. `Refusal` and `SessionError` are written from fixed strings only. The rule kept is that no log, event or file carries an id, a name, a value, the snapshot's text or a diff's content. Probe readings fall under the same no-log rule.

@history_item n=18 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (the MCP server's input and its crossing, ratified) — added the local stdio vector, with no listener and no auth, its bounds and its `id` echo, recorded with "the next carrier is a new question". The probe adds no carrier, and nothing it reads may enter an MCP result.

@history_item n=19 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (`serde_json` reviewed; the `tool_call` span) — added a second `escher_driver` span, `tool_call`, recorded as a boundary widening (a new log field) that needed the founder's word. This is the precedent: a span or log field the probe added would be a widening, not routine.

@history_item n=20 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (the driver session socket: threat vector, access, input rows) — added the local IPC vector: owner-only `0700` directory and `0600` socket, no handshake or peer check, label 1 to 32 bytes of `a-z0-9-` checked before boot, `StateDirNotPrivate`. A probe that drives a served task through a session runs inside this boundary, with a fixed literal label.

@history_item n=21 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (what a sink-installing host logs) — scoped the sink's reach by measurement. It also set the trap that a "nothing in the logs" check over a process that installs no sink passes vacuously. Any probe log-absence claim must run against a host that installs the sink.

@history_item n=22 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — set the sink's three outcomes: engine targets print only safe fields, `escher_` targets print with 11 content-named fields redacted, and any other target is dropped whole at every level. It recorded that `host_log` is the check that must run against a sink-installing host. These are the allowlist lists the probe must leave unchanged.

@history_item n=23 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — wired `escher_telemetry::init`'s allowlist scrub. Its reach is that sink only: the std panic hook's raw message and the upstream `fmt::init()` / `env_logger` sinks stay unscrubbed. The re-measured MountedData panic reaches stderr past the scrub, so probe panics and assertions must carry fixed text only.

@history_item n=24 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (what the driver logs: one span, eight fixed-word or count fields) — set the one `command` span with eight fixed fields. Never fielded: any argument, `Refusal`, `Fault`, screen text, label or record of ids. Only a cause's fixed name reaches a log. This is the span census the probe must leave at eight fields.

@history_item n=25 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — ratified the sink's span record class and set a standing rule: a count of a diff's nodes may reach a log, while no node, id, name or value of it may. A field holding any of those is a new crossing question. This bounds probe output to verdicts and counts.

@history_item n=26 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list in the log) — measured page text reaching no log through a gate-style check. A new engine surface holds no tracing token, `packages/escher-driver/src` holds two `info_span!`, and `stand_act_spans` reads eight span fields, with 0 needles at `trace` under host and role. This is the precedent for the chunk's "grep finds no new `tracing`/`log` site" acceptance.

@history_item n=27 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — made `host_log` a standing check at `RUST_LOG=trace` with 0 id, name and sentinel needles under both builds. This is the existing log-needle shape a probe could lean on for its "nothing read is logged" evidence.

@history_item n=28 marker=2026-10-06-stand-test-contract
**2026-10-06-stand-test-contract** — defined the agent-run contract: `scripts/agent-run.sh`'s closed argv (an existing blitz-tests file stem), and events that log harness metadata only. No captured test output or panic text goes into events, and `run.log` is never printed. Probe runs through the harness inherit this, and a new probe test file becomes a selectable stem.

@history_item n=29 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — added the stable-element-id reader. A non-element, stale or detached node reads `None`, no path panics, and the id is computed on demand and written nowhere. Link (1) (node to component) must read through this option-returning shape and never log what it reads.

@history_item n=30 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — sent the id to the platform adapter, ratified, and set a standing rule that any further consumer of the id beyond the platform accessibility API re-opens the `id` row. A probe that reads ids as a new consumer must be recorded against that row, even in process.

@history_item n=31 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — recorded the snapshot as an in-process reader of ids and names: no crossing, recorded where the row lists who reads it, not a widening. It set a trap that the first wire form is a crossing. This is the precedent for a probe that reads in process: a new reader is recorded, its first wire form is escalated.

@history_item n=32 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — set the anchored, component and document path tiers, with the anchored tier ratified by the founder. It added `unkeyed_actionable` as a reader whose entry holds a process-local `NodeId` in `node`, with the trap that a wire form must leave `node` behind. Probe code mapping nodes to components or handlers likewise keeps `NodeId`s inside the test process.

@history_item n=33 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the in-process lint as an id reader, and the text extent reader) — named `Session::lint` as an in-process id reader that crosses nothing: no verb, field, span or log site. It was marked PROVISIONAL until the founder's word, and its verb was later a separate ratified widening. This is the precedent: an in-process probe reader is recorded, and turning it into a carrier is its own fork.

@history_item n=34 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`type` on a `multiple` select, and the public select readers) — recorded new public readers in `dioxus_native_dom` and `BaseDocument`, each taking a node through `get_node` so a stale id reads none, with their callers named and no type, verb or field added. This is the precedent for any "exposed with a cost" bridge reader the probe's verdicts name inside `dioxus-native-dom`.

@history_item n=35 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select readers and the list's public surface) — extended the `dioxus_native_dom` reader set (`is_select`, `closed_select_of`) with each caller named, and recorded a page-text-holding read type as never logged. This is the precedent for recording a new bridge-held reader and its callers.

@history_item n=36 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — made a password's snapshot value read as `MASKED_VALUE`: never the text, never its length. This is the mask the probe's readings of control values and their state must keep.

@history_item n=37 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — extended the mask to file inputs (no host path), while the DOM and form submission keep the value. A probe that reads state behind a file input from the DOM would meet the unmasked path, so it must keep that path out of every reading and evidence.

@history_item n=38 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — made `Session::run` validate first: nothing public runs a `Command` that did not come from `validate`, and no panic path on what a call supplies. Any probe call through a session must go through `validate` inside `Session::run`.

@history_item n=39 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — closed the cause set at eight (`time-unavailable` retired), ratified 2026-10-18. Settle waits on no clock and moves none. A served task driven through the session stays inside the closed eight causes, and the probe adds none.

@history_item n=40 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (wire v3, fifteen session errors) — set `SESSION_ERRORS` at 15, with wire `v3`. This is the closed session-error table the probe must not extend.

@history_item n=41 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — made a harness settle report a load in flight at once (`NotSettled`/`Loads`), never waiting on one, bounded at 64 passes. Its rule is that a settle verb or busy reply on the wire is a new crossing question. The probe's headless measurements settle through this.

@history_item n=42 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — made the driver's acting step wait on a load for at most `LOAD_WAIT` (5 s), returning `settled: false` with a class only, with no argument or result field of a wait. This bounds any acting step a probe's served-task run takes.

@history_item n=43 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (six served slugs, the load page's in-process stand-in) — added six served slugs in a closed `TASKS` table. The stand boots offline but for the load page, answered in process by `LoadNet`. It also recorded that the flight booker's `select#flight-type` writes its `flight_type` signal by a closed match. These are the served tasks a probe can use beside the lean ones. That signal write is a handler-to-state case (link 3) whose ratified behaviour must keep holding.
