@proposal n=1 detector=D-arch-resources severity=warning file=standard-contracts anchor=fct-t7zq5feb basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-xxsa6syv
section: ## Standard Contracts
change: the crate re-exports 59 names (57 until 2026-11-08-what-can-i-do-here), adding `ActionNode` and `VerbReading` from the new private module `actions`, the crate's eighteenth private module (packages/escher-driver/src/lib.rs:158).
sidecar: driver session: re-exports 57 → 59 (`ActionNode`, `VerbReading` from the private `actions` module), private modules 17 → 18.
rationale: report Crates / modules and Symbols / APIs: escher-driver gains the private module `actions`, its crate root re-exporting 57 → 59 names (`lib.rs:158`).
@proposal n=2 detector=D-arch-resources severity=warning file=standard-contracts anchor=fct-t7zq5feb basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-yftgk4an
section: ## Standard Contracts
change: `VERBS` lists fourteen verbs — first the ten asked of the held instance — `actions` joining after `graph` since 2026-11-08-what-can-i-do-here (`schema::ACTIONS`, `Level::Instance`, no argument, an argument refused `malformed` with `Fault::Unnamed`; one result field `nodes` of `FieldKind::ActionNodes`; packages/escher-driver/src/schema.rs:429-452), a further widening of the validated verb set (§Established Decisions → [Driver session]); `VERBS` is `[SNAPSHOT, CLICK, TYPE, PRESS, ADVANCE, SCROLL, SCREENSHOT, LINT, GRAPH, ACTIONS, START, STATUS, STOP, HELP]`.
sidecar: driver session: `VERBS` 13 → 14, instance verbs 9 → 10, `actions` the tenth after `graph`.
rationale: report Symbols / APIs (`schema::ACTIONS`, `schema.rs:429-452`) and Schema / config: `VERBS` 14, `actions` the tenth instance verb after `graph`.
@proposal n=3 detector=D-arch-resources severity=warning file=standard-contracts anchor=fct-t7zq5feb basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-wuaab5w2
section: ## Standard Contracts
change: `FieldKind` adds `ActionNodes` (`action-nodes`), the kind of `actions`' `nodes` field — thirteen kinds since 2026-11-08-what-can-i-do-here; `graph`'s and `actions`' fields share the name `nodes` with different kinds.
sidecar: driver session: `FieldKind` 12 → 13 (`ActionNodes`, `action-nodes`).
rationale: report Schema / config: `FieldKind` thirteen (`ActionNodes`, `action-nodes`); Deviations: the two `nodes` fields differ in kind.
@proposal n=4 detector=D-cascade severity=warning file=standard-contracts anchor=fct-t7zq5feb basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-tj4uicby
section: ## Standard Contracts
change: `describe()`'s counts read 14 verbs, 14 node keys, 12 key names, 3 busy classes, 8 causes, 4 exit statuses, 15 session errors.
sidecar: driver session: describe's verb count 13 → 14.
rationale: report Counts / qualifiers moved [describe]: `[13, 14, 12, 3, 8, 4, 15]` → `[14, 14, 12, 3, 8, 4, 15]`.
@proposal n=5 detector=D-arch-resources severity=warning file=standard-contracts anchor=fct-t7zq5feb basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-yftgk4an
section: ## Standard Contracts
change: `Command` adds `Actions` (validated from `("actions", [])`, an argument refused `malformed` unnamed; its `spec()` names `schema::ACTIONS`), after `Graph`.
sidecar: driver session: `Command::Actions` added.
rationale: report Symbols / APIs: `Command::Actions`, validated from `("actions", [])`; `Command` gains one variant [cmd-variants].
@proposal n=6 detector=D-arch-resources severity=warning file=standard-contracts anchor=fct-t7zq5feb basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-yftgk4an
section: ## Standard Contracts
change: `Outcome` adds `Actions { nodes: Vec<ActionNode> }` (packages/escher-driver/src/execute.rs:99-104), its arm calling `Session::actions` and `record_result` listing it among the no-step outcomes, so an actions read dispatches nothing, runs no step and settles nothing; `ActionNode { id, owner: Option<String>, keyed: bool, verbs: Vec<VerbReading> }` and `VerbReading { verb: &'static str, cause: Option<Cause> }`.
sidecar: driver session: `Outcome::Actions` added, with `ActionNode` and `VerbReading`.
rationale: report Symbols / APIs: `Outcome::Actions { nodes: Vec<ActionNode> }` (`execute.rs:99-104`), `ActionNode` (`actions.rs:14-28`), `VerbReading` (`actions.rs:30-37`).
@proposal n=7 detector=D-cascade severity=warning file=standard-contracts anchor=fct-t7zq5feb basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-yftgk4an
section: ## Standard Contracts
change: the record of ids is fed also from the snapshot an `actions` read takes, every id that read lists recorded.
sidecar: driver session: the session's record of ids is fed by the `actions` read too.
rationale: report Symbols / APIs: `Session::actions` reads one snapshot and one `element_ids` map per call and records every id it reads in the session's record of ids.
@proposal n=8 detector=D-cascade severity=warning file=standard-contracts anchor=fct-t7zq5feb basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-wuaab5w2
section: ## Standard Contracts
change: an accepted `actions` records `verb` alone on the `command` span, as an accepted `graph` does, a refused one `verb` and `cause`; never fielded adds the actions answer, its ids and its verb readings.
sidecar: driver session: the `command` span records `verb` alone for `actions`.
rationale: report Schema / config: the `command` span's eight fields unchanged, `verb` alone for `actions`, `cause` when refused; Outcome (obs) `stand_act_spans` reads the `actions` line recording `verb` alone.
@proposal n=9 detector=D-cascade severity=warning file=standard-contracts anchor=fct-t7zq5feb basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-6wtnpthi
section: ## Standard Contracts
change: `tools/list` answers the fourteen verbs of `VERBS` in order (`screenshot` after `scroll`, then `lint`, then `graph`, then `actions`), `help` last.
sidecar: driver session: the MCP listing 13 → 14 tools, `actions` after `graph`.
rationale: report Outcome (tests) counts: `tools/list` fourteen in table order; `mcp_tests`' listing test renamed `the_fourteen_…`.
@proposal n=10 detector=D-cascade severity=warning file=standard-contracts anchor=fct-t7zq5feb basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-7rhac4bh
section: ## Standard Contracts
change: the `tool_call` span's `tool` holds `actions` too, beside `lint` and `graph`, no field added; an `actions` tool call parents one `command`.
sidecar: driver session: `tool_call`'s `tool` set gains `actions`.
rationale: report Outcome (obs): `stand_mcp_tools` reads `tool_call` parenting one `command` for the `actions` tool call.
@proposal n=11 detector=D-cascade severity=warning file=standard-contracts anchor=fct-t7zq5feb basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-yftgk4an
section: ## Standard Contracts
change: an `actions` outcome is written `{"nodes":[{"id":…,"owner":…,"keyed":…,"verbs":[{"verb":…,"cause":…}]}]}` by the crate-private `json::action_node` (packages/escher-driver/src/json.rs:251-272), `owner` and `cause` left out when absent, a cause by `Cause::name`, holding ids, owner ids, the table's verb words, closed cause words and booleans only.
sidecar: driver session: the `actions` answer's written form.
rationale: report Symbols / APIs: `json.rs`'s `action_node` (`json.rs:251-272`); Schema / config: the answer carries ids, owner ids, verb words, cause words and booleans, never a value, typed text or `NodeId`.
@proposal n=12 detector=D-new-detail severity=warning file=standard-contracts anchor=fct-t7zq5feb fact=F1 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-yftgk4an
section: ## Standard Contracts
change: `Session::actions(&mut self) -> Vec<ActionNode>` (packages/escher-driver/src/actions.rs:40-102) reads one snapshot and one `element_ids` map per call and records every id it reads; for each listed node and each `Level::Instance` verb with an `ArgKind::Id` argument, in table order (today `click`, `type`, `scroll`, `screenshot`; no verb name written in the code, `target_verbs`), it builds the call `call_naming` spells (a required non-target argument a fixed in-bound value by `ArgKind`, `Text` the empty text), runs it through `validate` and then `check_target`, and reads `stale` for a node that resolves to no element.
sidecar: driver session: `Session::actions` added (F1).
rationale: report Symbols / APIs: `Session::actions` (`actions.rs:40-102`), `target_verbs()` (`actions.rs:105-111`), `call_naming` (`actions.rs:113-136`).
@proposal n=13 detector=D-new-detail severity=warning file=standard-contracts anchor=fct-t7zq5feb fact=F2 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-yftgk4an
section: ## Standard Contracts
change: `check_target(harness, &Command, &Target) -> Result<Cleared, Refusal>`, crate-private in execute.rs (packages/escher-driver/src/execute.rs:498-526, `Cleared` `Point` · `Rect` · `Free` at execute.rs:478-496), is the one per-verb target check: `click` and `type` meet `action_point` (disabled, then off-screen, then covered), `scroll` meets none, and an id-taking `screenshot` meets `in_view`, then an empty crop, then `can_paint_text` false (`fonts-unavailable`) before painting; its only callers are the four id-taking arms of `Session::execute`, which keep no check of their own, and `Session::actions`; `Target` is `pub(crate)` and `locate` ends in `target_of` (execute.rs:430-439), which `Session::actions` also calls.
sidecar: driver session: the shared per-verb target check `check_target` (F2).
rationale: report Symbols / APIs: `check_target` (`execute.rs:498-526`), `Cleared` (`execute.rs:478-496`), `target_of` (`execute.rs:430-439`); Outcome (tests): every id-taking arm decides through `check_target`.
@proposal n=14 detector=D-new-detail severity=warning file=standard-contracts anchor=fct-t7zq5feb fact=F3 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-ai45gnz6
section: ## Standard Contracts
change: the `actions` read changes nothing: after it `has_changes` is false and the snapshot, both focus readings and the diff are unchanged, an open select list stays open, and actions.rs makes no `pump`, `tick`, `dispatch`, `scroll_into_view`, `close_select_list` or `focus_select` call.
sidecar: driver session: the `actions` read changes nothing (F3).
rationale: report Outcome (arch): `the_read_changes_nothing_and_holds_no_typed_text_or_mask` and the open-list test in `stand_actions`.
@proposal n=15 detector=D-new-detail severity=warning file=standard-contracts anchor=fct-t7zq5feb fact=F4 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-tj4uicby
section: ## Standard Contracts
change: `actions` answers whether a call would be accepted, not what it would do (Accepted is not acting); it takes no argument and answers the whole screen at once, unpaged — the Cells answer 106,088 bytes for 678 nodes, within the 1,048,576-byte answer bound (as measured at 2026-11-08-what-can-i-do-here).
sidecar: driver session: `actions`' stated limits, Accepted is not acting and the unpaged answer (F4).
rationale: report Counts / qualifiers moved (new): the Cells `actions` answer 106,088 bytes for 678 nodes through the binary; `stand_actions` asserts it within 1,048,576; `res-7cc5bi4k` widened to own the unpaged answer.
@proposal n=16 detector=D-cascade severity=warning file=standard-contracts anchor=fct-tofwjjiv basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-tj4uicby
section: ## Standard Contracts
change: a call no record can spell is a verb outside the ten instance verbs (an `actions` step kept like a `lint` or a `graph` step, its answer whole, a cause kept and an absent cause left out).
sidecar: driver session recording: instance verbs kept 9 → 10, an `actions` step kept whole.
rationale: report Counts [instance]: instance verbs 9 → 10; Outcome (tests): `recording_tests` reads a kept `actions` step's `answer` back with its `nodes`.
@proposal n=17 detector=D-cascade severity=warning file=standard-contracts anchor=fct-tofwjjiv dependent_of=D-cascade basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-tj4uicby
section: ## Standard Contracts
change: `Check::from_bytes` reads the ten instance verbs.
sidecar: driver session recording: a kept check's verbs 9 → 10.
rationale: report Counts [instance]: «nine instance verbs» → ten.
@proposal n=18 detector=D-cascade severity=warning file=body anchor=fct-ksgwc2yv basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-wuaab5w2
section: ## Standard Contracts
change: the command and refusal schema stated once as `'static` data — fourteen verbs, eight closed causes, no refusal holding what a call supplied.
sidecar: architecture body Driver session: thirteen → fourteen verbs.
rationale: report Schema / config: `VERBS` 14; Counts [verbs]: «thirteen verbs» → fourteen.
@remark
F5 and F6 (DioxusDocument::actionable_nodes, ActionableNode, the shared Readings rule) and the Retired mark leg belong to the Dioxus DOM bridge contract, no picked entity here, so the new-detail pass places them; the body line fct-srb7slak/fct-d5zuqqfb registers no actionable_nodes (D-arch-resources), and F7 is established-decisions.
