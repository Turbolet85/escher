@leaf_group group=architecture+test-plan
@leaf_read leaf=.claude/docs/commands.md lines=82
last: - Fonts missing in headless tests → `system-fonts` feature /
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=37
last: - `.claude/rules/verification-harness.md` · `.claude/docs/te
@leaf_edit n=1 leaf=.claude/docs/commands.md line=58 class=stale
leaf-text: (the one verb taking a canvas's virtual node id `{owner}///{key}`,
source: ctr-7vwjtnb2 «click takes a virtual node id, as layout does: it acts on the canvas at the virtual region and is refused in the one order disabled, off-screen, covered, not-found, stale with nothing run;»
replace-with: (taking a canvas's virtual node id `{owner}///{key}`, as `layout` does,
@leaf_edit n=2 leaf=.claude/docs/commands.md line=58 class=stale
leaf-text: every other id verb reading a virtual id `not-found`
source: ctr-7vwjtnb2 «type, scroll, screenshot with an id and canvas read a virtual id not-found.»
replace-with: `type`, `scroll`, `screenshot --id` and `canvas` reading a virtual id `not-found`
@leaf_edit n=3 leaf=.claude/docs/commands.md line=58 class=omission
leaf-text: reading where the element sits and why at its size, changing nothing and running no step)
source: ctr-mxl62qim «Layout on a virtual node id answers it placed by app in its canvas, at the region's offset from the canvas's border box, with the snapshot's bounds, the canvas's shifts and both size causes layout;»
replace-with: reading where the element sits and why at its size, changing nothing and running no step; a virtual node id answered placed by `app` in its canvas, at the region's offset from the canvas's border box, with the snapshot's bounds, the canvas's shifts and both size causes `layout`)
@leaf_edit n=4 leaf=.claude/docs/commands.md line=58 class=omission
leaf-text: the whole screen unpaged, status `0`)
source: ctr-mfokuw36 «A listed virtual node's verbs are read against its canvas, each cause the one its call returns: a verb that takes no virtual id reads not-found, and click and layout meet the checks their call meets.»
replace-with: the whole screen unpaged, status `0`; a listed virtual node's verbs read against its canvas, a verb taking no virtual id reading `not-found`, `click` and `layout` the checks their call meets)
