@account n=1 entity=standard-contracts#1.1 v=not_in_scope
why: the Document trait, config and changed set are a component the read only uses, no behaviour of it named by the scope
@account n=2 entity=standard-contracts#2.1 v=carried
why: the document's clock is what the read must never move, since it runs no step and moves no time
@account n=3 entity=standard-contracts#2.3 v=carried
why: the read must reach an open list only through the public read-only readers and never call the writer close_select_list
@account n=4 entity=standard-contracts#2.4 v=not_in_scope
why: crate-private list-state reader inside blitz-dom, which the chunk neither calls nor changes
@account n=5 entity=standard-contracts#2.6 v=carried
why: select_of, closed_select_of, is_select and is_masked_input are the select readers the owner-served node and the shared path read
@account n=6 entity=standard-contracts#4.2 v=not_in_scope
why: the select list's default actions run on dispatched events, and the read dispatches nothing
@account n=7 entity=standard-contracts#5.1 v=carried
why: visible_region and the hit walk are the off-screen and covered readings the shared refusal path keeps
@account n=8 entity=standard-contracts#6.1 v=carried
why: states the off-screen limit under rotation that the read's off-screen cause re-asserts
@account n=9 entity=standard-contracts#6.2 v=carried
why: execute.rs in_view and crop read visible_region unchanged, and the chunk edits execute.rs's shared path
@account n=10 entity=standard-contracts#6.4 v=carried
why: the driver's covered reads an open select list first through hit_with_scrollbar, a refusal the read must agree with
@account n=11 entity=standard-contracts#6.5 v=not_in_scope
why: a hit-walk rule the shared path keeps by the engine's own code, the scope naming no non-invertible case
@account n=12 entity=standard-contracts#9.1 v=not_in_scope
why: node data, widgets and serialization are untouched by a read of actionable nodes
@account n=13 entity=standard-contracts#10.1 v=carried
why: is_disabled is the disabled reading the shared path's first cause reads
@account n=14 entity=standard-contracts#10.2 v=not_in_scope
why: a crate-private list row reading the driver cannot call and the chunk does not change
@account n=15 entity=standard-contracts#10.3 v=carried
why: the select's rules hold no public type, bounding how the read reaches an option's reading without an engine change the plan may add
@account n=16 entity=standard-contracts#10.4 v=carried
why: owner_select and option_disabled are not public, bounding how the read reaches an option's owner and disabled reading without an engine change
@account n=17 entity=standard-contracts#10.5 v=not_in_scope
why: the list's public paint read types and colours, which the read neither paints nor lists
@account n=18 entity=standard-contracts#10.7 v=not_in_scope
why: the list's row cap and placement, a geometry the read leaves to visible_region
@account n=19 entity=standard-contracts#15.1 v=carried
why: pump, dispatch and the input helpers are the calls the read must avoid to run no step and move nothing
@account n=20 entity=standard-contracts#16.1 v=carried
why: tick and advance_time move the harness's clock, which the read must never move
@account n=21 entity=standard-contracts#17.1 v=carried
why: the stand is where the read is proven, its lean tasks and pinned viewport the proof's screens
@account n=22 entity=standard-contracts#18.1 v=not_in_scope
why: the stand's font, a value the read's answer does not measure
@account n=23 entity=standard-contracts#19.1 v=carried
why: Cells is a served stand task whose screen passes the snapshot text's budget, a screen the read lists
@account n=24 entity=standard-contracts#19.2 v=carried
why: the load page's keyed button is an actionable node on a served screen the read is proven on
@account n=25 entity=standard-contracts#19.3 v=carried
why: the load page's fetch is in flight for 200 ms, and the read waits on no load
