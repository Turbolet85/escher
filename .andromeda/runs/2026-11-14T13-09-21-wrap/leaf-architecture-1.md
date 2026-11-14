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
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=30 class=omission
leaf-text: error types are enums with `Display` and `From` per wrapped error.
source: dec-ywcujahe «decision/std-result-under-dioxus-prelude · std Result spelled under the Dioxus prelude glob — A file that glob-imports the dioxus_native prelude spells a two-parameter result as std::result::Result.»
replace-with: error types are enums with `Display` and `From` per wrapped error; a file that glob-imports the `dioxus_native` prelude spells a two-parameter result as `std::result::Result`, since on Android the glob brings manganis's one-parameter `Result` alias, a failure only the CI Android job sees.
