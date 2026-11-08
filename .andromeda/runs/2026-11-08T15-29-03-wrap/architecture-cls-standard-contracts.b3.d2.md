@answer n=1 entity=standard-contracts#17.1 v=no
why: the headless stand options, boot and time step live in seven_guis stand.rs, which the session does not reach
@answer n=2 entity=standard-contracts#18.1 v=no
why: the stand's bundled font and its font context live in stand.rs and lib.rs, which the session does not reach
@answer n=3 entity=standard-contracts#19.1 v=no
why: boot_cells and its screen live in stand.rs, which the session does not reach
@answer n=4 entity=standard-contracts#19.2 v=no
why: the load page and its options replacement live in stand.rs, which the session does not reach
@answer n=5 entity=standard-contracts#19.3 v=no
why: LoadNet and LOAD_ANSWER_DELAY live in stand.rs, which the session does not reach
@answer n=6 entity=standard-contracts#19.4 v=no
why: the stand's graph hooks live in tasks/mod.rs, which the session does not reach
@answer n=7 entity=standard-contracts#20.1 v=no
why: the telemetry sink and its allowlist live in escher-telemetry, which the session does not reach
@answer n=8 entity=standard-contracts#21.1 v=yes mode=stale
why: the driver's counts move, verbs thirteen to fourteen with actions as the tenth instance verb, FieldKinds twelve to thirteen, re-exports 57 to 59, and Session::actions and the shared check_target join the session
@answer n=9 entity=standard-contracts#22.1 v=yes mode=stale
why: the recording reads the nine instance verbs, which become ten with actions, so a kept actions step must be spelt and read as a step
@answer n=10 entity=standard-contracts#22.2 v=no
why: the replay's rule lives in replay.rs, which the session does not reach, and its step-by-step span and comparison still hold for an actions step
@answer n=11 entity=standard-contracts#22.3 v=yes mode=stale
why: VERBS holds thirteen in this entity's lint clause, and with actions it holds fourteen
@answer n=12 entity=standard-contracts#22.4 v=no
why: the load wait's rule lives in session.rs, which the session does not reach, and actions runs no step so the verbs that wait on nothing still hold
@answer n=13 entity=standard-contracts#23.1 v=no
why: the select-option overlap reading lives in lint.rs and the stand lint tests, which the session does not reach
@answer n=14 entity=standard-contracts#23.2 v=no
why: the load page served witness lives in stand_load_page.rs, which the session does not reach
@answer n=15 entity=standard-contracts#23.3 v=yes mode=stale
why: json.rs, which the session reaches whole, holds the cited graph writer at lines 241 to 331, and the new action_node writer joins that module so the cited lines may have moved
@answer n=16 entity=standard-contracts#23.4 v=yes mode=add-to
why: the residual res-7cc5bi4k now owns the unpaged actions answer beside the unpaged graph, per F4, actions answers the whole screen at once and unpaged at 106,088 bytes for 678 nodes, within the bound
@answer n=17 entity=standard-contracts#23.5 v=yes mode=stale
why: common/mod.rs, which the session reaches whole, holds the cited GRAPH_DRIVES table at lines 254 to 334, and no report shows its claim still holds on the changed file
@answer n=18 entity=standard-contracts#24.1 v=no
why: the blitz-vibey-script JavaScript engine is not reached by the session
@answer n=19 entity=standard-contracts#25.1 v=no
why: document_may_load and the fetch gate live in blitz-vibey-script, which the session does not reach
@answer n=20 entity=standard-contracts#26.1 v=no
why: the CSS property accessors live in blitz-vibey-script, which the session does not reach
@answer n=21 entity=standard-contracts#26.2 v=no
why: URLSearchParams lives in blitz-vibey-script, which the session does not reach
@answer n=22 entity=standard-contracts#27.1 v=no
why: the high-level launch surface lives in dioxus-native and blitz, which the session does not reach, and ActionableNode reaches it only through the existing glob re-export
@answer n=23 entity=standard-contracts#28.1 v=yes mode=contradict
why: the actionable rule's listener leg no longer reads the data-dioxus-id mark but the element's exact current listeners holding an event other than mounted, the readers now share one Readings rule, and ActionableNode and actionable_nodes join beside UnkeyedActionable
@answer n=24 entity=standard-contracts#28.2 v=no
why: buffered_mutations lives in dioxus_document.rs, which the session does not reach
@answer n=25 entity=standard-contracts#28.3 v=no
why: the registry dispatch lives in dioxus_document.rs, which the session does not reach, and event dispatch still reads the data-dioxus-id mark
