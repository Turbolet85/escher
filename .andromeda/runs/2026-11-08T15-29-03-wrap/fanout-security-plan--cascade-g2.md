@proposal n=1 detector=D-cascade severity=warning file=input-validation anchor=fct-tbmm2cwc basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-tj4uicby
section: ## Input Validation
change: in valid_step's closed keys, «`tool` one of the thirteen verbs» reads «`tool` one of the fourteen verbs» — the pipe's VERBS joining `actions` after `graph`
sidecar: input-validation cold-agent check row: valid_step's tool set thirteen → fourteen verbs, `actions` joining after `graph` (2026-11-08-what-can-i-do-here)
rationale: the report's Counts (cold-tools, verbs) says the cold-agent pipe's VERBS joins `actions` after `graph` and `scripts/cold-agent.sh` gains line 53; the row's count is stale
@proposal n=2 detector=D-cascade severity=warning file=input-validation anchor=fct-pbvi5fsv basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-znp5nbcg
section: ## Input Validation
change: «`graph` is one of the pipe's thirteen `VERBS`» reads «one of the pipe's fourteen `VERBS`», and the row adds that `actions` joins after `graph` the same way — admitted by `valid_step`, validated again by the replay's server, required of no cold agent, and answered by the test's fake server (`if name == "actions":`, .github/scripts/test_cold_agent.py:144-145)
sidecar: input-validation cold-agent graph-step row: the pipe's VERBS thirteen → fourteen, `actions` admitted like `graph` and answered by the fake server (2026-11-08-what-can-i-do-here)
rationale: the report's Counts (cold-tools) moves the pipe's VERBS to include `actions`, and New text by line lists test_cold_agent.py 144-145 «if name == "actions":»
@proposal n=3 detector=D-cascade severity=warning file=input-validation anchor=fct-7jixhrbw basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-wuaab5w2
section: ## Input Validation
change: after the UI graph in the answer's carried set, add «and the actions answer — an accepted `actions`'s line `{"nodes":[{"id":…,"owner":…,"keyed":…,"verbs":[{"verb":…,"cause":…}]}]}`, holding ids, owner ids, keyed booleans, the table's verb words and closed cause words, never a control value, typed text or `NodeId`, a boundary widening ratified by the founder (2026-11-08, inputs#I4), the `keyed` field provisional, under the unchanged 1,048,576-byte bound (the Driver command schema's `actions` row)»
sidecar: input-validation socket row: the actions answer joins the socket's carried set, ratified 2026-11-08 (inputs#I4), keyed provisional (2026-11-08-what-can-i-do-here)
rationale: the report's Schema / config and Expected amendments (security-plan §Input Validation, carried) say the actions answer joins the socket's carried set with ids, owner ids, verb and cause words and booleans only, the crossing ratified at inputs#I4
@proposal n=4 detector=D-cascade severity=warning file=input-validation anchor=fct-ifbk2u2m basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-tj4uicby
section: ## Input Validation
change: «a verb outside the nine instance verbs» reads «a verb outside the ten instance verbs», and «a `lint` step kept like every other with its answer whole» reads «a `lint` or `actions` step kept like every other with its answer whole»
sidecar: input-validation recording directory row: instance verbs nine → ten, an `actions` step kept with its answer whole (2026-11-08-what-can-i-do-here)
rationale: the report's Counts (instance) moves instance verbs 9 → 10, and its Outcome says recording_tests reads a kept `actions` step's answer back whole with its nodes
@proposal n=5 detector=D-cascade severity=warning file=input-validation anchor=fct-v4gvn7w2 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-tj4uicby
section: ## Input Validation
change: «`tool` one of the nine instance verbs» reads «`tool` one of the ten instance verbs»
sidecar: input-validation recorded check row: the reader's tool set nine → ten instance verbs, `actions` the tenth (2026-11-08-what-can-i-do-here)
rationale: the report's Counts (instance) moves «nine instance verbs» to ten, `actions` the tenth instance verb after `graph`
@proposal n=6 detector=D-cascade severity=warning file=input-validation anchor=fct-awfxp663 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-wuaab5w2
section: ## Input Validation
change: «a closed `'static` table of thirteen verbs» reads «of fourteen verbs», and after «a ninth, `graph` (no argument, running no step; the `graph` row below)» add «a tenth, `actions` (no argument, any argument refused `malformed` unnamed; one result field, `nodes`, `FieldKind::ActionNodes`, a thirteenth field kind; running no step)»
sidecar: input-validation Driver command schema row: VERBS thirteen → fourteen, `actions` the tenth instance verb with its `action-nodes` field (2026-11-08-what-can-i-do-here)
rationale: the report's Schema / config says VERBS holds 14 with `actions` the tenth instance verb after `graph` and FieldKind thirteen with ActionNodes, and Symbols / APIs gives schema::ACTIONS with no argument
@proposal n=7 detector=D-new-detail severity=warning file=input-validation anchor=fct-awfxp663 fact=F3 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-a7grs7bs
section: ## Input Validation
change: add to the row: `Command::Actions` is validated from `("actions", [])`, and any argument is refused `malformed` — in process, on the command line over the socket (status 1) and over MCP (`isError`)
sidecar: input-validation Driver command schema row: Command::Actions validated with no argument, an argument refused malformed on every surface (2026-11-08-what-can-i-do-here)
rationale: F3; the report's Symbols / APIs gives Command::Actions validated from ("actions", []) and its Outcome (security) shows the malformed refusal in process, on the command line (status 1) and over MCP (isError)
@proposal n=8 detector=D-new-detail severity=warning file=input-validation anchor=fct-awfxp663 fact=F2 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-yftgk4an
section: ## Input Validation
change: add to the row: the per-verb target check is one function, `check_target`, for `click`, `type`, `scroll` and `screenshot` and for the actions read — `click` and `type` meet `action_point` (disabled, off-screen, covered), `scroll` meets none, and `screenshot` with an id meets `in_view`, then an empty crop, then `can_paint_text` (`fonts-unavailable`), all before painting; no id-taking arm keeps a check of its own
sidecar: input-validation Driver command schema row: the shared per-verb target check check_target, its order per verb and its callers (2026-11-08-what-can-i-do-here)
rationale: F2; the report's Symbols / APIs gives check_target at execute.rs:498-526 as the shared check, its per-verb order, and that the four id-taking arms and Session::actions are its only callers
@proposal n=9 detector=D-new-detail severity=warning file=input-validation anchor=fct-awfxp663 fact=F1 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-yftgk4an
section: ## Input Validation
change: add to the row: `Session::actions` reads one snapshot and one `element_ids` map per call and, for each listed node and each id-taking instance verb, builds a call with fixed in-bound arguments and passes it through `validate` and then `check_target`, running nothing; a node resolving to no element reads `stale`
sidecar: input-validation Driver command schema row: Session::actions validates and target-checks a built call per node and verb, running nothing (2026-11-08-what-can-i-do-here)
rationale: F1; the report's Symbols / APIs gives Session::actions at actions.rs:40-102, call_naming's fixed in-bound arguments, and the stale reading of an unresolved node; its Outcome (arch) shows the read changes nothing
@proposal n=10 detector=D-cascade severity=warning file=input-validation anchor=fct-gucmzkph basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-yiyuwutp
section: ## Input Validation
change: after the `graph` clause add «`actions` with no other argument than one `--session <dir>` answers the actions answer's one line of JSON, `{"nodes":[…]}`, on stdout, status 0, by the ratified crossing, and an argument to it is refused `malformed`, status 1 (examples/seven_guis/tests/cli_commands.rs:495-516)»
sidecar: input-validation CLI arguments row: the `actions` command's one JSON line and its malformed refusal, status 1 (2026-11-08-what-can-i-do-here)
rationale: the report's Outcome (security) shows an argument to `actions` refused on the command line, status 1 (cli_commands), and New text by line lists cli_commands.rs 495-516 «fn actions_answers_one_line_of_the_actionable_nodes()»
@proposal n=11 detector=D-cascade severity=warning file=input-validation anchor=fct-f4eyvu3v basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-6wtnpthi
section: ## Input Validation
change: «`tools/list` the thirteen verbs, help last (`screenshot`, `lint` and `graph`, listed after `lint`, among them» reads «the fourteen verbs, help last (`screenshot`, `lint`, `graph`, listed after `lint`, and `actions`, after `graph`, among them», and adds «an `actions` tool call's result the text block of its JSON line `{"nodes":[…]}`, its `tool_call` span `tool` = `actions` parenting one `command` span recording `verb` alone, an argument refused with `isError`»
sidecar: input-validation MCP server row: tools/list thirteen → fourteen, `actions` after `graph`, its tool call's nodes line and span (2026-11-08-what-can-i-do-here)
rationale: the report's Outcome (tests counts) reads tools/list fourteen in table order, (obs) stand_mcp_tools reads tool_call parenting one command, and (security) an argument refused isError over MCP
@proposal n=12 detector=D-new-detail severity=warning file=input-validation anchor=fct-d2yejoup fact=F4 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-yftgk4an
section: ## Input Validation
change: add to the row: the actionable rule is one crate-private reading, `Readings` — focusable, an interactive role, or current listeners holding an event other than `mounted`; both `unkeyed_actionable` and the read-only `DioxusDocument::actionable_nodes` read it, and the latter returns ids, owner and keyed only
sidecar: input-validation markup id row: one actionable rule, Readings, read by unkeyed_actionable and actionable_nodes, the latter ids, owner and keyed only (2026-11-08-what-can-i-do-here)
rationale: F4; the report's Symbols / APIs gives Readings at actionable.rs:62-81 as the one rule, its listener leg on exact current listeners, and ActionableNode { id, owner, keyed } at actionable.rs:49-60
@proposal n=13 detector=D-cascade severity=warning file=input-validation anchor=fct-d2yejoup basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-otmrcodh
section: ## Input Validation
change: «`VERBS` holding thirteen» reads «`VERBS` holding fourteen», and after the `graph` reader add «escher-driver's `Session::actions` is a further reader of the id — it takes no argument, reads `actionable_nodes` and returns `ActionNode`s holding ids, owner ids, keyed booleans and the table's verb and closed cause words, no `NodeId` — crossing as the `actions` verb's answer on every ratified carrier, a ratified boundary widening (the founder, 2026-11-08, inputs#I4; `keyed` provisional)», so «no driver, CLI or MCP command yet» no longer holds of the actionable reading
sidecar: input-validation markup id row: Session::actions a further reader of the id, crossing as the actions answer, VERBS fourteen (2026-11-08-what-can-i-do-here)
rationale: the report's Expected amendments carry the actions answer onto every ratified carrier on the founder's word (inputs#I4), with `keyed` as the actionable-key check's first wire form, provisional; Counts move VERBS 13 → 14
@proposal n=14 detector=D-cascade severity=warning file=input-validation anchor=fct-d2yejoup basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-yftgk4an
section: ## Input Validation
change: the session's record of ids, «fed only from snapshots the session itself takes — once `validate` has admitted a call, and, since 2026-10-19-visual-lint, on `Session::lint`», adds «and, since 2026-11-08-what-can-i-do-here, on `Session::actions`, which takes no argument and records every id it reads»
sidecar: input-validation markup id row: the record of ids also fed on Session::actions (2026-11-08-what-can-i-do-here)
rationale: the report's Symbols / APIs says Session::actions records every id it reads in the session's record of ids
@proposal n=15 detector=D-cascade severity=warning file=body anchor=fct-cvyxuuhn basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-wuaab5w2
section: ## Input Validation
change: «a closed `'static` table of thirteen verbs» reads «a closed `'static` table of fourteen verbs»
sidecar: security-plan body Driver command schema broad line: thirteen → fourteen verbs (2026-11-08-what-can-i-do-here)
rationale: the report's Schema / config says VERBS holds 14, `actions` the tenth instance verb
@proposal n=16 detector=D-cascade severity=warning file=body anchor=fct-6hwcr6nz basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-wuaab5w2
section: ## Input Validation
change: the bounded replies' carried set adds «the actions answer's ids, owner ids, keyed booleans, verb words and closed cause words» after the UI graph's entry
sidecar: security-plan body socket broad line: the actions answer joins the replies' carried set (2026-11-08-what-can-i-do-here)
rationale: the report's Schema / config and Expected amendments say the actions answer joins the socket's carried set with ids, owner ids, verb and cause words and booleans only
@remark
F5 and F6 (threat-model-summary), F7 (api-security), F8 (data-protection) and F9 (logging-monitoring) fall to other categories and are not placed here; the Retired data-dioxus-id mark leg appears in no picked entity of this group (the report keeps security-plan.md:153 no-change); no new dependency, auth or unvalidated input surface, so D-security-input, D-security-auth and D-security-deps do not fire
