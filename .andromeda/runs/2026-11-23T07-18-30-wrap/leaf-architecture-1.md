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
@leaf_edit n=1 leaf=.claude/docs/services/blitz-paint.md line=32 class=omission
leaf-text: (`anyrender_vello_cpu` with no `multithreading`)
source: dec-rlsleth2 «Harness turns on blitz-paint's custom-widget feature — blitz-test-harness names blitz-paint's custom-widget feature, so a harness or driver shot paints an app's widget in every build selection reaching the harness.»
replace-with: (`anyrender_vello_cpu` with no `multithreading`, and blitz-paint's `custom-widget` feature named, so a harness or driver shot paints an app's widget in every build selection reaching the harness)
@leaf_stands n=1 leaf=.claude/docs/conventions.md line=48
because: The `custom-widget` on this line names a feature in upstream's blitz-dom clippy line (dec-q62kxraz, ctr-o6gihoto, unchanged this pass), while the pass's new row dec-rlsleth2 concerns blitz-paint's custom-widget feature in blitz-test-harness and takes nothing from the clippy line.
@leaf_stands n=2 leaf=.claude/docs/conventions.md line=6
because: dec-ve2bmybh still states that the checks holding a `Session` keep their shared helpers in a second module, `session_common`, untouched by this pass, so the line's `session_common/mod.rs` still says what its row says.
@leaf_stands n=3 leaf=.claude/docs/services/blitz-dom.md line=20
because: The `custom-widget` here is one of blitz-dom's default features and part of upstream's blitz-dom clippy line (dec-q62kxraz, unchanged this pass); the new row dec-rlsleth2 is about blitz-paint's feature in the harness and changes neither.
