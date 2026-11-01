# Classified — security · input-validation.b5

## Answers
input-validation#1.96 · yes · scrolling.rs is a conflict surface, and its target clamp and missing-node return are the scrolled-box bounds the Needs keep
input-validation#1.97 · yes · scrolling.rs is a conflict surface, and the open select list's scroll rules are the ratified boundary the Needs keep unchanged in behaviour
input-validation#1.98 · no · form.rs is not reached, and form submission's method and enctype are named under Not needed
input-validation#1.99 · no · form.rs is not reached, and submitted-control rules are form submission, named under Not needed
input-validation#1.100 · no · form.rs is not reached, and line-ending rules are form submission, named under Not needed
input-validation#1.101 · no · pointer.rs is not reached, and the file dialog is named under Not needed
input-validation#1.102 · no · events.rs is not reached, and the chunk adopts no new upstream API
input-validation#1.103 · yes · dioxus_document.rs is a conflict surface, and the stable element id rules the Needs keep live there
input-validation#1.104 · no · mutation_writer.rs is not reached, and the scope names no dangerous_inner_html rule
input-validation#1.105 · no · mutation_writer.rs is not reached, and the scope names no attribute-type rule
input-validation#1.106 · no · mutation_writer.rs is not reached, and the scope names no falsy-attribute set
input-validation#1.107 · no · the status check sits in bootstrap/fetch.js, which is not reached, and the scope names no status rule
input-validation#1.108 · yes · the Needs name the GET and HEAD-only outbound method rule for any new script or fetch path
input-validation#1.109 · no · document.rs and selectors.rs are not reached, and the scope names no selector rule
input-validation#1.110 · no · dom/element/scrolling.rs is not reached, and the scope names no scroll-option rule
input-validation#1.111 · no · attributes.rs is not reached, and width and height reflection is not one of the markup readers the delta moves
input-validation#1.112 · no · timers.rs is not reached, and the scope names no timer rule
input-validation#1.113 · no · runtime/css.rs is not reached and is not one of the runtime files in conflict
input-validation#1.114 · yes · setProperty and cssText drop invalid declarations through the same parse the Needs name for new paint properties reached by page CSS, a fork over the scope
input-validation#1.115 · yes · the take's new paint properties could enter Stylo's name list and so gain script accessors, the path the remote-content boundary covers
input-validation#1.116 · no · dom/mod.rs and dom/node.rs are not reached, and the scope names no node-argument rule
input-validation#1.117 · no · geometry.js is not reached, and the scope names no conversion rule
input-validation#1.118 · no · selection.rs is not reached, and the scope names no selection rule
input-validation#1.119 · no · browser persistence is named under Not needed
input-validation#1.120 · no · browser persistence is named under Not needed
