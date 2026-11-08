@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/dioxus-native-dom.md lines=44
last: - `.andromeda/architecture.md` · `.claude/docs/services/blit
@leaf_read leaf=.claude/docs/stack.md lines=59
last: 4. Commit as `chore: bump {crate} to {version}`.
@leaf_read leaf=.claude/docs/workflow.md lines=47
last: A reference-based CLAUDE.md: a thin index, with depth in `.c
@leaf_edit n=1 leaf=.claude/docs/services/dioxus-native-dom.md line=16 class=stale
leaf-text: or a listener (`data-dioxus-id`)
source: fct-ljxizw75 «or its current listeners, from `element_facts` (the vdom walk's exact reading), hold an event other than `mounted`»
replace-with: or current listeners holding an event other than `mounted` (read from the vdom walk's `element_facts` through the crate-private `Readings`, never the `data-dioxus-id` mark, so an element whose listener a re-render removed is not counted)
@leaf_edit n=2 leaf=.claude/docs/services/dioxus-native-dom.md line=16 class=stale
leaf-text: no wire form and no driver command yet
source: fct-kllkxz3d «the check's first wire form is `ActionableNode`'s `keyed` reading (true where the id holds no `/`), which the `actions` verb carries on every carrier, PROVISIONAL until the founder's own word»
replace-with: its first wire form `ActionableNode`'s `keyed` reading (true where the id holds no `/`), which the driver's `actions` verb carries on every carrier, PROVISIONAL until the founder's own word
@leaf_edit n=3 leaf=.claude/docs/services/dioxus-native-dom.md line=16 class=omission
leaf-text: Read-only, logs nothing;
source: fct-vvyrpx4f «`DioxusDocument::actionable_nodes(&self) -> Vec<ActionableNode>`, behind `accessibility`, is read-only: it returns `ActionableNode { id: String, owner: Option<String>, keyed: bool }`»
replace-with: Read-only, logs nothing; beside it the read-only `DioxusDocument::actionable_nodes() -> Vec<ActionableNode>` (`ActionableNode { id, owner: Option<String>, keyed: bool }`, re-exported beside `UnkeyedActionable`) lists, over the UI graph's nodes in snapshot pre-order, each node the same actionable rule holds for or the graph gives an owner, `keyed` true where the id holds no `/`, both readers reading the one rule;
@leaf_edit n=4 leaf=.claude/docs/services/dioxus-native-dom.md line=17 class=stale
leaf-text: read by `execute.rs` `locate` alone
source: fct-7zhttwuk «`is_select`, the driver's select test, read within the workspace by `execute.rs`'s crate-private `target_of` alone»
replace-with: read within the workspace by `execute.rs`'s crate-private `target_of` alone, which `locate` ends in and `Session::actions` also calls
