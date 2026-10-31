# Classified — a11y-plan · a11y-scope-summary.b1

## Answers
a11y-scope-summary#1.1 · no · the root Cargo.toml is not reached and no manifest moved, so accesskit "0.25" and the workspace members hold
a11y-scope-summary#1.2 · no · packages/accesskit_xplat is unedited and outside the change
a11y-scope-summary#1.3 · no · build_accessibility_tree stays at accessibility.rs:10-124, above the insert at 238-242, and the blitz-dom feature gates are unmoved
a11y-scope-summary#1.4 · no · custom_widget.rs is not reached and its TODO hook is untouched
a11y-scope-summary#1.5 · no · the blitz-html manifest is not reached and no manifest moved
a11y-scope-summary#1.6 · no · blitz-shell is outside the change, so its default feature and AccessibilityState stand
a11y-scope-summary#1.7 · no · the blitz manifest is not reached and no manifest moved
a11y-scope-summary#1.8 · no · blitz-traits node_id.rs is not reached
a11y-scope-summary#1.9 · no · no manifest or Cargo.lock is reached and NodeState gained no field, so the feature edge, the accessibility modules and the accesskit graph read as before
a11y-scope-summary#1.10 · yes · stale · the stand_contrast.rs citation 670-727 moves because lines are added from line 12 on (STATE_ROWS 18 → 20)
a11y-scope-summary#1.11 · no · apps/browser is not reached and its manifest did not move
a11y-scope-summary#1.12 · no · tests/blitz-tests/Cargo.toml is not reached and the new test files add no dependency
a11y-scope-summary#1.13 · no · no file under examples/assets was edited
a11y-scope-summary#1.14 · yes · contradict · the session paints the open list, opens it on Enter and Space and reads a multiple select's size, so the named limits no popup painted, Enter and Space opening nothing and size unread are retired
a11y-scope-summary#2.1 · no · the reached example files sit under examples/seven_guis/tests, outside the examples/*.rs glob the search covered
a11y-scope-summary#2.2 · yes · stale · the session adds accessibility-tree reads (expanded, select_list_bounds) in reached files, so the absence over the listed s06 files can have moved
a11y-scope-summary#2.3 · yes · stale · the session adds accessibility reads in reached files, so the aria|accessib absence over the listed s07 files can have moved
a11y-scope-summary#2.4 · yes · stale · the session adds keyboard and focus handling in reached files (press_open_list_key, focus staying on the select), so the absence over the listed s13 files can have moved
