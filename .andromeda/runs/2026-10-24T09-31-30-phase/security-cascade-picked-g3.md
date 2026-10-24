# security · picked accounting · group 3 of 4

## Picked
input-validation#1.79 · carried · `286874cc1` rewrites `packages/dioxus-native-dom/src/events.rs`, so the form-validity reading and its cite are re-read on the take
input-validation#1.80 · carried · `286874cc1` rewrites `dioxus_document.rs` and moves the `data-dioxus-id` write into `mutation_writer.rs`, so the `usize` parse is re-read on the take
input-validation#1.81 · carried · `286874cc1` rewrites `mutation_writer.rs` (+/− 647 lines), the `dangerous_inner_html` path's file
input-validation#1.82 · carried · `286874cc1` moves the writer from `AttributeValue` to `BorrowedAttributeValue`, the reading's own type
input-validation#1.83 · carried · the falsy-boolean removal sits in the rewritten `mutation_writer.rs`, with upstream adding an `Any(_) => false` arm to `is_falsy`, and `dioxus_falsy_boolean_attrs` is its reading
input-validation#1.101 · not in scope · `fuzzy.rs` is untouched by upstream's delta past `17de049b1`
input-validation#1.102 · carried · `f8723f495` edits `attr_test.rs`, moving the cited `checkLayout` lines and its node lookup
input-validation#1.103 · not in scope · `js_wrapper.rs` is untouched by upstream's delta
input-validation#1.104 · not in scope · `test_runners/mod.rs` is untouched by upstream's delta
input-validation#1.105 · not in scope · `net_provider.rs` is untouched by upstream's delta
input-validation#1.106 · not in scope · `harness_test.rs` is untouched by upstream's delta
input-validation#1.107 · not in scope · upstream's delta touches no `.github/` file
input-validation#1.109 · not in scope · `286874cc1` changes only the Enter key match at `cells.rs:391`, outside the cited validation lines, and the flight booker and circle drawer are untouched
