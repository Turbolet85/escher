# Classified — security · input-validation.b5

## Answers
input-validation#1.96 · yes · scrolling.rs is a conflict surface, and its clamp to 0 through the maximum offset and its no-op scroll to a missing node are the target clamping the resolution keeps
input-validation#1.97 · yes · the open select list's own scroll sits in scrolling.rs, a conflict surface, and in the select list whose behaviour the take must leave unchanged
input-validation#1.98 · no · form method and enctype parsing in form.rs; form submission is named Not needed and form.rs is not reached
input-validation#1.99 · no · submitted-control filtering in form.rs; form submission is named Not needed and form.rs is not reached
input-validation#1.100 · no · CRLF line-ending normalization in form.rs; line-ending rules are named Not needed and form.rs is not reached
input-validation#1.101 · no · the file input accept attribute is not a filter; the file dialog is named Not needed and events/pointer.rs is not reached
input-validation#1.102 · no · NativeFormData::valid in events.rs; form validity is outside the scope and events.rs is not reached
input-validation#1.103 · yes · data-dioxus-id parsing in dioxus_document.rs, a conflict surface, is the stable element id rule the identity surface keeps
input-validation#1.104 · no · dangerous_inner_html in mutation_writer.rs, which is not reached and is not named by the scope
input-validation#1.105 · no · unsupported AttributeValue types in mutation_writer.rs, which is not reached and is not named by the scope
input-validation#1.106 · no · falsy boolean attribute removal in mutation_writer.rs, which is not reached and is not named by the scope
input-validation#1.107 · no · Response status RangeError in bootstrap/fetch.js, not reached; an existing native check the scope does not name
input-validation#1.108 · yes · the JS-facing fetch check that allows only GET and HEAD is the rule the Needs bind, since a new script or fetch path adds no outbound method past GET and HEAD
input-validation#1.109 · no · selector SyntaxError in dom/document.rs and selectors.rs, not reached and not named by the scope
input-validation#1.110 · no · ScrollBehavior and ScrollLogicalPosition checks in vibey-script's scrolling.rs, not reached; an existing native check the scope does not name
input-validation#1.111 · no · width/height reflection in attributes.rs, not reached; the delta moves no width or height reader, and the clamped-numeric rule binds the readers it does move
input-validation#1.112 · no · timer delay and clear-id checks in timers.rs, not reached and not named by the scope
input-validation#1.113 · no · CSS.registerProperty in css.rs, not reached and not named by the scope
input-validation#1.114 · yes · setProperty and cssText drop invalid declarations, the script route to the declaration validation by parsing that the scope names for the take's new paint properties
input-validation#1.115 · yes · the accessors are built from Stylo's property list, which the take may move, so new paint properties reach script through this crossing under the declaration rule
input-validation#1.116 · no · node-argument TypeErrors in dom/mod.rs and dom/node.rs, not reached and not named by the scope
input-validation#1.117 · no · geometry.js conversions, not reached and not named by the scope
input-validation#1.118 · no · document selection in selection.rs, not reached; the innerText and outerText getters are not in the scope
input-validation#1.119 · no · sqlite parameter binding in apps/browser/persistence, which no commit the scope names touches
input-validation#1.120 · no · stored URLs re-parsed on load in apps/browser/persistence, which no commit the scope names touches
