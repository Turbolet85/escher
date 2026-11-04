@verdict n=1 ref_file=fanout-architecture--cascade-g1.md ref_n=7 v=distinct
why: no standing group states that the bridge's listener attributes and mark outlive a removed listener or that the vdom walk reads listeners exactly, the anchor's placeholder clause stating registration only
@verdict n=2 ref_file=fanout-architecture--cascade-g1.md ref_n=8 v=distinct
why: no standing group states when dioxus-core 0.7.10 reaches remove_event_listener or that a listener turned to none is written as an on{event} attribute
@verdict n=3 ref_file=cascade-compare-architecture.md ref_n=3 v=twin to_file=fanout-architecture--cascade-g1.md to_n=7
why: its over-report of a removed listener's attribute and mark, the mounted case and the exact vdom reading are the fact proposal 7 states (with proposal 8's spread clause folded in), placed at another anchor
