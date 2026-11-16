@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/dioxus-native-dom.md lines=44
last: - `doc:architecture` · `.claude/docs/services/blitz-test-harn
@leaf_read leaf=.claude/docs/services/seven_guis.md lines=44
last: - `.claude/rules/a11y.md` (the stand is in its paths) · `.clau
@leaf_edit n=1 leaf=.claude/docs/services/dioxus-native-dom.md line=14 class=omission
leaf-text: `RootErrorMark`, compared by identity, its `Debug` printing its name alone.
source: ctr-ixzsilx6 «Public and re-exported: the document provides a boundary registry at the root scope before its first build, every error boundary built under it registers, and boundary_marks reads them by identity»
replace-with: `RootErrorMark`, compared by identity, its `Debug` printing its name alone; public and re-exported, `boundary_marks` and `BoundaryMark` — the document provides a boundary registry at the root scope before its first build, every error boundary built under it registers, and `boundary_marks` reads them by identity, the registry growing for the document's life since a dropped boundary cannot be told.
