@answer n=1 entity=standard-contracts#6.6 v=no
why: the stylo style-data filter is an engine behaviour the chunk neither calls nor changes, since the graph reads the vdom and handlers and not styles
@answer n=2 entity=standard-contracts#7.1 v=no
why: layout types and Taffy traits, and the graph reads already-built nodes and changes no layout
@answer n=3 entity=standard-contracts#8.1 v=no
why: the mix-blend-mode stacking arm is an engine change for paint, and the graph paints nothing
@answer n=4 entity=standard-contracts#9.1 v=yes
why: the masked-control test covers file inputs and control values the graph carries, and this entity names the FileInput variant and range_input_value(), which the graph's masking must read
@answer n=5 entity=standard-contracts#10.1 v=yes
why: is_disabled is the pointer gate the reacts-to-a-click reading must honour for a disabled control, a fork the scope leaves open
@answer n=6 entity=standard-contracts#10.2 v=no
why: row_disabled is crate-private to the select list popup, which the graph neither reaches nor reads
@answer n=7 entity=standard-contracts#10.3 v=no
why: the crate-private select rules and readers are not reached by the graph read, and the chunk names no select
@answer n=8 entity=standard-contracts#10.4 v=no
why: the crate-private select cursor, gestures and single write path sit outside the graph's read and its checks
@answer n=9 entity=standard-contracts#10.5 v=no
why: the select list public read types and colour constants concern a popup the graph does not read
@answer n=10 entity=standard-contracts#10.6 v=no
why: the fresh-list focus restyle is a popup behaviour on the paint and shell path the chunk does not touch
@answer n=11 entity=standard-contracts#10.7 v=no
why: the list row cap and placement are popup layout the graph does not read
@answer n=12 entity=standard-contracts#10.8 v=no
why: the out-of-flow flag is a layout-pass flag that the graph's read does not consult
@answer n=13 entity=standard-contracts#10.9 v=no
why: the inline-with-block-children paint skip is an engine change, and the graph paints nothing
@answer n=14 entity=standard-contracts#10.10 v=no
why: the class atom sync is blitz-dom's internal write path, which the chunk does not reach and does not write classes through
@answer n=15 entity=standard-contracts#10.11 v=no
why: the :open state of details and dialog is a style state the graph's read does not consult
@answer n=16 entity=standard-contracts#11.1 v=no
why: the provider trait contracts govern fetch and shell calls the chunk does not implement or make, and the load page's counter rule is carried by standard-contracts#15.1
@answer n=17 entity=standard-contracts#12.1 v=no
why: the blitz-html and blitz-net fetch path is not reached, since the chunk makes no fetch
@answer n=18 entity=standard-contracts#13.1 v=no
why: the paint and shell path, including the password paint masking, is not painted by the chunk, and the graph's masking is its own rule
@answer n=19 entity=standard-contracts#13.2 v=no
why: the select list paint function is a paint path the chunk does not touch
@answer n=20 entity=standard-contracts#13.3 v=no
why: the text stroke paint path is not touched, since the chunk paints nothing
@answer n=21 entity=standard-contracts#13.4 v=no
why: the mix-blend-mode and isolation paint path is not touched, since the chunk paints nothing
@answer n=22 entity=standard-contracts#13.5 v=no
why: the blend paint witness and its upstream sync owner concern an engine path the chunk does not change
@answer n=23 entity=standard-contracts#14.1 v=no
why: the windowed shell redraw path is not run by the headless stand, which the chunk's read uses
@answer n=24 entity=standard-contracts#15.1 v=yes
why: the settle semantics, the load page's counter over LoadNet and the HarnessOptions fields are what the stand's checks and the graph read run through
@answer n=25 entity=standard-contracts#16.1 v=yes
why: the harness's VirtualClock, which tick and advance_time move and which the graph read must leave unmoved, bounds the stand's replay and the Timer task's time
