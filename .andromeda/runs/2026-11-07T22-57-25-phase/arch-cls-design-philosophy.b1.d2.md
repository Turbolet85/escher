@answer n=1 entity=design-philosophy#1.1 v=yes
why: the graph reader is an external read of the headless DOM through DioxusDocument in dioxus-native-dom, which must stay headless and separable from escher-driver
@answer n=2 entity=design-philosophy#2.1 v=yes
why: the DioxusDocument bullet (UI events pushed by handle_ui_event, changes flushed by poll) is the path a click's handler state takes to the graph read and the replayed click check; the provider and networking bullets are not reached
@answer n=3 entity=design-philosophy#3.1 v=yes
why: the graph is keyed on the agent-visible accesskit node, whose inclusion and roles follow the WAI-ARIA tree exclusion and HTML-AAM role mapping cited here
@answer n=4 entity=design-philosophy#4.1 v=yes
why: the graph keys and the reserved virtual-node id must alias no DOM NodeId or stable id, per the safe versioned identity rule and NodeId at public APIs
@answer n=5 entity=design-philosophy#5.1 v=yes
why: the stand checks and the graph read must run headless with no window and synthesized input through real dispatch; the test-home sub-bullet is standing S1 and is not the reason
