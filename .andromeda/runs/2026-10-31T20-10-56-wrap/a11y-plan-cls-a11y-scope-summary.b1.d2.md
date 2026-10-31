# Classified — a11y-plan · a11y-scope-summary.b1

## Answers
a11y-scope-summary#1.1 · no · no manifest or lock moved and Cargo.toml and Cargo.lock are not reached, so the workspace accesskit and accesskit_xplat membership holds
a11y-scope-summary#1.2 · no · packages/accesskit_xplat is unedited, so its crate claim holds
a11y-scope-summary#1.3 · yes · add-to · F1 names the select list a new accessibility-scope entity, an engine overlay the tree reads; the build claim at lib.rs:86-87 and accessibility.rs:10-124 still holds, the edit sits past line 124
a11y-scope-summary#1.4 · no · custom_widget.rs is not reached and the session edits no custom-widget hook
a11y-scope-summary#1.5 · no · blitz-html's Cargo.toml is not reached and no manifest moved
a11y-scope-summary#1.6 · no · blitz-shell is outside the change and its window.rs and accessibility.rs are not reached
a11y-scope-summary#1.7 · no · blitz's Cargo.toml is not reached and no manifest moved
a11y-scope-summary#1.8 · no · node_id.rs is not reached
a11y-scope-summary#1.9 · no · no manifest or lock moved, so the feature edges and the stand graph claims still hold
a11y-scope-summary#1.10 · yes · stale · stand_contrast.rs gained lines from line 12 on, so its cited range 670-727 moved
a11y-scope-summary#1.11 · no · apps/browser is not reached and its manifest is unmoved
a11y-scope-summary#1.12 · no · tests/blitz-tests/Cargo.toml is not reached and no manifest moved, so the feature and dev-dependency lines hold
a11y-scope-summary#1.13 · no · examples/assets was not edited
a11y-scope-summary#1.14 · yes · contradict · the session retires the named limits that no popup is painted, that Enter and Space open nothing on a single select, and that a multiple select's size is unread, all listed in the reach statement
a11y-scope-summary#2.1 · no · none of the 18 examples/*.rs files is reached, the change touches only examples/seven_guis/tests
a11y-scope-summary#2.2 · yes · stale · the session reaches accessibility-tree code (accessibility.rs, snapshot.rs, lib.rs) and the 17 s06 files are not shown, so the absence cannot be shown to hold
a11y-scope-summary#2.3 · yes · stale · the session reaches accessibility-handling files (accessibility.rs, snapshot.rs) and the 8 s07 files are not shown, so the absence cannot be shown to hold
a11y-scope-summary#2.4 · no · its hits are in wpt/runner/src/test_runners/mod.rs, which is not reached
