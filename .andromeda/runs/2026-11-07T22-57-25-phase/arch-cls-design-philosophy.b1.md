@answer n=1 entity=design-philosophy#1.1 v=yes
why: the new registry and graph reader sit in the headless dioxus-native-dom crate and must stay separately usable, driven by external code, with no paint or shell dependency
@answer n=2 entity=design-philosophy#2.1 v=no
why: the chunk adds or changes no DocumentConfig provider; the graph makes no fetch, navigation, shell or parse call (task not_needed items 3 and 4)
@answer n=3 entity=design-philosophy#3.1 v=yes
why: the graph is keyed on the agent-visible accesskit node from build_accessibility_tree, so the WAI-ARIA tree exclusion and HTML-AAM role mapping that decide those nodes bind the reader
@answer n=4 entity=design-philosophy#4.1 v=yes
why: the graph and registry hold identities over a changing tree, so a dropped node's id must stop resolving and the virtual-node id must not alias a reused slot
@answer n=5 entity=design-philosophy#5.1 v=yes
why: the graph read and its kept checks run on the headless stand with no window, through the real event-dispatch replay under Session::run
