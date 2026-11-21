@leaf_group group=architecture+security-plan+design-system+test-plan+obs-plan+a11y-plan
@leaf_read leaf=.claude/docs/gotchas.md lines=68
last: - Decisions and rationale: `doc:architecture` (the records st
@leaf_read leaf=CLAUDE.md lines=166
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=87 class=stale
leaf-text: a single select's open list `packages/blitz-dom/src/node/select_list.rs`
source: dec-sl5sqsqp «A drop-down, a single select of display size 1, opens a list the engine paints over the page, by a click, by Enter, or by a Space with no typed text standing.»
replace-with: a drop-down's open list `packages/blitz-dom/src/node/select_list.rs`
@leaf_edit n=2 leaf=CLAUDE.md line=122 class=stale
leaf-text: a single select opens a list the engine paints over the page by a click, by Enter, or by a Space with no typed text standing, a direct engine change
source: dec-sl5sqsqp «A drop-down, a single select of display size 1, opens a list the engine paints over the page, by a click, by Enter, or by a Space with no typed text standing.»
replace-with: a drop-down, a single select of display size 1, opens a list the engine paints over the page by a click, by Enter, or by a Space with no typed text standing (a single select sized above 1 is an in-flow list box and opens none), a direct engine change
@leaf_edit n=3 leaf=CLAUDE.md line=122 class=stale
leaf-text: a closed select's options its client rect and the lint's `overlaps` skipping them, a `multiple` select's rows reading their own client rects and linted as usual
source: ctr-s6q6q2ec «A closed drop-down's options, read by the element and never the role, and an open list's rows and groups are skipped by `overlap` on either side; a list box's rows are linted as usual.»
replace-with: a closed drop-down's options its client rect and the lint's `overlaps` skipping them, a list box's rows, multiple or of display size above 1, reading their own client rects and linted as usual
@leaf_edit n=4 leaf=CLAUDE.md line=122 class=omission
leaf-text: `BaseDocument::get_client_bounding_rect` reads an element where its transforms move it (ratified by the founder, 2026-10-18)
source: ctr-z5p2evku «get_client_bounding_rect reads an element where its transforms move it, and a node a restyle turned display none keeps the client rect it was last laid out at.»
replace-with: `BaseDocument::get_client_bounding_rect` reads an element where its transforms move it (ratified by the founder, 2026-10-18), and a node a restyle turned display none keeps the client rect it was last laid out at
