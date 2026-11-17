@leaf_group group=masters
@leaf_read leaf=.claude/docs/gotchas.md lines=68
last: - Decisions and rationale: `doc:architecture` (the records s
@leaf_read leaf=CLAUDE.md lines=165
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=34 class=omission
leaf-text: headless `Harness` for blitz documents: virtual time, the run seed
source: ctr-xrs2rifi «Harness::from_component_alone(component, props maker, HarnessOptions) builds a harness over one component mounted as the one child of an anonymous root, the props maker run once in that root's scope»
replace-with: headless `Harness` for blitz documents: a component mounted alone (`Harness::from_component_alone`, `vdom_alone`, the props given as a maker run once in the root's scope), virtual time, the run seed
@leaf_edit n=2 leaf=CLAUDE.md line=122 class=omission
leaf-text: the seven_guis stand boots through it (`seven_guis::stand`) at a pinned viewport with its bundled font,
source: ctr-ymqabqx2 «seven_guis::stand, native only, boots the four lean tasks, Cells, a load page and one component alone in a pinned 800x600 Light viewport with one bundled font.»
replace-with: the seven_guis stand boots through it (`seven_guis::stand`) — the four lean tasks, Cells, a load page and one component alone (`stand::boot_alone`, which no verb, argument, MCP tool, socket call or kept check reaches) — at a pinned viewport with its bundled font,
