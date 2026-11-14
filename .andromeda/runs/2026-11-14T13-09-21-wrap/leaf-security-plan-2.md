@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=32
last: - 2026-10-15: A listener or env read that only a test makes 
@leaf_edit n=1 leaf=.claude/rules/security.md line=22 class=omission
leaf-text: a test name or an assertion message).
source: dec-oostyleo «A fixture state name is compared whole against the app's closed table of named states before any session starts; any other name is refused as an unknown state and nothing is booted.»
replace-with: a test name or an assertion message). A fixture state name is compared whole against the app's closed table of named states before any session starts; any other name is refused as an unknown state and nothing is booted.
@leaf_edit n=2 leaf=.claude/rules/security.md line=22 class=omission
leaf-text: holds nothing of the call;
source: dec-ggzpqcfs «A reach error is one of a closed set: unknown state, refused at a step with the step's index and its closed refusal cause, or not settled at a step with the busy reading; each reads a fixed text»
replace-with: holds nothing of the call; a reach error is likewise one of a closed set (unknown state, refused at a step with the step's index and its closed refusal cause, or not settled at a step with the busy reading), each a fixed text holding nothing a caller supplied;
