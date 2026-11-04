@answer n=1 entity=input-validation#1.51 v=no
why: SVG width and height parsing is an engine parse guard that the probe's four-link reads never reach
@answer n=2 entity=input-validation#1.52 v=no
why: border and body margin integer parsing sits outside the four links the probe reads
@answer n=3 entity=input-validation#1.53 v=no
why: markup numeric fallbacks for rows, cols, spans and colspan are layout parse defaults the probe does not reach
@answer n=4 entity=input-validation#1.54 v=no
why: rowspan clamping is a table layout guard the probe does not reach
@answer n=5 entity=input-validation#1.55 v=no
why: size on a multiple select is a layout hint and select-list rule the probe does not measure
@answer n=6 entity=input-validation#1.56 v=no
why: ol start parsing is a list numbering rule the probe does not read
@answer n=7 entity=input-validation#1.57 v=no
why: attr_parsed returns None on a parse failure, a generic helper the probe's id and listener reads do not call
@answer n=8 entity=input-validation#1.58 v=yes
why: disabled gates the click default action on form controls, the click reaction the scope's reacts-to-a-click reading measures
@answer n=9 entity=input-validation#1.59 v=no
why: open on details and dialog steers the :open state, which the probe does not read
@answer n=10 entity=input-validation#1.60 v=no
why: autofocus on focusable elements is a focus-entry rule the probe does not drive
@answer n=11 entity=input-validation#1.61 v=no
why: dir matching is case-insensitive text direction that the probe does not read
@answer n=12 entity=input-validation#1.62 v=no
why: canvas src u64 parsing is a canvas binding the probe does not touch
@answer n=13 entity=input-validation#1.63 v=no
why: custom-widget attribute replay sits behind a feature the probe does not enable
@answer n=14 entity=input-validation#1.64 v=yes
why: the stable element id that the probe reads for node to component is this id rule, author key then anchored or component path, written to no log
@answer n=15 entity=input-validation#1.65 v=yes
why: accessible names reach the probe through the snapshot it may read, so the probe's test output must keep them masked and out of any log
@answer n=16 entity=input-validation#1.66 v=no
why: aria-describedby and aria-description reach only the platform adapter, which the stand does not build and the probe does not read
@answer n=17 entity=input-validation#1.67 v=no
why: aria-expanded, aria-disabled and aria-level are accessibility-tree closed values the probe does not read
@answer n=18 entity=input-validation#1.68 v=no
why: open and aria-expanded on a select steer nothing, a select rule the probe's listener reads do not reach
@answer n=19 entity=input-validation#1.69 v=no
why: control-state readings reach the adapter and the snapshot's state words, which the probe's verdicts do not depend on
@answer n=20 entity=input-validation#1.70 v=no
why: a native checkbox's checked state in the tree is an accessibility-tree value the probe does not read
@answer n=21 entity=input-validation#1.71 v=no
why: inert removes hit and focus on a subtree, and the probe's served tasks put nothing under inert
@answer n=22 entity=input-validation#1.72 v=no
why: which select opens a list and what a row takes is a select-list rule the probe does not drive
@answer n=23 entity=input-validation#1.73 v=no
why: the tree's focus value and the inert focus clear are accessibility-tree rules the probe does not read
@answer n=24 entity=input-validation#1.74 v=no
why: range min, max, step and value sanitizing and its default actions are not named by the scope and the served tasks do not use them
@answer n=25 entity=input-validation#1.75 v=yes
why: Enter and Space dispatch the Click whose listener the probe reads, so the click reaction reached by key activation is in the scope
