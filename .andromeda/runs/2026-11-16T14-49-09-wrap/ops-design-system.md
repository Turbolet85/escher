@row_add n=1 row=decision/reversed-boxes-scroll-to-negative-offsets area=motion fact=F1 detector=none
name: Reversed boxes scroll to negative offsets
rule: A reversed box (RTL, row-reverse, column-reverse, wrap-reverse) scrolls to negative offsets, its content measured against Taffy's scrollable overflow, for every document, remote pages included.
why: Ratified; upstream's reversed scroll origins taken whole.
guard: reversed_flex_origins_scroll_to_negative_offsets_and_hit_the_painted_child
basis: fct-w7kvqm7c
because: The take changes how every document's reversed boxes scroll, a ratified behaviour change no motion row states yet.
@row_add n=2 row=decision/scroll-size-is-cssom-scroll-size area=surface fact=F6 detector=none
name: Scroll size answers the CSSOM scroll size
rule: An element's scrollWidth and scrollHeight answer the CSSOM scroll size, reversed scroll origins included, and touch-action pan axes read scroll width over client width.
except: MountedData's scroll size, left on Taffy's extent.
basis: fct-w7kvqm7c
because: The take moves the scroll size a page reads to the CSSOM scroll size with one named exception, a convention no surface row states yet.
@row_none n=1 fact=F2 type=4
relied: Programmatic scroll clamps to a signed scroll range of min and max, and into-view scroll skips a box whose range is zero on both ends.
@row_none n=2 fact=F3 type=4
relied: The overlay scrollbar thumb runs over the signed scroll range, starting a reversed box's thumb at its negative extreme.
@row_none n=3 fact=F4 type=4
relied: A hit stopped at an inline root's paint clip answers in content-box coordinates, the frame of the final self hit and the text hits.
@row_none n=4 fact=F5 type=4
relied: Elements with no computed styles yet are hit-tested before the first style resolve and are never scroll containers.
