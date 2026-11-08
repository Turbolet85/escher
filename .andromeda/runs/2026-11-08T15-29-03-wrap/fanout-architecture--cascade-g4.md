@proposal n=1 detector=D-cascade severity=warning file=standard-contracts anchor=fct-6m5fkd33 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-tj4uicby
section: ## Standard Contracts
change: in the lint verb's clause, replace «and `VERBS` holds thirteen, `graph` listed after `lint`» with «and `VERBS` holds fourteen, `graph` listed after `lint` and `actions` after `graph`»; the rest of the entity is unchanged
sidecar: standard-contracts: the lint entity's VERBS count goes from thirteen to fourteen, with actions after graph (2026-11-08-what-can-i-do-here)
rationale: the report's Counts row moves VERBS 13 → 14, and Schema / config makes actions the tenth instance verb, after graph
@proposal n=2 detector=D-cascade severity=warning file=standard-contracts anchor=fct-g75i5t3p basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-yftgk4an
section: ## Standard Contracts
change: replace «and `snapshot`, `screenshot`, `lint` and `graph`, which run no step, wait on nothing» with «and `snapshot`, `screenshot`, `lint`, `graph` and `actions`, which run no step, wait on nothing»
sidecar: standard-contracts: the load wait's list of no-step verbs gains actions (2026-11-08-what-can-i-do-here)
rationale: Symbols / APIs says record_result lists Outcome::Actions among the no-step outcomes, and the arch outcome says actions.rs makes no pump, tick or dispatch call
@proposal n=3 detector=D-new-detail severity=warning file=standard-contracts anchor=fct-mi62uuno fact=F1 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-yftgk4an
section: ## Standard Contracts
change: add after the graph verb entity: «The actions verb: `actions`, the tenth instance verb, listed after `graph`, takes no argument (an argument is refused `malformed`) and answers `Outcome::Actions { nodes: Vec<ActionNode> }` through `Session::actions(&mut self) -> Vec<ActionNode>`. `ActionNode { id, owner, keyed, verbs: Vec<VerbReading> }` and `VerbReading { verb, cause: Option<Cause> }` are both re-exported. `Session::actions` reads one snapshot and one `element_ids` map per call and records every id it reads. For each listed node and each `Level::Instance` verb with an `ArgKind::Id` argument, in table order (today `click` · `type` · `scroll` · `screenshot`; no verb name is written in the code), it builds the call `call_naming` spells, runs it through `validate` and then `check_target`, and reads `stale` for a node that resolves to no element. Its written form, by hand in the crate-private `json` module, is `{"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}`, with `owner` and `cause` left out when absent and a cause written by `Cause::name`.»
sidecar: standard-contracts: new entity for the actions verb and Session::actions (2026-11-08-what-can-i-do-here)
rationale: F1, with the report's Symbols / APIs (ActionNode, VerbReading, Session::actions, target_verbs, call_naming, Command::Actions, Outcome::Actions, json action_node)
@proposal n=4 detector=D-new-detail severity=warning file=standard-contracts anchor=fct-mi62uuno fact=F2 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-yftgk4an
section: ## Standard Contracts
change: add after the actions verb entity: «The per-verb target check: the crate-private `check_target(harness, &Command, &Target) -> Result<Cleared, Refusal>` in execute.rs is the one per-verb target check. `click` and `type` meet `action_point` (disabled, then off-screen, then covered). `scroll` meets none. An id-taking `screenshot` meets `in_view`, then an empty `crop`, then `can_paint_text` false (`fonts-unavailable`), before painting. Its only callers are the four id-taking arms of `Session::execute`, none of which keeps a check of its own, and `Session::actions`. `Target` is crate-private, and `locate` ends in `target_of`, which `Session::actions` also calls.»
sidecar: standard-contracts: new entity for check_target, the shared per-verb target check (2026-11-08-what-can-i-do-here)
rationale: F2, with the report's Symbols / APIs on check_target, Cleared and target_of, and the Deviations entry fct-kwnslo3j on the can_paint_text order
@proposal n=5 detector=D-new-detail severity=warning file=standard-contracts anchor=fct-mi62uuno fact=F3 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-ai45gnz6
section: ## Standard Contracts
change: add after the per-verb target check entity: «The actions read changes nothing. After it, `has_changes` is false, and the snapshot, both focus readings and the diff are unchanged. An open select list stays open. actions.rs makes no `pump`, `tick`, dispatch, `scroll_into_view`, `close_select_list` or `focus_select` call; `stand_actions` holds this in both layout modes.»
sidecar: standard-contracts: new entity stating that the actions read changes nothing (2026-11-08-what-can-i-do-here)
rationale: F3, and the report's (arch) outcome, the_read_changes_nothing_and_holds_no_typed_text_or_mask
@proposal n=6 detector=D-new-detail severity=warning file=standard-contracts anchor=fct-ahl2enqw fact=F4 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-tj4uicby
section: ## Standard Contracts
change: add after the unpaged graph entity: «The actions answer says whether a call would be accepted, not what it would do (Accepted is not acting). It takes no argument and answers the whole screen at once, unpaged. Cells' answer reads 106,088 bytes for 678 nodes, within the 1,048,576-byte answer bound, which `stand_actions` holds in both layout modes. The unpaged answer is a stated limit beside «An unpaged graph», owned by the same residual `res-7cc5bi4k` (target: next), widened to name the actions answer.»
sidecar: standard-contracts: new entity for the unpaged actions answer and its bound; res-7cc5bi4k widened (2026-11-08-what-can-i-do-here)
rationale: F4, the report's Counts entry for the Cells answer (106,088 bytes, 678 nodes), and the operator's I5 yes widening res-7cc5bi4k
@proposal n=7 detector=D-retired severity=warning file=standard-contracts anchor=fct-kllkxz3d basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-vbjuu4aa
section: ## Standard Contracts
change: in unkeyed_actionable's actionable rule, replace the third leg («or the `data-dioxus-id` attribute the bridge writes for every listener but `onmounted` and that stays, with its event attribute, on an element whose listener a re-render removed, so the check over-includes such an element by its listener, the conservative side (read, not run)») with «or its current listeners, read from `element_facts` (the vdom walk's exact reading), holding an event other than `mounted`. This is the crate-private `Readings`, which `actionable_nodes` also reads, so neither reader counts an element whose listener a re-render removed. The `data-dioxus-id` mark stays, written by the bridge and read by event dispatch»; the `listener` field now reads «the element's current listeners hold an event other than `mounted`»
sidecar: standard-contracts: unkeyed_actionable's listener leg moves from the data-dioxus-id mark to the exact current listeners (2026-11-08-what-can-i-do-here)
rationale: the report's Retired bullet names the mark leg, and Symbols / APIs states that unkeyed_actionable's domain, order, fields and remedy() are unchanged
@proposal n=8 detector=D-cascade severity=warning file=standard-contracts anchor=fct-kllkxz3d basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-otmrcodh
section: ## Standard Contracts
change: replace «the check has no wire form, and no driver, CLI or MCP command exposes it yet» with «the check's first wire form is `ActionableNode`'s `keyed` reading (true where the id holds no `/`), which the `actions` verb carries on every carrier. It is PROVISIONAL until the founder's own word»
sidecar: standard-contracts: the actionable-key check gains its first wire form, keyed on the actions answer, provisional (2026-11-08-what-can-i-do-here)
rationale: the report's Expected amendments (Dioxus DOM bridge) name the keyed reading as the check's first wire form, on the overseer's call (I4), PROVISIONAL for the founder
@proposal n=9 detector=D-cascade severity=warning file=standard-contracts anchor=fct-ie5oemgh basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-yftgk4an
section: ## Standard Contracts
change: replace «the UI graph's handlers and `NodeState.clickable` read these, never the listener mark» with «the UI graph's handlers, `NodeState.clickable` and, through the crate-private `Readings`, `unkeyed_actionable`'s listener leg and `actionable_nodes` read these, never the listener mark»
sidecar: standard-contracts: the exact listener reading gains unkeyed_actionable and actionable_nodes as readers (2026-11-08-what-can-i-do-here)
rationale: the report's Symbols / APIs says both readers read Readings, whose listeners come from element_facts; its unkeyed disposition lists fct-ie5oemgh as changed
@proposal n=10 detector=D-new-detail severity=warning file=standard-contracts anchor=fct-ie5oemgh fact=F6 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-yftgk4an
section: ## Standard Contracts
change: add after the listener-reading entity: «The one actionable rule: the crate-private `Readings` in dioxus-native-dom holds an element actionable when `Node::is_focussable`, or one of the 27 interactive roles, or current listeners (from `element_facts`, the exact reading of the vdom walk) hold an event other than `mounted`. `unkeyed_actionable` and `actionable_nodes` both read it and share the private `fn roles`.»
sidecar: standard-contracts: new entity for Readings, the one actionable rule (2026-11-08-what-can-i-do-here)
rationale: F6, and the report's Symbols / APIs on Readings and fn roles
@proposal n=11 detector=D-new-detail severity=warning file=standard-contracts anchor=fct-xabeutlj fact=F5 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-yftgk4an
section: ## Standard Contracts
change: add after the UI graph entity: «The actionable nodes: `DioxusDocument::actionable_nodes(&self) -> Vec<ActionableNode>`, behind `accessibility`, is read-only. It returns `ActionableNode { id: String, owner: Option<String>, keyed: bool }` (`Debug`, `Clone`, `PartialEq`, re-exported beside `UnkeyedActionable`) over the UI graph's nodes in snapshot pre-order. It keeps each node the actionable rule holds for, or whose graph `owner` is `Some`, and `keyed` is true where the id holds no `/`. `keyed` is the first wire form of the actionable-key check, PROVISIONAL until the founder's own word.»
sidecar: standard-contracts: new entity for actionable_nodes and ActionableNode (2026-11-08-what-can-i-do-here)
rationale: F5, and the report's Symbols / APIs on actionable.rs (ActionableNode, actionable_nodes, the lib.rs re-export)
@proposal n=12 detector=D-cascade severity=warning file=standard-contracts anchor=fct-c5e4c7ao basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-tj4uicby
section: ## Standard Contracts
change: replace «the verbs are the thirteen of `VERBS` — `snapshot`, …, `lint`, `graph`, `start <task> [--record <dir>]`» with «the verbs are the fourteen of `VERBS` — `snapshot`, …, `lint`, `graph`, `actions`, `start <task> [--record <dir>]`»; the rest of the entity is unchanged
sidecar: standard-contracts: the command line's verbs go from thirteen to fourteen, adding actions after graph (2026-11-08-what-can-i-do-here)
rationale: the report's Counts row moves VERBS 13 → 14, and cli_commands gains actions_answers_one_line_of_the_actionable_nodes
@proposal n=13 detector=D-cascade severity=warning file=body anchor=fct-ksgwc2yv basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-tj4uicby
section: ## Standard Contracts
change: in the Driver session broad line, replace «thirteen verbs» with «fourteen verbs»
sidecar: architecture body: the Driver session broad line goes from thirteen to fourteen verbs (2026-11-08-what-can-i-do-here)
rationale: the report's Counts row moves «thirteen verbs / tools / driver commands» to fourteen
@remark
F7 (the actions boundary widening on rul-izni57qv, founder-ratified, I4 and I6) belongs to established-decisions, another group; the body lines fct-elnqiowc (Session::lint changes nothing) and fct-srb7slak (bridge readers) could also name Session::actions and actionable_nodes, which is for the new-detail pass
