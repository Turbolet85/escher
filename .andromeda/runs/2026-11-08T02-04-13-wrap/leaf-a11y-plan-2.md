@leaf_group group=a11y-plan-2
@leaf_read leaf=.claude/rules/a11y.md lines=46
last: - 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable
@leaf_edit n=1 leaf=.claude/rules/a11y.md line=26 class=stale
leaf-text: (`NODE_FIELDS` thirteen)
source: fct-yfrcl4qo «`NodeState` holds no such field and `NODE_FIELDS` reads fourteen, so a list's open state reaches the tree and not the snapshot»
replace-with: (`NODE_FIELDS` fourteen)
@leaf_edit n=2 leaf=.claude/rules/a11y.md line=27 class=omission
leaf-text: and its diff `Snapshot::diff` read that model and map nothing again)
source: fct-d5bs73pt «and its read-only UI graph reader (`DioxusDocument::graph`, a `UiGraph`, its `graph` module), while its `signal_registry` module (`use_graph_state`, `DeclaredWrite`, `SignalRegistry`) is not gated»
replace-with: and its diff `Snapshot::diff` read that model and map nothing again, as does the read-only UI graph `DioxusDocument::graph`, a `UiGraph` of one node per snapshot node, names and closed words only, while the `signal_registry` module (`use_graph_state`, `DeclaredWrite`, `SignalRegistry`) is not gated)
@leaf_edit n=3 leaf=.claude/rules/a11y.md line=28 class=omission
leaf-text: returns the ones that read a positional path, each with its `remedy()`.
source: fct-77ewqepn «the snapshot field `clickable` says an element reacts to a click, read from the vdom walk's current listeners and not the listener mark»
replace-with: returns the ones that read a positional path, each with its `remedy()`; the snapshot field `clickable` says an element reacts to a click, read from the vdom walk's current listeners and not the listener mark, which over-includes an element whose listener was removed.
