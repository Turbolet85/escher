@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/dioxus-native-dom.md lines=44
last: - `doc:architecture` · `.claude/docs/services/blitz-test-harn
@leaf_edit n=1 leaf=.claude/docs/services/dioxus-native-dom.md line=15 class=stale
leaf-text: an anchored path is the only id holding `//`
source: ctr-kc6ehpdb «An author key holds no `/`, an anchored path is the only served id holding `//`, and a component or document path has no empty segment, so no `id` attribute can make two ids equal»
replace-with: an anchored path is the only served id holding `//`
@leaf_edit n=2 leaf=.claude/docs/services/dioxus-native-dom.md line=15 class=stale
leaf-text: which no tier holds and no walk writes
source: ctr-y6jxwmav «A reserved fifth id form {owner}///{key} is spelled by the public virtual_node_id and written by no walk; DioxusDocument::virtual_nodes gives it in process to a canvas's taken nodes, and no carrier serves it.»
replace-with: which no tier holds and no walk writes, and which only the in-process `DioxusDocument::virtual_nodes` gives a canvas's taken nodes, no carrier serving it
@leaf_edit n=3 leaf=.claude/docs/services/dioxus-native-dom.md line=14 class=omission
leaf-text: so one dropped in a step reads till the next render.
source: ctr-hkso5edi «An ungated in-process read of one canvas's declarations, answering the taken nodes, each with id {owner}///{key}, and the refused declarations by index, or NoCanvas for a stale or non-canvas node and NotLaidOut before the first layout; no panic path.»
replace-with: so one dropped in a step reads till the next render; ungated too, `DioxusDocument::virtual_nodes`, an in-process read of one canvas's declarations answering the taken nodes, each with id `{owner}///{key}`, and the refused declarations by index (`VirtualNodeRefusal`: `EmptyKey`, `KeyHoldsSlash`, `DuplicateKey` (the first of a key kept), `EmptyName` or `RegionOutside`, validated in declaration order, a refusal holding the index alone), or `NoCanvas` for a stale or non-canvas node and `NotLaidOut` before the first layout, with no panic path.
