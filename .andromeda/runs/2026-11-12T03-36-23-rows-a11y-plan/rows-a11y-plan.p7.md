@row_none n=1 rec=1 type=5
@row_add n=1 row=decision/active-descendant-kept-unread area=screen-reader-support rec=2
name: Missing active descendant kept unread on the CRUD list
rule: The CRUD list's rows are not focusable and the accessibility tree's focus stays on the list, never the option; the missing aria-activedescendant is kept unread.
why: ratified
guard: accessibility_active_descendant_unread
@row_none n=2 rec=3 type=4
@row_add n=2 row=contract/accessibility-tree/root area=screen-reader-support rec=4
boundary: the accessibility tree
item: root
promise: The tree root is a Window node with id u64::MAX.
@row_add n=3 row=contract/accessibility-tree/focus area=screen-reader-support rec=4
boundary: the accessibility tree
item: focus
promise: Tree focus is the focused DOM node only where that node is among the built nodes, else u64::MAX, the window, so the tree never names a node it did not build.
@row_none n=3 rec=5 type=0
what: a test's stated goal, with no rule of its own
@row_none n=4 rec=6 type=4
relied: Callers receive no initial tree synchronously; initial-tree requests, action requests and deactivation all reach one EventHandler.
@row_none n=5 rec=7 type=4
@row_add n=4 row=decision/accessibility-feature-gate area=screen-reader-support rec=8
name: Screen-reader integration behind the accessibility feature
rule: Screen-reader integration through AccessKit is exposed only behind the `accessibility` feature, in blitz via blitz-shell and in the Dioxus crates.
@row_none n=6 rec=9 type=4
@row_none n=7 rec=10 type=4
@row_none n=8 rec=11 type=5
@row_none n=9 rec=12 type=5
@row_none n=10 rec=13 type=7
@row_none n=11 rec=14 type=5
@row_none n=12 rec=15 type=5
@row_none n=13 rec=16 type=5
@row_none n=14 rec=17 type=5
@row_none n=15 rec=18 type=5
@row_none n=16 rec=19 type=5
@row_none n=17 rec=20 type=4
@row_none n=18 rec=21 type=4
relied: A text input already focused when its text editor is constructed has IME enabled with no fresh focus, in every document.
@row_none n=19 rec=22 type=4
@row_none n=20 rec=23 type=7
@row_add n=5 row=decision/wcag-contrast-ratio-helper area=visual-design-verification rec=24
name: WCAG contrast ratio matching Chrome
rule: Border bevels and scrollbar thumbs use a WCAG contrast ratio helper that matches Chrome's GetContrastRatio.
@row_add n=6 row=decision/scrollbar-thumb-contrast-stroke area=visual-design-verification rec=25
name: Contrast stroke on default scrollbar thumbs
rule: Default scrollbar thumbs carry a thin contrast stroke.
why: so the thumbs read over same-coloured content
@row_add n=7 row=decision/author-thumb-visible-state-change area=visual-design-verification rec=26
name: Near-white author thumb visibly changes state
rule: A near-white author scrollbar thumb must still visibly change on hover and on drag.
@row_add n=8 row=decision/text-contrast-required-ratio area=visual-design-verification rec=27
name: Text contrast required ratio per SC 1.4.3
rule: Text contrast is the WCAG 2.x ratio of the text colour against the background composited source-over down to the first opaque layer, and 3.0 is required of large text (at least 24 px, or 14 pt at weight 700 or more) and 4.5 of other text.
@row_add n=9 row=decision/contrast-unread-past-non-colour-paint area=visual-design-verification rec=27,28
name: Contrast unread past non-colour paint
rule: The text contrast reading reads nothing where an element on its walk has opacity below 1, a non-empty filter, a background-image layer or a mix-blend-mode other than normal.
guard: a_blended_ancestor_reads_none_and_an_unblended_one_its_pair
@row_add n=10 row=decision/stand-accent-colour area=visual-design-verification rec=27
name: Stand accent colour
rule: The stand's accent is rest #3a5ce5, hover #2a4cd3, active #1f3fc1, stand-wide in every task CSS that declares it, a stand change and no engine change.
why: white on the earlier accent #4a6cf7 read 4.39 against 4.5, failing
@row_add n=11 row=decision/contrast-exempt-disabled-controls area=visual-design-verification rec=27
name: Disabled controls exempt from contrast
rule: A contrast row is exempt exactly where the snapshot's enabled reads Some(false); a label[for=…] row is never exempt.
@row_add n=12 row=decision/stand-contrast-asserted area=visual-design-verification rec=27,30
name: SC 1.4.3 asserted on the stand
rule: SC 1.4.3 is asserted on every measured non-exempt stand row, in both layout modes, on the four lean tasks, the load page and Cells, and the check runs in the workspace test leg, per package and in the a11y leg.
guard: stand_contrast
@row_add n=13 row=decision/lint-contrast-shares-one-reader area=visual-design-verification rec=31,32
name: Lint contrast shares the harness reader
rule: The visual lint's contrast reading uses the harness's one contrast reader, its required ratio and the stand check's exempt rule, so a blended text's pair is unread by the lint as well.
why: no second formula or threshold
@row_add n=14 row=contract/lint-contrast-finding/id area=visual-design-verification rec=31
boundary: the lint's contrast finding
item: id
promise: A contrast finding carries the id of the node read below its required ratio.
@row_add n=15 row=contract/lint-contrast-finding/defect area=visual-design-verification rec=31
boundary: the lint's contrast finding
item: defect
promise: A contrast finding carries its defect as a closed word.
@row_add n=16 row=contract/lint-contrast-finding/bounds area=visual-design-verification rec=31
boundary: the lint's contrast finding
item: bounds
promise: A contrast finding carries the node's bounds as numbers.
@row_add n=17 row=contract/lint-contrast-finding/ratio area=visual-design-verification rec=31
boundary: the lint's contrast finding
item: ratio
promise: A contrast finding carries the measured contrast ratio as a number.
@row_add n=18 row=contract/lint-contrast-finding/required area=visual-design-verification rec=31
boundary: the lint's contrast finding
item: required
promise: A contrast finding carries the required contrast ratio as a number.
@row_add n=19 row=decision/stand-lint-clean-at-boot area=visual-design-verification rec=31,33
name: Stand reads no lint finding at boot
rule: The stand at boot reads no visual lint finding of any kind on the four lean tasks, in both layout modes.
guard: the_stand_at_boot_reads_its_measured_findings
@row_add n=20 row=decision/highlighted-row-contrast-unread area=visual-design-verification rec=34
name: Highlighted list row contrast unread by the lint
rule: The visual lint's contrast reads an open list row's plain pair even when the row is highlighted, so a highlighted row's contrast is unread by the lint, a stated limit.
@row_none n=21 rec=29 type=4
@row_none n=22 rec=35 type=4
@row_none n=23 rec=36 type=5
@row_add n=21 row=decision/multiple-select-chosen-row-pair area=visual-design-verification rec=37
name: Multiple select chosen-row colour pair
rule: A multiple select's chosen rows read #000000 on #c8c8c8, Chromium's inactive selected pair, hardcoded in the default sheet.
why: hardcoded by design, as the engine holds no tokens
guard: select_interaction
@row_add n=22 row=decision/select-list-colours-engine-constants area=visual-design-verification rec=38
name: Open select list colours are engine constants
rule: The open select list's highlight text, highlight background, disabled text and border colours are engine constants, not tokens.
why: the engine holds no tokens
@row_add n=23 row=decision/select-list-rows-one-height area=visual-design-verification rec=39
name: Open select list rows take one height
rule: An open select list's rows take one height, the select's font on one line, so a row with a larger authored font clips, a stated limit.
@row_none n=24 rec=40 type=7
@row_none n=25 rec=41 type=7
@row_none n=26 rec=42 type=5
@row_none n=27 rec=43 type=5
@row_none n=28 rec=44 type=5
@row_none n=29 rec=45 type=5
@row_none n=30 rec=46 type=4
@row_none n=31 rec=47 type=5
@row_none n=32 rec=48 type=5
@row_none n=33 rec=49 type=4
@row_none n=34 rec=50 type=5
@row_none n=35 rec=51 type=4
@row_none n=36 rec=52 type=5
@row_none n=37 rec=53 type=7
@row_add n=24 row=absence/select-list-motion area=visual-design-verification rec=54
capability: Motion on an open select's list
@row_none n=38 rec=55 type=4
