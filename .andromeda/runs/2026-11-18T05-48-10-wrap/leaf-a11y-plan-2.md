@leaf_group group=a11y-plan-2
@leaf_read leaf=.claude/rules/a11y.md lines=46
last: - 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable
@leaf_edit n=1 leaf=.claude/rules/a11y.md line=19 class=stale
leaf-text: its pin `.github/scripts/test_capability_statement.py` reads none of these claims, so a change that moves the reach is made in that document by hand.
source: dec-3z3w6342 «The capability statement's pin reads only the a11y leg's file count among the section's claims; no check reads the rest.»
replace-with: its pin `.github/scripts/test_capability_statement.py` reads only the `a11y` leg's file count among these claims and no check reads the rest, so a change that moves the reach is made in that document by hand.
