@leaf_group group=masters
@leaf_read leaf=.claude/docs/gotchas.md lines=68
last: - Decisions and rationale: `doc:architecture` (the records s
@leaf_read leaf=CLAUDE.md lines=166
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=112 class=stale
leaf-text: `press --key [--shift]`
source: ctr-lshli5ex «contract/escher-session/press-key-shift-ctrl · escher-session command line · press --key [--shift] [--ctrl] — The verb press takes --key and the optional bare flags --shift and --ctrl.»
replace-with: `press --key [--shift] [--ctrl]`
@leaf_edit n=2 leaf=CLAUDE.md line=112 class=omission
leaf-text: `lint` · `graph` · `actions` · `layout --id`
source: ctr-pta2cbd4 «contract/escher-session/graph · escher-session command line · graph — The verb graph takes an optional --part beside --session.»
replace-with: `lint` · `graph [--part]` · `actions` · `layout --id`
@leaf_edit n=3 leaf=CLAUDE.md line=122 class=stale
leaf-text: answers it whole and unpaged on every carrier and in a kept check — ids, names, closed flag words and booleans, never a value
source: ctr-rzq3uzv4 «contract/graph-verb/unpaged · The graph verb · paging — graph answers whole when no part is asked and the answer is within the 10,000-byte snapshot text budget, else in parts of whole nodes within it;»
replace-with: answers it on every carrier and in a kept check, whole when no `part` is asked and it is within the 10,000-byte snapshot budget, else in parts of whole nodes (an optional `part`, 1 to 1000, the states in part 1 alone) — ids, names, closed flag words, booleans and a part's part counts, never a value
