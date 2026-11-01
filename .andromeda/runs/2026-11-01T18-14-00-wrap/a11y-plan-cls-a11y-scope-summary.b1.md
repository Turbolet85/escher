# Classified — a11y-plan · a11y-scope-summary.b1

## Answers
a11y-scope-summary#1.1 · no · the manifests and lock moved by no line, so the accesskit 0.25 and accesskit_xplat workspace membership at Cargo.toml:3, 57 and 142 holds
a11y-scope-summary#1.2 · no · packages/accesskit_xplat/Cargo.toml is not in the reached set and no manifest moved
a11y-scope-summary#1.3 · yes · stale · packages/blitz-dom/src/lib.rs is reached and the removed orphan taffy comment there may move the citation at lib.rs:86-87, which the report does not show still holds
a11y-scope-summary#1.4 · yes · stale · document.rs (merge surface, 290 lines added from line 45 on) and render.rs (13 ranges from line 14 on) move the citations at document.rs:365-366, document.rs:2436-2438 and render.rs:248-249
a11y-scope-summary#1.5 · yes · stale · packages/blitz-dom/src/node/custom_widget.rs is reached and the report does not show its TODO hook at lines 6 and 144-145 still in place after the CustomWidget attribute replay
a11y-scope-summary#1.6 · no · packages/blitz-html/Cargo.toml is unmoved and no manifest changed
a11y-scope-summary#1.7 · no · blitz-shell's manifest is unmoved and its window.rs and accessibility.rs are not reached
a11y-scope-summary#1.8 · no · packages/blitz/Cargo.toml is unmoved
a11y-scope-summary#1.9 · no · node_id.rs is not reached, only net.rs in blitz-traits is, and the claim concerns node_id.rs alone
a11y-scope-summary#1.10 · no · every manifest and the lock stay byte-identical, so the Dioxus accessibility feature path holds as the report's Untouched section records
a11y-scope-summary#1.11 · no · stand_keyboard.rs, stand_contrast.rs and stand_load_page.rs hold their stated rows, and examples/seven_guis has no diff
a11y-scope-summary#1.12 · no · apps/browser/Cargo.toml is unmoved
a11y-scope-summary#1.13 · no · tests/blitz-tests/Cargo.toml is unmoved, so its accessibility feature and accesskit dev-dependency lines hold
a11y-scope-summary#1.14 · no · examples/assets/input.html is not reached
a11y-scope-summary#1.15 · no · docs/capabilities.md is not reached and the session changes no tier, criterion or named limit it lists
a11y-scope-summary#2.1 · no · no example .rs file is reached and examples/seven_guis has no diff, so no ARIA, role or tabindex token enters the sweep
a11y-scope-summary#2.2 · no · the session's hunks carry no accesskit, aria-, role or screen-reader code and accessibility.rs holds no change
a11y-scope-summary#2.3 · no · the session's hunks carry no aria or accessibility code and accessibility.rs holds no change
a11y-scope-summary#2.4 · yes · stale · the session's focus-and-key fix and the quarantine list timeout-quarantine.txt, a merge surface, feed the sweep's hits, and the report does not show the wpt hits at test_runners/mod.rs:412 and :438 unchanged
