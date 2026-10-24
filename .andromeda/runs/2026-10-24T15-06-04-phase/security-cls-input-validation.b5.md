# Classified — security · input-validation.b5

## Answers
input-validation#1.90 · no · JS API argument checks are Not needed as script execution (blitz-vibey-script), and runtime.rs is not reached
input-validation#1.91 · no · style setProperty and cssText checks are JS API argument checks, Not needed as script execution, and blitz-vibey-script is not reached
input-validation#1.92 · no · native node-argument checks are JS API argument checks, Not needed as script execution, and blitz-vibey-script is not reached
input-validation#1.93 · no · geometry.js conversions are JS API argument checks, Not needed as script execution, and blitz-vibey-script is not reached
input-validation#1.94 · no · document.getSelection argument checks are JS API argument checks, Not needed as script execution, and blitz-vibey-script is not reached
input-validation#1.95 · no · sqlite bind values belong to the browser app's history, which is Not needed, and apps/browser/persistence is not reached
input-validation#1.96 · no · stored-URL re-parsing belongs to the browser app's sqlite history, which is Not needed, and apps/browser/persistence is not reached
input-validation#1.97 · no · blitz-dom node/serialize.rs is not reached, and no Needs item writes markup out
input-validation#1.98 · no · inline SVG well-formedness is an SVG input check the Not-needed list excludes, and neither serialize.rs nor inline_svg_serialize.rs is reached
input-validation#1.99 · no · the selector rules live in blitz-test-harness inspect.rs and scoped_query_selector.rs, neither reached, and no Needs item names a selector or query helper, a re-check every change owes
input-validation#1.100 · no · the WPT runner's quarantine is Not needed and wpt/runner/ is not reached
input-validation#1.101 · no · fuzzy-range parsing lives in wpt/runner/, not reached, and no Needs item names the WPT runner
input-validation#1.102 · no · checkLayout parsing lives in wpt/runner/, not reached, and no Needs item names the WPT runner
input-validation#1.103 · no · wrapper HTML escaping lives in wpt/runner/, not reached, and no Needs item names the WPT runner
input-validation#1.104 · no · script-type matching lives in wpt/runner/, not reached, and no Needs item names the WPT runner
input-validation#1.105 · no · data: URL decoding lives in wpt/runner/src/net_provider.rs, not reached, and no Needs item names the WPT runner
input-validation#1.106 · no · page-message parsing lives in wpt/runner/, not reached, and no Needs item names the WPT runner
input-validation#1.107 · no · the PR lookup lives in wpt-post-results.yml, which the Not-needed list keeps untouched, and no Needs item names it
input-validation#1.108 · no · the comment splice lives in .github/scripts/wpt_diff_to_pr.py, not reached, and no Needs item names the WPT post-results path
input-validation#1.109 · no · the flight booker, cells and circle drawer rules live in examples/seven_guis/src/tasks/, not reached, and no Needs item names a task input rule, a re-check every change owes
input-validation#1.110 · no · the widget colour parse and the TodoMVC trim sit in examples/, not reached, and no Needs item names an example app
input-validation#1.111 · no · the graphite and google form rules sit in examples/assets/, not reached, and no Needs item names these attributes, the sweep's fixtures being tests/fixtures pages
