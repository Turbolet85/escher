@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/dioxus-native-dom.md lines=44
last: - `doc:architecture` · `.claude/docs/services/blitz-test-harn
@leaf_read leaf=.claude/docs/services/escher-telemetry.md lines=46
last: - `.claude/rules/observability.md` · `.claude/docs/obs-summar
@leaf_edit n=1 leaf=.claude/docs/services/dioxus-native-dom.md line=14 class=stale
leaf-text: per snapshot node in pre-order, a virtual node excepted,
source: ctr-qgpk23sy «Read-only under accessibility, returns one node per snapshot node in pre-order, each with id, owner, component, handlers and reads, and states with name, component, declared, writers and readers.»
replace-with: per snapshot node in pre-order, a canvas's virtual nodes among them (each owned by its canvas, with no handlers or reads),
@leaf_edit n=2 leaf=.claude/docs/services/dioxus-native-dom.md line=15 class=omission
leaf-text: which the accessibility tree serves, and from it the snapshot and its diff, spelled by
source: ctr-rdehirc3 «A reserved fifth form {owner}///{key}, given a canvas's taken nodes and served in the accessibility tree and from it the snapshot, its diff, the UI graph, actions and layout, is spelled by the public»
replace-with: which the accessibility tree serves, and from it the snapshot, its diff, the UI graph, actions and layout, spelled by
