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
source: dec-n4rgyoyp «except: The driver crate opens every file with `//!`; upstream engine modules and examples largely open without it, and some upstream public items carry no `///`.»
replace-with: Modules, files and examples open with a `//!` doc, examples saying what they demonstrate, and public items carry `///` — the driver crate opens every file with `//!`, while upstream engine modules and examples largely open without it and some upstream public items carry no `///`.
@leaf_edit n=2 leaf=.claude/docs/conventions.md line=39 class=omission
leaf-text: Mutations go through `DocumentMutator`, which flushes deferred work on `Drop`;
source: dec-xmlwfni7 «DOM mutations go through `DocumentMutator`, which flushes deferred work on drop and requests a redraw only for in-document mutations.»
replace-with: Mutations go through `DocumentMutator`, which flushes deferred work on `Drop` and requests a redraw only for in-document mutations;
