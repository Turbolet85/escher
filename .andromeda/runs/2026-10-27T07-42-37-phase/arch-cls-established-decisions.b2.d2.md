# Classified — arch · established-decisions.b2

## Answers
established-decisions#21.1 · yes · runtime.rs, a conflict surface the take resolves and splits into modules, holds its timer, extension, history, single-frame and change-synthesis rules, and dom/element.rs, a merge surface, its detached-node rule, all of which bind
established-decisions#22.1 · yes · dioxus_document.rs, a merge surface, holds the dioxus://index.html base_url default and the rule that DioxusDocument::new adds no stylesheet of its own, both of which bind
established-decisions#23.1 · no · the browser app's history persistence is Not needed, the reached files name no apps/browser, apps/readme or examples file, and the scope names no browser behaviour these rules state
established-decisions#24.1 · no · the WPT runner's execution, test kinds, outputs and bucketing are Not needed and its rules sit in wpt/runner files the reached files do not name, while WPT_COMMIT holds only a pinned sha this entity does not state
established-decisions#25.1 · yes · escher-driver's dependency facts bind through the reached Cargo.toml and Cargo.lock surfaces that the scope re-reads for new packages (its five crates, the unconditional accessibility feature on dioxus-native-dom, no platform adapter in the seven_guis graphs), its wire, socket and MCP contracts being Not needed
established-decisions#26.1 · yes · stand_replay replays the two committed Counter checks in the stand re-run, and the diffs and snapshot text they hold at rest are stand numbers a moved Taffy, Stylo or winit can shift, though the recording format and owner-only modes are Not needed
