@row_set n=1 row=decision/engine-dependency-pins cell=rule seen=2 detector=D-security-deps
value: The pins stay: Stylo 0.22.0, Taffy the git pin of upstream DioxusLabs/taffy at commit a3997d55, Dioxus 0.7.3 and winit =0.31.0-beta.3.
basis: fct-vduzvhfi
because: The Taffy git rev moved from e90751fd to a3997d55 of upstream DioxusLabs/taffy, so the pinned commit the row names must follow.
@row_add n=1 row=decision/reversed-scroll-origins area=input-validation fact=F1 detector=none
name: Reversed boxes scroll to negative offsets
rule: In every document, remote pages included, a reversed box (RTL, row-reverse, column-reverse, wrap-reverse) scrolls to negative offsets, and scroll width and height answer the CSSOM scroll size.
why: An engine change for every document, ratified.
guard: inline_box_scrollable_overflow
basis: fct-w7kvqm7c
because: The take measures a box's content against Taffy's scrollable overflow, a ratified behaviour change for every document that no row of the section states.
@row_add n=2 row=decision/unstyled-node-hit-and-geometry-no-panic area=error-handling fact=F3 detector=none
name: Node with no computed styles read without panic
rule: A node with no computed styles yet is hit-testable and reads as no scroll container, so geometry and hit reads on it never panic.
why: The scroll-container test on a node with no computed styles otherwise panics.
guard: hit_testing_before_the_first_resolve
basis: fct-w7kvqm7c
because: The take makes hit testing reach nodes with no computed styles and answers false for their scroll-container test, a no-panic rule no row of the section states.
@row_none n=1 fact=F2 type=4
relied: A caller asking for a node's layout data before layout gets none and nothing is written.
