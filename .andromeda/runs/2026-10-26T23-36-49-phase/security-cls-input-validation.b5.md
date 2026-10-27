# Classified — security · input-validation.b5

## Answers
input-validation#1.90 · no · the CSS.registerProperty argument checks are blitz-vibey-script's JS API, which the scope lists as not needed
input-validation#1.91 · no · the setProperty and cssText declaration drops are blitz-vibey-script's JS API, which the scope lists as not needed
input-validation#1.92 · no · the native node-argument TypeErrors are blitz-vibey-script's JS API, which the scope lists as not needed
input-validation#1.93 · no · the geometry.js conversion checks are blitz-vibey-script's JS API, which the scope lists as not needed
input-validation#1.94 · no · the getSelection argument checks are blitz-vibey-script's JS API, which the scope lists as not needed
input-validation#1.95 · no · the sqlite placeholder binding is the browser app's persistence, which the scope lists as not needed
input-validation#1.96 · no · the stored-URL re-parsing is the browser app's sqlite persistence, which the scope lists as not needed
input-validation#1.97 · no · the HTML text and attribute escaping is blitz-dom's serializer output, which the lint and the two tests do not produce, since the lint writes JSON and no markup
input-validation#1.98 · no · the inline-SVG serialization check is blitz-dom markup the chunk neither edits nor calls, and its run is the fast leg's re-check every change owes
input-validation#1.99 · no · the harness selector parse and query helper are not reached, since the lint takes no selector and the scope names no query
input-validation#1.100 · no · WPT timeout-quarantine entries are WPT runner code the chunk does not reach
input-validation#1.101 · no · WPT fuzzy-range parsing is WPT runner code the chunk does not reach
input-validation#1.102 · no · checkLayout attribute parsing is WPT runner code the chunk does not reach
input-validation#1.103 · no · WPT wrapper-HTML escaping is WPT runner code the chunk does not reach
input-validation#1.104 · no · WPT script-type filtering is WPT runner code the chunk does not reach
input-validation#1.105 · no · WPT data: URL decoding is the runner's net provider, and the scope lists blitz-net's data: fetches as not needed
input-validation#1.106 · no · WPT page-message JSON checks are WPT runner code the chunk does not reach
input-validation#1.107 · no · the PR lookup is in wpt-post-results.yml, a workflow the scope lists as not needed
input-validation#1.108 · no · the comment splice is wpt_diff_to_pr.py, not a reached file, and any check of it in the fast leg is a re-check every change owes
input-validation#1.109 · no · the 7GUIs date, cells, circle and select-mode rules are value handlers; the lint's findings carry no value and stand_lint asserts findings, not these rules
input-validation#1.110 · no · the widget colour parse and the TodoMVC trim are examples code the chunk does not reach
input-validation#1.111 · no · the graphite and google fixture form rules are example assets the chunk does not reach
