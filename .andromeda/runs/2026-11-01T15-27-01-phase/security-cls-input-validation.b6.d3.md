# Classified — security · input-validation.b6

## Answers
input-validation#1.121 · no · serialize.rs escaping is not a reached file and no named scope item moves serialization
input-validation#1.122 · no · the inline SVG serialization test is not reached, and the SVG attribute readers in Needs are input readers, not output
input-validation#1.123 · no · the harness selector panic lives in blitz-test-harness inspect.rs, which is not reached, and the chunk adds no selector query
input-validation#1.124 · no · WPT runner quarantine is out of scope under Not needed
input-validation#1.125 · no · WPT runner fuzzy ranges are out of scope under Not needed
input-validation#1.126 · no · WPT runner checkLayout parsing is out of scope under Not needed
input-validation#1.127 · no · WPT runner wrapper HTML is out of scope under Not needed
input-validation#1.128 · no · WPT runner script-type rule is out of scope under Not needed
input-validation#1.129 · no · WPT net_provider data: errors are out of scope under Not needed
input-validation#1.130 · no · WPT harness page-message parsing is out of scope under Not needed
input-validation#1.131 · no · wpt-post-results.yml is not reached and is WPT CI, out of scope
input-validation#1.132 · no · wpt_diff_to_pr.py comment splice is not reached and is WPT CI, out of scope
input-validation#1.133 · yes · examples/seven_guis/src/tasks/cells.rs is a conflict surface, so its tokenizer, #CYCLE and division-by-zero rules bind; flight booker and circle drawer are not reached
input-validation#1.134 · yes · cells.rs is a conflict surface and the cells-grid key match at lines 425-433 lives in it, so the keys bind through the conflict resolution
input-validation#1.135 · yes · the CustomWidget color attribute parse is the attribute reader that a26c6ddeb replays, under the closed-parser rule; the TodoMVC trim is not moved
input-validation#1.136 · no · HTML fixture inputs under examples/assets are facts of fixtures the chunk does not edit
