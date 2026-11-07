@answer n=1 entity=input-validation#1.51 v=no
why: SVG width and height parsing in stylo.rs is a blitz-dom parser the chunk edits none of
@answer n=2 entity=input-validation#1.52 v=no
why: border and body margin unsigned-integer parsing is a blitz-dom parser the chunk does not touch and no scope item names
@answer n=3 entity=input-validation#1.53 v=no
why: markup numeric attribute parsing with default fallbacks is a parser the chunk edits none of
@answer n=4 entity=input-validation#1.54 v=no
why: rowspan clamping is a table-layout read the chunk does not change and no re-run check names
@answer n=5 entity=input-validation#1.55 v=no
why: a multiple select's size hint is select-list behaviour the chunk does not act on and its select_popup tests are not re-run
@answer n=6 entity=input-validation#1.56 v=no
why: ol start parsing is a construct-time read the chunk does not touch and no scope item names
@answer n=7 entity=input-validation#1.57 v=no
why: attr_parsed returning None on a parse failure is a blitz-dom reading the replay does not change
@answer n=8 entity=input-validation#1.58 v=yes
why: disabled is read by presence, so the replay's falsy-boolean removal must not write disabled="false", and the stand_keyboard and stand_snapshot_state checks it names are re-run witnesses
@answer n=9 entity=input-validation#1.59 v=yes
why: open is read by presence and flush_open_state runs on each replayed set or clear, so a falsy open must be removed rather than written as "false"
@answer n=10 entity=input-validation#1.60 v=yes
why: autofocus fires during replayed attribute writes and ignores "false", so a falsy autofocus must be removed rather than written
@answer n=11 entity=input-validation#1.61 v=no
why: dir matching is a case-insensitive string read the chunk does not touch
@answer n=12 entity=input-validation#1.62 v=no
why: canvas src u64 acceptance is a blitz-dom mutator read the chunk does not change and no scope item names
@answer n=13 entity=input-validation#1.63 v=no
why: custom-widget attribute replay is a feature-gated blitz-dom path the replay does not call and no scope item names
@answer n=14 entity=input-validation#1.64 v=yes
why: the stable element id is the value scope item 7 keeps identical through the replay, and the new buffer must log no id
@answer n=15 entity=input-validation#1.65 v=yes
why: the accessible name is a snapshot value scope item 7 keeps identical, its rewritten name is asserted by stand_accessibility_ids, and label association depends on the replay keeping call order
@answer n=16 entity=input-validation#1.66 v=no
why: aria-description and described_by reach only the platform adapter, which the snapshot does not read, and no re-run check reads them
@answer n=17 entity=input-validation#1.67 v=no
why: aria-expanded, aria-disabled and aria-level are tree values the chunk does not act on, and the snapshot's enabled keeps its presence reading
@answer n=18 entity=input-validation#1.68 v=no
why: an open or aria-expanded on a select steers nothing, a select behaviour the chunk does not touch
@answer n=19 entity=input-validation#1.69 v=yes
why: the snapshot's selected, pressed, checked and invalid words and the password and file mask are item 7 values the replay must keep identical
@answer n=20 entity=input-validation#1.70 v=yes
why: the native checked state reaches the tree and the snapshot's checked, and the replay's falsy checked must remove rather than write
@answer n=21 entity=input-validation#1.71 v=yes
why: inert is read by presence and the mutator flush clears focus on an inert node during replay, so a falsy inert must be removed
@answer n=22 entity=input-validation#1.72 v=no
why: which select opens a list and what a row takes is select-popup behaviour the chunk does not act on and no re-run check names
@answer n=23 entity=input-validation#1.73 v=yes
why: the tree's focus is a value the chunk's MountedData set_focus during render can move, and stand_accessibility_ids asserts it after Tab
@answer n=24 entity=input-validation#1.74 v=yes
why: range value writes re-sanitize in call order, so the replay must keep the original attribute order, and the value reaches the snapshot
@answer n=25 entity=input-validation#1.75 v=no
why: Enter and Space click dispatch is a keyboard default action the replay and the mounted-event flush do not touch and no scope item names
