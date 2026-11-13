@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=32
last: - 2026-10-15: A listener or env read that only a test makes 
@leaf_edit n=1 leaf=.claude/rules/security.md line=16 class=omission
leaf-text: HTML is parsed with scripting disabled;
source: dec-xetdufyi «decision/untrusted-bytes-decoded-without-panic · Page and stylesheet bytes decoded without panic — A page whose bytes are not UTF-8 is decoded lossily and never panics, and a stylesheet is decoded from its bytes by its BOM and @charset rule.»
replace-with: HTML is parsed with scripting disabled; in every document a page whose bytes are not UTF-8 is decoded lossily and never panics, and a stylesheet is decoded from its bytes by its BOM and `@charset` rule — an engine change for every document;
@leaf_edit n=2 leaf=.claude/rules/security.md line=16 class=omission
leaf-text: invalid CSS declarations are dropped, not errors;
source: dec-oyp6kwfc «decision/presentational-hints-typed-parse · Presentational hint attributes read by a typed parse — The font element's face and size, valign, cellspacing and cellpadding are read as presentational hints by a typed parse, and a value that does not parse is ignored.»
replace-with: invalid CSS declarations are dropped, not errors; the `font` element's `face` and `size`, `valign`, `cellspacing` and `cellpadding` are read as presentational hints by a typed parse, a value that does not parse ignored — an engine change for every document;
