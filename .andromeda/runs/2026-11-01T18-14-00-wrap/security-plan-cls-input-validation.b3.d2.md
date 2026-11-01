# Classified — security-plan · input-validation.b3

## Answers
input-validation#1.51 · no · its cited files layout/mod.rs and layout/table.rs are not in the session's reached files
input-validation#1.52 · no · its cited table.rs:630-634 sits in layout/table.rs, which the session did not reach
input-validation#1.53 · yes · stale · default.css:183-193 and stylo.rs:1132-1147 are merge surfaces the session reached whole, so the cited lines can move
input-validation#1.54 · yes · stale · construct.rs:480-486 sits in layout/construct.rs, a merge surface the session reached whole
input-validation#1.55 · yes · stale · element.rs:502-505 and :742-744 sit in node/element.rs, a merge surface the session reached whole
input-validation#1.56 · yes · stale · element.rs:480-485, :516-520 and :743 sit in node/element.rs, a merge surface the session reached whole
input-validation#1.57 · yes · stale · element.rs:488-489 and :543-549, mutator.rs:356-359 and :502-505, and stylo.rs:513 sit in merge surfaces the session reached whole
input-validation#1.58 · yes · stale · mutator.rs:1215-1227 sits in a merge surface the session reached whole
input-validation#1.59 · no · its only cited file, tests/blitz-tests/tests/dir_attribute.rs, is not in the session's reached files
input-validation#1.60 · yes · stale · mutator.rs:1525-1536 sits in a merge surface the session reached whole
input-validation#1.61 · no · its cited files (element_id.rs, snapshot.rs, actionable.rs, window.rs, execute.rs, session.rs and the stand tests) are not in the session's reached files
input-validation#1.62 · yes · stale · document.rs:695-733 and :3531-3545 sit in a merge surface the session reached whole, so the cited lines can move
input-validation#1.63 · no · its cited accessibility.rs and accessibility_description.rs are not in the session's reached files
input-validation#1.64 · no · its cited accessibility.rs, snapshot.rs and tests are not in the session's reached files
input-validation#1.65 · yes · stale · select_popup.rs is a reached test file whose row-3 edit can move the cited lines :624-647
input-validation#1.66 · no · its cited accessibility.rs, snapshot.rs, snapshot_text.rs, json.rs and accessibility_states.rs are not in the session's reached files
input-validation#1.67 · no · its cited accessibility.rs:199-224 and accessibility_states tests are not in the session's reached files
input-validation#1.68 · yes · stale · mutator.rs:1123-1135 and node.rs:574-591 and :1450-1459 sit in merge surfaces the session reached whole
input-validation#1.69 · yes · stale · document.rs:772-806 sits in a merge surface the session reached whole
input-validation#1.70 · yes · stale · mutator.rs:1123-1135 sits in a merge surface the session reached whole
