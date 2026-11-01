# Classified — a11y-plan · a11y-scope-summary.b1

## Answers
a11y-scope-summary#1.1 · no · the workspace manifests and lock are byte-identical, so the accesskit 0.25 and workspace member lines stand
a11y-scope-summary#1.2 · no · the accesskit_xplat package manifest is untouched; no manifest or lock moved
a11y-scope-summary#1.3 · yes · stale · the orphan taffy comment removed from the reached blitz-dom/src/lib.rs may move the cited lines 86-87; the tree builder itself holds
a11y-scope-summary#1.4 · yes · stale · document.rs gained 290 lines from line 45 on, so the cited 365-366 and 2436-2438 move, and render.rs gained 13 ranges from line 14 on, so the cited 248-249 move
a11y-scope-summary#1.5 · no · the commented-out TODO hook stays; the CustomWidget attribute replay changes set_custom_widget and the contract row alone
a11y-scope-summary#1.6 · no · blitz-html's accessibility forward in its manifest is untouched; no manifest moved
a11y-scope-summary#1.7 · no · blitz-shell's manifest and its window and accessibility sources are not reached
a11y-scope-summary#1.8 · no · blitz's manifest is untouched; no manifest moved
a11y-scope-summary#1.9 · no · node_id.rs is not reached
a11y-scope-summary#1.10 · no · manifests and lock are byte-identical so the feature reach holds; dioxus_document.rs moves only citations past 235-239, which this entity does not cite
a11y-scope-summary#1.11 · no · the cited stand, keyboard, contrast, Cells and load-page files hold no edit, every row stands and the load page's lint stays 0
a11y-scope-summary#1.12 · no · apps/browser's manifest is not reached
a11y-scope-summary#1.13 · no · tests/blitz-tests/Cargo.toml is untouched; no manifest moved
a11y-scope-summary#1.14 · no · examples/assets/input.html is not reached
a11y-scope-summary#1.15 · no · docs/capabilities.md is not reached, and its named limits and the a11y leg's fourteen files stand
a11y-scope-summary#2.1 · no · the examples/*.rs files are not reached and seven_guis has no diff
a11y-scope-summary#2.2 · no · the report names no accessibility, ARIA or screen-reader change in the reached files, and accessibility.rs holds no change
a11y-scope-summary#2.3 · no · the report names no edit to accessibility code; seven_guis and escher-driver hold no diff
a11y-scope-summary#2.4 · yes · stale · wpt/runner/timeout-quarantine.txt is a reached merge surface, and the sweep's only hits are a quarantine reason and a quarantined test path, which the merge can move
