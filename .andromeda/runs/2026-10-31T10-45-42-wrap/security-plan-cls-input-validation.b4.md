# Classified — security-plan · input-validation.b4

## Answers
input-validation#1.67 · yes · contradict · its stated limit that a multiple select and an optgroup's options are not modelled no longer holds, since both are built (F3, F4) and type works on a multiple select (F6), and its test counts move to 18 and 10
input-validation#1.68 · yes · stale · every other element's value reads as before, yet a multiple select's snapshot value now reads its first chosen option in tree order, and snapshot.rs is reached
input-validation#1.69 · no · its rule lives in layout/replaced.rs, which the session did not reach, and no fact concerns aspect ratios
input-validation#1.70 · no · its rule lives in layout/inline/layout.rs, which the session did not reach, and no fact concerns inline-box heights
input-validation#1.71 · yes · add-to · F10 joins it, since the new public readers go through get_node and a stale id reads none, as this reader does
input-validation#1.72 · yes · add-to · F1 joins it, since a transform with a zero or non-finite determinant now takes no hit, the same non-finite transformed-bounds rule this row states
input-validation#1.73 · no · its rule lives in blitz-paint's gradient.rs, which the session did not reach, and blitz-paint shows no diff
input-validation#1.74 · no · its rule lives in blitz-paint's kurbo_css/css_box.rs, which the session did not reach, and blitz-paint shows no diff
input-validation#1.75 · no · its rule lives in blitz-paint's render.rs, which the session did not reach, and blitz-paint shows no diff
input-validation#1.76 · no · scrolling.rs is reached, but the re-read shows scroll_into_view is not edited, so the clamp and missing-node claims stand, and visible_region, which moved, is not named in this row
input-validation#1.77 · yes · stale · form.rs is reached whole and its method and enctype parse at :80-111 and :373-382 is not re-read, so those lines can have moved
input-validation#1.78 · yes · add-to · option_disabled is now the one rule readings and the form share, under a group too, and a multiple select's chosen set submits as form entries (F5)
input-validation#1.79 · yes · stale · form.rs is reached whole and the CRLF normalization at :427-464 is not re-read, so its lines can have moved
input-validation#1.80 · yes · stale · pointer.rs is reached whole and the accept claim at :955-957 sits beside the :959-962 lines the report lists as touched, so it can have moved
input-validation#1.81 · no · its rule lives in dioxus-native-dom's events.rs, which the session did not reach, and valid still returns true
input-validation#1.82 · yes · stale · dioxus_document.rs is reached whole and the data-dioxus-id parse at :48-55 is not re-read, so its lines can have moved
input-validation#1.83 · no · its rule lives in mutation_writer.rs, which the session did not reach, and the set_inner_html call is not part of the multiple-select recording
input-validation#1.84 · no · its rule lives in mutation_writer.rs, which the session did not reach, and the ignored value types are unchanged
input-validation#1.85 · yes · add-to · a multiple write or removal on this path now records the select, and a removal settles it to one chosen option (mutator.rs), which this row must carry
input-validation#1.86 · no · its rule lives in blitz-vibey-script's bootstrap/fetch.js, which the session did not reach
input-validation#1.87 · no · its rule lives in blitz-vibey-script's bootstrap/fetch.js, which the session did not reach
input-validation#1.88 · no · its rules live in blitz-vibey-script's dom/document.rs and selectors.rs, which the session did not reach
input-validation#1.89 · no · its rule lives in blitz-vibey-script's dom/element/scrolling.rs, which the session did not reach
input-validation#1.90 · no · its rule lives in blitz-vibey-script's dom/element/attributes.rs, which the session did not reach
input-validation#1.91 · no · its rule lives in blitz-vibey-script's runtime/timers.rs, which the session did not reach
