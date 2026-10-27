# Classified — architecture · established-decisions.b2

## Answers
established-decisions#21.1 · yes · stale · the module split moved code out of runtime.rs (kept at 151 lines) and dom/element.rs (a merge surface), so the line citations into both no longer point at it
established-decisions#22.1 · yes · stale · dioxus_document.rs, which holds the base_url default cited at :90-94, is a conflict surface whose resolution can move that line
established-decisions#23.1 · yes · add-to · F9, tab lenses now survive a webview props replacement, joins this browser-app list beside its tab entries (apps/browser/src/tab.rs is a changed merge surface)
established-decisions#24.1 · no · the session reaches no wpt/runner file, and its only moved thing, the upstream WPT pin, is not stated by this decision
established-decisions#25.1 · no · escher-driver has an empty diff and Cargo.toml, Cargo.lock and deny.toml are byte-identical to the base, so the driver's feature, graph and crossing claims still hold
established-decisions#26.1 · no · escher-driver has an empty diff and the two committed stand checks are not reached, so the recording and its no-dependency claims still hold
