@row_set n=1 row=decision/changed-set-marks cell=rule seen=1 detector=none
value: The engine's changed set marks a mutation of an in-document node, a focus or checked change, a text control's input, a select's changed choice and a single select's list opening or closing, a scroll that closes a list included; node creation, hover, active, any other scroll and layout do not.
basis: fct-uluruybu
because: The changed-set never-list takes the select-list qualifier by the operator's direction, since a scroll that closes a list marks its select, so the row's flat "scroll does not" no longer holds.
@row_add n=1 row=decision/empty-alt-image-decorative area=7-screen-reader-support fact=F1 detector=none
name: Empty-alt image read as decorative
rule: An img whose alt is the empty string is decorative and reads the generic container role in the accessibility tree, not image.
why: Ratified as upstream's behaviour.
guard: an_image_with_an_empty_alt_is_decorative
basis: fct-xve6mmf5
because: The take makes the accessibility builder read an empty-alt img as a generic container, a behaviour ratified as upstream's that no row of the plan states.
@row_add n=2 row=decision/hidden-checkable-label-toggle area=keyboard-navigation fact=F2 detector=none
name: Native checkable state kept with no box
rule: A native checkbox or radio keeps its checked state with no box, so a click on its label toggles it even when the control is hidden by display none.
why: Ratified as upstream's behaviour.
guard: clicking_label_toggles_hidden_checkbox
basis: fct-xve6mmf5
because: The take makes the mutator keep a checkbox or radio's checked state with no box, so a label click toggles a hidden control, a ratified behaviour no row of the plan states.
