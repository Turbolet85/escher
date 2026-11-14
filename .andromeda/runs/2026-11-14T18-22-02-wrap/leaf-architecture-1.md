@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - Architecture: `doc:architecture` (the records store) · dire
@leaf_read leaf=.claude/docs/gotchas.md lines=69
last: - Decisions and rationale: `doc:architecture` (the records s
@leaf_read leaf=.claude/docs/services/blitz-dom.md lines=45
last: - `doc:architecture` (the records store) · `.claude/docs/stac
@leaf_read leaf=.claude/docs/services/blitz-paint.md lines=36
last: - `doc:architecture` (the records store) · `.claude/docs/serv
@leaf_read leaf=.claude/docs/services/blitz-shell.md lines=35
last: - `doc:architecture` (the records store) · `.claude/docs/serv
@leaf_read leaf=.claude/docs/services/blitz-traits.md lines=34
last: - `doc:architecture` (the records store) · `.claude/docs/serv
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=32 class=omission
leaf-text: `clippy::expect_used` / `clippy::unwrap_used` allowed only locally, next to a justification.
source: dec-nc3nzjgd «except: The browser app's favicon test module allows `clippy::expect_used` with no justification beside it.»
replace-with: `clippy::expect_used` / `clippy::unwrap_used` allowed only locally, next to a justification, except the browser app's favicon test module, which allows `clippy::expect_used` with no justification beside it.
