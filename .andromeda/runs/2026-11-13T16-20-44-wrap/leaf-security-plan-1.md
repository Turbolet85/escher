@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=56
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=13 class=omission
leaf-text: stale iframe responses discarded by request id;
source: dec-xetdufyi «decision/untrusted-bytes-decoded-without-panic · Page and stylesheet bytes decoded without panic — A page whose bytes are not UTF-8 is decoded lossily and never panics, and a stylesheet is decoded from its bytes by its BOM and @charset rule.»
replace-with: stale iframe responses discarded by request id; in every document, remote pages included, a page whose bytes are not UTF-8 is decoded lossily and never panics, and a stylesheet is decoded from its bytes by its BOM and `@charset` rule — upstream's behaviour, an engine change for every document, ratified;
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=13 class=omission
leaf-text: snapshotting a never-styled node is skipped (Stylo would panic);
source: dec-oyp6kwfc «decision/presentational-hints-typed-parse · Presentational hint attributes read by a typed parse — The font element's face and size, valign, cellspacing and cellpadding are read as presentational hints by a typed parse, and a value that does not parse is ignored.»
replace-with: snapshotting a never-styled node is skipped (Stylo would panic); in every document, remote pages included, the `font` element's `face` and `size`, `valign`, `cellspacing` and `cellpadding` are read as presentational hints by a typed parse, a value that does not parse ignored — upstream's behaviour, an engine change for every document, ratified;
