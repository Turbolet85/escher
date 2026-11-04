@answer n=1 entity=input-validation#1.51 v=no
why: the SVG width and height rule lives in blitz-dom stylo.rs, and the session changed no package file
@answer n=2 entity=input-validation#1.52 v=no
why: the border and body margin parse is in blitz-dom stylo.rs, which the session left unchanged
@answer n=3 entity=input-validation#1.53 v=no
why: the markup numeric attribute fallbacks sit in blitz-dom layout files the session did not change
@answer n=4 entity=input-validation#1.54 v=no
why: the rowspan clamp is in blitz-dom table.rs, which the session did not change
@answer n=5 entity=input-validation#1.55 v=no
why: the multiple select size read is in blitz-dom select_list.rs and stylo.rs, both unchanged by the session
@answer n=6 entity=input-validation#1.56 v=no
why: the ol start parse is in blitz-dom construct.rs, which the session did not change
@answer n=7 entity=input-validation#1.57 v=no
why: attr_parsed in blitz-dom element.rs is unchanged, and the session touches no package code
@answer n=8 entity=input-validation#1.58 v=no
why: the disabled presence rule lives in blitz-dom and dioxus-native-dom code the session left alone
@answer n=9 entity=input-validation#1.59 v=no
why: the open state read lives in blitz-dom element.rs and mutator.rs, which the session did not change
@answer n=10 entity=input-validation#1.60 v=no
why: the autofocus attribute path in blitz-dom mutator.rs is untouched by the session
@answer n=11 entity=input-validation#1.61 v=no
why: the dir attribute match is in blitz-dom engine code the session did not change
@answer n=12 entity=input-validation#1.62 v=no
why: the canvas src parse in blitz-dom mutator.rs is unchanged by the session
@answer n=13 entity=input-validation#1.63 v=no
why: the custom widget attribute replay in blitz-dom element.rs and mutator.rs is untouched
@answer n=14 entity=input-validation#1.64 v=yes mode=stale
why: the session makes no .andromeda/residuals.md stand, so the residuals.md line this row cites for the a11y witness is a stale path
@answer n=15 entity=input-validation#1.65 v=no
why: the aria-label and label reads are in blitz-dom accessibility.rs, which the session left unchanged, and the new target is not an accessibility_label test
@answer n=16 entity=input-validation#1.66 v=yes mode=stale
why: the session makes no .andromeda/residuals.md stand, so the residuals.md line this row cites for its witness is a stale path
@answer n=17 entity=input-validation#1.67 v=no
why: the aria-expanded, aria-disabled and aria-level reads sit in blitz-dom accessibility.rs, unchanged by the session
@answer n=18 entity=input-validation#1.68 v=no
why: the select open and aria-expanded steering is engine code the session did not change
@answer n=19 entity=input-validation#1.69 v=yes mode=stale
why: the session makes no .andromeda/residuals.md stand, so the residuals.md line this row cites for the adapter witness is a stale path
@answer n=20 entity=input-validation#1.70 v=no
why: the native checkbox and radio checked read is in blitz-dom accessibility.rs, which the session left alone
@answer n=21 entity=input-validation#1.71 v=no
why: the inert check lives in blitz-dom node.rs, mutator.rs and accessibility.rs, none changed by the session
@answer n=22 entity=input-validation#1.72 v=no
why: the select list gate in blitz-dom document.rs is untouched by the session
@answer n=23 entity=input-validation#1.73 v=no
why: the tree focus write and the inert focus clear are in blitz-dom accessibility.rs and mutator.rs, both unchanged
@answer n=24 entity=input-validation#1.74 v=no
why: the range input rules are in blitz-dom range.rs and event files the session did not touch
@answer n=25 entity=input-validation#1.75 v=no
why: the Enter and Space activation arms are in blitz-dom keyboard.rs and events mod.rs, unchanged by the session
