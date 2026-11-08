@proposal n=1 detector=D-cascade severity=warning file=existing-scopes anchor=fct-kaaobbkf basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-yftgk4an
section: ## Existing Scopes
change: In the dioxus-native-dom unit, the actionable module reads: actionable (crate-private, compiled only under `accessibility`: the one actionable rule, the crate-private `Readings`, read by both the actionable-key check behind `DioxusDocument::unkeyed_actionable` and the read-only `DioxusDocument::actionable_nodes`, its `UnkeyedActionable` and `ActionableNode` types re-exported).
sidecar: existing-scopes dioxus-native-dom: actionable module names actionable_nodes, the shared Readings rule and the ActionableNode re-export (2026-11-08-what-can-i-do-here)
rationale: report Symbols / APIs: ActionableNode re-exported from the crate root beside UnkeyedActionable, actionable_nodes read-only, Readings the ONE rule both readers read; Crates / modules: dioxus-native-dom's crate root adds ActionableNode
@proposal n=2 detector=D-cascade severity=warning file=existing-scopes anchor=fct-mzrxx2a3 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-xxsa6syv
section: ## Existing Scopes
change: In the escher-driver unit, «seventeen private modules» reads «eighteen private modules», and after the `graph` verb's clause it adds: and the `actions` verb — schema's `ACTIONS` and its `FieldKind::ActionNodes`, command's `Command::Actions`, execute holding `Outcome::Actions` and the crate-private per-verb target check `check_target` (with `Cleared` and `target_of`) that the four id-taking arms and the read share, json the action node's hand-written form, and the private module actions (`Session::actions`, `ActionNode`, `VerbReading`, a read that changes nothing) — re-exported at the crate root.
sidecar: existing-scopes escher-driver: eighteen private modules, the actions module and the actions verb's schema, command, outcome, check_target and json form (2026-11-08-what-can-i-do-here)
rationale: report Crates / modules: escher-driver gains the private module actions, seventeen to eighteen, re-exports 57 to 59 (ActionNode, VerbReading); Symbols / APIs: ACTIONS, FieldKind::ActionNodes, Command::Actions, Outcome::Actions, pub(crate) check_target and Cleared, target_of, json action_node
@proposal n=3 detector=D-cascade severity=warning file=existing-scopes anchor=fct-4zmppehn basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-ccyc2gwa
section: ## Existing Scopes
change: In the blitz-tests unit, after `stand_graph`'s clause it adds: `stand_actions`, 8 tests in both layout modes — each served screen's actionable nodes listed with the table's verbs and counts, every listed cause on the lean tasks equal to what `Session::run` returns (3 `disabled` controls), the stated Cells and load-page causes, the flight booker's options read `covered` until the list opens, the last CRUD row `off-screen` until a scroll, the fixtures read `covered`, unkeyed and `fonts-unavailable`, the read changing nothing and holding no typed text or mask, and the whole Cells answer within the answer bound — in `stand-repeat` by its `stand_` prefix.
sidecar: existing-scopes blitz-tests: the new stand_actions target, 8 tests in both layout modes (2026-11-08-what-can-i-do-here)
rationale: report Changes Files: new tests/blitz-tests/tests/stand_actions.rs; Outcome lists its tests and gates read 8 passed; Harness / gate surface: stand_actions enters stand-repeat through the stand_*.rs glob
@proposal n=4 detector=D-cascade severity=warning file=existing-scopes anchor=fct-4zmppehn basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-yftgk4an
section: ## Existing Scopes
change: In the blitz-tests unit's session_common clause, its readers add `stand_actions` after `stand_graph`, and its holdings end «…and the `graph` builder and the `graphed` reader, and the `actions` builder and the `actioned` reader».
sidecar: existing-scopes blitz-tests: session_common read by stand_actions, holding the actions builder and actioned reader (2026-11-08-what-can-i-do-here)
rationale: report Symbols / APIs, Tests' shared modules: session_common gains the actions() builder and actioned() reader; Counts: session_common declarers 33 to 34, stand_actions the thirty-fourth
@proposal n=5 detector=D-cascade severity=warning file=existing-scopes anchor=fct-4zmppehn basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-yftgk4an
section: ## Existing Scopes
change: In the blitz-tests unit's tests/common/mod.rs clause, its per-task tables add `ACTIONABLE_AT_BOOT`, the measured count of actionable nodes at boot per served task (counter 2, flight-booker 7, timer 3, crud 11, cells 678, load 2).
sidecar: existing-scopes blitz-tests: common/mod.rs holds ACTIONABLE_AT_BOOT (2026-11-08-what-can-i-do-here)
rationale: report Symbols / APIs, Tests' shared modules: blitz-tests common gains ACTIONABLE_AT_BOOT [(&str, usize); 6]; Counts: ACTIONABLE_AT_BOOT = 2 · 7 · 3 · 11 · 678 · 2, measured
@proposal n=6 detector=D-cascade severity=warning file=existing-scopes anchor=fct-4zmppehn basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-njnsyi3r
section: ## Existing Scopes
change: In the blitz-tests unit's stand_mcp_tools clause, «a `graph` tool call answering the readers' graph and parenting one `command` span that records its verb alone» reads «a `graph` and an `actions` tool call, each answering its reader and parenting one `command` span that records its verb alone, through one shared helper».
sidecar: existing-scopes blitz-tests: stand_mcp_tools holds an actions tool-call test beside graph's, one shared helper (2026-11-08-what-can-i-do-here)
rationale: report Deviations: the graph tool-call test body became the shared helper and the actions test runs the same assertions; Counts: stand_mcp_tools tests 7 to 8
@proposal n=7 detector=D-cascade severity=warning file=existing-scopes anchor=fct-qkp2micb basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-yftgk4an
section: ## Existing Scopes
change: In the seven_guis unit, its shared module tests/common/mod.rs «holds `cells_needles(part)` and `graph_needles()` among its readers» reads «holds `cells_needles(part)`, `graph_needles()` and `actions_needles()` among its readers».
sidecar: existing-scopes seven_guis: tests/common/mod.rs holds actions_needles (2026-11-08-what-can-i-do-here)
rationale: report Symbols / APIs, Tests' shared modules: seven_guis common gains actions_needles(), read by host_log and mcp_log; Counts: the needle helpers gain actions_needles
@proposal n=8 detector=D-cascade severity=warning file=existing-scopes anchor=fct-o6vmxcuj basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-tj4uicby
section: ## Existing Scopes
change: `tests/session_common/mod.rs` is read «by 34», and `tests/fixtures/mod.rs` «by 32 (24 `stand_*` files, `stand_act_popup` and `stand_actions` among them, …)».
sidecar: existing-scopes: session_common declarers 33 to 34, fixtures declarers 31 to 32 with stand_* files 23 to 24 (2026-11-08-what-can-i-do-here)
rationale: report Counts / qualifiers moved: session_common declarers 33 to 34 with stand_actions the thirty-fourth [sc-readers], fixtures declarers 31 to 32 and its stand_* files 23 to 24 [fx-readers]
@remark
The report moves no count for blitz-tests' tests/common/mod.rs declarers (22, in #1.31 and #3.2), though ACTIONABLE_AT_BOOT was added there; if stand_actions declares mod common that count is owed 23. F1-F7 are standard-contracts and established-decisions facts, not existing-scopes, and go to their own groups.
