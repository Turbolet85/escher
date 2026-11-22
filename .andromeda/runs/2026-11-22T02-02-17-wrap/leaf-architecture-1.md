@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - Architecture: `doc:architecture` (the records store) · dir
@leaf_read leaf=.claude/docs/services/blitz-dom.md lines=44
last: - `doc:architecture` (the records store) · `.claude/docs/sta
@leaf_read leaf=.claude/docs/services/blitz-paint.md lines=35
last: - `doc:architecture` (the records store) · `.claude/docs/ser
@leaf_read leaf=.claude/docs/services/blitz-shell.md lines=34
last: - `doc:architecture` (the records store) · `.claude/docs/ser
@leaf_read leaf=.claude/docs/services/blitz-traits.md lines=33
last: - `doc:architecture` (the records store) · `.claude/docs/ser
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=7 class=omission
leaf-text: Modules and files open with a `//!` doc; examples open with a `//!` comment saying what they demonstrate.
source: dec-mzios4kg «Modules and files open with `//!` docs. · except: The driver crate opens every file with `//!`; upstream engine modules largely open without it.»
replace-with: Modules and files open with a `//!` doc — the driver crate every file, while upstream engine modules largely open without it; examples open with a `//!` comment saying what they demonstrate, most upstream examples with a block comment or none.
@leaf_edit n=2 leaf=.claude/docs/services/blitz-dom.md line=21 class=omission
leaf-text: two upstream engine changes for every document, not counted among escher's direct ones.
source: dec-6beca3l4 «An inline-box placeholder carries a private engine fallback font family registered in every font context, so an inline-block inside text no font can shape is placed once its own font loads.»
replace-with: two upstream engine changes for every document, not counted among escher's direct ones. An inline-box placeholder carries a private engine fallback font family registered in every font context, so an inline-block inside text no font can shape is placed once its own font loads — a direct engine change for every document, flagged upstreamable.
@leaf_edit n=3 leaf=.claude/docs/services/blitz-dom.md line=33 class=tree
leaf-text: `src/events/{driver,focus,keyboard,pointer,ime}.rs`
source: dec-iouqf4lq «Engine and user-agent events are queued on the document and dispatched by the event driver's flush of pending events»
replace-with: `src/events/{mod,driver,keyboard,pointer,wheel,ime}.rs` (focus handling in `src/document.rs`)
