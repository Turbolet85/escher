# Classified — security · input-validation.b5

## Answers
input-validation#1.96 · yes · the clamp of scroll targets and the no-effect return on a missing node sit in scrolling.rs, a predicted conflict surface whose target clamping the scope keeps as recorded
input-validation#1.97 · yes · the open select list's scroll_into_view, wheel and Geometry::clamped sit on select_list.rs and scrolling.rs, the select list whose behaviour the scope reads row by row, and select_popup.rs is an edited test
input-validation#1.98 · no · method and enctype parsing in form.rs is not a reached file, and form submission's method and enctype rules are not needed
input-validation#1.99 · no · submitted-control exclusion in form.rs is not a reached file, and form submission is not needed
input-validation#1.100 · no · CRLF normalization in form.rs is not a reached file, and the line-ending rules are not needed
input-validation#1.101 · no · the accept attribute in events/pointer.rs is not a reached file, and the file dialog is not needed
input-validation#1.102 · no · NativeFormData::valid in dioxus-native-dom events.rs is not a reached file, and the scope names no form validity path
input-validation#1.103 · yes · the data-dioxus-id usize parse sits in dioxus_document.rs, a conflict surface, under the stable element id rules the scope keeps
input-validation#1.104 · no · dangerous_inner_html in mutation_writer.rs is not a reached file, and the scope names no such mutation path
input-validation#1.105 · no · AttributeValue type filtering in mutation_writer.rs is not a reached file, and the scope names no such mutation path
input-validation#1.106 · no · the falsy boolean-attribute predicate in mutation_writer.rs is not a reached file, and the scope names no such predicate
input-validation#1.107 · no · the Response status RangeError sits in bootstrap/fetch.js, not a reached file, and the scope names only the fetch method bound
input-validation#1.108 · yes · the chunk's new script or fetch path must add no method past GET and HEAD, the bound fetch.js:81-83 enforces
input-validation#1.109 · no · the selector SyntaxError in vibey-script document.rs and selectors.rs is not a reached file, and the scope does not name it
input-validation#1.110 · no · the ScrollBehavior and ScrollLogicalPosition checks in vibey-script dom/element/scrolling.rs are not a reached file, and the scope's scroll work is blitz-dom's
input-validation#1.111 · no · width/height reflection in attributes.rs is not a reached file, and the scope does not name it
input-validation#1.112 · no · timer delay and clear-id rules in runtime/timers.rs are not a reached file, and the scope does not name them
input-validation#1.113 · no · CSS.registerProperty in runtime/css.rs is not a reached file, and the scope does not name the registration
input-validation#1.114 · yes · setProperty and cssText drop invalid declarations by parsing, the declaration validation the scope names for the paint properties of 6fca6308d, ac6f9f582 and e5929f9e2 that page CSS reaches
input-validation#1.115 · yes · the accessor set follows Stylo's property list, which a whole merge of git Stylo may move, a fork the scope leaves open; the style_property_accessors witness is not in the chunk's re-runs
input-validation#1.116 · no · the node-argument TypeError and replaceChild checks sit in vibey-script dom/mod.rs and dom/node.rs, not reached files, and the scope does not name them
input-validation#1.117 · no · geometry.js conversions are not a reached file, and the scope does not name them
input-validation#1.118 · no · document selection in dom/selection.rs is not a reached file, and the scope names neither the selection API nor its innerText and outerText getters
input-validation#1.119 · no · sqlite params! binding in apps/browser/persistence is not a reached file, and the scope names no persistence path
input-validation#1.120 · no · stored-URL re-parse on load in apps/browser/persistence is not a reached file, and the scope names no persistence path
