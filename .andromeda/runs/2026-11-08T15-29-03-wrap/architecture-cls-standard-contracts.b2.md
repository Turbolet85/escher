@answer n=1 entity=standard-contracts#6.6 v=no
why: the style-data claim is about blitz-dom's stylo.rs, an engine file the session did not change
@answer n=2 entity=standard-contracts#7.1 v=no
why: the layout-trait and stylo_taffy claims live in blitz-dom and stylo_taffy, engine files the session did not change
@answer n=3 entity=standard-contracts#8.1 v=no
why: the mix-blend-mode reading in node.rs is engine behaviour, which the session leaves unchanged
@answer n=4 entity=standard-contracts#9.1 v=no
why: the NodeData, SpecialElementData and Widget inventory is blitz-dom's, unchanged, and the session adds no node kind
@answer n=5 entity=standard-contracts#10.1 v=no
why: is_disabled in element.rs is an engine reading the session leaves unchanged, and the actionable rule it names is not the one the session moved
@answer n=6 entity=standard-contracts#10.2 v=no
why: row_disabled in select_list.rs is engine code the session did not touch
@answer n=7 entity=standard-contracts#10.3 v=no
why: the node::select rules are engine code the session did not touch
@answer n=8 entity=standard-contracts#10.4 v=no
why: the select cursor and gesture items are engine code the session did not touch
@answer n=9 entity=standard-contracts#10.5 v=no
why: SelectListView and SelectListRow are engine read types the session did not change
@answer n=10 entity=standard-contracts#10.6 v=no
why: the fresh-list rule and its named test are engine behaviour and no reached file holds them
@answer n=11 entity=standard-contracts#10.7 v=no
why: the MAX_ROWS and placement rules are engine code the session did not touch
@answer n=12 entity=standard-contracts#10.8 v=no
why: the out_of_flow flag and its readers are engine code the session did not touch
@answer n=13 entity=standard-contracts#10.9 v=no
why: IS_INLINE_WITH_ONLY_BLOCK_CHILDREN is engine layout code the session did not touch
@answer n=14 entity=standard-contracts#10.10 v=no
why: ElementData classes is engine state the session did not change
@answer n=15 entity=standard-contracts#10.11 v=no
why: flush_open_state is engine code the session did not touch
@answer n=16 entity=standard-contracts#11.1 v=no
why: the provider traits live in blitz-traits, which the session did not change
@answer n=17 entity=standard-contracts#12.1 v=no
why: blitz-html and blitz-net are not reached, and no dependency moved
@answer n=18 entity=standard-contracts#13.1 v=no
why: paint_scene and the blitz-shell re-exports are engine and shell code the session did not change
@answer n=19 entity=standard-contracts#13.2 v=no
why: paint_select_list is blitz-paint code the session did not touch
@answer n=20 entity=standard-contracts#13.3 v=no
why: stroke_text and RunPaint are blitz-paint code the session did not touch
@answer n=21 entity=standard-contracts#13.4 v=no
why: the blend paint path is blitz-paint code the session did not touch
@answer n=22 entity=standard-contracts#13.5 v=no
why: the blend's missing frame witness is unchanged, since no test was added or moved in blitz-paint
@answer n=23 entity=standard-contracts#14.1 v=no
why: BlitzShellProvider is blitz-shell code the session did not change
@answer n=24 entity=standard-contracts#15.1 v=yes mode=stale
why: execute.rs is reached whole, and the driver's scroll arm, one of the four id-taking arms, now meets the shared check_target, so the claim that the driver's scroll calls scroll_into_view can have moved
@answer n=25 entity=standard-contracts#16.1 v=no
why: harness.rs is unchanged and the session moves no harness clock
