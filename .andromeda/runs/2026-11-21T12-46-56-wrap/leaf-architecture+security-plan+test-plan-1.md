@leaf_group group=architecture+security-plan+test-plan-1
@leaf_read leaf=.claude/docs/services/escher-driver.md lines=63
last: - The command line, through the real `escher-session` binary
@leaf_edit n=1 leaf=.claude/docs/services/escher-driver.md line=6 class=stale
leaf-text: a seed reaches a run only from in-process code that builds its VirtualDom
source: dec-uemnwvfz «so each driver session and kept check's replay reads the default seed; a seed reaches a run only from in-process code building its VirtualDom or script document.»
replace-with: a seed reaches a run only from in-process code that builds its VirtualDom or script document
@leaf_edit n=2 leaf=.claude/docs/services/escher-driver.md line=36 class=stale
leaf-text: focus stays and the key still acts
source: dec-sllwljwg «so a key press on a control whose anonymous block was dropped does not panic, and once re-focused the key acts.»
replace-with: the key acts once the control is re-focused
