# Classified — security · input-validation.b6

## Answers
input-validation#1.121 · no · serialize.rs text escaping is not in the reached files and no conflict or merge surface holds that rule
input-validation#1.122 · no · inline SVG serialization well-formedness lives in serialize.rs and inline_svg_serialize.rs, neither reached
input-validation#1.123 · no · selector parse and harness query panic live in blitz-test-harness and scoped_query_selector.rs, neither reached nor changed
input-validation#1.124 · no · WPT runner quarantine entries are the WPT runner's, out of scope (Not needed)
input-validation#1.125 · no · WPT runner fuzzy ranges are out of scope (Not needed)
input-validation#1.126 · no · WPT runner checkLayout attribute parsing is out of scope (Not needed)
input-validation#1.127 · no · WPT runner wrapper HTML is out of scope (Not needed)
input-validation#1.128 · no · WPT runner script-type filter is out of scope (Not needed)
input-validation#1.129 · no · WPT runner net_provider data: URL errors are out of scope; the default fetcher's data: narrowing is a different rule in blitz-vibey-script
input-validation#1.130 · no · WPT runner page-message parsing is out of scope (Not needed)
input-validation#1.131 · no · wpt-post-results.yml PR lookup is not a reached file and the WPT CI is out of scope
input-validation#1.132 · no · wpt_diff_to_pr.py comment splice is not reached and is WPT tooling, out of scope
input-validation#1.133 · yes · cells.rs (conflict surface) holds the tokenizer rejection, cycle detection and division-by-zero error that the resolution must keep
input-validation#1.134 · yes · cells.rs (conflict surface) holds the cells-grid keydown match, clamp, Enter editor and early return on in-editor click
input-validation#1.135 · yes · a26c6ddeb replays CustomWidget attributes (Needs), so the color attribute's parse_color fallback to black under closed-parser rules binds; the TodoMVC trim is not reached
input-validation#1.136 · no · HTML fixtures under examples/assets and their form facts are Not needed
