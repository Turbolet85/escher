# security-plan · picked accounting · group 3 of 4

## Picked
input-validation#1.61 · not touched · `accessibility.rs` is reached but the report's edits there are focus, native `Toggled`, `set_multiselectable` and the group role, with no description read and entry 4 at 0, so the row's claim holds
input-validation#1.62 · not touched · the report's edits to `accessibility.rs` and `snapshot.rs` read none of the three attributes and leave `enabled` alone, so the closed parses and the snapshot's reading hold
input-validation#1.63 · proposed · a native checkable now writes `Toggled` under a checkable role, so the measured absence and «nothing else writes `toggled`» no longer hold
input-validation#1.64 · proposed · `DocumentMutator::flush` now clears focus on an inert node, so the focus-fixup limit ends
input-validation#1.65 · not touched · the report's `change` arms are checkbox and radio alone and `text_range_no_change_event` pins the range's one `input`, its reached files adding no range line, so the row holds
input-validation#1.66 · proposed · `handle_click` gains an `option` arm and its checkbox and radio arms send `change`, so «any other element unchanged» moves
input-validation#1.67 · proposed · `multiple` and `optgroup` are built, an option's click is refused on a closed single select alone, and the row's counts move
input-validation#1.68 · not touched · `new-readers` reads `is_masked_input` as no change and no scrub or redaction shape moved, and `snapshot.rs`' moved `value` is a select's, where no mask applies, so the mask holds on the reached files
input-validation#1.71 · proposed · `text_extent` holds, and `visible_region`, a geometry reader of the same document, is new text beside it
input-validation#1.72 · proposed · the culling holds, and the non-invertible transform's answer is new text beside it
input-validation#1.77 · not touched · `form.rs` adds only :281 and tests at 491-618, leaving `method` and `enctype` as they read
input-validation#1.78 · not touched · `form.rs` adds only :281, a select option's own rule that the `select` rows state, and tests, so the row's exclusions hold
input-validation#1.79 · not touched · `form.rs` adds only :281 and tests at 491-618, leaving the CRLF normalization as it reads
input-validation#1.80 · not touched · `pointer.rs`' edits are the checkbox and radio arms and the `option` arm, leaving the file input's `accept` as it reads
input-validation#1.82 · not touched · `dioxus_document.rs` adds `multiple_select_values` and its call, leaving the `data-dioxus-id` parse as it reads
input-validation#1.85 · not touched · the report's `multiple-select` sweep reads `input-validation.md:299` as no change, and `mutator.rs`' recording of a `multiple` removal lands on the `select` row
input-validation#1.94 · not touched · `tests/dom.rs` adds only the radio test and `runtime/events.rs` only loses its `change` synthesis, so the CSS accessors and their witness hold
input-validation#1.112 · proposed · Cells' `cells-grid` takes a closed key match this row does not state
