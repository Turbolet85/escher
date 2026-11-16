@row_add n=1 row=decision/reversed-box-negative-scroll-offsets area=surface-desktop-native fact=F1 detector=none
name: Reversed boxes scroll to negative offsets
rule: A reversed box (RTL, row-reverse, column-reverse, wrap-reverse) scrolls to negative offsets in every document: scroll width and height answer the CSSOM scroll size, a programmatic scroll clamps to the scroll range, and an into-view scroll skips a box whose range is zero at both ends.
why: Taken from upstream's reversed scroll origins and ratified.
basis: fct-w7kvqm7c
because: The chunk takes upstream's reversed scroll origins, a ratified behaviour change for every document that no row of the area states, and the into-view row's every-scroller rule stands true beside it.
