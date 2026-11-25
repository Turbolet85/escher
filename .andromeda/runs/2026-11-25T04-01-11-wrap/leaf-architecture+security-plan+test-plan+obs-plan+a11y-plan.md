@leaf_group group=architecture+security-plan+test-plan+obs-plan+a11y-plan
@leaf_read leaf=.claude/docs/gotchas.md lines=68
last: - Decisions and rationale: `doc:architecture` (the records s
@leaf_read leaf=CLAUDE.md lines=169
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=33 class=omission
leaf-text: and the run's seeded randomness (`SeededRandom`, `use_seeded_random`, `DEFAULT_SEED`)
source: ctr-hkso5edi «contract/dioxus-bridge/virtual-nodes · Dioxus DOM bridge · DioxusDocument::virtual_nodes — An ungated in-process read of one canvas's declarations, answering the taken nodes, each with id {owner}///{key}»
replace-with: the run's seeded randomness (`SeededRandom`, `use_seeded_random`, `DEFAULT_SEED`) and the in-process read of a canvas's declared virtual nodes (`virtual_nodes`, `VirtualNodeRefusal`)
@leaf_edit n=2 leaf=CLAUDE.md line=122 class=omission
leaf-text: `BaseDocument::font_ctx` public behind that feature)
source: ctr-jwsqrnv6 «contract/nodes-widgets/virtual-nodes · blitz-dom nodes and widgets · Widget::virtual_nodes and VirtualNodeDeclaration — Behind custom-widget, a defaulted method returning an empty list through which a widget declares regions»
replace-with: `BaseDocument::font_ctx` public behind that feature; a widget declaring regions as virtual nodes by `Widget::virtual_nodes`, defaulted empty and never called by the engine, read in process alone by `DioxusDocument::virtual_nodes` as taken nodes of id `{owner}///{key}` and refused declarations by index and closed reason, no carrier, snapshot, graph, canvas answer or accessibility tree holding a declared node)
@leaf_edit n=3 leaf=.claude/docs/gotchas.md line=31 class=omission
leaf-text: and a focus given up after a layout marks its node.
source: dec-coubgznd «decision/virtual-node-declaration-marks-nothing · A declaration change marks and signals nothing yet — A change of a canvas's declared virtual nodes writes no changed-set mark and sends no invalidation signal»
replace-with: and a focus given up after a layout marks its node, while a change of a canvas's declared virtual nodes marks nothing and sends no invalidation signal.
