# Classified — architecture · standard-contracts.b1

## Answers
standard-contracts#1.1 · no · document-core source and symbols untouched per the task, so the Document trait and DocumentConfig claims stand
standard-contracts#2.1 · no · the document clock is document-core source the session left unchanged, so its claims stand
standard-contracts#2.2 · no · select_value and its crate-private writers are document-core source the session left unchanged
standard-contracts#3.1 · no · mutation, query, CSSOM and accessibility-builder source is document-core and untouched per the task
standard-contracts#4.1 · no · events source is untouched per the task (driver, focus, keyboard and pointer), so its claims stand
standard-contracts#5.1 · no · scrolling, selection and tree source is untouched per the task, so scroll_into_view and its callers stand
standard-contracts#6.1 · no · client-rect and transform source is untouched per the task, so get_client_bounding_rect and its readers stand
standard-contracts#6.2 · no · text_extent is unchanged source and its one caller, Session::lint, is unmoved, so the claim stands
standard-contracts#6.3 · no · the hit walk's node and paint-tree source is untouched per the task, so its dropped-node skip stands
standard-contracts#7.1 · no · layout and paint-tree source is untouched per the task, so the layout API claims stand
standard-contracts#8.1 · no · node, widget and range-input source is untouched per the task, so its type claims stand
standard-contracts#9.1 · no · ElementData::is_disabled is nodes-and-widgets source the session left unchanged
standard-contracts#9.2 · no · the crate-private select rules are nodes-and-widgets source the session left unchanged
standard-contracts#9.3 · no · Node::out_of_flow is nodes-and-widgets and layout source the session left unchanged
standard-contracts#9.4 · no · ElementData::classes is nodes-and-widgets source the session left unchanged
standard-contracts#10.1 · no · provider traits are untouched per the task, so their signatures and defaults stand
standard-contracts#11.1 · no · the HTML and networking crates are untouched per the task, so the blitz-html and blitz-net API claims stand
