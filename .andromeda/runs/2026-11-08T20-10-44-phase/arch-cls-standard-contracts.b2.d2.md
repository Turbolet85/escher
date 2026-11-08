@answer n=1 entity=standard-contracts#6.6 v=no
why: the style-resolve context of every document is an engine path the chunk does not change or read
@answer n=2 entity=standard-contracts#7.1 v=no
why: the layout pass traits and stylo_taffy exports are layout internals the chunk neither builds nor reads
@answer n=3 entity=standard-contracts#8.1 v=no
why: the mix-blend-mode stacking-context arm is a paint and stacking reading outside the chunk's scope
@answer n=4 entity=standard-contracts#9.1 v=no
why: the NodeData and SpecialElementData variants, range input values and the Widget trait are node types the chunk does not match on or expose
@answer n=5 entity=standard-contracts#10.1 v=yes
why: is_disabled is the one reading of focusability, and the focusable arm of the one actionable rule in the scope, which the replay compares key by key, depends on it
@answer n=6 entity=standard-contracts#10.2 v=no
why: row_disabled is the select list's reading of a disabled row, and no select behaviour changes
@answer n=7 entity=standard-contracts#10.3 v=no
why: the crate-private node::select rules are select behaviour, which the scope marks as not changing
@answer n=8 entity=standard-contracts#10.4 v=no
why: the select cursor, gesture and option internals are select behaviour, which the scope marks as not changing
@answer n=9 entity=standard-contracts#10.5 v=no
why: SelectListView and SelectListRow are select list read types, and no select list changes
@answer n=10 entity=standard-contracts#10.6 v=no
why: the fresh-list guard against a focus restyle is select and focus behaviour outside the scope
@answer n=11 entity=standard-contracts#10.7 v=no
why: MAX_ROWS and the list's placement are select list behaviour outside the scope
@answer n=12 entity=standard-contracts#10.8 v=no
why: the out_of_flow recording is a layout flag, and neither the coverage read nor the actions answer names a layout or out-of-flow reading
@answer n=13 entity=standard-contracts#10.9 v=no
why: the inline-with-block-children paint skip is painting, which the scope marks as not built
@answer n=14 entity=standard-contracts#10.10 v=no
why: ElementData classes are a class-attribute reading that the chunk's reads do not name
@answer n=15 entity=standard-contracts#10.11 v=no
why: flush_open_state and the :open match are an open-state reading that the chunk does not touch
@answer n=16 entity=standard-contracts#11.1 v=no
why: the NetProvider and NetHandler trait defaults are carried by the stand's load page, which the chunk neither implements nor changes, and the load page's counter is already stated by the harness entity
@answer n=17 entity=standard-contracts#12.1 v=no
why: the blitz-html and blitz-net public surface is an HTML parse and fetch path the scope marks as not changing
@answer n=18 entity=standard-contracts#13.1 v=no
why: paint_scene and the shell re-exports are painting and windowing, which the scope marks as not built
@answer n=19 entity=standard-contracts#13.2 v=no
why: paint_select_list is select list painting, which the scope marks as not changing
@answer n=20 entity=standard-contracts#13.3 v=no
why: stroke_text and RunPaint are text painting that the chunk does not build
@answer n=21 entity=standard-contracts#13.4 v=no
why: mix-blend-mode and isolation paint layers are painting that the chunk does not build
@answer n=22 entity=standard-contracts#13.5 v=no
why: the blend frame witness is an upstream-sync CARRY about painting, and the chunk paints no blended element
@answer n=23 entity=standard-contracts#14.1 v=no
why: BlitzShellProvider redraw requests belong to the shell loop and windowing, which the scope does not touch
@answer n=24 entity=standard-contracts#15.1 v=yes
why: the harness's pinned viewport and font options, from_vdom, settle and the stand's load-page load counter carry the headless stand and the replay test's settle
@answer n=25 entity=standard-contracts#16.1 v=no
why: the harness's VirtualClock, tick and pump time are not moved by the coverage read, and the replay names no clock step
