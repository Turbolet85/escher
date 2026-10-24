# Classified — security-plan · input-validation.b3

## Answers
input-validation#1.51 · yes · stale · construct.rs gained lines at 6, 23 and 29 above the list-start read, so its cite at 494-500 shifts
input-validation#1.52 · yes · stale · the take adds ElementData::classes and its sync in element.rs, so the attr_parsed cites at 483-486 and 715-717 shift
input-validation#1.53 · yes · stale · element.rs gains the ElementData::classes lines, so the disabled cites at 463-501 and 716 shift while the presence read stays untouched
input-validation#1.54 · yes · stale · mutator.rs gained lines at 375-378 and 499-502, so the autofocus cite at 1106-1118 shifts while the autofocus logic stays untouched
input-validation#1.55 · no · its only cite is the test file dir_attribute.rs, which the take did not move
input-validation#1.56 · yes · stale · mutator.rs gained lines at 375-378 and 499-502, so the canvas src cite at 1397-1408 shifts while the u64 parse stays untouched
input-validation#1.57 · no · the stable id derivation is untouched and its cites sit in dioxus-native-dom, escher-driver and stand files the take did not move
input-validation#1.58 · no · the accessible-name reads are untouched and none of its cite files is in the take's moved list
input-validation#1.59 · no · the aria-describedby and aria-description reads are untouched and none of its cite files is in the take's moved list
input-validation#1.60 · no · the aria-expanded, aria-disabled and aria-level reads are untouched and none of its cite files is in the take's moved list
input-validation#1.61 · no · the aria-selected, aria-pressed, aria-checked and aria-invalid reads are untouched and none of its cite files is in the take's moved list
input-validation#1.62 · yes · stale · node.rs gained lines at 125-128 and from 425 on, so its cites at 558-575 and 1424-1433 shift while the inert read stays untouched
input-validation#1.63 · yes · stale · mutator.rs gained lines at 375-378 and 499-502, so its cites at 384-386 and 517-524 shift while the range rules stay untouched
input-validation#1.64 · no · keyboard activation is untouched and its cite files are not in the take's moved list
