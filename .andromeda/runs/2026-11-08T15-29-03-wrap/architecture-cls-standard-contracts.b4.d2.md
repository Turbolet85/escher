@answer n=1 entity=standard-contracts#28.4 v=no
why: the Cells keyboard contract sits in examples/seven_guis/src/tasks/cells.rs, which the session does not reach, and the grid's keyboard and focus are unchanged
@answer n=2 entity=standard-contracts#29.1 v=yes mode=add-to
why: F6, the one actionable rule reads the exact current listeners holding an event other than mounted, joins the exact listener reading this entry names
@answer n=3 entity=standard-contracts#29.2 v=no
why: NodeState.clickable still reads the exact listeners rather than the actionable rule or the mark, so the Cells 676 and mark 677 counts hold as measured
@answer n=4 entity=standard-contracts#29.3 v=no
why: the dioxus-core listener removal and set_attribute behaviour is in no reached file, and the session changes only the actionable check's listener leg
@answer n=5 entity=standard-contracts#30.1 v=yes mode=stale
why: lib.rs is reached and gains the ActionableNode re-export beside UnkeyedActionable, so the crate-root re-export placement beside MASKED_VALUE may have moved
@answer n=6 entity=standard-contracts#30.2 v=no
why: the select onchange, value and option bounds readings sit in dioxus_document.rs and snapshot.rs, neither reached, and read no part of the actionable rule
@answer n=7 entity=standard-contracts#30.3 v=no
why: NODE_FIELDS keeps its fourteen keys and NodeState gains no list field, the session's ActionableNode being a separate record
@answer n=8 entity=standard-contracts#30.4 v=no
why: the multiple select row bounds in snapshot.rs are unchanged and that file is not reached
@answer n=9 entity=standard-contracts#30.5 v=no
why: multiple_select_values in dioxus_document.rs is unchanged and that file is not reached
@answer n=10 entity=standard-contracts#30.6 v=no
why: flush_queued_mounted_events in dioxus_document.rs is unchanged and that file is not reached
@answer n=11 entity=standard-contracts#30.7 v=yes mode=add-to
why: F5, actionable_nodes is one more read-only accessibility reader over the UI graph's nodes in snapshot pre-order, joins the graph's read-only readers this entry names
@answer n=12 entity=standard-contracts#30.8 v=no
why: the graph's owner, readers and writers readings are unchanged, and actionable_nodes only reads owner from graph.rs, which the session does not reach
@answer n=13 entity=standard-contracts#30.9 v=no
why: the graph registry in signal_registry.rs is unchanged and that file is not reached
@answer n=14 entity=standard-contracts#31.1 v=yes mode=stale
why: the thirteen of VERBS becomes fourteen with actions after graph, a count the session moved in the escher-session CLI contract
@answer n=15 entity=standard-contracts#31.2 v=no
why: multi_doc's arguments, environment variables and output are unchanged and examples/multi_doc.rs is not reached
@answer n=16 entity=standard-contracts#32.1 v=no
why: the browser app's persistence, history and about-page readings are not reached by the driver session change
@answer n=17 entity=standard-contracts#33.1 v=no
why: the WPT runner's outputs are not reached by the session and no WPT file is in its changes
@answer n=18 entity=standard-contracts#34.1 v=yes mode=stale
why: the cold-agent pipe's thirteen tools becomes fourteen with its verb list joining actions, and README.md:49's driver-command count moves likewise, both files reached
@answer n=19 entity=standard-contracts#34.2 v=no
why: the embolden leg of ci-leg.sh is not reached and the session makes no paint change
@answer n=20 entity=standard-contracts#34.3 v=no
why: the stand-repeat disk-freeing pin reads the ci workflow, which the session does not change
@answer n=21 entity=standard-contracts#34.4 v=no
why: the blitz-paint font-embolden local witness is unchanged, and no engine file changed
@answer n=22 entity=standard-contracts#35.1 v=yes mode=stale
why: scripts/cold-agent.sh is reached and its pipe verb list joins actions, so the cited prompt and verdict lines 19-31, 178-271 and 428-506 may have moved
