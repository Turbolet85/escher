@row_set n=1 row=contract/ci-leg-legs/a11y cell=promise seen=1 detector=none
value: The `a11y` leg runs eighteen named accessibility integration tests on one `cargo test --workspace --locked` line; other focus and accessibility tests stay in the `test` leg.
basis: fct-mekgbk34
because: The a11y leg's named targets moved from fourteen to eighteen, so the contract's stated count must follow.
@row_set n=2 row=decision/removed-node-state-cleared cell=rule seen=1 fact=F2 detector=none
value: Interaction state on removed nodes is cleared or retargeted; a focused node removed from the document drops focus to the root element, the accessibility tree's focus then naming the window.
basis: fct-buw3tqhj
because: The chunk measured and pinned what a removed focused node leaves behind, which the row states only as cleared or retargeted.
@row_add n=1 row=decision/hidden-focused-control-keeps-focus area=established-decisions fact=F1 detector=none
name: A focused control that no longer shows keeps focus
rule: A focused control whose box goes away by visibility hidden, display none or a focus-within rule keeps engine focus while the accessibility tree's focus names the window, and the next Tab moves on to the following control.
why: a named SC 2.4.3 gap, kept as a stated limit
guard: a_focused_control_that_no_longer_shows_keeps_focus_the_tree_naming_the_window
basis: fct-7tsj26zm
because: The chunk pins the hidden-focus limit as a named SC 2.4.3 gap, a stated limit future work must know, which no row says.
