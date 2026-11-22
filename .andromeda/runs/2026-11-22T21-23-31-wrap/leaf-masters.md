@leaf_group group=masters
@leaf_read leaf=.claude/docs/gotchas.md lines=68
last: - Decisions and rationale: `doc:architecture` (the records s
@leaf_read leaf=CLAUDE.md lines=166
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=92 class=stale
leaf-text: unit tests inline, bar schema's in `schema_tests.rs`
source: dec-rlfpyplu «An escher-driver module holds its unit tests in a #[path] child file of its own, never a module of the crate's public tree.»
replace-with: each module's unit tests in a `#[path]` child file of its own, schema's in `schema_tests.rs`
@leaf_edit n=2 leaf=CLAUDE.md line=93 class=stale
leaf-text: `Session::lint`, in process and as the `lint` verb; unit tests inline
source: dec-rlfpyplu «An escher-driver module holds its unit tests in a #[path] child file of its own, never a module of the crate's public tree.»
replace-with: `Session::lint`, in process and as the `lint` verb; unit tests in a `#[path]` child file of its own
@leaf_edit n=3 leaf=CLAUDE.md line=122 class=omission
leaf-text: nothing the caller typed written back; the stand's is
source: dec-ekl4sc6j «No escher-session command's ending writes back anything the caller typed, a recording path included. · except: The layout answer's id field names the element the call's id named.»
replace-with: nothing the caller typed written back but the layout answer's id field, which names the element the call's id named; the stand's is
