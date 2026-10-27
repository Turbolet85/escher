# Classified — architecture · standard-contracts.b1

## Answers
standard-contracts#1.1 · no · no file under packages/blitz-dom/ changed, so the document core's contracts stand
standard-contracts#2.1 · no · the clock files (clock.rs, config.rs, iframe.rs, lib.rs) are blitz-dom files the session did not change
standard-contracts#2.2 · no · select_value and its crate-private writers live in blitz-dom, which the session did not change
standard-contracts#3.1 · no · the mutation, selector, CSSOM and accessibility code and its witness tests are outside the reached set
standard-contracts#4.1 · no · the event code in blitz-dom and blitz-traits is unchanged and outside the reached set
standard-contracts#5.1 · no · the scrolling, selection and tree code and its cited refusal.rs and tests are outside the reached set
standard-contracts#6.1 · no · get_client_bounding_rect is blitz-dom code the session did not change
standard-contracts#6.2 · no · lint.rs is reached, yet Session::lint stays text_extent's one non-test caller, re-read unchanged after the findings walk split
standard-contracts#6.3 · no · the hit walk in node.rs and paint_tree.rs is blitz-dom code the session did not change
standard-contracts#7.1 · no · the layout and stylo_taffy code lies outside the reached set
standard-contracts#8.1 · no · the node and widget code is blitz-dom code the session did not change
standard-contracts#9.1 · no · is_disabled in element.rs is blitz-dom code the session did not change
standard-contracts#9.2 · no · node/select.rs is unchanged and its readings reader is a different function from the driver's new private readings helper, re-read unmoved
standard-contracts#9.3 · no · out_of_flow in node.rs and layout/mod.rs is blitz-dom code the session did not change
standard-contracts#9.4 · no · the classes field in element.rs, mutator.rs and stylo.rs is blitz-dom code the session did not change
standard-contracts#10.1 · no · the provider traits in blitz-traits and blitz-dom's html.rs lie outside the reached set
standard-contracts#11.1 · no · the blitz-html and blitz-net crates lie outside the reached set
