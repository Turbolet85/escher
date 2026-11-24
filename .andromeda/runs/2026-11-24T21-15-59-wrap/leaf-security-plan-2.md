@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=32
last: - 2026-10-15: A listener or env read that only a test makes —
@leaf_edit n=1 leaf=.claude/rules/security.md line=12 class=stale
leaf-text: upstream's design, ratified (provisional).
source: dec-fjqcv7ai «decision/dropped-file-read-unrestricted · Dropped-file reader without path restriction — The Dioxus native bridge's dropped-file reader reads a dropped file's metadata and bytes through std::fs with no path restriction.»
replace-with: upstream's design, ratified.
