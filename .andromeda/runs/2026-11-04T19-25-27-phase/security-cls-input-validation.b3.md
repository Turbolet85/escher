@answer n=1 entity=input-validation#1.51 v=no
why: SVG width and height parsing in stylo.rs; the scope names no SVG element and the probe reads no legacy-attribute value.
@answer n=2 entity=input-validation#1.52 v=no
why: border and body margin parse as unsigned integers; the scope names no such attribute and the probe reads none.
@answer n=3 entity=input-validation#1.53 v=no
why: textarea rows and cols, replaced-element size, col span and colspan defaults; the scope names no textarea, replaced element or table, and the probe reads none of them.
@answer n=4 entity=input-validation#1.54 v=no
why: rowspan clamp in table layout; the scope names no table and the probe reads no rowspan value.
@answer n=5 entity=input-validation#1.55 v=no
why: multiple-select size hint; the scope names no multiple select and the probe reads no select size.
@answer n=6 entity=input-validation#1.56 v=no
why: ol start parse; the scope names no ordered list and the probe reads no start value.
@answer n=7 entity=input-validation#1.57 v=no
why: attr_parsed returns None on failure for tabindex, which the probe never reads; disabled no longer goes through it.
@answer n=8 entity=input-validation#1.58 v=yes
why: the click reading must honour the presence reading of disabled, since a disabled button gets no click through handle_click's walk stop, so a listener in the map does not mean the button reacts.
@answer n=9 entity=input-validation#1.59 v=no
why: open on details and dialog; the scope names no details, dialog or summary toggle and the probe reads no open state.
@answer n=10 entity=input-validation#1.60 v=no
why: autofocus on a focusable element; the scope names no autofocus attribute.
@answer n=11 entity=input-validation#1.61 v=no
why: dir matching is case-insensitive; the scope names no dir attribute and the probe reads no direction.
@answer n=12 entity=input-validation#1.62 v=no
why: canvas src parses as u64; the scope names no canvas element.
@answer n=13 entity=input-validation#1.63 v=no
why: custom-widget attribute replay sits behind the custom-widget feature, which the scope does not name.
@answer n=14 entity=input-validation#1.64 v=yes
why: this is the element-id derivation the probe reads for node to component (author key, else anchored, component or document path) and its rule that the id is written to no log.
@answer n=15 entity=input-validation#1.65 v=no
why: accessible names reach only the platform adapter, the snapshot, the driver and MCP, none of which the probe runs or reads.
@answer n=16 entity=input-validation#1.66 v=no
why: aria-describedby and aria-description descriptions reach only the platform adapter, which the probe does not build or read.
@answer n=17 entity=input-validation#1.67 v=no
why: aria-expanded, aria-disabled and aria-level closed parses reach only the accessibility tree and the snapshot, which the probe does not read.
@answer n=18 entity=input-validation#1.68 v=no
why: open and aria-expanded on a select steer nothing; the scope names no select's open state.
@answer n=19 entity=input-validation#1.69 v=no
why: aria-selected, aria-pressed, aria-checked and aria-invalid states reach only the platform adapter and the snapshot, which the probe does not read.
@answer n=20 entity=input-validation#1.70 v=no
why: a native checkbox's or radio's checked state in the tree reaches only the adapter and the snapshot; the probe reads no checked state and no control value.
@answer n=21 entity=input-validation#1.71 v=no
why: inert gates hit test and focus, but the scope names no inert element for the probe's clicks or focus to meet.
@answer n=22 entity=input-validation#1.72 v=no
why: which select opens a list and what a row takes; the scope names no select and the probe drives none.
@answer n=23 entity=input-validation#1.73 v=no
why: the tree's focus value and the inert focus clear; the probe reads no accessibility tree and no inert focus.
@answer n=24 entity=input-validation#1.74 v=no
why: range input min, max, step, value and its key, pointer and click defaults; the scope names no range input and the probe drives none.
@answer n=25 entity=input-validation#1.75 v=yes
why: a click reaching a handler may come from a focused button's Enter or Space, which dispatches the same Click through handle_click, so the click reading must hold that route.
