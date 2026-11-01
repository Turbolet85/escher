# Classified — security-plan · input-validation.b4

## Answers
input-validation#1.71 · yes · stale · mutator.rs is a merge surface and this entity cites its lines 349–355, 380–382 and 515–522 (the value re-seed and re-sanitize writes)
input-validation#1.72 · no · the keyboard activation and click gates live in events/keyboard.rs, events/pointer.rs and events/mod.rs, none reached by the session
input-validation#1.73 · yes · stale · the bootstrap runtime carrying the no-synthesized-change claim is a merge surface, with url_search_params.js added as its first script
input-validation#1.74 · yes · stale · mutator.rs, node/element.rs and dioxus_document.rs (all merge surfaces) hold the cited selectedness, option-walk and select_value lines
input-validation#1.75 · yes · stale · document.rs is a merge surface and this entity cites its select_list field lines 365–366, 808–815, 905 and 1419
input-validation#1.76 · yes · stale · tests/blitz-tests/tests/select_popup.rs is reached with its row-3 skip removed, so the cited test line ranges 249–273 and 294–361 may have moved
input-validation#1.77 · yes · stale · document.rs (merge surface, lines 902–918) and select_popup.rs (reached, lines 401–426, 779–799, 1101–1118) are cited
input-validation#1.78 · yes · stale · select_popup.rs is reached with its row-3 skip removed, so the cited hostile-markup test lines 866–907 and 934–971 may have moved
input-validation#1.79 · no · its multiple-select keys and clicks live in events/keyboard.rs, events/pointer.rs and the select tests, none reached by the session
input-validation#1.80 · yes · stale · mutator.rs (merge surface) is cited at lines 1325–1329 for the optgroup arm
input-validation#1.81 · yes · stale · dioxus_document.rs (merge surface) is cited at lines 31–45 for the multiple-select set entries
input-validation#1.82 · no · the driver's type rests on escher-driver and execute.rs, neither reached by the session
input-validation#1.83 · yes · stale · document.rs (merge surface, lines 751–770) is cited for the select readers
input-validation#1.84 · yes · stale · document.rs (merge surface, lines 892–1075) and blitz-dom lib.rs (reached, re-exports at lines 106–109) are cited
input-validation#1.85 · yes · stale · render.rs (merge surface) is cited at lines 1017–1091 for the masked-input paint
input-validation#1.86 · no · aspect-ratio handling lives in layout/replaced.rs, which the session does not reach
input-validation#1.87 · no · the inline-box height clamp lives in layout/inline/layout.rs, which the session does not reach
input-validation#1.88 · no · text_extent.rs is not reached and the lint reader is unchanged
input-validation#1.89 · yes · stale · document.rs (merge surface, lines 2945–2949) is cited for visible_region's transformed-box reading
input-validation#1.90 · yes · stale · render.rs (merge surface, lines 412–427) is cited, and render_element's culling now stands on the scaled size (e42ee1f25)
input-validation#1.91 · yes · stale · render.rs (merge surface, lines 248–249) is cited for the paint_select_list call site
input-validation#1.92 · yes · stale · node/node.rs (merge surface, lines 1485–1490) is cited for the non-invertible transform hit test
input-validation#1.93 · no · gradient clamps live in blitz-paint/src/gradient.rs, which the session does not reach
input-validation#1.94 · no · border-radius scaling lives in blitz-paint/src/kurbo_css/css_box.rs, which the session does not reach
input-validation#1.95 · yes · stale · render.rs (merge surface, lines 311–315) is cited for the hidden-input skip
