@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=56
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=19 class=omission
leaf-text: and a live run also any layout answers of ids, closed words and numbers
source: dec-gkweah5x «A cold-agent live run carries the stand's screen, ids, accessible names and control values in snapshot texts and diffs, and layout, graph and actions answers, through the claude client to the model provider»
replace-with: and a live run also any layout, graph and actions answers of ids, closed words, booleans and numbers
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=13 class=omission
leaf-text: a failed, empty or non-CSS stylesheet blocks no render
source: dec-iigg6kvs «In every document, remote pages included, a stylesheet that fails to load, is empty, is not CSS or is not valid UTF-8 blocks no render.»
replace-with: a failed, empty, non-CSS or not valid UTF-8 stylesheet blocks no render
@leaf_edit n=3 leaf=.claude/docs/security-summary.md line=33 class=omission
leaf-text: the WPT runner's `env_logger` log them as-is
source: dec-kihe7ixw «upstream's fmt::init subscribers, env_logger and the panic hook stay unscrubbed.»
replace-with: the WPT runner's `env_logger` log them as-is, and the chained panic hook prints the raw panic message to stderr unscrubbed
