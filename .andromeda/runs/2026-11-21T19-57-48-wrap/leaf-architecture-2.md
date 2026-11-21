@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/dioxus-native-dom.md lines=44
last: - `doc:architecture` · `.claude/docs/services/blitz-test-harn
@leaf_edit n=1 leaf=.claude/docs/services/dioxus-native-dom.md line=17 class=stale
leaf-text: answering `None` for a select whose list is open
source: ctr-w3cesngn «Answers None for a select whose list is open and for a select BaseDocument::is_list_box reads true.»
replace-with: answering `None` for a select whose list is open and for a select `BaseDocument::is_list_box` reads true
@leaf_edit n=2 leaf=.claude/docs/services/dioxus-native-dom.md line=17 class=stale
leaf-text: a `multiple` select's rows and groups reading client rects of their own
source: ctr-qi3yfel5 «A list box's rows and groups, a multiple select's or a single select's of display size above 1, read their own client rects as their bounds.»
replace-with: a list box's rows and groups (a `multiple` select's, or a single select's of display size above 1) reading client rects of their own
@leaf_edit n=3 leaf=.claude/docs/services/dioxus-native-dom.md line=17 class=stale
leaf-text: an option of a closed select, under an `optgroup` too
source: ctr-qjomhxg2 «An option of a closed drop-down, under an optgroup too, reads its select's client rect as bounds and is the select's child, named by its label; while the list is open a row reads its own place in the list.»
replace-with: an option of a closed drop-down (a single select of display size 1), under an `optgroup` too
