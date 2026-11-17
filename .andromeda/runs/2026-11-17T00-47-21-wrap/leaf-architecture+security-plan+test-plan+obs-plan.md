@leaf_group group=architecture+security-plan+test-plan+obs-plan
@leaf_read leaf=.claude/docs/gotchas.md lines=69
last: - Decisions and rationale: `doc:architecture` (the records s
@leaf_read leaf=CLAUDE.md lines=165
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=122 class=stale
leaf-text: a log's scrubbed fields and the first reached handler
source: ctr-wkieu5ai «Present only when the step caused something: an item per panic, error or record: kind, level, source, target, message cut at 1024 bytes, location, an error's boundary and chain, a log's fields, error kept»
replace-with: a log's fields, its `error` field kept as written within the text bound and the rest of its hidden set redacted, and the first reached handler
@leaf_edit n=2 leaf=CLAUDE.md line=122 class=stale
leaf-text: whose one stylesheet fetch the in-process stand-in `LoadNet` answers
source: dec-6mytiutr «The stand load page's boot, with its self-answering LoadNet stand-in answering one fixed sheet after a delay, and the error-text check's failing page, whose FailingNet stand-in fails its fetch.»
replace-with: whose one stylesheet fetch the in-process stand-in `LoadNet` answers, and the error-text check's failing page, whose `FailingNet` stand-in fails its fetch
@leaf_edit n=3 leaf=CLAUDE.md line=108 class=omission
leaf-text: a leg with no CI job of its own), as CI runs them
source: dec-pe4jrwxz «Doc leg gates a public doc comment edit — A chunk that edits a public doc comment lists the doc leg in its local gate block.»
replace-with: a leg with no CI job of its own), as CI runs them — it runs no rustdoc, so a chunk that edits a public doc comment lists the `doc` leg in its local gate block too
