@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=32
last: - 2026-10-15: A listener or env read that only a test makes —
@leaf_edit n=1 leaf=.claude/rules/security.md line=22 class=stale
leaf-text: location, boundary, cause chain and fields, a content-named field read as redacted (a boundary widening to the error text, ratified)
source: dec-vztg2sue «with its message, location, boundary, cause chain and fields, error as written within the text bound, every other content-named field redacted.»
replace-with: location, boundary, cause chain and fields, its `error` field as its author wrote it within the text bound, every other content-named field read as redacted (a boundary widening to the error text, ratified)
