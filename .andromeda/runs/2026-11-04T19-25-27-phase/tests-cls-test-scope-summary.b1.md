@answer n=1 entity=test-scope-summary#1.1 v=no standing=S2
why: the BlitzTestsTargetsTest one-target-per-file convention, which a new blitz-tests probe file owes equally
@answer n=2 entity=test-scope-summary#2.1 v=no
why: the examples/seven_guis integration targets and the binary's host, MCP and CLI tests; the chunk boots the stand library in process and adds no seven_guis test
@answer n=3 entity=test-scope-summary#2.2 v=no
why: the browser tab test; the chunk reaches no browser crate
@answer n=4 entity=test-scope-summary#2.3 v=no
why: the blitz-dom document, mutator, net and form unit tests; the chunk reads links through the bridge and edits no blitz-dom code
@answer n=5 entity=test-scope-summary#2.4 v=no
why: the blitz-dom covered behaviours (media defaults, changed set, font-face sniffing, select model, dropped-node hit test); none is a call, verb or value the probe's needs name
@answer n=6 entity=test-scope-summary#3.1 v=no
why: the paint skip of an inline box holding only block children, which the probe's stand pages do not reach
@answer n=7 entity=test-scope-summary#4.1 v=no
why: the upstream commits for table damage, Stylo prefs, sub-pixel sizes and culling; none touches the MountedData path or the bridge re-reads
@answer n=8 entity=test-scope-summary#5.1 v=no
why: the non-invertible transform hit test; the stand pages carry no transform and the click dispatches by node
@answer n=9 entity=test-scope-summary#6.1 v=no
why: select multiple and groups; the probe drives no select
@answer n=10 entity=test-scope-summary#7.1 v=no
why: the chosen-row colour pair; the probe reads no colour or frame
@answer n=11 entity=test-scope-summary#8.1 v=no
why: the driver's press without Ctrl and the unmodelled Ctrl+A; the probe uses the harness press_with and driver coverage is not this chunk's
@answer n=12 entity=test-scope-summary#9.1 v=no
why: a key press past a dropped anonymous block in layout is a different panic path from the RefCell hazard, and the stand pages drop no block
@answer n=13 entity=test-scope-summary#10.1 v=no
why: the key_press_after_container_lost_its_box target and its run selection; the probe's own selection is set by the chunk's needs, not by this target
@answer n=14 entity=test-scope-summary#11.1 v=no
why: the windowed select list's stated limits, read and not run; the probe reads no select popup
@answer n=15 entity=test-scope-summary#12.1 v=no
why: the blitz-dom layout inline tests (list markers, text-transform, writing mode, scrolled bounds); the probe reads no layout
@answer n=16 entity=test-scope-summary#12.2 v=no
why: the pseudo-element flow classification target; the probe reads no flow
@answer n=17 entity=test-scope-summary#12.3 v=no
why: the blitz-dom node units (scrollbar, range, select rules, out_of_flow, classes); none is named by the probe's needs
@answer n=18 entity=test-scope-summary#12.4 v=no
why: the :open state flush; the probe reads no open state
@answer n=19 entity=test-scope-summary#13.1 v=no
why: the node/select_list.rs list state; the probe reads no select popup
@answer n=20 entity=test-scope-summary#14.1 v=no
why: the open_state check; the probe reads no open state
@answer n=21 entity=test-scope-summary#15.1 v=yes
why: the VirtualClock's sum of advances on every clone, which the probe's time reads through tick and advance_time
@answer n=22 entity=test-scope-summary#16.1 v=no
why: the blitz-net TCP unit tests; the chunk's LoadNet is a stand-in and blitz-net has no caller here
@answer n=23 entity=test-scope-summary#17.1 v=no
why: the paint unit tests, the password mask and the caret check; the probe paints no frame and types into no password field
@answer n=24 entity=test-scope-summary#18.1 v=no
why: the blending paint carry; the probe reads no blended element
@answer n=25 entity=test-scope-summary#19.1 v=yes
why: the harness settle loop and load counter, which the probe's hazard reading rests on through pump and settle
