@amendment marker=2026-11-04-dioxus-runtime-probe
title: the advisory fix, pulldown-cmark 0.13.6 and the deny.toml ignore
section: §Infrastructure Patterns (body: the Build system item); §Stack and Technologies (stack-and-technologies: the Markdown app (rdme) row)
change:
  - The rdme row reads pulldown-cmark resolved 0.13.6, past RUSTSEC-2026-0414 and RUSTSEC-2026-0433 (was resolved 0.13.5, past RUSTSEC-2026-0414 alone).
  - The Build system item's citation of the root `deny.toml`, the cargo-deny advisory gate's configuration, spans the file's one added line; the gate's text is unchanged.
why: the report's Dependencies and Schema / config facts: pulldown-cmark moved 0.13.5 to 0.13.6 in `Cargo.lock` alone, a semver-compatible update reached only through `rdme`, fixing RUSTSEC-2026-0433, and `deny.toml`'s ignore list gained RUSTSEC-2026-0429 (zip 2.4.2, `extract` never called), after the Dependency audit read red on those two advisories; both edits on the operator's fix directive in the session, classed widening by the operator at the wrap's scope read.
kept: `rdme` never sets `ENABLE_WIKILINKS`, so no reader is exposed; the rest of the row stands.
ref: .andromeda/runs/2026-11-04T21-47-15-wrap/
@amendment marker=2026-11-04-dioxus-runtime-probe
title: the MountedData hazard re-measured, its fix routed to a port
section: §Established Decisions (established-decisions: the task-from-a-key-handler hazard row); §Project Intent (project-intent: the upstream sync's left-out commits)
change:
  - The hazard row names its mechanism: `DioxusDocument::poll` holds the document's `RefCell` across `render_immediate`, where dioxus-core 0.7.10 first polls a queued task through `handle_task_wakeup`, so such a task that writes a signal and calls `set_focus` borrows through `doc_mut` and panics (`RefCell already borrowed`); was: the task first polled inside `render_immediate` and a `MountedData` call in it panicking.
  - Its measurement: now headlessly through the harness's pump in both layout modes, pinned by `dioxus_runtime_probe` (the pins stated in test-plan §6 E2E Test Strategy), as measured at this chunk's report; was through the harness's `poll(None)`, as measured at 2026-10-31-engine-debts.
  - The row gains its fix: a port of upstream `286874cc1`'s buffered render onto the Dioxus 0.7 bridge, owned by the route entry «MountedData buffered render».
  - Project Intent's `286874cc1`, among the left-out commits, gains one clause: its buffered render, the hazard's fix, is to be ported onto the Dioxus 0.7 bridge by that route entry, the 0.8 move itself still waiting.
  - The left-out commits' owner: now a `residual` row in the store (target: next); was a `.andromeda/residuals.md` line.
why: the report's MountedData record, re-measured at HEAD by the probe's two `should_panic` pins, precondition first; the fix is not made here but routed to a chunk of its own on the founder's ruling, by dialog in the overseer session, relayed by the operator; the route entry is the P5 card's row the operator approved by his own word. The residuals live as `residual` rows in the store since the bases door, and no `.andromeda/residuals.md` stands.
kept: no engine change in this chunk; Cells still calls its handles synchronously; the windowed shell's same lines read and not run; Stylo "0.22.0", Taffy "0.15" and Dioxus "0.7.3" stand, with no git dependency.
ref: .andromeda/runs/2026-11-04T21-47-15-wrap/
@amendment marker=2026-11-04-dioxus-runtime-probe
title: a node's component and a held signal's readers through Dioxus public API
section: §Established Decisions (established-decisions: two new rows after the MountedData hazard row)
change:
  - New: a node's owning component reads through Dioxus 0.7.10's public API with no fork-level change, a walk from `VirtualDom::base_scope` through `ScopeState::try_root_node`, `VNode.template.roots`, `VNode.dynamic_nodes`, `VNode::mounted_root` and `VComponent::mounted_scope`, each component named by the field `VComponent.name`; it names a component for every task-body element on Counter and CRUD, and a deleted CRUD row resolves to none, in both layout modes.
  - New: a `GlobalSignal`'s `Readable::subscribers()`, visited inside `VirtualDom::in_runtime`, give through `ReactiveContext::origin_scope` exactly its reader components' scopes, and a writer's click changes exactly the readers' nodes in the snapshot diff, in both layout modes; for hook-local `use_signal` state neither its readers nor the state a handler writes is reachable without a fork-level change, dioxus-core's `Scope.hooks` being `pub(crate)`, read from the code, not run.
why: the report's Outcome, link 1 measured on Counter and CRUD and links 3 and 4 measured where exposed, the check reading public API alone with no symbol added to any crate.
ref: .andromeda/runs/2026-11-04T21-47-15-wrap/
@amendment marker=2026-11-04-dioxus-runtime-probe
title: the dioxus_runtime_probe target
section: §Existing Scopes (existing-scopes: a new blitz-tests item)
change:
  - New: blitz-tests' Dioxus runtime probe, one file and one target, `dioxus_runtime_probe`, 10 tests (8 looping both layout modes, the MountedData hazard pinned as two `should_panic` tests, one per mode) over the stand's Counter, CRUD and Cells and its own pages `toggled_listener`, `shared_state` and `hazard`, reading Dioxus 0.7.10's public API alone: a node's owning component, its listeners and a held signal's readers.
  - It holds no print, env read, subscriber or listener and has no `stand_` prefix, so it is in neither `run stand`, `stand-repeat` nor the `a11y` leg, run by the workspace `test` leg and `run all`.
why: the report's Changes: one blitz-tests test target added, auto-discovered with no manifest line and no `stand_` prefix; its Counts: 10 tests, the hazard pin one per mode; the silence census over the new file reads 0.
ref: .andromeda/runs/2026-11-04T21-47-15-wrap/
@amendment marker=2026-11-04-dioxus-runtime-probe
title: the bridge's listener attributes over-report
section: §Standard Contracts (standard-contracts: the unkeyed-actionable check's listener clause; two new rows on the Dioxus DOM bridge's listeners)
change:
  - New: an element's listeners read two ways: the vdom walk's dynamic attributes (`VNode.dynamic_attrs`, `VNode::mounted_dynamic_attribute`) read its current listeners exactly, `mounted` included, while the bridge's listener attributes (an attribute named the event, valued `"<rust func>"`, beside the `data-dioxus-id` mark, written by `create_event_listener`) over-report: a listener a re-render removes leaves its attribute and the mark standing while a click no longer acts, `remove_event_listener` lowering the per-kind count alone, and a `mounted` listener writes none, in both layout modes.
  - New: dioxus-core 0.7.10 reaches `remove_event_listener` only when a listener attribute leaves an element's attribute list (a spread); a listener whose value turns to `AttributeValue::None` under the same name is written through `set_attribute` as an `on{event}` attribute, so the bridge's event attribute stays either way (read in the registry copy); `create_event_listener` and `remove_event_listener` have no workspace caller, dioxus-core calling them.
  - The unkeyed-actionable check's `data-dioxus-id` clause gains its limit: the mark stays, with its event attribute, on an element whose listener a re-render removed, so the check over-includes such an element by its listener, the conservative side (read, not run).
why: the report's measured facts: the plan's prediction that the attribute stays after the listener is removed measured true, the vdom walk's exact reading found beside it, and the toggled page's spread the one form that reaches `remove_event_listener`; the check's over-inclusion is the report's stated consequence, read and not run.
kept: the check's id and role sources, its fields and its founder-ratified surface stand; only its listener reader's limit is stated.
ref: .andromeda/runs/2026-11-04T21-47-15-wrap/
