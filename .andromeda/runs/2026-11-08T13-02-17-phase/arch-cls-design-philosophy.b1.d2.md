@answer n=1 entity=design-philosophy#1.1 v=yes
why: the new bridge reads sit in the headless dioxus-native-dom crate that external code drives, so they must stay embeddable with no window, paint or driver dependency
@answer n=2 entity=design-philosophy#2.1 v=yes
why: the read dispatches nothing and runs no step, so it must not call DioxusDocument's handle_ui_event or poll while reading the graph
@answer n=3 entity=design-philosophy#3.1 v=yes
why: the actionable set's interactive-role test and the disabled and click-listener refusal reading are web semantics the new code must match to browser behaviour and the cited specs
@answer n=4 entity=design-philosophy#4.1 v=yes
why: the new snapshot read and node model are public bridge APIs keyed on stable ids rather than NodeId, which this rule that public APIs use NodeId and slotmap keys stay at storage must be weighed against
@answer n=5 entity=design-philosophy#5.1 v=yes
why: the chunk's proof runs headless on the stand with no window, GPU or compositor, as this entity's headless harness states
