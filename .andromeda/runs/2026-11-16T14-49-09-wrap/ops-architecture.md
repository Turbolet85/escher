@row_set n=1 row=decision/programmatic-scroll-one-box cell=rule seen=1 fact=F2 detector=none
value: `scroll_to` and `scroll_by` clamp to one box, between that box's scroll minimum and maximum, a signed range that runs negative on a reversed box.
basis: fct-w7kvqm7c
because: The scroll state now carries a min and a max and the programmatic scroll clamps between them, so the row's clamp must name the signed range it clamps to.
@row_set n=2 row=contract/scroll-hit/hit-clip cell=promise seen=1 fact=F3 detector=none
value: A hit stops at the padding box of a box paint clips, and a hit stopped there answers in that box's content-box frame, the frame of the final self hit and the text hits.
basis: fct-w7kvqm7c
because: The clip stop's early return now answers content-box coordinates, so the promise of a hit on a clipping box must say the frame it answers in.
@row_set n=3 row=decision/client-rect-own-box cell=why seen=1 fact=F6 detector=D-arch-decisions
value: ratified as a direct engine change for every document; differs from upstream, whose scroll-shifted geometry and scroll-shifted self match are kept out
basis: fct-p2ax2w7w
because: The take keeps upstream's scroll-shifted geometry body and self match out to hold the own-scroll rule, so the row's reason must name that difference from upstream.
@row_set n=4 row=contract/scroll-hit/scroll-into-view cell=promise seen=1 fact=F7 detector=none
value: scroll_into_view scrolls every scrolling box on the target's chain, skipping a box whose scroll min and max are both zero, nested ones instantly with no scroll event, then the viewport; a row of an open select list writes the list's scroll_y at once and moves no chain or viewport.
basis: fct-w7kvqm7c
because: scroll_into_view now reads the scroll range and skips a box whose min and max are both zero, so the promise must state which boxes it passes over.
@row_set n=5 row=decision/scenarios-both-layout-modes cell=except seen=1 fact=F9 detector=none
value: upstream-taken blitz-tests targets are kept as taken even where they run one layout mode; escher's own targets run both
basis: fct-r6rg3azk
because: Upstream's six added inline_box_scrollable_overflow tests are kept as written in one layout mode, an exception this row states nowhere.
@row_retire n=1 row=decision/deferred-upstream-dependency-bumps seen=3 heir=decision/deferred-stylo-dioxus-bumps detector=D-arch-decisions
why: Taffy from #1372 on is taken at a3997d55, so the row's Taffy deferral and its name no longer stand.
basis: fct-vduzvhfi
because: The Taffy pin moved past #1372 with its reversed-scroll-origin change taken, ending the deferral the row records, while the Stylo and Dioxus deferrals stand.
@row_add n=1 row=decision/reversed-scroll-origins area=established-decisions fact=F1 detector=D-arch-decisions
name: Reversed boxes scroll to negative offsets
rule: A box's content is measured against Taffy's scrollable overflow for every document, so a reversed box (RTL, row-reverse, column-reverse, wrap-reverse) scrolls to negative offsets.
why: an upstream behaviour change taken, ratified for its take
basis: fct-w7kvqm7c
because: The take changes how every document measures and scrolls a reversed box, a ratified decision no row states.
@row_add n=2 row=decision/unstyled-elements-hit-tested area=established-decisions fact=F4 detector=none
name: Elements without computed styles are hit tested
rule: Elements with no computed styles yet are hit tested, and such a node is never a scroll container.
why: an upstream behaviour change taken, ratified for its take
basis: fct-w7kvqm7c
because: The take hit tests elements before their styles are computed and reads such a node as no scroll container, a behaviour for every document no row states.
@row_add n=3 row=decision/scroll-size-readings area=established-decisions fact=F5 detector=none
name: CSSOM scroll size for scrollWidth and scrollHeight
rule: scrollWidth and scrollHeight answer the CSSOM scroll size, reversed origins included, while the Dioxus mounted-data scroll size stays on Taffy's extent.
basis: fct-p2ax2w7w
because: The node's scroll width and height now answer the CSSOM scroll size while the mounted-data reading was deliberately left on Taffy's extent, a split no row states.
@row_add n=4 row=contract/nodes-widgets/try-layout-data area=standard-contracts fact=F8 detector=D-arch-resources
boundary: blitz-dom nodes and widgets
item: Node::try_layout_data
promise: Public and read-only: a node's layout data, None before the first layout.
basis: fct-w7kvqm7c
because: The chunk widens blitz-dom's public API by one read accessor, which the registry of standard contracts does not yet hold.
@row_add n=5 row=decision/deferred-stylo-dioxus-bumps area=project-intent detector=D-arch-decisions
name: Deferred Stylo and Dioxus bumps
rule: Stylo stays "0.22.0" and Dioxus "0.7.3": git Stylo 0.23-pre and Dioxus 0.8.0-alpha.3 wait for escher's next version.
basis: fct-vduzvhfi
because: The heir keeps the Stylo and Dioxus deferrals of the retired row once its Taffy deferral ended with the bump to a3997d55.
