# Classified — architecture · standard-contracts.b1

## Answers
standard-contracts#1.1 · yes · stale · vibey-script document.rs gained lines above the cited ScriptDocument impl at 402-446, and dioxus_document.rs, a conflict surface, holds the cited DioxusDocument impl at 239-311
standard-contracts#2.1 · no · clock.rs and config.rs are not reached, and the task's Untouched section holds the document clock's claim
standard-contracts#2.2 · no · its select_value and select-choice writers sit in document.rs, which is not reached, and no touch names select
standard-contracts#3.1 · yes · add-to · set_attribute and remove_attribute now flush open state on an open write (F1), which this mutation contract must carry, and window.rs gained lines at 193-194 above its cited View::poll reads
standard-contracts#4.1 · no · its citations sit in the unreached events module and blitz-traits events.rs, and the shell redraw route and IME enable change no event claim it states
standard-contracts#5.1 · yes · stale · stylo.rs, a merge surface, holds the cited resolve_stylist at 70 and resolve_undisplayed_style at 199-241, and a merge can move any of its lines
standard-contracts#6.1 · yes · stale · the script getBoundingClientRect binding sits in a reached blitz-vibey-script merge surface, so the readers this claim names can have moved
standard-contracts#6.2 · no · text_extent.rs and lib.rs are not reached, and no touch names the text extent reader
standard-contracts#6.3 · no · hit_inner and its dropped-node skips sit in node.rs and paint_tree.rs, neither reached, and the Untouched section re-read hit_inner unchanged
standard-contracts#7.1 · yes · stale · construct.rs, a merge surface, holds the cited ConstructionTask types at 35-53, and a merge can move any of its lines
standard-contracts#8.1 · yes · stale · element.rs, a merge surface, holds the cited SpecialElementData, range and ElementData accessor lines, and a merge can move any of them
standard-contracts#9.1 · yes · stale · element.rs, a merge surface, holds the cited is_disabled at 516-520, and a merge can move that line
standard-contracts#9.2 · no · its node/mod.rs and select.rs citations are not reached, and no touch names the crate-private select rules
standard-contracts#9.3 · no · its out_of_flow field and its record and read methods cite node.rs and layout/mod.rs, neither reached, and no touch names them
standard-contracts#9.4 · yes · add-to · ElementData gains the public flush_open_state (F1), synced from the mutator's open arms as classes is, and mutator.rs gained lines above the cited 379-382 and 503-506
standard-contracts#10.1 · no · its citations sit in blitz-traits, blitz-dom html.rs and tests, none reached, and the session changes no trait or default it states
standard-contracts#11.1 · no · its blitz-html and blitz-net citations sit in unreached files, and the session changes no crate it names
