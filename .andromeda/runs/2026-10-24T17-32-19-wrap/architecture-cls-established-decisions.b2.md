# Classified — architecture · established-decisions.b2

## Answers
established-decisions#20.1 · no · its files (packages/blitz-vibey-script, apps/browser/src/document_loader.rs) are not among the reached files and no engine source changed, so the JavaScript decision still holds
established-decisions#21.1 · no · its files (packages/dioxus-native-dom, packages/dioxus-native, packages/blitz-dom/src/config.rs, examples/custom_widget.rs) are not among the reached files and no engine source changed, so the Dioxus integration decision still holds
established-decisions#22.1 · no · its files (apps/browser, apps/readme, examples/wasm_hello, examples/preact) are not among the reached files and the session changes no browser code
established-decisions#23.1 · no · its files (wpt/runner/src) are not among the reached files and the session changes no WPT runner code
established-decisions#24.1 · no · its files (packages/escher-driver/src, packages/escher-driver/Cargo.toml, Cargo.lock) are not among the reached files and the session adds no dependency, carrier, verb, span field, env read or listener, so every stated claim holds
established-decisions#25.1 · no · its recording code and the committed Counter checks are not among the reached files, the checks are not rewritten, and the new host_log check reads client stderr without changing what is recorded or logged
