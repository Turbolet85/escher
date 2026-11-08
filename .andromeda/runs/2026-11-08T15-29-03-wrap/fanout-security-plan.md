@proposal n=1 detector=D-security-input severity=warning basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-yftgk4an
section: Input Validation
change: In fct-cvyxuuhn, `validate` checks every call against a closed `'static` table of fourteen verbs (thirteen → fourteen, `actions` the tenth instance verb after `graph`, taking no argument, any argument refused `malformed` with nothing run) and seven bounded argument kinds; the rest of the claim stands.
sidecar: Input Validation (Driver command schema): the verb table holds fourteen verbs; `actions` validated from `("actions", [])`, an argument refused `malformed` (2026-11-08-what-can-i-do-here).
rationale: The report's Symbols / APIs bullet (fct-yftgk4an) adds `Command::Actions`, validated from `("actions", [])` with an argument refused `malformed`, and `schema::ACTIONS` at schema.rs:429-452; Schema / config (fct-wuaab5w2) and Counts moved (fct-tj4uicby) state `VERBS` 13 → 14. The new surface is validated as §Input Validation mandates, but the body's statement of that validation still counts thirteen.
@proposal n=2 detector=D-security-input severity=warning basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-wuaab5w2
section: Input Validation
change: In fct-6hwcr6nz, the socket's bounded replies also carry an `actions` answer's ids, owner ids, the table's verb words, closed cause words and booleans (`keyed`), never a value, typed text or `NodeId`; the rest of the claim stands.
sidecar: Input Validation (Driver session socket): the `actions` answer joins the socket's carried set: ids, owner ids, verb words, closed cause words, booleans (2026-11-08-what-can-i-do-here).
rationale: The report's Schema / config bullet (fct-wuaab5w2) states the answer carries ids, owner ids, the table's verb words, closed cause words and booleans, never a value, typed text or NodeId; its written form is `json.rs`'s `action_node` (json.rs:251-272); the plan's expected amendment (fct-otmrcodh) says the actions answer joins the socket's, MCP's and a kept check's carried sets.
@proposal n=3 detector=D-security-input severity=warning dependent_of=D-security-input basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-wuaab5w2
section: Threat Model Summary
change: In fct-z5ttw3ol (local IPC, the driver session's socket), the carried calls and answers also include an `actions` answer's ids, owner ids, the table's verb words, closed cause words and booleans, and still no `NodeId`; the rest of the vector stands.
sidecar: Threat Model Summary (local IPC vector): the socket also carries the `actions` answer, closed words, ids and booleans only (2026-11-08-what-can-i-do-here).
rationale: Same carried-set claim as proposal 2's in §Input Validation, restated in the socket vector; the report (fct-wuaab5w2, fct-nnb75cma) lists the `actions` verb on the socket `call` with ids, owner ids, verb and cause words and bools only.
@proposal n=4 detector=D-security-input severity=warning dependent_of=D-security-input basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-wuaab5w2
section: Threat Model Summary
change: In fct-dgkl4a64 (local stdio, the driver's MCP server), a result also carries an `actions` answer's ids, owner ids, the table's verb words, closed cause words and booleans to the client; the rest of the vector stands.
sidecar: Threat Model Summary (local stdio vector): an MCP result also carries the `actions` answer, closed words, ids and booleans only (2026-11-08-what-can-i-do-here).
rationale: The report's Coverage of new surfaces (fct-nnb75cma) lists the `actions` verb over MCP, and Schema / config (fct-wuaab5w2) names what its answer holds; the MCP vector's carried set does not yet name it.
@remark
D-security-auth and D-security-deps hold (no auth/secret change; Dependencies none). D-retired: the retired mark listener leg has no standing security-plan body site (the report's unkeyed dispositions mark security-plan's sites no-change; fct-qk7naa6l's data-dioxus-id parse stays true). The detail files' thirteen/nine-instance counts and the actions rows (input-validation, api-security, logging-monitoring, data-protection) are the cascade's.
