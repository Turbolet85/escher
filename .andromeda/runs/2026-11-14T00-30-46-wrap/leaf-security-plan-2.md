@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=32
last: - 2026-10-15: A listener or env read that only a test makes 
@leaf_edit n=1 leaf=.claude/rules/security.md line=13 class=stale
leaf-text: reads `file:` URLs from disk with no path restriction
source: dec-ik2uwgla «A script fetch, module import or script src, and every engine stylesheet, @import, font, image or iframe load, may read a file: URL only for a file: document or a top-level load, in every document, remote pages included; blitz-net refuses any other before a disk read.»
replace-with: reads a `file:` URL from disk only for a `file:` document or a top-level load — a stylesheet, `@import`, font, image or iframe load of one for any other document, remote pages included, refused before any disk read with `FileForbidden` (fixed text «file: load refused for a non-file document», no URL or path, through the existing error path, no log site of its own)
@leaf_edit n=2 leaf=.claude/rules/security.md line=15 class=stale
leaf-text: a `file:` URL loads only for a `file:` document
source: dec-gsml2o7x «A file: URL loads only for a file: document or a top-level load; for any other document, remote pages included, JS fetch, module import and script src refuse it with a FetchError, and blitz-net refuses the engine's stylesheet, @import, font, image and iframe loads before any disk read.»
replace-with: a `file:` URL loads only for a `file:` document or a top-level load
@leaf_edit n=3 leaf=.claude/rules/security.md line=13 class=omission
leaf-text: — forward it.
source: dec-34g6yji2 «One stamping constructor for sub-resource requests — blitz-dom builds a sub-resource request only through the one constructor that stamps its loading document as the request's initiator.»
replace-with: — forward it. blitz-dom builds a sub-resource request only through the one constructor that stamps its loading document as the initiator (clippy `disallowed-methods` on `Request::get`, the constructor alone under a single `expect`) — keep it so.
