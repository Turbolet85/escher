# Classified — arch · standard-contracts.b4

## Answers
standard-contracts#18.1 · no · recording is named Not needed, the sync changes no driver code and only re-runs the driver's checks
standard-contracts#18.2 · no · replay is named Not needed, the sync changes no driver code, and the committed checks it replays are not touched
standard-contracts#18.3 · no · the visual lint is named Not needed, the sync changes no driver code and only re-runs the driver's checks
standard-contracts#18.4 · no · the load wait is named Not needed, the sync changes no driver code and only re-runs the driver's checks
standard-contracts#19.1 · no · the JavaScript runtime and ScriptDocument are named Not needed, and upstream's delta changes no script behaviour
standard-contracts#20.1 · yes · the Needs name dioxus-native's launch, launch_cfg, launch_cfg_with_props and the Config builders, which change with the packages/dioxus-native re-sync
standard-contracts#21.1 · yes · the Dioxus 0.8 sync replaces dioxus_document.rs, events.rs, lib.rs and mutation_writer.rs, and the Needs carry the bridge's element ids, snapshot text, parts and diff, unkeyed_actionable and the accessibility_tree override
standard-contracts#22.1 · yes · is_masked_input is the value-mask test of the bridge's snapshot, which the Needs carry, and its public re-export sits in lib.rs, a file the Dioxus 0.8 sync replaces
standard-contracts#22.2 · yes · the Needs carry the carried select model and the bridge's snapshot, and the select's Input and Change routing sits in dioxus_document.rs, which the Dioxus 0.8 sync replaces
