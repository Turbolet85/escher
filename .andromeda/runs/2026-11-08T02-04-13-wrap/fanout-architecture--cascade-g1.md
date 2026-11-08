@proposal n=1 detector=D-cascade severity=warning file=established-decisions anchor=fct-nl4nv5ny basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-zklbw7ne
section: ## Established Decisions
change: Add the snapshot's `NodeState.clickable` to the readers of `ElementData::is_disabled`. It is true where the vdom walk's current listeners hold `click` and the element is not a form control that `is_disabled` reports, so a `disabled` form control with a click listener reads not clickable, while a `<div disabled>` with one reads clickable.
sidecar: DOM semantics: the `clickable` snapshot reading joins the readers of the one `disabled` reading (2026-11-07-ui-graph)
rationale: The report's Symbols / APIs (the snapshot) say `clickable` is true where the walk's current listeners hold `click` and the element is not a form control that `ElementData::is_disabled` reports.
@proposal n=2 detector=D-cascade severity=warning file=established-decisions anchor=fct-qcfqswsq basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-zklbw7ne
section: ## Established Decisions
change: Add a Dioxus-integration row. The bridge's `DioxusEventHandler::handle_event` sets the `SignalRegistry` dispatch (chain head first, and its event name) around `runtime.handle_event` and clears it after, but only when `runtime.has_context::<SignalRegistry>(ScopeId::ROOT)` holds. This changes the event path of every Dioxus document but does nothing without a registry: a page that registers no state reaches no registry and dispatches as before. The buffered render, `initial_build`, `poll` and `flush_queued_mounted_events` are untouched, and dioxus-core, the manifests and `mutation_writer.rs` are unchanged.
sidecar: Dioxus integration: the bridge's dispatch now records against a registered state's registry, and does nothing without one (2026-11-07-ui-graph)
rationale: The report's Symbols / APIs (the bridge dispatch) call this "a change on every Dioxus document's event path, inert without a registry". The arch outcome (fct-dvu6no2e) confirms that every manifest is unchanged and that mutation_writer.rs is byte-equal.
@proposal n=3 detector=D-cascade severity=warning file=established-decisions anchor=fct-b7w6xfsd basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-zklbw7ne
section: ## Established Decisions
change: Narrow the row's last clause. Hook-local `use_signal` state that is not registered stays unreachable without a fork-level change, because dioxus-core's `Scope.hooks` is `pub(crate)`. A state registered through `use_graph_state(name, writers, subscribers)` is reachable with no fork-level change and only dioxus-core's public types. Its readers are read from its `Subscribers`. Its writes are recorded by a `ReactiveContext::new_with_callback` subscribed to the signal, against the standing dispatch and the writing scope. `DioxusDocument::graph` reads both.
sidecar: Held-signal readers: registered hook-local state is now reachable through public API, and unregistered state still is not (2026-11-07-ui-graph)
rationale: The report's Symbols / APIs (the registry and the graph reader) show `use_graph_state` taking only dioxus-core types, and the Dependencies record no crate or manifest change. This contradicts the row's claim that hook-local state is "not reachable without a fork-level change" for any state the app registers.
@proposal n=4 detector=D-cascade severity=escalate file=established-decisions anchor=fct-kukffiv3 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-qxguy4c4
section: ## Established Decisions
change: Add to the Driver session row's widenings: the `graph` verb's answer crosses the ratified carriers (the socket, the command line, MCP and a kept check). That answer holds stable ids, component names, registered state names, handler event names, the closed flag words `undeclared`/`unconfirmed` and booleans, and never a value or a `NodeId`. The `graph` verb takes no argument and refuses one as `malformed` with nothing run. Each snapshot node also carries a new closed-bool node key, `clickable`. There is no new carrier, exit, cause, session error, span field or dependency. This is a boundary widening ratified by the founder: «Новый глагол graph (Рекомендую)» at P4 and «Ещё и статическая подсказка» at P5, by dialog, relayed (inputs#I4, inputs#I5, inputs#I9).
sidecar: Driver session: the `graph` verb's names and the `clickable` key cross the ratified carriers, a ratified boundary widening (2026-11-07-ui-graph)
rationale: The report's Coverage of new surfaces (the `graph` verb, names only, sentinel 0) and the Expected amendments ("arch §Established Decisions → Driver session ... A boundary widening, ratified") apply here. Addendum 1 (inputs#I9) halts this widening on one Validate card with one confirm before any apply, hence escalate.
@proposal n=5 detector=D-cascade severity=escalate file=body anchor=fct-nb3g6bqn dependent_of=D-cascade basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-qxguy4c4
section: ## Established Decisions
change: In the broad line's list of what crosses, add a UI graph's names: "ids, names, masked values, snapshot text, diffs, shots, lint findings and a UI graph's names and closed flag words cross, never a `NodeId`".
sidecar: Body Driver session line: a UI graph's names join what crosses (2026-11-07-ui-graph)
rationale: This is the body occurrence of the Driver-session widening proposed for established-decisions#30.1, and it follows that proposal's ratification card.
@proposal n=6 detector=D-cascade severity=escalate file=established-decisions anchor=fct-7p2g4lnp basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-zklbw7ne
section: ## Established Decisions
change: Add a `graph` answer to what a kept check holds at rest. It holds the screens' ids, names, values, snapshot text, diffs, a lint's findings and a UI graph's component, state and handler-event names with its closed flag words. The recording is unchanged code: it routes `graph` through `VERBS` and `Outcome::to_json` like every verb.
sidecar: Kept checks: a `graph` answer's names rest in a kept check (2026-11-07-ui-graph)
rationale: The report's Symbols / APIs (escher-driver) say the recording is unchanged code and that `escher-session` answers `graph` "in a kept check". The same ratified widening covers kept checks (Expected amendments, inputs#I4, inputs#I9).
@proposal n=7 detector=D-cascade severity=warning file=conventions anchor=fct-bhxvo7z2 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-z2vdklsi
section: ## Conventions
change: Add to Feature gating: dioxus-native-dom's `graph` module (`DioxusDocument::graph`, `UiGraph`) is gated by `accessibility`, beside the snapshot model it reads. Its `signal_registry` module (`use_graph_state`, `DeclaredWrite`, `SignalRegistry`) is not gated, so an app registers state whatever its features.
sidecar: Feature gating: the `graph` module is behind `accessibility`, and `signal_registry` is ungated (2026-11-07-ui-graph)
rationale: The report's Crates / modules say: "dioxus-native-dom gains `graph` (gated `accessibility`) and `signal_registry` (not gated)".
@proposal n=8 detector=D-cascade severity=warning file=conventions anchor=fct-zd5gb466 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-fvf7qs7b
section: ## Conventions
change: In the list of checks that hold a `Session` and share through `tests/blitz-tests/tests/session_common/mod.rs`, add `stand_graph` (since 2026-11-07-ui-graph). It also reads the `GRAPH_DRIVES` drive table from `tests/blitz-tests/tests/common/mod.rs`.
sidecar: Tests: `stand_graph` joins the Session-holding checks that read `session_common` (2026-11-07-ui-graph)
rationale: The report's Changes list `stand_graph.rs` as new and its New text shows `use session_common::{` at 26-28. The decision fact fct-ej52fywd places `GRAPH_DRIVES` in common/mod.rs.
@proposal n=9 detector=D-new-detail severity=warning file=conventions anchor=fct-2wv4ux4q fact=F10 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-firm5m3h
section: ## Conventions
change: Add to Apps and examples: seven_guis' tasks register each signal through `tasks::use_graph_signal(name, writers, init)`. Its `&'static [DeclaredWrite]` slice, built from `tasks::writes` const fn calls, is written `const { &[...] }` (or as a `const` item). A bare `&[writes(..)]` is a temporary that is not promoted to `'static` and errors E0716.
sidecar: Apps and examples: the `const { &[...] }` form for a slice of declared writes, and the tasks' state registered through `use_graph_signal` (2026-11-07-ui-graph)
rationale: F10 and the report's Rust gotcha (five `E0716` on `cells.rs`). The seven_guis Symbols / APIs say every task's signals register with declarations written `const { &[…] }`.
@remark
F9 (slug cross-cutting-patterns) fits beside established-decisions#25.3 but is outside group 1's categories; F1-F8 are standard-contracts; the graph verb (D-arch-resources) needs registering under §Standard Contracts by its own group.
