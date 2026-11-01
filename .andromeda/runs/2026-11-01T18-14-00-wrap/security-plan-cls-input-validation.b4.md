# Classified — security-plan · input-validation.b4

## Answers
input-validation#1.71 · yes · stale · the re-seed and re-sanitize arms cited at mutator.rs:349-355, :380-382 and :515-522 sit in a merge surface the session reached whole
input-validation#1.72 · no · its Enter and Space paths cite only events/keyboard.rs, events/mod.rs and events/pointer.rs, none reached, and no touch names them
input-validation#1.73 · yes · stale · the blitz-vibey-script runtime whose no-change claim this row states has bootstrap.rs as a merge surface, and url_search_params.js now heads its script list
input-validation#1.74 · yes · stale · its citations into mutator.rs, element.rs and dioxus_document.rs (merge surfaces) move with the reached files
input-validation#1.75 · yes · stale · its citations into document.rs (:365-366, :808-815, :905, :1419), a merge surface, can move
input-validation#1.76 · yes · stale · its pins at tests/blitz-tests/tests/select_popup.rs:249-273 and :294-361 cite a file the session reached whole
input-validation#1.77 · yes · stale · document.rs:902-918 is a merge surface, and its select_popup.rs pins at :401-426, :779-799 and :1101-1118 cite a reached file
input-validation#1.78 · yes · stale · its pins at tests/blitz-tests/tests/select_popup.rs:866-907 and :934-971 cite a file the session reached whole
input-validation#1.79 · no · its cites (events/keyboard.rs, events/pointer.rs) and pinning tests sit in files the session did not reach, and no touch names a multiple select
input-validation#1.80 · yes · stale · its mutator.rs:1325-1329 citation is a merge surface the session reached whole
input-validation#1.81 · yes · stale · dioxus_document.rs:31-45 is a merge surface the session reached whole
input-validation#1.82 · no · its cites are escher-driver execute.rs and schema.rs, which hold no diff, and no touch names the driver's type
input-validation#1.83 · yes · stale · its document.rs:751-765 and :767-770 citations sit in a merge surface the session reached whole
input-validation#1.84 · yes · stale · document.rs:892-900, :1001-1023 and :1033-1075 sit in a merge surface the session reached whole
input-validation#1.85 · yes · stale · render.rs:1017-1091 (the password paint mask, draw_masked_input_text) is a merge surface the session reached whole
input-validation#1.86 · no · its cite layout/replaced.rs is not a reached file, and no touch names aspect ratios
input-validation#1.87 · no · its cite layout/inline/layout.rs:470-478 is not a reached file, and no touch names inline-box heights
input-validation#1.88 · no · its cite text_extent.rs:18-54 is not a reached file, and no touch names text extent
input-validation#1.89 · yes · stale · document.rs:2945-2949 is a merge surface the session reached whole
input-validation#1.90 · yes · stale · render_element's border box and culling now stand on the scaled size (e42ee1f25), and the culling at render.rs:412-427 is in a merge surface
input-validation#1.91 · yes · stale · render.rs:248-249, the paint_select_list call site, is a merge surface the session reached whole
input-validation#1.92 · yes · stale · node/node.rs:1485-1490 is a merge surface the session reached whole
input-validation#1.93 · no · its cite gradient.rs:497-504 and :521-535 is not a reached file, and no touch names gradient lengths
input-validation#1.94 · no · its cite kurbo_css/css_box.rs:57-75 is not a reached file, and the touched border-box reading in render.rs is not the radius rule
input-validation#1.95 · yes · stale · render.rs:311-315 is a merge surface the session reached whole
