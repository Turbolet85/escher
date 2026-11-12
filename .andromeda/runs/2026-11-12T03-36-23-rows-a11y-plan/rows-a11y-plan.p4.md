@row_none n=1 rec=1 type=5

@row_none n=2 rec=2 type=5

@row_none n=3 rec=3 type=5

@row_none n=4 rec=4 type=5

@row_none n=5 rec=5 type=5

@row_none n=6 rec=6 type=5

@row_none n=7 rec=7 type=5

@row_none n=8 rec=8 type=5

@row_none n=9 rec=9 type=5

@row_none n=10 rec=10 type=5

@row_none n=11 rec=11 type=5

@row_none n=12 rec=12 type=5

@row_none n=13 rec=13 type=7

@row_none n=14 rec=14 type=7

@row_none n=15 rec=15 type=7

@row_none n=16 rec=16 type=7

@row_none n=17 rec=17 type=7

@row_none n=18 rec=18 type=7

@row_none n=19 rec=19 type=7

@row_none n=20 rec=20 type=7

@row_add n=1 row=decision/a11y-ci-leg area=ci-integration rec=21
name: The a11y CI leg
rule: The CI job "Accessibility (a11y) tests" runs the accessibility test files, among them the stand's keyboard, contrast and accessibility-id checks, the tree-reading checks and the tier-0 witnesses, on one cargo test line in the slow tier, with no required status check.
why: The existing a11y job was widened rather than a new job, required status check or repository setting added, by the founder's chosen option.
guard: LegScriptTest's a11y tuple

@row_add n=2 row=decision/stand-checks-repeated-three-rounds area=ci-integration rec=21
name: Stand checks repeated three rounds
rule: Every stand_ test file and the named a11y witnesses outside the stand_ prefix run three rounds in the stand-repeat leg under the zero-failure budget.
guard: test_stand_repeat_leg_runs_three_rounds; stand_requirement_sweep

@row_none n=21 rec=22 type=6

@row_none n=22 rec=23 type=4

@row_add n=3 row=decision/invalid-state-not-by-class-alone area=cognitive-accessibility rec=24
name: Invalid date state stated beyond its class
rule: The flight booker's invalid date inputs state aria-invalid beside the invalid class and its red styling, and the Book button is disabled, so the error cue does not rest on colour.
why: So the invalid state is in the accessibility tree and the snapshot and not carried by the class alone.
guard: stand_flight_booker; stand_snapshot_state

@row_add n=4 row=decision/stand-shared-tab-order-table area=cognitive-accessibility rec=24
name: Stand focus order asserted against a shared table
rule: The stand asserts each lean task's forward focus sequence against one shared tab-order table kept in the stand checks' shared module.
guard: stand_accessibility_ids

@row_none n=23 rec=25 type=4

@row_none n=24 rec=26 type=4

@row_none n=25 rec=27 type=4

@row_none n=26 rec=28 type=4

@row_none n=27 rec=29 type=4

@row_add n=5 row=decision/focusable-elements area=keyboard-navigation rec=30
name: What is focusable
rule: An element is focusable if it holds a sub-document, or if it is not inert on itself or an ancestor, is no disabled form control, and has tabindex of zero or more or, with no tabindex, is a link or area with href or a button, input, select, textarea, frame, iframe or summary.
why: The narrowing of disabled to form controls and the inert read were ratified.
guard: stand_keyboard

@row_add n=6 row=decision/disabled-by-presence area=keyboard-navigation rec=30,39
name: Disabled read by attribute presence
rule: The disabled attribute is read by its presence alone on a button, input, select or textarea, any value, so disabled="false" or a bare disabled matches :disabled, is not focusable and takes no activation, while a disabled attribute on a div does not count.
why: The presence reading and its narrowing to form controls were ratified.
guard: focusability_updates; stand_keyboard

@row_add n=7 row=decision/focus-fixup-reads-inert-alone area=keyboard-navigation rec=32,31
name: Focus fixup reads inertness alone
rule: A focused node that becomes inert gives focus up at the flush, in every document, with no DOM blur or focusout firing; disabled and hidden take no part, so a focused control that no longer shows keeps focus and takes keys.
why: The limit of reading inertness alone was ratified.
guard: inert_subtree; key_press_after_container_lost_its_box

@row_none n=28 rec=33 type=9

@row_none n=29 rec=34 type=4
relied: Tab moves focus to the next focusable node and Shift+Tab to the previous one, dispatching focus events.

@row_none n=30 rec=35 type=4
relied: With a select's list open, Tab chooses the current row and closes the list before moving focus, in either direction.

@row_none n=31 rec=36 type=4
relied: Focus stays on a select while its list is open, and the Tab walk holds with the list open and after each close.

@row_none n=32 rec=37 type=4
relied: Cached focusability is recomputed when tabindex, href or disabled is set or removed, so a roving tabindex works.

@row_none n=33 rec=38 type=4

@row_none n=34 rec=40 type=4

@row_none n=35 rec=41 type=4

@row_none n=36 rec=42 type=4

@row_none n=37 rec=43 type=4

@row_none n=38 rec=44 type=4
relied: The :focus-visible selector never matches, while :focus and :focus-within do.

@row_none n=39 rec=45 type=4

@row_none n=40 rec=46 type=4
relied: Removing the focused node resets focus to the body and runs blur side effects.

@row_none n=41 rec=47 type=4

@row_none n=42 rec=48 type=4

@row_add n=8 row=decision/click-focuses-buttons area=keyboard-navigation rec=49
name: A click focuses a button
rule: A pointer click on a button of any type or an input of type submit, reset or button focuses it, submits the form owner when it is a submit button and ends the walk, in every document; a click on any other non-interactive element still clears focus.
why: Ratified.
guard: stand_keyboard; stand_act_obstructed

@row_add n=9 row=decision/disabled-control-stops-click-walk area=keyboard-navigation rec=49
name: A disabled form control stops the click walk
rule: A click on a disabled form control is stopped by the walk before it focuses or acts, while a disabled ancestor div does not stop it.
why: Ratified.

@row_none n=43 rec=50 type=4

@row_none n=44 rec=51 type=4

@row_none n=45 rec=52 type=4

@row_add n=10 row=decision/double-click-on-document-clock area=keyboard-navigation rec=53
name: Double click read on the document's clock
rule: Two pointer clicks on one point within 500 ms and 2 px are a double click, read on the document's clock, a harness-built document using the harness's virtual clock, while each click still focuses its target as one click does.
why: Ratified.
guard: stand_virtual_clock

@row_none n=46 rec=54 type=4

@row_none n=47 rec=55 type=4
