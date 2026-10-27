# Classified — a11y-plan · a11y-scope-summary.b1

## Answers
a11y-scope-summary#1.1 · no · the workspace manifest is not reached and the session took no dependency, so the accesskit and accesskit_xplat member lines hold
a11y-scope-summary#1.2 · no · accesskit_xplat is not reached, and the platform adapter and its windowed AT witness residual are untouched
a11y-scope-summary#1.3 · no · lib.rs, accessibility.rs and the blitz-dom manifest are not reached, and the report shows no line of the accessibility builder changed
a11y-scope-summary#1.4 · no · custom_widget.rs is not reached
a11y-scope-summary#1.5 · no · blitz-html's manifest is not reached and no feature it names moved
a11y-scope-summary#1.6 · no · window.rs's only shift is +2 at 193-194 and its View::poll accessibility refresh is re-read unmoved, so the state at 111-113 holds; Cargo.toml and accessibility.rs are not reached
a11y-scope-summary#1.7 · no · blitz's manifest is not reached and its default accessibility feature is unchanged
a11y-scope-summary#1.8 · no · blitz-traits node_id.rs is not reached and NodeId is untouched
a11y-scope-summary#1.9 · yes · stale · the upstream take moved from f9cbb2756 to ebacd68bd (eleven taken, sixteen left out), so this entity's re-read at f9cbb2756 on line 18 is stale; no dependency moved and the stand's graph still holds neither platform adapter
a11y-scope-summary#1.10 · no · examples/seven_guis/src/stand.rs and the cells and load-page checks are not reached, and the stand run (205 · 0 · 7) and the load page's open §5 row are unmoved
a11y-scope-summary#1.11 · no · apps/browser/Cargo.toml is not reached, so the browser's accessibility feature stays out of its default set
a11y-scope-summary#1.12 · no · tests/blitz-tests/Cargo.toml is not reached and no manifest line moved; accesskit stays a dev-dependency
a11y-scope-summary#1.13 · no · examples/assets/input.html is not reached, and the fixture's focusable divs and text input are unchanged
a11y-scope-summary#1.14 · no · docs/capabilities.md is not reached, and the a11y leg's fourteen files and the named limits are unmoved
a11y-scope-summary#2.1 · no · no examples/*.rs file is reached, so the 18-file search stands
a11y-scope-summary#2.2 · no · the session adds no accessibility-tree, ARIA or screen-reader code (builder, accessibility_tree override and shell refresh unmoved), so the absence stands
a11y-scope-summary#2.3 · no · the session adds no accessibility handling, and the shell's accessibility refresh in View::poll is re-read unmoved, so the absence stands
a11y-scope-summary#2.4 · no · the 12 listed wpt-runner files are not reached, and the session's focus-related change (the IME enable) sits in blitz-dom construct.rs outside them
