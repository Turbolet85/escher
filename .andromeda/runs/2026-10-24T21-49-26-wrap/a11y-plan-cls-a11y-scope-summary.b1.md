# Classified — a11y-plan · a11y-scope-summary.b1

## Answers
a11y-scope-summary#1.1 · no · no manifest changed, and the root Cargo.toml is not a reached file, so its accesskit and accesskit_xplat workspace lines still hold
a11y-scope-summary#1.2 · no · accesskit_xplat's package manifest is not a reached file and no crate source changed
a11y-scope-summary#1.3 · no · no engine or builder code changed, so build_accessibility_tree and the blitz-dom feature lines it cites still hold
a11y-scope-summary#1.4 · no · the custom-widget TODO sits in blitz-dom source the session did not change
a11y-scope-summary#1.5 · no · blitz-html's manifest is not a reached file, so its accessibility forwarding still holds
a11y-scope-summary#1.6 · no · blitz-shell's manifest and its View's adapter wrap are unchanged, and the tree equality the session re-asserted is a claim this entity does not make
a11y-scope-summary#1.7 · no · the blitz manifest is not a reached file, so its accessibility forwarding still holds
a11y-scope-summary#1.8 · no · node_id.rs is not a reached file and no Rust source changed
a11y-scope-summary#1.9 · yes · add-to · the session re-measured the stand graph (cargo tree probe 0 for accesskit_xplat and accesskit_winit, unmoved), a new re-read this entity carries beside its earlier ones
a11y-scope-summary#1.10 · no · the browser app's manifest is not a reached file, so its accessibility feature still sits outside the default set
a11y-scope-summary#1.11 · no · tests/blitz-tests/Cargo.toml is not a reached file, and the leg's added test targets add no feature or dev-dependency line
a11y-scope-summary#1.12 · no · examples/assets/input.html is not a reached file, and the fixture's markup, the only thing the entity states, is unchanged
a11y-scope-summary#2.1 · no · the session changed none of the 18 example files the search read, which are a slice count, not the a11y leg
a11y-scope-summary#2.2 · no · the session changed none of the 17 listed s06 files, which are a slice count, not the a11y leg
a11y-scope-summary#2.3 · no · the session changed none of the 8 s07 slice files, which are a slice count, not the a11y leg
a11y-scope-summary#2.4 · no · the session changed none of the 12 listed s13 wpt runner files the search read
