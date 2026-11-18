@row_add n=1 row=decision/unshown-focused-control-keeps-focus area=surface fact=F1 detector=none
name: Focused control that no longer shows keeps focus
rule: A focused control that no longer shows, hidden, display none or hidden by its own focus-within, keeps engine focus and its keys while the accessibility tree's focus names the window, and the next Tab moves to the control after it, in both render modes.
why: a named SC 2.4.3 limit, stated as a gap rather than fixed
guard: a_focused_control_that_no_longer_shows_keeps_focus_the_tree_naming_the_window
basis: fct-7tsj26zm
because: The chunk pins the hidden-focus limit for any focused control as a named SC 2.4.3 gap in both modes, wider than the standing select-only row, so the surface gains the limit as its own decision.
@row_none n=1 fact=F2 type=4
relied: a focused node a page unmounts drops engine focus to the root and the tree's focus to the window, which the Cells editor's keyboard check relies on to never reach the hidden-focus limit; the code and its test hold it, no reason stated.
