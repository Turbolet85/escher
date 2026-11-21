@row_set n=1 row=decision/mutation-controlled-tests cell=rule seen=2 fact=F4 detector=none
value: A new check is controlled: a named mutation of what it guards turns it red, a case it leaves green widened until it does, and the restored tree reads green; a control removing a bound loosens it to twice the budget instead, so the mutated run stays bounded.
basis: fct-4bie5mcx, fct-m2ceszre, fct-f6ipvs3u
because: The popup gate's control read DOES NOT HOLD on its first case, since other arms route around the guard, and the case was widened until the guard's own reading showed, a convention the control rule now states.
@row_add n=1 row=decision/box-presence-read-with-display area=3-test-harness-contract fact=F1 detector=none
name: Box presence read with computed display
rule: A check asking whether a node has a box reads the node's computed display beside its client rect.
why: A node a restyle turns display none keeps the client rect it last laid out at.
basis: fct-4cvifvmq, fct-dgz34bi3, fct-m2ceszre
because: The run measured that a node turned display none keeps its stale client rect, so a box-presence check reads the computed display beside the rect, a convention no row states.
@row_add n=2 row=decision/scrolling-box-boot-lint-pinned area=integration-test-strategy fact=F2 detector=none
name: Boot lint of a scrolling box pinned to its findings
rule: A check over a page holding a scrolling list box pins the boot lint's clipped, truncated and off-screen findings on its scrolled-out rows rather than zero findings.
why: The lint reads any scrolling box's hidden rows as clipped, truncated or off-screen.
guard: a_sized_single_selects_page_lints_only_its_rows_scrolled_out_at_boot
basis: fct-m2ceszre, fct-g7zknho2
because: The plan's zero-finding boot lint was disproved by measurement on the list-box page, and the check pins the scrolled-out rows' findings, a convention no row states.
@row_add n=3 row=decision/moved-fixture-select-controlled area=test-data-fixtures fact=F5 detector=none
name: Fixture select whose choice moves is controlled
rule: A Dioxus fixture select whose choice a check moves is written as a controlled select.
why: A Dioxus fixture's selected literal is re-written on any re-render of its component, re-seeding the choice.
basis: fct-cvqtjad7, fct-54acpmv7
because: The run measured a selected literal re-seeding the choice on a re-render, so the sized fixture select was made controlled, a fixture convention no row states.
@row_none n=1 fact=F3 type=4
relied: A click on a list box's centre row lands on that row and is accepted whatever the closed-select arm reads, so a covered-refusal control clicks a row off the select's centre.
