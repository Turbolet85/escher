# Classified — security-plan · input-validation.b3

## Answers
input-validation#1.51 · yes · add-to · F8 and F9: `size` on a `multiple` select joins this numeric row, read by `size_rows` through `parse_unsigned_integer` and capped at 65534 like `rowspan`, and a hostile `size` is bounded by the F9 test; the `rowspan` clamp is unchanged.
input-validation#1.52 · no · the `<ol start>` parse in layout/construct.rs is not in the reached files, and the session's `size` cap sits in the rowspan row, not here.
input-validation#1.53 · no · `attr_parsed` and element.rs:502-505 are not in the reached files, and the session does not change the `tabindex` or `disabled` readers that use it.
input-validation#1.54 · yes · add-to · F5: a disabled select opens no list and a disabled row changes nothing, so Enter and Space opening a single select must gate on this same disabled read; pointer.rs:333, 472, 832 and keyboard.rs:329-360 and snapshot.rs:169-172, 593-640 moved with the additions in those reached files.
input-validation#1.55 · yes · add-to · F10: an author's `open` steers nothing on a select (`:open` and `flush_open_state` unmoved, test-pinned), so this details and dialog read gains that exclusion; stylo.rs:510 sits below the reached stylo.rs additions and may have moved.
input-validation#1.56 · no · autofocus lives in mutator.rs:1143-1155, not a reached file, and the session does not touch focus on insert.
input-validation#1.57 · no · `dir` matching is pinned only by dir_attribute.rs, not a reached file, and the session does not touch it.
input-validation#1.58 · no · the canvas `src` parse is in mutator.rs:1451-1462, not a reached file, and the session does not touch canvas.
input-validation#1.59 · yes · stale · the id row cites snapshot.rs:96-115 and 130-199 and execute.rs:190-206, 394-439 and 517-534, all in reached files whose lines the change moved; its claims at lines 200 and 203 re-read unmoved.
input-validation#1.60 · yes · contradict · a label's click on a bound single select now focuses it and opens nothing, no longer re-dispatching (`a_labels_click_focuses_the_select_and_opens_nothing`), so the re-dispatch clause no longer holds; its accessibility.rs and document.rs citations also sit in reached files that moved.
input-validation#1.61 · yes · stale · accessibility.rs is a reached file edited at the single select's aria-expanded write, so its citations 89-102 and 237-268 may have moved; the description reads are not changed.
input-validation#1.62 · yes · contradict · a single select now writes `set_expanded(open)` from engine state and reads no `aria-expanded`, every other element keeps the read, so the row's universal read no longer holds; F10 adds that an author `aria-expanded` steers nothing on a select.
input-validation#1.63 · yes · stale · the builder (accessibility.rs:182-236, 374, 422-442) and snapshot.rs:63-74, 173-186, 223-235 are in reached files whose lines moved; the select option's `selected` read and the other state reads are unchanged.
input-validation#1.64 · yes · stale · accessibility.rs:199-224 is in a reached file the chunk edited, and the report does not show the checked-state read holding on the changed file.
input-validation#1.65 · yes · add-to · F5: an inert select opens no list, so the inert row's readers gain the open-list gate; accessibility.rs:283 and 287-300 are in a reached file and may have moved.
input-validation#1.66 · yes · stale · accessibility.rs:104-110 and 122 are in a reached file the chunk edited, and the report does not show the focus-clear citations holding on the changed file.
input-validation#1.67 · yes · stale · keyboard.rs:106-111, 156-173, 532-553, pointer.rs:462-466, 522-524, 725-753, 840-852 and snapshot.rs:263-265 sit below the reached files' moved additions (keyboard from line 34, pointer from line 220); the range rules are unchanged.
