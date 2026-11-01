# Classified — a11y-plan · screen-reader-support.b1

## Answers
screen-reader-support#1.1 · no · the text-run label rule lives in packages/blitz-dom/src/accessibility.rs:269-273, which the change does not reach and which holds no change
screen-reader-support#1.2 · no · the aria-label and aria-labelledby naming rule lives in accessibility.rs:158-171 and the accessibility_names and accessibility_label tests, none reached
screen-reader-support#1.3 · yes · stale · the citation document.rs:695-733 and :3531-3545 sits past line 45 of the 290-line added span in the document.rs merge surface, so its digits move
screen-reader-support#1.4 · yes · stale · the citation document.rs:735-749 (select_value) sits past line 45 of the document.rs additions, so its digits move
screen-reader-support#1.5 · yes · stale · the citations select_popup.rs:561-610 and :624-647 sit past line 513, which the change turns into one harness.press per row, so their digits move
screen-reader-support#1.6 · no · the highlighted-row limit cites docs/capabilities.md and the snapshot, neither reached, and the change does not touch open-list row state
screen-reader-support#1.7 · no · the harness-only SC 2.1.1, 2.4.3 and 1.4.3 assertions still hold on the changed select_popup.rs, and its leg is unmoved
screen-reader-support#1.8 · no · the multiple-select bounds rest on the snapshot and stand_act_select.rs:320-361, neither reached, and no touch moves client rects
screen-reader-support#1.9 · yes · stale · dioxus_document.rs is a merge surface reached whole, and no report re-reads its lines 31-45 on the changed file
screen-reader-support#2.1 · yes · stale · the citation dioxus_document.rs:314-330 sits past the +5 hunk at 235-239, which moves citations past that block
screen-reader-support#3.1 · yes · stale · a claim that a hidden or disabled change takes effect now rests on the narrowed restyle path, so the reading it states has moved, although its rows hold green
screen-reader-support#3.2 · no · NodeState and NODE_FIELDS live in snapshot.rs, not reached, and no touch names an expanded key
screen-reader-support#3.3 · no · the open-list row screenshot rests on list painting, which holds no hunk, and stand_act_popup.rs is not reached
screen-reader-support#3.4 · no · the native checked reading lives in accessibility.rs:199-224 and accessibility_states.rs, neither reached
screen-reader-support#4.1 · no · the focusability it reads is re-read and unmoved, its listener mark is untouched, and the check still returns nothing on the stand
screen-reader-support#4.2 · no · cells-grid's aria_label and absent role rest on examples/seven_guis and residuals.md, neither reached
screen-reader-support#4.3 · no · the load page rests on stand.rs and stand_load_page.rs, unreached, and its stylesheet-landing reading holds; only a failed-load-waiting line would move
screen-reader-support#5.1 · no · the role and state attributes sit in crud.rs and flight_booker.rs, neither reached, and no touch moves them
screen-reader-support#6.1 · no · the tag-name reading lives in accessibility.rs:268, untouched
screen-reader-support#6.2 · no · the window root and tree focus rule live in accessibility.rs:13 and 104-123, untouched, and the focus fixup is unchanged
screen-reader-support#6.3 · no · the role-mapping test goal lives in accessibility_roles.rs, not reached
screen-reader-support#7.1 · no · accesskit_xplat holds no change
screen-reader-support#8.1 · no · the blitz-shell adapter lives in accessibility.rs, not reached
screen-reader-support#8.2 · no · feature wiring sits in packages/blitz/Cargo.toml and the Dioxus lib.rs files, and every manifest is byte-identical
screen-reader-support#9.1 · no · the lang rule lives in tests/blitz-tests/tests/lang_attribute.rs, not reached, and no touch names language or lang
