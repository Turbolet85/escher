@answer n=1 entity=design-philosophy#1.1 v=yes
why: the new record sits in dioxus-native-dom, a separately usable headless crate that the external driver drives, and must stay so
@answer n=2 entity=design-philosophy#2.1 v=yes
why: the new record hooks the DioxusDocument event path, where UI events are pushed through handle_ui_event, which this entity names
@answer n=3 entity=design-philosophy#3.1 v=no
why: the chunk implements no web semantics; it records DOM listeners beside the graph and reaches no cited spec or engine behaviour
@answer n=4 entity=design-philosophy#4.1 v=yes
why: the new exercised record keeps node identities across a run, so its stable-id key must not alias a reused slot and must hold the safe-identity rule
@answer n=5 entity=design-philosophy#5.1 v=yes
why: the owed replay test runs on the headless harness, and its synthesized actions must route through the real event-dispatch pipeline
