# architecture · picked accounting · group 4 of 7

## Picked
standard-contracts#1.1 · not touched · the Document trait, DocumentConfig, the changed set and the two read-only readers hold; blitz-dom document.rs and config.rs are not reached, and in the reached blitz-vibey-script document.rs and dioxus_document.rs only the citation digits moved (the sweep's), ScriptDocument and DioxusDocument still implementing Document
standard-contracts#3.1 · proposed · DocumentMutator::set_attribute and remove_attribute (conflict surface mutator.rs) now flush open state on an `open` write, set_attribute's arm a second independent `if` after escher's range re-sanitize arm; the rest of the row holds
standard-contracts#5.1 · not touched · the report re-reads scroll_into_view, visible_region, hit_inner and absolute_position unchanged; the script scrollIntoView caller only moved to dom/element/scrolling.rs, calling the same engine method
standard-contracts#6.1 · not touched · get_client_bounding_rect and visible_region are unchanged and the transformed-scrolling-box limit stands (0980cce5f not taken, inputs#I5); the script getBoundingClientRect reader only moved file
standard-contracts#7.1 · not touched · construct.rs's changes (IME at editor construction, the pseudo flow vote) leave LayoutPassState, the Taffy traits and the ConstructionTask types as stated; the pseudo vote is an established-decisions fact (F2)
standard-contracts#8.1 · not touched · NodeData, SpecialElementData, RangeInputData and the range accessors stand as stated in the reached element.rs, only their citation digits moving; the new public flush_open_state goes in a row beside #9.4
standard-contracts#9.1 · not touched · the report re-reads is_disabled unchanged; element.rs's added lines at 488-489 move only its citation digits
standard-contracts#9.4 · proposed · ElementData gains the public flush_open_state, called from ElementData::new and the mutator's open arms, an unregistered public API beside the classes row
