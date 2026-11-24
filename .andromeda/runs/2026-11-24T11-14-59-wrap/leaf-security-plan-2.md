@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=32
last: - 2026-10-15: A listener or env read that only a test makes 
@leaf_edit n=1 leaf=.claude/rules/security.md line=28 class=retired
leaf-text: save Taffy alone, which takes a git source — upstream DioxusLabs/taffy pinned at a full commit rev (a3997d55), no patch section, integrity resting on the rev as its lock entry has no checksum, still passing the lock, audit and MSRV legs; every other crate a crates.io registry version
source: dec-jlrj566g «The pins stay: Stylo 0.24, Taffy 0.16.0 from crates.io, Dioxus 0.7.3 and winit =0.31.0-beta.4.»
replace-with: Taffy 0.16.0 from crates.io, parley and every other crate a crates.io registry version
@leaf_edit n=2 leaf=.claude/rules/security.md line=12 class=omission
leaf-text: Remote HTML/CSS/images/fonts/sub-documents are untrusted:
source: dec-5vvljfpf «The local desktop's drag-and-drop is an untrusted input source to the engine and to a Dioxus app's handlers, carrying strings and file paths chosen by the user's OS session.»
replace-with: Remote HTML/CSS/images/fonts/sub-documents are untrusted, as is the local desktop's OS drag-and-drop, which carries strings and file paths chosen by the user's OS session to the engine and a Dioxus app's handlers (provisional):
@leaf_edit n=3 leaf=.claude/rules/security.md line=12 class=omission
leaf-text: on one re-focused the key still acts.
source: dec-fjqcv7ai «The Dioxus native bridge's dropped-file reader reads a dropped file's metadata and bytes through std::fs with no path restriction.»
replace-with: on one re-focused the key still acts. The Dioxus native bridge's dropped-file reader reads a dropped file's metadata and bytes through std::fs with no path restriction — upstream's design, ratified (provisional).
@leaf_stands n=1 leaf=.claude/rules/security.md line=26
because: Line 26 names no Taffy source or git pin — its only pin is the coupled winit pin, which dec-jlrj566g still holds at =0.31.0-beta.4 — so nothing the taffy-layout row lost bears on it.
