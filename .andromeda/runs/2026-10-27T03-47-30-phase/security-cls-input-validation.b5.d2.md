# Classified — security · input-validation.b5

## Answers
input-validation#1.90 · no · Boa's `CSS.registerProperty` argument checks; script execution on Boa is untouched by the chunk
input-validation#1.91 · no · Boa's `setProperty` and `cssText` declaration drops; script execution on Boa is untouched by the chunk
input-validation#1.92 · no · Boa's DOM node-argument TypeErrors and `replaceChild` checks; script execution on Boa is untouched by the chunk
input-validation#1.93 · no · Boa's `geometry.js` conversions; script execution on Boa is untouched by the chunk
input-validation#1.94 · no · Boa's `document.getSelection` setters; script execution on Boa is untouched, and the report it cites is not a reached file
input-validation#1.95 · no · sqlite `params!` binding in the browser persistence crate; browser persistence is untouched by the chunk
input-validation#1.96 · no · sqlite stored-URL re-parse on load; browser persistence is untouched by the chunk
input-validation#1.97 · no · blitz-dom serialize escaping; no engine change, and no claim in the chunk's needs names serialization
input-validation#1.98 · no · inline-SVG serialization round trip; SVG handling is in the chunk's no-engine-change list, and its test file is not re-read
input-validation#1.99 · no · harness selector-parse panic and scoped-query reads; no harness change, and the re-read tests are named for snapshot and lint readings, not selector parsing
input-validation#1.100 · no · WPT timeout-quarantine entries; the WPT runner is not a reached file
input-validation#1.101 · no · WPT fuzzy-range parsing; the WPT runner is not a reached file
input-validation#1.102 · no · WPT checkLayout attribute parsing; the WPT runner is not a reached file
input-validation#1.103 · no · WPT wrapper-HTML escaping; the WPT runner is not a reached file
input-validation#1.104 · no · WPT script-type filter; the WPT runner is not a reached file
input-validation#1.105 · no · WPT net provider's `data:` decode failures; the net provider is untouched by the chunk
input-validation#1.106 · no · WPT harness page-message JSON check; the WPT runner is not a reached file
input-validation#1.107 · no · the wpt-post-results PR lookup; the post-results guard is untouched (no workflow change)
input-validation#1.108 · no · the wpt_diff_to_pr.py comment splice, not a reached file; the capability-statement pin reads counts, not this splice
input-validation#1.109 · yes · the flight booker's `select#flight-type` closed input match over its two options (one-way, return) writes the flight type that `exactly_the_chosen_flight_type_reads_selected` (re-read in stand_snapshot_state.rs) reads as selected; its date, cells and circle clauses are not reached
input-validation#1.110 · no · the widget's `color` fallback and the TodoMVC trim, in examples the chunk does not reach
input-validation#1.111 · no · the graphite and google fixtures' form inputs, example assets the chunk does not reach
