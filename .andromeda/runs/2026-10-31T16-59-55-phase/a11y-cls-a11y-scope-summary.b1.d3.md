# Classified — a11y · a11y-scope-summary.b1

## Answers
a11y-scope-summary#1.1 · no · the workspace AccessKit version and member declarations; the chunk changes no dependency and no rule over them binds
a11y-scope-summary#1.2 · no · the accesskit_xplat platform adapter crate; Not needed, the chunk adds no adapter
a11y-scope-summary#1.3 · yes · BaseDocument::build_accessibility_tree in accessibility.rs (reached whole) builds the tree that carries expanded, the option hidden-content carve-out and selected
a11y-scope-summary#1.4 · no · the custom-widget tree hook in custom_widget.rs, which is not reached; the select is a native node
a11y-scope-summary#1.5 · no · blitz-html feature forwarding; no reached file and no rule the chunk meets
a11y-scope-summary#1.6 · no · blitz-shell's default feature and adapter ownership; the shell holds no list and the chunk adds no adapter
a11y-scope-summary#1.7 · no · blitz feature forwarding only
a11y-scope-summary#1.8 · no · NodeId::as_u64 interop doc comment; the chunk states no id rule
a11y-scope-summary#1.9 · no · the dioxus crates' accessibility feature reach and the stand's lack of a platform adapter; the chunk adds no feature or adapter
a11y-scope-summary#1.10 · yes · the load page's lint 0 is a value the overlaps edit in lint.rs must keep, and the Cells and load tab orders and contrast rows sit in the stand_keyboard and stand_contrast tables the chunk edits
a11y-scope-summary#1.11 · no · browser app feature default; the app is not reached
a11y-scope-summary#1.12 · no · blitz-tests feature and dev-dependency declarations; the chunk adds no test file or dependency
a11y-scope-summary#1.13 · no · input fixture markup; the chunk edits no fixture
a11y-scope-summary#1.14 · yes · docs/capabilities.md names the accessibility limits (no popup painted, Enter and Space opening nothing on a single select, a multiple select's size unread) that this chunk closes and must restate by hand
a11y-scope-summary#2.1 · no · absence of ARIA in examples/*.rs; the chunk adds no example markup
a11y-scope-summary#2.2 · no · absence of accessibility tree code in the s06 files; none of them is reached
a11y-scope-summary#2.3 · no · absence of accessibility handling in the s07 files; none of them is reached
a11y-scope-summary#2.4 · no · absence of ARIA, roles or keyboard handling in the s13 wpt runner files; none of them is reached
