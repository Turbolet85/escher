@leaf_group group=architecture+security-plan+test-plan-1
@leaf_read leaf=.claude/docs/services/escher-driver.md lines=63
last: - The command line, through the real `escher-session` binary
@leaf_edit n=1 leaf=.claude/docs/services/escher-driver.md line=17 class=omission
leaf-text: and `contrast` `Harness::text_contrast` below `required_ratio()`
source: ctr-s5m2lrhi «contract/lint-verb/contrast · The lint verb · contrast — A node with direct non-whitespace text or a text control, reported below the required ratio, exempt when disabled; an open list's highlighted row is read at its painted pair»
replace-with: and `contrast` `Harness::text_contrast` below `required_ratio()`, an open list's highlighted row read at its painted pair, the engine's highlight colours, through `Harness::highlight_contrast`
@leaf_edit n=2 leaf=.claude/docs/services/escher-driver.md line=20 class=stale
leaf-text: fourteen keys
source: ctr-se7kzogx «contract/node-keys/highlighted · node keys · highlighted — A closed bool, true on an open single select list's highlighted option alone, read from the accessibility tree's active descendant; written only when true»
replace-with: fifteen keys
@leaf_edit n=3 leaf=.claude/docs/services/escher-driver.md line=20 class=omission
leaf-text: it is no disabled form control) · `bounds` · `differs`
source: ctr-se7kzogx «contract/node-keys/highlighted · node keys · highlighted — A closed bool, true on an open single select list's highlighted option alone, read from the accessibility tree's active descendant; written only when true»
replace-with: it is no disabled form control) · `highlighted` (since 2026-11-20-select-list-and-selection-debts: true on an open single select list's highlighted option alone, read from the accessibility tree's active descendant) · `bounds` · `differs`
@leaf_edit n=4 leaf=.claude/docs/services/escher-driver.md line=20 class=omission
leaf-text: with `parent`, `enabled`, `checked`, `value`, `selected` and `pressed` left out when absent, `invalid` and `clickable` always written
source: ctr-t4s5ouag «contract/answer-json/node · answer JSON · node — An object keyed by the node keys in order, parent, enabled, checked, value, selected and pressed left out when absent, highlighted written only when true»
replace-with: with `parent`, `enabled`, `checked`, `value`, `selected` and `pressed` left out when absent, `highlighted` written only when true, `invalid` and `clickable` always written
@leaf_edit n=5 leaf=.claude/docs/services/escher-driver.md line=24 class=stale
leaf-text: `NODE_FIELDS` is `[NodeFieldSpec; 14]`
source: ctr-se7kzogx «contract/node-keys/highlighted · node keys · highlighted — A closed bool, true on an open single select list's highlighted option alone, read from the accessibility tree's active descendant; written only when true»
replace-with: `NODE_FIELDS` is `[NodeFieldSpec; 15]`
@leaf_edit n=6 leaf=.claude/docs/services/escher-driver.md line=24 class=stale
leaf-text: 14 node keys
source: ctr-se7kzogx «contract/node-keys/highlighted · node keys · highlighted — A closed bool, true on an open single select list's highlighted option alone, read from the accessibility tree's active descendant; written only when true»
replace-with: 15 node keys
@leaf_edit n=7 leaf=.claude/docs/services/escher-driver.md line=32 class=stale
leaf-text: in the snapshot and the diff — `differs` naming `bounds`, a closed set
source: dec-rt2rzqbq «decision/select-list-crossing · Select list crosses only as bounds and pixels — The select list crosses the carriers only as a row's bounds and closed bool highlighted in the snapshot and diff, differs naming either»
replace-with: and, since 2026-11-20-select-list-and-selection-debts, as the closed bool `highlighted` on the open list's one highlighted row, in the snapshot and the diff — `differs` naming either, a closed set
@leaf_edit n=8 leaf=.claude/docs/services/escher-driver.md line=32 class=stale
leaf-text: session error, node key, log field or span
source: dec-rt2rzqbq «decision/select-list-crossing · Select list crosses only as bounds and pixels — The select list crosses the carriers only as a row's bounds and closed bool highlighted in the snapshot and diff, differs naming either, the words select and select-list»
replace-with: session error, log field or span
