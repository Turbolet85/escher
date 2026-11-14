@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=32
last: - 2026-10-15: A listener or env read that only a test makes 
@leaf_edit n=1 leaf=.claude/rules/security.md line=13 class=omission
leaf-text: no log site of its own); where it reads
source: dec-gsml2o7x «A file: URL loads only for a file: document or a top-level load; for any other document, remote pages included, script fetch, module import and script src fail, blitz-net refuses engine loads before a disk read»
replace-with: no log site of its own), and for such a document the browser app names no `file:` favicon and drops a `file:` link navigation silently; where it reads
@leaf_edit n=2 leaf=.claude/rules/security.md line=16 class=stale
leaf-text: an `href` that cannot be resolved
source: dec-ihosj2ae «In every document, a stylesheet href that cannot be resolved starts no load and blocks no render.»
replace-with: a stylesheet `href` that cannot be resolved
@leaf_edit n=3 leaf=.claude/rules/security.md line=28 class=omission
leaf-text: builds pass `--locked` — keep both.
source: dec-gg74dumo «Every action a CI workflow uses is pinned to a 40-hex commit SHA, with its ref kept as a trailing comment.»
replace-with: builds pass `--locked` — keep both. Every action a CI workflow uses is pinned to a 40-hex commit SHA, its ref kept as a trailing comment (`WorkflowPinTest`) — keep it so.
