@answer n=1 entity=test-scope-summary#20.1 v=no
why: the failed-load exit is settle_waiting's behaviour on a load that fails, and the served boots run on LoadNet which answers, so the probe's settle pass never reaches it
@answer n=2 entity=test-scope-summary#21.1 v=no
why: the escher-telemetry scrub branches are the sink's own tests, the probe logs nothing and reads no sink capture
@answer n=3 entity=test-scope-summary#21.2 v=no
why: the escher-driver session surfaces are not reached, the probe runs in process and never through a session host, socket or driver verb
@answer n=4 entity=test-scope-summary#22.1 v=no
why: blitz-vibey-script's script and Preact tests are not reached, the probe is over the Dioxus runtime and not the script engine
@answer n=5 entity=test-scope-summary#23.1 v=yes
why: the probe reads through the dioxus-native-dom bridge and sits in its cfg(test) bridge_tests child, so the crate's unit-test census and its 83 percent coverage floor bind any probe code kept there
@answer n=6 entity=test-scope-summary#23.2 v=no
why: the IME-on-mount test is another test in dioxus_document.rs, a file the chunk only re-reads at the poll path, so it is a re-check every change owes and no more
@answer n=7 entity=test-scope-summary#23.3 v=yes
why: the absent upstream test pins a task polled during render with document access, the mechanism of the MountedData hazard this chunk re-verifies and measures
@answer n=8 entity=test-scope-summary#23.4 v=no
why: the absent test guards the flush of a removed element's mounted event, another unit of dioxus_document.rs off the poll and RefCell path the chunk re-reads
@answer n=9 entity=test-scope-summary#24.1 v=no
why: dioxus-native and stylo_taffy are neither edited nor read by the probe, and their lack of tests is not a rule it must meet
@answer n=10 entity=test-scope-summary#25.1 v=yes
why: the stand's boot in TaskShell with the bundled font, the fresh-boot identity and the stable element ids are what the probe's boots and reads run on, its directory-layout lines being standing S2 and S3 and not the reason
@answer n=11 entity=test-scope-summary#26.1 v=no
why: transformed visible_region is an engine check that the probe's client-rect and snapshot reads do not reach
@answer n=12 entity=test-scope-summary#27.1 v=no
why: a checkbox click's input and change pair is not a reading the probe makes, since its stand tasks name no checkable control
@answer n=13 entity=test-scope-summary#28.1 v=yes
why: the probe's type_text and dispatch_recorded readings of CRUD's text inputs record the event names a typed text fires, which this check pins as input with no change
@answer n=14 entity=test-scope-summary#29.1 v=no
why: a control in a zoomed scrolling box is not a probe reading, which runs no zoom, transform or wheel path
@answer n=15 entity=test-scope-summary#30.1 v=no
why: failed-stylesheet unblocking is not reached, the served load page is answered by LoadNet and the probe reads no stylesheet failure
@answer n=16 entity=test-scope-summary#31.1 v=no
why: the select list through a held driver session is not reached, the probe runs in process and drives no popup
@answer n=17 entity=test-scope-summary#32.1 v=no
why: lint's select and overlap readings are not reached, the probe reads no lint
@answer n=18 entity=test-scope-summary#33.1 v=no
why: a native checkbox or radio in the accessibility tree is on none of the probe's stand tasks
@answer n=19 entity=test-scope-summary#34.1 v=no
why: inert-subtree focus loss is not a probe reading
@answer n=20 entity=test-scope-summary#35.1 v=no
why: the select list's engine behaviour is not reached, the probe's stand tasks are Counter or CRUD and one served page, none a select popup
@answer n=21 entity=test-scope-summary#36.1 v=no
why: select-list frame painting is not reached, the probe reads no frame or colour
@answer n=22 entity=test-scope-summary#37.1 v=yes
why: the probe's press and press_with key path on the served Cells or load boot meets Cells' keyboard editor and the load page's Enter and Space behaviour this check states
@answer n=23 entity=test-scope-summary#38.1 v=no
why: contrast ratios and the focus outline on the load page and Cells are colour readings the probe does not make
@answer n=24 entity=test-scope-summary#39.1 v=no
why: the 0.2.0 sweep test's helper messages belong to another test file, and the chunk adds no witness row to that table
@answer n=25 entity=test-scope-summary#40.1 v=no
why: the stand-repeat leg's named-target reach is owned by a working-route CARRY, and a kept stand_ probe joins the leg by the glob the leg already holds
