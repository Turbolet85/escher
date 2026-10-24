# Classified — a11y-plan · screen-reader-support.b1

## Answers
screen-reader-support#1.1 · no · cites packages/blitz-dom/src/accessibility.rs:242-246, an engine file the session leaves unchanged; no TextRun rule moved
screen-reader-support#1.2 · no · the aria-label and aria-labelledby naming cites accessibility.rs, accessibility_names.rs and accessibility_label.rs, none reached; no naming rule or count moved
screen-reader-support#1.3 · no · the label association cites accessibility.rs, document.rs, accessibility_names.rs and accessibility_label.rs, none reached; no association rule moved
screen-reader-support#1.4 · no · the select option claim cites accessibility.rs, document.rs, the snapshot files and stand_act_select.rs, none reached; the session's select runner change is on select_interaction, a §5 keys row, not this option claim
screen-reader-support#2.1 · no · author_id and the stand control tables cite dioxus_document.rs, stand_accessibility_ids.rs and common/mod.rs, none reached; no stand table or driver source changed
screen-reader-support#3.1 · yes · stale · stand_snapshot_parts.rs moved two items out and gained 23 lines, so its cited lines 59-181 and 220-281 shift; the caret claim's test password_caret_click now also runs in stand-repeat, behaviour unmoved
screen-reader-support#4.1 · no · the actionable-key rule cites actionable.rs, stand_actionable_keys.rs and stand_non_lean_tasks.rs, none reached; no key, role or count moved
screen-reader-support#5.1 · no · the CRUD list and mode-button state claims cite accessibility_active_descendant_unread.rs and stand files, none reached; the new stand-repeat runner is not a fact this row states, and its unread focus is unmoved
screen-reader-support#6.1 · no · the element tag-name claim cites accessibility.rs:241, unreached; no tag or tree rule moved
screen-reader-support#6.2 · no · the Window root and tree focus claim cites accessibility.rs:13 and 108-116, unreached; the engine tree is untouched
screen-reader-support#6.3 · no · the role-mapping test goal cites accessibility_roles.rs, unreached and unchanged
screen-reader-support#7.1 · no · the accesskit_xplat routing claim cites packages/accesskit_xplat/src/lib.rs, unreached; the session touches no platform adapter
screen-reader-support#8.1 · no · the adapter creation claim cites blitz-shell's accessibility.rs, unreached; no shell code changed
screen-reader-support#8.2 · no · the accessibility feature wiring cites packages/blitz/Cargo.toml and two Dioxus lib.rs files, none reached; no manifest changed
screen-reader-support#9.1 · no · the lang and xml:lang claim cites lang_attribute.rs, unreached; it names no direction reader
screen-reader-support#9.2 · no · the default stylesheet's dir bidi-isolation mapping cites default.css, unreached; it names the stylesheet, not how escher's readers treat dir=rtl or writing-mode, so the text_extent touch does not reach it
screen-reader-support#9.3 · no · the html lang fixture count cites examples/assets pages, unreached; the session adds no such fixture
screen-reader-support#9.4 · no · the lang="en" page claim cites examples/assets and examples/preact pages, unreached and unchanged
screen-reader-support#10.1 · no · the lang absence in the other s02 fixtures is a search over pages none reached and the session does not add to
screen-reader-support#11.1 · no · the sr-only tooltip cites examples/assets/github_profile_reduced2.html, unreached
screen-reader-support#11.2 · no · the google aria-label claim cites examples/assets/google.html, unreached
screen-reader-support#11.3 · no · the google aria-hidden and aria-atomic claim cites examples/assets/google.html, unreached
screen-reader-support#11.4 · no · the servo-new aria-hidden claim cites examples/assets/servo-new.html, unreached
screen-reader-support#11.5 · no · the servo.html alt claim cites examples/assets/servo.html, unreached
screen-reader-support#11.6 · no · the servo_header_reduced.html missing-alt claim cites examples/assets/servo_header_reduced.html, unreached
