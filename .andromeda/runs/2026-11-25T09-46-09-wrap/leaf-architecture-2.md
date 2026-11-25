@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/dioxus-native-dom.md lines=44
last: - `doc:architecture` · `.claude/docs/services/blitz-test-harn
@leaf_edit n=1 leaf=.claude/docs/services/dioxus-native-dom.md line=15 class=stale
leaf-text: which no tier holds and no walk writes, and which only the in-process `DioxusDocument::virtual_nodes` gives a canvas's taken nodes, no carrier serving it,
source: ctr-y6jxwmav «A reserved fifth id form {owner}///{key} is spelled by the public virtual_node_id and written by no walk; DioxusDocument::virtual_nodes gives it in process to a canvas's taken nodes, and the accessibility tree serves it»
replace-with: which no tier holds and no walk writes, which `DioxusDocument::virtual_nodes` gives in process to a canvas's taken nodes, and which the accessibility tree serves, and from it the snapshot and its diff,
@leaf_edit n=2 leaf=.claude/docs/services/dioxus-native-dom.md line=15 class=omission
leaf-text: carries each one as its element node's AccessKit `author_id`, never on a `TextRun`, the document root or the `Window` node
source: ctr-yqyuvrwo «Under the accessibility feature the Dioxus document's tree holds each canvas's taken virtual nodes as children of the canvas's node, after its own: role and label as declared, author_id {owner}///{key}, disabled where declared»
replace-with: carries each one as its element node's AccessKit `author_id`, never on a `TextRun`, the document root or the `Window` node, and holds each canvas's taken virtual nodes as children of the canvas's node, after its own (role and label as declared, `author_id` `{owner}///{key}`, disabled where declared; a hidden, collapsed or inert canvas, or one whose read errs, takes none)
@leaf_edit n=3 leaf=.claude/docs/services/dioxus-native-dom.md line=14 class=omission
leaf-text: one `GraphNode { id, owner, component, handlers, reads }` per snapshot node in pre-order
source: ctr-qgpk23sy «Read-only under accessibility, returns one node per snapshot node in pre-order, a virtual node excepted, each with id, owner, component, handlers and reads»
replace-with: one `GraphNode { id, owner, component, handlers, reads }` per snapshot node in pre-order, a virtual node excepted
@leaf_edit n=4 leaf=.claude/docs/services/dioxus-native-dom.md line=17 class=omission
leaf-text: A node is every element the accessibility tree keeps, generic containers included;
source: ctr-r4ppwrjg «A node is an accessibility tree node with an author_id whose element resolves, or a canvas's taken virtual node; the Window, the document root and text runs are not nodes»
replace-with: A node is every element the accessibility tree keeps, generic containers included, or a canvas's taken virtual node;
@leaf_edit n=5 leaf=.claude/docs/services/dioxus-native-dom.md line=17 class=omission
leaf-text: `TextRun`s, the document root and the `Window` are not nodes.
source: ctr-ub7uhcef «A virtual node reads its id, declared role and name, enabled as not declared disabled, invalid and focused, clickable false, highlighted false, no checked, value, selected or pressed, bounds its region moved»
replace-with: `TextRun`s, the document root and the `Window` are not nodes. A virtual node reads its id, declared role and name, `enabled` as not declared disabled, `invalid` and `focused`, `clickable` and `highlighted` false, no `checked`, `value`, `selected` or `pressed`, and no children.
@leaf_edit n=6 leaf=.claude/docs/services/dioxus-native-dom.md line=17 class=omission
leaf-text: `bounds` (`BaseDocument::select_list_bounds(node)` first, a row's or a group's place in an open list, its select's client rect again once the list closes;
source: ctr-r47pe7o3 «A row's or group's place in an open list, a virtual node's region moved by its canvas's content-box origin and read untransformed, else the element's viewport-relative, unclamped client bounding rect.»
replace-with: `bounds` (`BaseDocument::select_list_bounds(node)` first, a row's or a group's place in an open list, its select's client rect again once the list closes; for a virtual node its region moved by its canvas's content-box origin, read untransformed;
