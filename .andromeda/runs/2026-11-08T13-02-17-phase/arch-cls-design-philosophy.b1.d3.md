@answer n=1 entity=design-philosophy#1.1 v=yes
why: the read adds code to the headless dioxus-native-dom graph, snapshot and actionable files (reached whole) and to the driver, which must stay headless with no paint or windowing reach
@answer n=2 entity=design-philosophy#2.1 v=yes
why: the read goes through DioxusDocument::graph and must dispatch nothing and run no step, so it must avoid the handle_ui_event push and the poll flush this entity names
@answer n=3 entity=design-philosophy#3.1 v=no
why: its cited spec behaviours (form owner reset, ARIA tree exclusion, HTML-AAM role mapping, CSS transforms, paint and network engines, scrollbars) are not what the read's scope names
@answer n=4 entity=design-philosophy#4.1 v=yes
why: the read's GraphNode and optional owner must stay off DOM NodeId, and any NodeId use inside the read must stay behind the public API this entity names
@answer n=5 entity=design-philosophy#5.1 v=yes
why: the read's proof on the headless stand must run through the test harness with no window, GPU or compositor
