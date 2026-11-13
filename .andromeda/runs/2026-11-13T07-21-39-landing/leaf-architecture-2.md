@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/dioxus-native-dom.md lines=44
last: - `doc:architecture` · `.claude/docs/services/blitz-test-har
@leaf_read leaf=.claude/docs/stack.md lines=59
last: 4. Commit as `chore: bump {crate} to {version}`.
@leaf_read leaf=.claude/docs/workflow.md lines=47
last: A reference-based CLAUDE.md: a thin index, with depth in `.c
@leaf_edit n=1 leaf=.claude/docs/services/dioxus-native-dom.md line=17 class=stale
leaf-text: else the `value` attribute — and a password or file `input`
source: ctr-r2f7smnf «A select's chosen option value, unmasked (a multiple select's first chosen), else the editor text, else a range input's current value, else the value attribute of a textarea or text-entry input.»
replace-with: else the `value` attribute of a textarea or a text-entry input — and a password or file `input`
@leaf_edit n=2 leaf=.claude/docs/services/dioxus-native-dom.md line=38 class=omission
leaf-text: `src/{actionable,dioxus_document,element_id,graph,mutation_writer,mutation_buffer,events,signal_registry,snapshot,snapshot_diff,snapshot_text,write_once_attr,keyboard_compat}.rs`
source: dec-wdtb3kpq «gates its accessibility-tree override and its `snapshot`, `snapshot_diff`, `snapshot_text`, `actionable` and `graph` modules; its `signal_registry` and `reached` modules stay ungated.»
replace-with: `src/{actionable,dioxus_document,element_id,graph,mutation_writer,mutation_buffer,events,reached,signal_registry,snapshot,snapshot_diff,snapshot_text,write_once_attr,keyboard_compat}.rs` (`reached`, like `signal_registry`, outside every `accessibility` gate: the reached record behind `record_reached` / `reached_handlers`)
@leaf_edit n=3 leaf=.claude/docs/stack.md line=9 class=tree
leaf-text: Omarchy 4.0.4 / Arch, `escher-0.1.0/chunks/2026-10-05-as-built-baseline/evidence/baseline.md`)
source: dec-qibp3grm «The flake's pinned Rust toolchain matches the workspace rust-version.»
replace-with: Omarchy 4.0.4 / Arch)
