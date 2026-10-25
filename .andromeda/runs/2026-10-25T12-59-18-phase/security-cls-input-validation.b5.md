# Classified — security · input-validation.b5

## Answers
input-validation#1.90 · no · CSS.registerProperty is a blitz-vibey-script JS API, and the task's Not needed list excludes script execution; the stand runs no page scripts
input-validation#1.91 · no · setProperty and cssText are blitz-vibey-script style JS surfaces, unreached, since the stand runs no page scripts
input-validation#1.92 · no · the Node-argument TypeError checks are blitz-vibey-script dom code the chunk does not reach
input-validation#1.93 · no · geometry.js conversions are a blitz-vibey-script file the chunk does not reach
input-validation#1.94 · no · document.getSelection is a blitz-vibey-script API the stand never reaches; the stand's select is a form control, not the document selection
input-validation#1.95 · no · sqlite parameter binding is in apps/browser persistence, which the task leaves untouched
input-validation#1.96 · no · stored-URL re-parsing is in apps/browser persistence, which the task leaves untouched
input-validation#1.97 · no · text and attribute escaping is blitz-dom node/serialize.rs, not reached, and no chunk path serializes DOM back to markup
input-validation#1.98 · no · inline SVG is a remote-content row the task names as not needed; the stand is offline example markup with no new parse path
input-validation#1.99 · no · selector parsing and the harness query panic live in blitz-test-harness inspect.rs and scoped_query_selector.rs, neither reached; the driver addresses elements by stable id, so no selector crosses it
input-validation#1.100 · no · WPT runner timeout-quarantine parsing is in wpt/runner, which the chunk does not touch
input-validation#1.101 · no · WPT runner fuzzy-range parsing is in wpt/runner, which the chunk does not touch
input-validation#1.102 · no · WPT runner checkLayout parsing is in wpt/runner, which the chunk does not touch
input-validation#1.103 · no · WPT runner wrapper HTML escaping is in wpt/runner, which the chunk does not touch
input-validation#1.104 · no · WPT runner script-type filter is in wpt/runner, which the chunk does not touch
input-validation#1.105 · no · the WPT runner's own data: URL provider (wpt/runner/src/net_provider.rs) is not reached by the stand or the cold-agent pipe
input-validation#1.106 · no · WPT page-message parsing is in wpt/runner harness_test.rs, not reached
input-validation#1.107 · no · the PR lookup is in wpt-post-results.yml, a workflow the task names as unchanged
input-validation#1.108 · no · the comment splice is in wpt_diff_to_pr.py, which the chunk does not reach
input-validation#1.109 · yes · flight_booker.rs and cells.rs are both edited, and the scope names Cells' formulas and the flight booker's new select, so the date, formula, cycle and division checks bind
input-validation#1.110 · no · parse_color and the TodoMVC trim are in examples/custom_widget.rs and examples/preact/index.html, neither reached, and the scope is 7GUIs only
input-validation#1.111 · no · the graphite and google form attributes are examples/assets markup, not reached, and the task names those pages as not stand pages
