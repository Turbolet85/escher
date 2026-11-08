@carried n=1 label=test-plan/test-strategy fact=fct-5d4ttzku
why: the coverage command must read checks run through this replay, and the owed test must hold an actions step to the same rule — red at the first step whose kept answer differs.
quote: read by the strict `Check::from_bytes` and run as an ordinary `cargo test` target (`stand_replay`) through `Check::replay` — every step re-validated through `Session::run`, red at the first step whose acceptance or kept answer differs, the answers compared as JSON values
@carried n=1 run=2 label=test-plan fact=fct-obwdwo5i
quote: run as an ordinary `cargo test` target through `Check::replay`, red at the first step whose acceptance or kept answer differs
@carried n=2 label=test-plan/unit-test-strategy fact=fct-klxgwgcy
why: the owed actions-step replay test must re-assert at the stand what replay.rs pins in unit: a fresh-session replay holds and a changed answer stops it naming its key.
quote: since 2026-10-19-recorded-check-replay replay.rs that a check replayed on a fresh session holds, that an acceptance flip and a changed answer each stop the replay naming their keys, that a shot holds at its size whatever its pixels and that a check of another app runs nothing (packages/escher-driver/src/replay.rs:141-318)
@carried n=2 run=2 label=test-plan/unit-test-strategy fact=fct-klxgwgcy
quote: schema_tests.rs (`schema::tests`) pins the verb table — fourteen verbs in a stated order and three levels
@carried n=2 run=3 label=test-plan fact=fct-mnyw2bw5
quote: its unit tests pin the wire grammar, the session's label rule and record of ids, the error and refusal texts, the verb schema, command validation, the executor's key table and span, the JSON forms, the command line, the client, MCP, recording, replay, image and lint
@carried n=3 label=test-plan/test-strategy fact=fct-krjmr356
why: the coverage claim and its canvas limit must each be named in the proof section, the limit pinned by its check or stated as asserted by no check.
quote: `docs/capabilities.md`, a document with no runtime surface, ends on `## Where each claim is proven`, naming the check behind each claim it makes — the stand checks the `stand-repeat` leg runs among them — and states each limit as pinned by its check or as asserted by no check
@carried n=4 label=test-plan/unit-test-strategy fact=fct-rg6yibnz
why: if the command is a new verb, the capability statement's verb count in words and README's tool count must move from fourteen to fifteen with no hand-copied word.
quote: every verb of `VERBS` (`packages/escher-driver/src/schema.rs`, lower-cased) is a code span of `docs/capabilities.md`; every cause of `Cause::name`'s match arms (`packages/escher-driver/src/refusal.rs`) is one; the statement states five counts in words read from the schema's tables
@carried n=4 run=2 label=test-plan fact=fct-pf335kvt
quote: test_capability_statement.py pins `docs/capabilities.md` to the driver schema's tables — every verb and refusal cause named as a code span, the verb, cause, session-error, node-key and exit-status counts stated in words — and README.md's count of the driver commands an MCP client gets as tools
@carried n=5 label=test-plan/unit-test-strategy fact=fct-fghufsse
why: a new verb must move the VERBS pin and describe's count row together, one fork the scope leaves to P4.
quote: with `VERBS` at 14, causes at 8 and `describe`'s count row `[14, 14, 12, 3, 8, 4, 15]`
@carried n=6 label=test-plan/unit-test-strategy fact=fct-zot44d7e
why: a new verb must enter the MCP listing test's table walk in VERBS' order, the test's name and count moving with it.
quote: mcp_tests.rs's `the_listing_is_the_fourteen_verbs_of_the_table` asserts the fourteen verbs of the table in `VERBS`' order, `graph` after `lint` and `actions` after `graph`
@carried n=7 label=test-plan/unit-test-strategy fact=fct-qc3s4qp3
why: a new verb's result field must take its own row in the per-verb field-kind table, and the actions help pinned word for word must stay unamended.
quote: schema_tests.rs keys its field-kind table per verb (`graph.nodes`, `actions.nodes`), the two verbs' `nodes` fields differing in kind, and pins the `actions` help word for word among its 12 `amended` rows
@carried n=8 label=test-plan/unit-test-strategy fact=fct-fwtnmbsl
why: escher-driver's unit-test count must be restated as measured once the chunk adds driver unit tests.
quote: escher-driver's unit tests read 102 — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md
@carried n=9 label=test-plan/unit-test-strategy fact=fct-hskbfmoc
why: the coverage counts the graph's current listeners, so the graph reader's pins — removed listeners gone, confirmed after a dispatch — must keep holding beside any new exercised reading in graph.rs.
quote: `graph::tests` pins the UI graph reader — each node's component; its current listeners, `mounted` included and a removed one gone; a declared write reading declared before a dispatch and confirmed after
@carried n=10 label=test-plan/unit-test-strategy fact=fct-3gle5qdo
why: the new exercised record must hold on a page with no registered state, where these pins require that no registry exists, so the record cannot live in the registry alone.
quote: a handler's write recorded against its dispatch and one outside any dispatch against none; a state's entry leaving with its scope and its `Debug` printing counts alone; a declaration naming its element or its prefix; and a page with no registered state holding no registry
@carried n=11 label=test-plan/unit-test-strategy fact=fct-mi2j5ztx
why: the bridge's dispatch record is recorded only where a state is registered; the new (element, event) record must be pinned also on a page with none, through the same test-only shim.
quote: `bridge_tests::a_dispatch_is_recorded_where_a_state_is_registered_and_acts_as_before_where_none_is` drives a dispatch through the test-only `#[cfg(test)]` `DioxusDocument::dispatch_to_vdom` and reads the registry's dispatch record where a state is registered, a page with none acting as before
@carried n=12 label=test-plan/unit-test-strategy fact=fct-3cpa7acl
why: the command is keyed on the graph node id with its optional owner, leaving the reserved virtual form open for 0.4.0 while producing no virtual node.
quote: pin the reserved `{owner}///{key}` form over an adversarial page; `virtual_node_id` returns `None` for an empty or `/`-bearing key, and no caller produces one, the walk not calling it
@carried n=13 label=test-plan/test-strategy fact=fct-7m7lxsmy
why: each new check file the chunk adds must be recorded in these reader and target counts, restated as measured.
quote: `tests/session_common/mod.rs`, read through `mod session_common;` by 34 checks, as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md
@carried n=13 run=2 label=test-plan/test-strategy fact=fct-7m7lxsmy
quote: 160 `.rs` files, 159 test targets and the unbuilt `all.rs`, as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md
@binding n=1
**test-strategy (recorded-check replay) ↔ unit-test-strategy (replay.rs)**: the owed stand test must hold an actions step to the replay rule the strategy states and the unit tests pin — replay on a fresh session holds, a differing answer stops it naming the step and key (test-plan/test-strategy fct-5d4ttzku) (test-plan/unit-test-strategy fct-klxgwgcy).
@binding n=2
**unit-test-strategy (signal registry, bridge record, graph reader) ↔ the scope's exercised record**: the new (element, event) record must hold on an app with no registered state, while the registry's pin keeps such a page holding no registry and the bridge test keeps it acting as before (test-plan/unit-test-strategy fct-3gle5qdo) (test-plan/unit-test-strategy fct-mi2j5ztx) (test-plan/unit-test-strategy fct-hskbfmoc).
@binding n=3
**unit-test-strategy (VERBS, MCP listing, field kinds, capability statement) ↔ test-strategy (claims mapped to checks)**: on the new-verb fork every pin of the fourteen moves together to fifteen, and the coverage claim and its canvas limit enter capabilities.md's proof section (test-plan/unit-test-strategy fct-fghufsse) (test-plan/unit-test-strategy fct-zot44d7e) (test-plan/unit-test-strategy fct-rg6yibnz) (test-plan/test-strategy fct-krjmr356).
@binding n=4
**unit-test-strategy (reserved id form) ↔ the scope's canvas limit**: the count stays keyed on the graph node id with its optional owner, never a NodeId, and no virtual node is produced (test-plan/unit-test-strategy fct-3cpa7acl).
@binding n=5
**test-strategy (directory counts) ↔ unit-test-strategy (driver unit-test count)**: each new check file and driver unit test is restated in its count, as measured in the report (test-plan/test-strategy fct-7m7lxsmy) (test-plan/unit-test-strategy fct-fwtnmbsl).
