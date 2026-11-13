@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - Architecture: `doc:architecture` (the records store) · dir
@leaf_read leaf=.claude/docs/gotchas.md lines=68
last: - Decisions and rationale: `doc:architecture` (the records s
@leaf_read leaf=.claude/docs/services/blitz-dom.md lines=44
last: - `doc:architecture` (the records store) · `.claude/docs/sta
@leaf_read leaf=.claude/docs/services/blitz-paint.md lines=35
last: - `doc:architecture` (the records store) · `.claude/docs/ser
@leaf_read leaf=.claude/docs/services/blitz-shell.md lines=34
last: - `doc:architecture` (the records store) · `.claude/docs/ser
@leaf_read leaf=.claude/docs/services/blitz-traits.md lines=33
last: - `doc:architecture` (the records store) · `.claude/docs/ser
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=18 class=stale
leaf-text: In-repo crates are declared with `default-features = false`
source: dec-n67vjgmm «dioxus-native-dom, accesskit_xplat) are declared with `default-features = false`; the other in-repo crates without it.»
replace-with: The in-repo engine crates (blitz-dom, blitz-html, blitz-net, blitz-paint, blitz-vibey-script, blitz-shell, blitz-traits, stylo_taffy, dioxus-native, dioxus-native-dom, accesskit_xplat) are declared with `default-features = false`, the other in-repo crates without it
@leaf_edit n=2 leaf=.claude/docs/conventions.md line=25 class=omission
leaf-text: its `signal_registry` module ungated, so an app registers state whatever its features
source: dec-wdtb3kpq «its `signal_registry` and `reached` modules stay ungated.»
replace-with: its `signal_registry` and `reached` modules ungated, so an app registers state whatever its features and the reached record compiles on the wasm build that leaves `accessibility` off
@leaf_edit n=3 leaf=.claude/docs/conventions.md line=35 class=omission
leaf-text: `unsafe` blocks carry an `allow unsafe_code` or `// SAFETY:` comment.
source: dec-mxk72o5x «Upstream engine code holds unannotated blocks (a node's tree pointer, stylo data, the taffy conversion, layout); escher's own crates hold no `unsafe`.»
replace-with: `unsafe` blocks carry an `allow unsafe_code` or `// SAFETY:` comment; upstream engine code holds unannotated blocks (a node's tree pointer, stylo data, the taffy conversion, layout), and escher's own crates hold no `unsafe`.
@leaf_edit n=4 leaf=.claude/docs/conventions.md line=48 class=stale
leaf-text: `clippy::collapsible_if` allowed crate-wide in browser, blitz-dom, blitz-html, blitz-vibey-script
source: dec-ncdgwndo «`clippy::collapsible_if` is allowed crate-wide in five crates (browser, blitz-dom, blitz-html, blitz-paint, blitz-vibey-script)»
replace-with: `clippy::collapsible_if` allowed crate-wide in browser, blitz-dom, blitz-html, blitz-paint, blitz-vibey-script
@leaf_edit n=5 leaf=.claude/docs/services/blitz-dom.md line=15 class=tree
leaf-text: `escher-0.3.0/chunks/2026-10-31-select-popup/evidence/merge-surface.md`
source: dec-z4huwp4c «The select list is a direct engine change for every document, remote pages included.»
replace-with: `.andromeda/cache/chunks/2026-10-31-select-popup/evidence/merge-surface.md`
