@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=32
last: - 2026-10-15: A listener or env read that only a test makes
@leaf_edit n=1 leaf=.claude/rules/security.md line=13 class=omission
leaf-text: blitz-dom builds a sub-resource request only through the one constructor that stamps its loading document as the initiator (clippy `disallowed-methods` on `Request::get`, the constructor alone under a single `expect`) — keep it so.
source: dec-xqvruskj «An SVG image or feImage element's href, the unprefixed href else xlink:href, is fetched as an external image sub-resource through the stamping constructor, carrying its document as initiator»
replace-with: blitz-dom builds a sub-resource request only through the one constructor that stamps its loading document as the initiator (clippy `disallowed-methods` on `Request::get`, the constructor alone under a single `expect`) — keep it so. An SVG `image` or `feImage` element's href (the unprefixed `href`, else `xlink:href`) is fetched as an external image sub-resource through that constructor, under the `file:` refusal; a `use` element's href is never fetched — a boundary widening by upstream merge, ratified.
