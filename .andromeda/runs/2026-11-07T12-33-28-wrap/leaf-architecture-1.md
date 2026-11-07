@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - Architecture: `.andromeda/architecture.md` · directory lay
@leaf_read leaf=.claude/docs/gotchas.md lines=68
last: - Decisions and rationale: `.andromeda/architecture.md` §Est
@leaf_read leaf=.claude/docs/services/blitz-dom.md lines=44
last: - `.andromeda/architecture.md` · `.claude/docs/stack.md` · `
@leaf_read leaf=.claude/docs/services/blitz-paint.md lines=35
last: - `.andromeda/architecture.md` · `.claude/docs/services/blit
@leaf_read leaf=.claude/docs/services/blitz-shell.md lines=34
last: - `.andromeda/architecture.md` · `.claude/docs/services/blit
@leaf_read leaf=.claude/docs/services/blitz-traits.md lines=33
last: - `.andromeda/architecture.md` · `.claude/docs/services/blit
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=36 class=omission
leaf-text: Borrow conflicts are worked around by taking a child list or layout out and restoring it;
source: fct-knw5nmxh «by rendering the vdom into a crate-private `MutationBuffer` with the document's `RefCell` unborrowed, then borrowing the document once and replaying the buffer through one `MutationWriter`»
replace-with: Borrow conflicts are worked around by taking a child list or layout out and restoring it, and, on the Dioxus bridge, by rendering the vdom into a crate-private `MutationBuffer` with the document's `RefCell` unborrowed, then borrowing the document once and replaying the buffer through one `MutationWriter`;
