@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=56
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=7 class=omission
leaf-text: and CLI arguments of the examples.
source: dec-5vvljfpf «The local desktop's drag-and-drop is an untrusted input source to the engine and to a Dioxus app's handlers, carrying strings and file paths chosen by the user's OS session.»
replace-with: CLI arguments of the examples, and the local desktop's drag-and-drop, an untrusted input source to the engine and to a Dioxus app's handlers carrying strings and file paths chosen by the user's OS session.
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=15 class=omission
leaf-text: the WPT net provider joins request paths onto the WPT base.
source: dec-fjqcv7ai «The Dioxus native bridge's dropped-file reader reads a dropped file's metadata and bytes through std::fs with no path restriction. · why: Upstream's design, ratified.»
replace-with: the WPT net provider joins request paths onto the WPT base; the Dioxus native bridge's dropped-file reader reads a dropped file's metadata and bytes through std::fs with no path restriction (upstream's design, ratified).
@leaf_stands n=1 leaf=.claude/docs/security-summary.md line=23
because: The line's virtual node reading `covered` by an element or a later-declared node of its canvas over its region's centre already says what ctr-lndu2awg, ctr-uj3teaik and ctr-26pvuvyx now say, the retired "only by an element" wording appearing nowhere in it.
