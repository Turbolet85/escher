@answer n=1 entity=design-philosophy#1.1 v=yes
why: names dioxus-native-dom as the headless core renderer and DioxusDocument as its wrapper, the bridge the probe reads through while the stand runs headless; the crate list and paint claims are not reached
@answer n=2 entity=design-philosophy#2.1 v=yes
why: DioxusDocument's handle_ui_event and poll are the verbs the probe drives, and the offline stand's provider defaults (Dummy providers when none is supplied) are what its offline boot rests on
@answer n=3 entity=design-philosophy#3.1 v=no
why: its spec citations cover paint, networking, forms, CSS, scrolling and ARIA mapping; none is a surface the four links read or the scope names, and the probe's reads are the bridge's element_id and listener map
@answer n=4 entity=design-philosophy#4.1 v=yes
why: incremental and non-incremental layout must match after any mutation, which the scope runs in both layout modes where a verdict's reading touches layout, and versioned NodeId ids are what the element_id reads of links (1) and (2) depend on
@answer n=5 entity=design-philosophy#5.1 v=yes
why: the probe runs headless through the Harness, with input synthesized through the real event-dispatch pipeline and poll(None) pumping, which is the scope's measurement path
