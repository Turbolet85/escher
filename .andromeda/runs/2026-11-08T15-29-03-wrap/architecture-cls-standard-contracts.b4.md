@answer n=1 entity=standard-contracts#28.4 v=no
why: the Cells keyboard wiring in examples/seven_guis/src/tasks/cells.rs is not reached by the session
@answer n=2 entity=standard-contracts#29.1 v=yes mode=add-to
why: the actionable rule in Readings now reads element_facts' current listeners (F6), joining the listener readers this row names
@answer n=3 entity=standard-contracts#29.2 v=no
why: clickable still reads element_listeners, which the session leaves unchanged, and stand_non_lean_tasks.rs is not reached
@answer n=4 entity=standard-contracts#29.3 v=no
why: the mutation buffer and writer listener calls are not reached, and the bridge's event attribute still stands
@answer n=5 entity=standard-contracts#30.1 v=no
why: is_masked_input in snapshot.rs is not reached and no reading of it changes
@answer n=6 entity=standard-contracts#30.2 v=no
why: the select value, form-data and closed-select readings in snapshot.rs and dioxus_document.rs are not reached
@answer n=7 entity=standard-contracts#30.3 v=no
why: NODE_FIELDS stays at fourteen keys and the snapshot and NodeState gain no field
@answer n=8 entity=standard-contracts#30.4 v=no
why: the multiple-select row and group bounds in snapshot.rs are not reached
@answer n=9 entity=standard-contracts#30.5 v=no
why: multiple_select_values in dioxus_document.rs is not in the session's file set and its values stay as they are
@answer n=10 entity=standard-contracts#30.6 v=no
why: flush_queued_mounted_events and its removed-element skip are not reached
@answer n=11 entity=standard-contracts#30.7 v=yes mode=add-to
why: actionable_nodes joins graph() as one more read-only accessor under accessibility, holding no NodeId and taking its records from the UI graph's nodes in pre-order (F5)
@answer n=12 entity=standard-contracts#30.8 v=no
why: actionable_nodes reads the graph owner but the session changes no owner, reader or writer reading
@answer n=13 entity=standard-contracts#30.9 v=no
why: use_graph_state and the SignalRegistry are not reached by the session
@answer n=14 entity=standard-contracts#31.1 v=yes mode=stale
why: the thirteen-verb VERBS list moves to fourteen with actions joining after graph, so the count and the verb list both change
@answer n=15 entity=standard-contracts#31.2 v=no
why: the multi_doc example is outside the session's file set and its environment variables are unchanged
@answer n=16 entity=standard-contracts#32.1 v=no
why: browser app persistence and about pages are not reached by the session
@answer n=17 entity=standard-contracts#33.1 v=no
why: the WPT runner outputs are not reached by the session
@answer n=18 entity=standard-contracts#34.1 v=yes mode=stale
why: README.md and docs/capabilities.md are reached and the fourteen-command count moves in README, so the README.md:49 citation may have moved
@answer n=19 entity=standard-contracts#34.2 v=no
why: ci-leg.sh and its embolden leg are not reached, and the session changes no CI leg
@answer n=20 entity=standard-contracts#34.3 v=no
why: the stand-repeat disk test lives in test_ci_workflows.py, which is not reached, and no job changed
@answer n=21 entity=standard-contracts#34.4 v=no
why: the font-embolden witness and select_popup_painted are not reached by the session
@answer n=22 entity=standard-contracts#35.1 v=yes mode=stale
why: the escher server behind the cold-agent pipe now exposes fourteen tools with actions joining, so the thirteen-tools count moves
