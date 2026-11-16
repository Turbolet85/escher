@row_set n=1 row=decision/layout-tests-in-both-layout-modes cell=except seen=2 fact=F1 detector=D-tests-coverage
value: Upstream integration targets taken in a sync are kept as written, and those that run one layout mode stand as named exceptions, upstream's reversed-scroll-origin tests among them.
basis: fct-onh4d7vj (test-plan §5: upstream's inline_box_scrollable_overflow tests kept as written, one layout mode); fct-oi7xanpr
because: The take brings upstream's reversed-scroll-origin integration tests as written in one layout mode, so the row's exception must name them as one of its named exceptions.
@row_none n=1 fact=F2 type=4
relied: The WPT conformance runner's checkLayout scroll size checks compare against the CSSOM scroll size a page reads, reversed origins included.
