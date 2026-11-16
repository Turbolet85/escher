@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - Architecture: `doc:architecture` (the records store) · dire
@leaf_read leaf=.claude/docs/gotchas.md lines=68
last: - Decisions and rationale: `doc:architecture` (the records st
@leaf_read leaf=.claude/docs/services/blitz-dom.md lines=44
last: - `doc:architecture` (the records store) · `.claude/docs/stac
@leaf_read leaf=.claude/docs/services/blitz-paint.md lines=35
last: - `doc:architecture` (the records store) · `.claude/docs/serv
@leaf_read leaf=.claude/docs/services/blitz-shell.md lines=34
last: - `doc:architecture` (the records store) · `.claude/docs/serv
@leaf_read leaf=.claude/docs/services/blitz-traits.md lines=33
last: - `doc:architecture` (the records store) · `.claude/docs/serv
@leaf_edit n=1 leaf=.claude/docs/services/blitz-dom.md line=15 class=omission
leaf-text: its one non-test caller escher-driver's `Session::lint`
source: ctr-qixddwqw «contract/base-document/layout-reason · BaseDocument · layout_reason — Public, read-only: answers a LayoutReason (placing word, container, offset, shifts, a size reason per axis with its cause), or None for an»
replace-with: its one non-test caller escher-driver's `Session::lint`; and a fourth public read-only reader, `BaseDocument::layout_reason`, answering a `LayoutReason` (placing word, container, offset, shifts, a size reason per axis with its cause), or `None` for an unresolved id, a non-element, an element with no computed styles or one with no box (display none, display contents, a display none ancestor), with no log and no panic
@leaf_edit n=2 leaf=.claude/docs/services/blitz-dom.md line=15 class=omission
leaf-text: `BoundingRect`, CSSOM View accessors, `Widget` trait
source: ctr-j2a3bgj3 «contract/blitz-dom-crate-root/layout-reason-types · blitz-dom crate root · LayoutReason, Placed, Shift, SizeReason, FlexInputs and their enums — The layout reason's structs and its non-exhaustive enums»
replace-with: `BoundingRect`, the layout reason's public read types re-exported from the crate root (the structs `LayoutReason`, `Placed`, `Shift`, `SizeReason`, `FlexInputs` and the non-exhaustive enums `PlacedBy`, `ShiftBy`, `SizeCause`, `Length`, `SizeKeyword`), CSSOM View accessors, `Widget` trait
