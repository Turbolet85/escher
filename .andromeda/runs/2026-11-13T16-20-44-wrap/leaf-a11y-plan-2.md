@leaf_group group=a11y-plan-2
@leaf_read leaf=.claude/rules/a11y.md lines=46
last: - 2026-10-06: On the dev host the AT-SPI bus reads `IsEnabl
@leaf_edit n=1 leaf=.claude/rules/a11y.md line=27 class=stale
leaf-text: node creation, hover, active, scroll and layout marking none
source: dec-3xjtbjfc «a single select's list opening or closing, a scroll that closes a list included; node creation, hover, active, any other scroll and layout do not.»
replace-with: a scroll that closes a select's list marking it, node creation, hover, active, any other scroll and layout marking none
@leaf_edit n=2 leaf=.claude/rules/a11y.md line=23 class=omission
leaf-text: only `<html>` and `<body>` should map to `Unknown` on a semantic page.
source: dec-xnppuiyn «An img whose alt is the empty string is decorative and reads the generic container role in the accessibility tree, not image.»
replace-with: only `<html>` and `<body>` should map to `Unknown` on a semantic page; an `img` whose `alt` is the empty string is decorative and reads the generic container role, not image (upstream's behaviour, `an_image_with_an_empty_alt_is_decorative` its pin).
@leaf_edit n=3 leaf=.claude/rules/a11y.md line=33 class=omission
leaf-text: a click on a checkbox or radio toggles it, dispatches `input` and moves focus to it
source: dec-iinqqgma «A native checkbox or radio keeps its checked state with no box, so a click on its label toggles it even when the control is hidden by display none.»
replace-with: a click on a checkbox or radio toggles it, dispatches `input` and moves focus to it (a native checkbox or radio keeps its checked state with no box, so a click on its label toggles it even when it is hidden by `display: none`, `clicking_label_toggles_hidden_checkbox` its pin)
