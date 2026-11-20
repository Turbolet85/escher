@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/dioxus-native-dom.md lines=44
last: - `doc:architecture` · `.claude/docs/services/blitz-test-harn
@leaf_read leaf=.claude/docs/services/escher-telemetry.md lines=46
last: - `.claude/rules/observability.md` · `.claude/docs/obs-summary
@leaf_edit n=1 leaf=.claude/docs/services/dioxus-native-dom.md line=17 class=stale
leaf-text: `NodeState { enabled, checked, value, selected, pressed, invalid, focused, clickable }` — eight fields
source: ctr-r4pssjgw «NodeState carries nine fields in the order enabled, checked, value, selected, pressed, invalid, focused, clickable, highlighted.»
replace-with: `NodeState { enabled, checked, value, selected, pressed, invalid, focused, clickable, highlighted }` — nine fields
@leaf_edit n=2 leaf=.claude/docs/services/dioxus-native-dom.md line=17 class=retired
leaf-text: and `NODE_FIELDS` keeping its fourteen keys
source: ctr-qieygpzi «The snapshot carries no expanded key and NodeState no field for a list.»
replace-with: and `NodeState` no field for a list
@leaf_edit n=3 leaf=.claude/docs/services/dioxus-native-dom.md line=17 class=stale
leaf-text: names all eight
source: ctr-r4pssjgw «NodeState carries nine fields in the order enabled, checked, value, selected, pressed, invalid, focused, clickable, highlighted.»
replace-with: names all nine
@leaf_edit n=4 leaf=.claude/docs/services/dioxus-native-dom.md line=17 class=omission
leaf-text: `focused` is the tree's focus;
source: ctr-se7kzogx «A closed bool, true on an open single select list's highlighted option alone, read from the accessibility tree's active descendant; written only when true, as a bare text token after clickable and as JSON true.»
replace-with: `focused` is the tree's focus (on an open single select it stays on the select, which names its current option as the tree's active descendant, a page's `aria-activedescendant` never read); `highlighted` is true on an open single select list's highlighted option alone, read from the tree's active descendant;
@leaf_edit n=5 leaf=.claude/docs/services/dioxus-native-dom.md line=18 class=stale
leaf-text: `clickable` (`invalid` and `clickable` only when true)
source: ctr-qt3oevde «In fixed order enabled/disabled, checked/unchecked, value=, selected/unselected, pressed/unpressed, invalid, focused, clickable, highlighted; absent where the model holds nothing, invalid, clickable and highlighted written only when true.»
replace-with: `clickable` · `highlighted` (`invalid`, `clickable` and `highlighted` only when true)
@leaf_edit n=6 leaf=.claude/docs/services/dioxus-native-dom.md line=19 class=stale
leaf-text: (twelve variants, `Parent`
source: ctr-qn6vnmjb «Thirteen variants: Parent, Role, Name, Enabled, Checked, Value, Selected, Pressed, Invalid, Focused, Clickable, Highlighted, Bounds, with Reading::ALL.»
replace-with: (thirteen variants, `Parent`
@leaf_edit n=7 leaf=.claude/docs/services/dioxus-native-dom.md line=19 class=stale
leaf-text: `Clickable` · `Bounds`, with
source: ctr-qn6vnmjb «Thirteen variants: Parent, Role, Name, Enabled, Checked, Value, Selected, Pressed, Invalid, Focused, Clickable, Highlighted, Bounds, with Reading::ALL.»
replace-with: `Clickable` · `Highlighted` · `Bounds`, with
