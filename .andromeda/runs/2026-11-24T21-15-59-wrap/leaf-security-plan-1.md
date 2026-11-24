@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=56
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=7 class=omission
leaf-text: JavaScript run by `blitz-vibey-script`, and CLI arguments of the examples.
source: dec-5vvljfpf «decision/os-drag-and-drop-untrusted · OS drag-and-drop is an untrusted input source — The local desktop's drag-and-drop is an untrusted input source to the engine and to a Dioxus app's handlers»
replace-with: JavaScript run by `blitz-vibey-script`, CLI arguments of the examples, and the local desktop's drag-and-drop — an untrusted input source to the engine and to a Dioxus app's handlers, carrying strings and file paths chosen by the user's OS session.
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=15 class=omission
leaf-text: the WPT net provider joins request paths onto the WPT base.
source: dec-fjqcv7ai «decision/dropped-file-read-unrestricted · Dropped-file reader without path restriction — The Dioxus native bridge's dropped-file reader reads a dropped file's metadata and bytes through std::fs with no path restriction.»
replace-with: the WPT net provider joins request paths onto the WPT base; the Dioxus native bridge's dropped-file reader reads a dropped file's metadata and bytes through std::fs with no path restriction (upstream's design, ratified).
