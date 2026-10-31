# Classified — security-plan · input-validation.b3

## Answers
input-validation#1.51 · yes · add-to · F8 and F9: a multiple select's `size` is read through `parse_unsigned_integer` with the same 65534 ceiling this row states for `rowspan`, and hostile size and option counts stay bounded.
input-validation#1.52 · no · `<ol start>` is parsed in packages/blitz-dom/src/layout/construct.rs, which the change does not reach, and no session fact touches a start attribute.
input-validation#1.53 · no · `attr_parsed` lives in packages/blitz-dom/src/node/element.rs, which the change does not reach, and the session's `size` read goes through `parse_unsigned_integer`, not `attr_parsed`.
input-validation#1.54 · yes · add-to · F5: a disabled select opens no list and a disabled row leaves an open list open, so the `disabled` presence reading now also gates the select list; the keyboard and pointer arms it cites moved under lines added in reached files.
input-validation#1.55 · yes · add-to · F10: an author's `open` on a select steers nothing, `:open` and `flush_open_state` unmoved, which the `open` row carries beside its details and dialog reading.
input-validation#1.56 · no · `autofocus` is read in packages/blitz-dom/src/mutator.rs, which the change does not reach, and no session fact touches autofocus.
input-validation#1.57 · no · `dir` is cited only to a test file outside the reached list, and no session fact touches the `dir` read.
input-validation#1.58 · no · the canvas `src` parse is in packages/blitz-dom/src/mutator.rs, which the change does not reach.
input-validation#1.59 · yes · stale · the id row cites packages/dioxus-native-dom/src/snapshot.rs:130-199, whose builder gained the list-bounds read at lines 147 and 152-155 inside that range, and packages/escher-driver/src/execute.rs (reached), where `locate` and the `Target` rename moved; no report line shows the id claim holds on those files.
input-validation#1.60 · yes · contradict · a click on a bound single select's label now focuses it and opens nothing and no longer re-dispatches, so "a click on a select's label re-dispatches to the select" no longer holds (touch line 210).
input-validation#1.61 · yes · stale · the description read sits in packages/blitz-dom/src/accessibility.rs (reached) at 237-268, the builder the session edits for the single select's `aria-expanded`, and no report line shows the description claim holds on the changed file.
input-validation#1.62 · yes · stale · a single select now writes `set_expanded` from engine state and reads no `aria-expanded`, a reading this row moved for selects while every other element keeps it (touch line 219).
input-validation#1.63 · yes · stale · the state readings it cites sit in packages/blitz-dom/src/accessibility.rs and packages/dioxus-native-dom/src/snapshot.rs (both reached, and the session edits the same builder), and packages/escher-driver/src/json.rs is reached; no report line shows these readings hold on the changed files.
input-validation#1.64 · yes · stale · the native checkbox and radio checked-state read sits in packages/blitz-dom/src/accessibility.rs, a reached file the session edits, and no report line shows that read holds on the changed file.
input-validation#1.65 · yes · stale · the hit path now answers an open list first through `hit_with_scrollbar`, ahead of the inert test in `hit_inner`, so the row's "before any other test" ordering moved, and its accessibility citations at accessibility.rs:283 and 287-300 are in a reached file.
input-validation#1.66 · yes · stale · the tree's focus write at packages/blitz-dom/src/accessibility.rs:104-110 and 122 is in a reached file the session edits, and no report line shows the focus claim holds on it.
input-validation#1.67 · yes · stale · the range default actions cite packages/blitz-dom/src/events/keyboard.rs:106-111, 156-173, 532-553 and packages/blitz-dom/src/events/pointer.rs:462-466, 840-852, all below lines added in those reached files, so the citations moved.
