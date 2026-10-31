# Classified — a11y · a11y-scope-summary.b1

## Answers
a11y-scope-summary#1.1 · no · a manifest pin the chunk leaves unchanged, since it bumps no dependency and changes no manifest
a11y-scope-summary#1.2 · no · the adapter crate is named only, and the platform adapter is out of this version (Not needed)
a11y-scope-summary#1.3 · yes · build_accessibility_tree at accessibility.rs:10-117 is where the chunk's native checkbox and radio toggled write lands, so the write stays inside the accessibility-feature build
a11y-scope-summary#1.4 · no · the custom-widget a11y hook is a TODO in custom_widget.rs, which the chunk does not reach, and custom-paint is a 0.4.0 gap the entry does not take
a11y-scope-summary#1.5 · no · blitz-html's feature forward is manifest wiring the chunk does not change, and blitz-tests enables blitz-dom's accessibility directly
a11y-scope-summary#1.6 · yes · each View's AccessibilityState wrapping the Adapter is where the shell's tree handoff, which the chunk's needs name, passes
a11y-scope-summary#1.7 · no · blitz's default-on accessibility forward is manifest wiring the chunk does not change, and the stand's graph holds no accesskit_xplat, so the forward is off on the windowed stand
a11y-scope-summary#1.8 · yes · NodeId::as_u64 is the integer AccessKit keys nodes by, which the node-for-node tree equality and the tree-to-engine focus agreement depend on
a11y-scope-summary#1.9 · yes · the windowed stand's boot the new redraw check sits beside, and the headless stable-id tree and refresh the fixup and checkedness changes reach through changed_nodes; the Dioxus feature wiring does not bind
a11y-scope-summary#1.10 · yes · the served pages' open gap is debt 6, Cells and the load page gaining Tab-order, keyboard-activation and contrast rows, and the load page's lint 0 must hold through the overlaps edit
a11y-scope-summary#1.11 · no · the browser app's accessibility feature is reached by no chunk file or named check
a11y-scope-summary#1.12 · no · standing S2 · an a11y assertion is a Rust test in blitz-tests over the AccessKit tree
a11y-scope-summary#1.13 · no · the input fixture's tabindex 0 and -1 divs are read by no reached or named check, and the Tab-order rules bind through stand_keyboard and inert_subtree
a11y-scope-summary#1.14 · yes · the reach the chunk re-asserts (SC 2.1.1, 1.4.3, 2.4.3) and the named limits its fixes or pins move (focus fixup, select multiple and optgroup, native checked state) are published here and edited by hand
a11y-scope-summary#2.1 · no · the search finds no ARIA or tabindex in the examples, and the chunk adds none and reads no example's attributes
a11y-scope-summary#2.2 · no · the s06 files hold no accessibility code, and the chunk adds none to them
a11y-scope-summary#2.3 · no · the s07 files hold no accessibility handling, and the chunk adds none to them
a11y-scope-summary#2.4 · no · the s13 hits are a WPT quarantine reason and test path, which no reached file or named check touches
