@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/dioxus-native-dom.md lines=44
last: - `doc:architecture` · `.claude/docs/services/blitz-test-har
@leaf_read leaf=.claude/docs/stack.md lines=59
last: 4. Commit as `chore: bump {crate} to {version}`.
@leaf_read leaf=.claude/docs/workflow.md lines=47
last: A reference-based CLAUDE.md: a thin index, with depth in `.c
@leaf_edit n=1 leaf=.claude/docs/services/dioxus-native-dom.md line=38 class=omission
leaf-text: src/{actionable,dioxus_document,element_id,graph,mutation_writer,mutation_buffer,events,reached,signal_registry,snapshot,snapshot_diff,snapshot_text,write_once_attr,keyboard_compat}.rs` (`reached`, like `signal_registry`, outside every `accessibility` gate
source: dec-wdtb3kpq «its `signal_registry`, `reached` and `seed` modules stay ungated.»
replace-with: src/{actionable,dioxus_document,element_id,graph,mutation_writer,mutation_buffer,events,reached,seed,signal_registry,snapshot,snapshot_diff,snapshot_text,write_once_attr,keyboard_compat}.rs` (`seed`, like `reached` and `signal_registry`, outside every `accessibility` gate: the run seed behind `SeededRandom`, `use_seeded_random` and `DEFAULT_SEED`) (`reached`, like `signal_registry`, outside every `accessibility` gate
