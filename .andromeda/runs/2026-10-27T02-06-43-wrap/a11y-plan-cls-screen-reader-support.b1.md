# Classified — a11y-plan · screen-reader-support.b1

## Answers
screen-reader-support#1.1 · no · its rule lives in blitz-dom's accessibility.rs, a file under packages/blitz-dom/ that the session did not change
screen-reader-support#1.2 · no · the aria-label and aria-labelledby naming lives in blitz-dom's accessibility.rs and the accessibility_names and accessibility_label checks, none reached and nothing under packages/blitz-dom/ changed
screen-reader-support#1.3 · no · the label binding lives in blitz-dom's accessibility.rs and document.rs, files under packages/blitz-dom/ that the session did not change
screen-reader-support#1.4 · no · the select's option nodes live in blitz-dom's accessibility.rs, dioxus-native-dom's snapshot.rs and the stand select checks, none reached
screen-reader-support#2.1 · no · the author_id rule lives in dioxus-native-dom's dioxus_document.rs and the stand_accessibility_ids check, neither reached, and the report's Untouched §7 line names author_id
screen-reader-support#3.1 · no · the snapshot model lives in dioxus-native-dom's snapshot modules, escher-driver's execute.rs, schema.rs and mcp/result.rs, none reached, and the reached mcp_tests.rs changed only the tools/list listing test, not the result path
screen-reader-support#4.1 · no · the actionable-key rule lives in dioxus-native-dom's actionable.rs and the stand_actionable_keys and stand_non_lean_tasks checks, none reached
screen-reader-support#4.2 · no · the load page's actionable-key reading lives in the stand's stand.rs and stand_load_page.rs, neither reached
screen-reader-support#5.1 · no · the CRUD list and flight booker ARIA states live in the task files and the stand_snapshot_state and accessibility_active_descendant_unread checks, none reached
screen-reader-support#6.1 · no · the element tag name is set in blitz-dom's accessibility.rs, a file under packages/blitz-dom/ that the session did not change
screen-reader-support#6.2 · no · the Window root and tree focus live in blitz-dom's accessibility.rs, a file under packages/blitz-dom/ that the session did not change
screen-reader-support#6.3 · no · the role-mapping test lives in accessibility_roles.rs, which the session did not reach
screen-reader-support#7.1 · no · the adapter's request routing lives in accesskit_xplat's lib.rs, which the session did not reach
screen-reader-support#8.1 · no · the adapter's creation lives in blitz-shell's accessibility.rs, which the session did not reach
screen-reader-support#8.2 · no · the accessibility feature gating lives in blitz's Cargo.toml and the Dioxus crates' lib.rs files, none reached, and no Cargo manifest changed
screen-reader-support#9.1 · no · the lang and xml:lang reading is checked in lang_attribute.rs, which the session did not reach
screen-reader-support#9.2 · no · the dir-attribute bidi rules live in blitz-dom's assets/default.css, a file under packages/blitz-dom/ that the session did not change
screen-reader-support#9.3 · no · the eleven fixture lang attributes live in examples/assets HTML files, which the session did not reach
screen-reader-support#9.4 · no · the lang="en" pages live in examples/assets/servo.html and the preact examples, none reached
screen-reader-support#10.1 · no · its absence search runs over the s02 fixture files, none of which the session reached
screen-reader-support#11.1 · no · the sr-only tooltip lives in the github_profile_reduced2 fixture, which the session did not reach
screen-reader-support#11.2 · no · the descriptive aria-labels live in the google fixture, which the session did not reach
screen-reader-support#11.3 · no · the aria-hidden and aria-atomic markup lives in the google fixture, which the session did not reach
screen-reader-support#11.4 · no · the burger's aria-hidden bars live in the servo-new fixture, which the session did not reach
screen-reader-support#11.5 · no · the image alt texts live in the servo fixture, which the session did not reach
