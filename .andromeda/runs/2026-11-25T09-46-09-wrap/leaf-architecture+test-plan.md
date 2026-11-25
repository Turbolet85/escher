@leaf_group group=architecture+test-plan
@leaf_read leaf=.claude/docs/commands.md lines=82
last: - Fonts missing in headless tests → `system-fonts` feature /
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=37
last: - `.claude/rules/verification-harness.md` · `.claude/docs/te
@leaf_edit n=1 leaf=.claude/docs/commands.md line=58 class=omission
leaf-text: `click --id <id>` (a click on a drop-down, a single select of display size 1,
source: ctr-7vwjtnb2 «click is the one verb taking a virtual node id: it acts on the canvas at the virtual region and is refused in the one order disabled, off-screen, covered, not-found, stale with nothing run»
replace-with: `click --id <id>` (the one verb taking a canvas's virtual node id `{owner}///{key}`, acting on the canvas at its declared region, refused in the one order `disabled`, `off-screen`, `covered`, `not-found`, `stale`, every other id verb reading a virtual id `not-found`; a click on a drop-down, a single select of display size 1,
