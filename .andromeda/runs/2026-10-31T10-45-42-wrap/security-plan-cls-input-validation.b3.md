# Classified — security-plan · input-validation.b3

## Answers
input-validation#1.51 · no · its clamp is at packages/blitz-dom/src/layout/table.rs:630-634, a file the change did not reach
input-validation#1.52 · no · its parse is at packages/blitz-dom/src/layout/construct.rs:475-481, a file the change did not reach
input-validation#1.53 · no · its parse-failure rule is at packages/blitz-dom/src/node/element.rs:502-505, a file the change did not reach
input-validation#1.54 · yes · stale · its gates sit in reached pointer.rs and keyboard.rs, its snapshot read in reached snapshot.rs, and its cited stand_keyboard.rs:889-930 was edited, so cited lines may have moved
input-validation#1.55 · yes · contradict · the no-test statement is retired, since open_state (4 tests) now asserts the presence read and :open matching
input-validation#1.56 · yes · stale · autofocus is read at mutator.rs:1143-1155, a reached file whose cited lines may have moved
input-validation#1.57 · no · its only cite, tests/blitz-tests/tests/dir_attribute.rs:45-50, is not among the reached files
input-validation#1.58 · yes · stale · canvas src is read at mutator.rs:1451-1462, a reached file whose cited lines may have moved
input-validation#1.59 · yes · stale · its id readers sit in reached snapshot.rs and execute.rs, whose cited lines may have moved
input-validation#1.60 · yes · add-to · F4: an optgroup's non-empty label now reads as a name, a new source in the names crossing beside aria-label and label for
input-validation#1.61 · yes · stale · its builder sits at accessibility.rs:89-102 and :237-264, a reached file whose cited lines may have moved
input-validation#1.62 · yes · stale · its builder sits at accessibility.rs:237-264 and :440-461 and its reads at snapshot.rs:51-52 and :167, reached files whose cited lines may have moved
input-validation#1.63 · yes · contradict · F8: a native checkbox or radio now writes Toggled under a checkable role, so the no-tree-node measured absence is retired
input-validation#1.64 · yes · contradict · F7: DocumentMutator::flush now calls clear_focus where the focus node reads inert, so the focus-fixup limit is retired
input-validation#1.65 · yes · add-to · F5: a multiple select's chosen set joins this row's control-value exits (form entries, input and change values, the snapshot's value)
input-validation#1.66 · yes · add-to · F9: a checkbox or radio click, Space's included, now sends change after input, beside the Click this row says Space dispatches
