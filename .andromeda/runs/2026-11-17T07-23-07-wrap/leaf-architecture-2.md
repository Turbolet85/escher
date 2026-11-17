@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/dioxus-native-dom.md lines=44
last: - `doc:architecture` · `.claude/docs/services/blitz-test-har
@leaf_read leaf=.claude/docs/stack.md lines=59
last: 4. Commit as `chore: bump {crate} to {version}`.
@leaf_read leaf=.claude/docs/workflow.md lines=47
last: A reference-based CLAUDE.md: a thin index, with depth in `.c
@leaf_edit n=1 leaf=.claude/docs/workflow.md line=15 class=stale
leaf-text: upstream `main` was last read at `ea07ee716`, three commits of that take taken by cherry-picks with the merge base unmoved and 112 left out with their measured reasons, its two behaviour changes for every document — the `@font-face` `font-stretch` read and the registry taffy 0.15.0 → 0.15.1 move — ratified
source: dec-z45xhlo5 «The twenty-nine commits of the upstream take read at `563d3db73` are engine behaviour changes for every document.»
replace-with: upstream `main` was last read at `563d3db73`, its take's commits taken by no-commit cherry-picks with the merge base unmoved and each commit left out carrying its measured reason, the twenty-nine commits of that take ratified as engine behaviour changes for every document (Taffy now from upstream's git source, pinned to one revision)
