# Classified — security · input-validation.b5

## Answers
input-validation#1.90 · no · blitz-vibey-script runtime rule, listed Not needed; a Stylo move reaches only its manifest
input-validation#1.91 · no · setProperty and cssText drops are blitz-vibey-script runtime, listed Not needed; a Stylo move reaches only its manifest
input-validation#1.92 · no · node-argument checks are blitz-vibey-script runtime, listed Not needed; a Stylo move reaches only its manifest
input-validation#1.93 · no · geometry.js conversions are blitz-vibey-script runtime, listed Not needed; a Stylo move reaches only its manifest
input-validation#1.94 · no · the Selection API is blitz-vibey-script runtime, listed Not needed; a Stylo move reaches only its manifest
input-validation#1.95 · no · the browser's sqlite history, listed Not needed; mimalloc changes only the allocator, not the bound values
input-validation#1.96 · no · the stored-URL reparse is the browser's sqlite history, listed Not needed; mimalloc changes only the allocator
input-validation#1.97 · no · the serializer node/serialize.rs is named by no Need or listed blitz-dom move
input-validation#1.98 · no · the inline-SVG serialize check covers the serializer that no Need or listed blitz-dom move names
input-validation#1.99 · no · the harness query helper's panic is test-side on developer-written selectors; no Need names selector parsing or the harness
input-validation#1.100 · no · the WPT runner's quarantine checks are listed Not needed; the runner commit changes neither them nor the local file reads
input-validation#1.101 · yes · the WPT runner commit f8723f495 changes the runner, and fuzzy-range parsing is none of the carved-out rules
input-validation#1.102 · yes · checkLayout's f32 parse and selector panic sit in the WPT runner the take changes, and no carve-out names them
input-validation#1.103 · yes · the wrapper's title and script escaping sit in the WPT runner the take changes, and no carve-out names them
input-validation#1.104 · yes · the script-type filter sits in the WPT runner the take changes, and no carve-out names it
input-validation#1.105 · yes · data: URL decoding sits in the WPT runner, which the take changes, and the carved-out local file reads are a different rule
input-validation#1.106 · yes · the page-message JSON check sits in the WPT runner the take changes, and no carve-out names it
input-validation#1.107 · yes · the Needs keep the repository guard on wpt-post-results.yml if the take edits it, and this PR lookup is that file's rule
input-validation#1.108 · no · wpt_diff_to_pr.py is named by no Need, and the Needs carry only the workflow guard for that pipeline
input-validation#1.109 · yes · the flight booker's dd.mm.yyyy rule sets the aria-invalid state the snapshot checks read, and those checks decide the Dioxus 0.8 resync
input-validation#1.110 · no · custom_widget.rs and the preact TodoMVC page are named by no Need
input-validation#1.111 · no · the examples/assets fixture pages are named by no Need, and no Need names a form or maxlength read
