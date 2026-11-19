@leaf_group group=architecture+security-plan+test-plan+a11y-plan
@leaf_read leaf=.claude/docs/gotchas.md lines=69
last: - Decisions and rationale: `doc:architecture` (the records st
@leaf_read leaf=CLAUDE.md lines=167
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=.claude/docs/gotchas.md line=7 class=stale
leaf-text: parley a registry version (0.12)
source: dec-qvdsgthp «Parley 0.13 with fontique does text layout, editing and fonts, with skrifa for font metrics and wuff for WOFF and WOFF2.»
replace-with: parley a registry version (0.13)
@leaf_edit n=2 leaf=.claude/docs/gotchas.md line=7 class=stale
leaf-text: winit is exact `=0.31.0-beta.3`
source: dec-odccjf3r «winit pinned exactly to a beta — winit is pinned exactly to a beta version, 0.31.0-beta.4.»
replace-with: winit is exact `=0.31.0-beta.4`
@leaf_edit n=3 leaf=.claude/docs/gotchas.md line=41 class=stale
leaf-text: `gecko`-only on Stylo 0.22
source: dec-qy3gqtf5 «Servo Stylo as the CSS engine — Servo Stylo 0.24, with selectors and cssparser, does styling and selector matching.»
replace-with: `gecko`-only in Stylo
@leaf_edit n=4 leaf=CLAUDE.md line=9 class=stale
leaf-text: Stylo 0.22
source: dec-qy3gqtf5 «Servo Stylo as the CSS engine — Servo Stylo 0.24, with selectors and cssparser, does styling and selector matching.»
replace-with: Stylo 0.24
@leaf_edit n=5 leaf=CLAUDE.md line=9 class=stale
leaf-text: Parley 0.12
source: dec-qvdsgthp «Parley 0.13 with fontique does text layout, editing and fonts, with skrifa for font metrics and wuff for WOFF and WOFF2.»
replace-with: Parley 0.13
@leaf_edit n=6 leaf=CLAUDE.md line=9 class=stale
leaf-text: winit 0.31.0-beta.3
source: dec-odccjf3r «winit pinned exactly to a beta — winit is pinned exactly to a beta version, 0.31.0-beta.4.»
replace-with: winit 0.31.0-beta.4
@leaf_edit n=7 leaf=CLAUDE.md line=9 class=stale
leaf-text: a native-API engine library with no server and no database
source: abs-osrjkzxe «absence/served-network-port — A served network port in a default build: no TCP or UDP port any binary serves with its default features»
replace-with: a native-API engine library with no server in a default build (the opt-in devtools feature's CDP server listens on 127.0.0.1) and no database
@leaf_edit n=8 leaf=CLAUDE.md line=54 class=stale
leaf-text: The driver session's Unix socket is the one listener:
source: dec-kap6zusi «The driver session's Unix-domain socket is the one listener of every default build, serving the session lifecycle and driver calls and their answers.»
replace-with: The driver session's Unix socket is the one listener of every default build (the opt-in devtools feature's CDP server on 127.0.0.1 aside):
@leaf_edit n=9 leaf=CLAUDE.md line=91 class=stale
leaf-text: `packages/blitz-shell/src/window.rs` (`View::poll`)
source: dec-pzi6d5tp «On a redraw, after resolve, the shell hands the adapter a fresh tree built from the wrapping document when the document's changed set is non-empty, then clears the set; a poll refreshes nothing.»
replace-with: `packages/blitz-shell/src/window.rs` (`View::redraw`)
