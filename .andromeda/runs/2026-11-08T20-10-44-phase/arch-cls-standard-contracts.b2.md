@answer n=1 entity=standard-contracts#6.6 v=no
why: style data and the style-resolve context are engine internals the chunk reads through and does not change or name
@answer n=2 entity=standard-contracts#7.1 v=no
why: Taffy layout pass types and stylo_taffy exports; layout is not_needed for the chunk and no scope item names them
@answer n=3 entity=standard-contracts#8.1 v=no
why: the mix-blend-mode stacking-context arm is engine painting behaviour the chunk does not touch
@answer n=4 entity=standard-contracts#9.1 v=no
why: the node and widget variant list, range input, and Widget trait are node types and custom widgets the chunk only reads through the DOM event path, with no scope item naming them
@answer n=5 entity=standard-contracts#10.1 v=yes
why: is_disabled is the one focusability reading, and focusable is the actionable rule the coverage read and the actions verb must stay consistent with (scope item 17)
@answer n=6 entity=standard-contracts#10.2 v=no
why: row_disabled is the select list's reading of a disabled row, and no select behaviour changes (not_needed item 2)
@answer n=7 entity=standard-contracts#10.3 v=no
why: the node::select rules and readers are select model internals, which the chunk does not change (not_needed item 2)
@answer n=8 entity=standard-contracts#10.4 v=no
why: select cursor, gesture and choice write-path internals, which the chunk does not change (not_needed item 2)
@answer n=9 entity=standard-contracts#10.5 v=no
why: SelectListView and SelectListRow public read types and the list colour constants, painted only by the select list (not_needed item 2)
@answer n=10 entity=standard-contracts#10.6 v=no
why: the just-opened select list's restyle guard, a select behaviour the chunk does not change (not_needed item 2)
@answer n=11 entity=standard-contracts#10.7 v=no
why: the select list's row cap and placement, a select behaviour the chunk does not change (not_needed item 2)
@answer n=12 entity=standard-contracts#10.8 v=no
why: Node out_of_flow recording is layout state, and layout is not_needed for the chunk
@answer n=13 entity=standard-contracts#10.9 v=no
why: the inline-with-only-block-children flag decides own-box painting, which the chunk does not paint
@answer n=14 entity=standard-contracts#10.10 v=no
why: ElementData classes atoms feed Stylo class matching, which the chunk does not read or change
@answer n=15 entity=standard-contracts#10.11 v=no
why: flush_open_state sets the :open pseudo-class state, which no chunk rule reads
@answer n=16 entity=standard-contracts#11.1 v=no
why: NetProvider, NetHandler and the other provider traits describe requests the command never makes (scope item 5 not_needed), and no scope item names their behaviour
@answer n=17 entity=standard-contracts#12.1 v=no
why: blitz-html and blitz-net public API and the fetch path, which the chunk does not reach (not_needed item 5)
@answer n=18 entity=standard-contracts#13.1 v=no
why: paint_scene, the shell, and the password mask paint path, where the chunk builds no painting (not_needed item 3) and no action step clicks a field
@answer n=19 entity=standard-contracts#13.2 v=no
why: paint_select_list is select painting, which no select behaviour change touches (not_needed items 2 and 3)
@answer n=20 entity=standard-contracts#13.3 v=no
why: stroke_text and RunPaint are text painting internals (not_needed item 3)
@answer n=21 entity=standard-contracts#13.4 v=no
why: mix-blend-mode and isolation paint layers, which no painting is built for (not_needed item 3)
@answer n=22 entity=standard-contracts#13.5 v=no
why: the blend frame-witness gap is a paint-side note with a CARRY owner, and the chunk builds no painting (not_needed item 3)
@answer n=23 entity=standard-contracts#14.1 v=no
why: BlitzShellProvider redraw requests through the windowed shell event loop, which the headless stand and the command do not use (not_needed item 3)
@answer n=24 entity=standard-contracts#15.1 v=yes
why: the stand's load page carries the load counter, so its one stylesheet fetch reads as a load in flight until answered, and the replay test on the headless stand must hold to that (scope item 13 names the load page)
@answer n=25 entity=standard-contracts#16.1 v=no
why: the harness clock moves only on tick and advance_time, and the coverage read moves no time (not_needed item 10), with the replay test calling neither
