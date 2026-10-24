# Classified — a11y-plan · a11y-scope-summary.b1

## Answers
a11y-scope-summary#1.1 · no · the root Cargo.toml is not reached and the session changes no manifest or lock
a11y-scope-summary#1.2 · no · packages/accesskit_xplat/Cargo.toml is not reached and the session adds no package
a11y-scope-summary#1.3 · no · the blitz-dom accessibility builder and its Cargo.toml are not reached and the session changes no engine file
a11y-scope-summary#1.4 · no · custom_widget.rs is not reached and the session changes no engine source
a11y-scope-summary#1.5 · no · packages/blitz-html/Cargo.toml is not reached and the session adds no feature
a11y-scope-summary#1.6 · no · blitz-shell's Cargo.toml and its window and accessibility files are not reached and no shell file changes
a11y-scope-summary#1.7 · no · packages/blitz/Cargo.toml is not reached and the session adds no feature
a11y-scope-summary#1.8 · no · node_id.rs is not reached and the session changes no node-id code
a11y-scope-summary#1.9 · no · the Dioxus manifests, examples/seven_guis/Cargo.toml and Cargo.lock are not reached and the session adds a test but no dependency, so the stand graph holds as before
a11y-scope-summary#1.10 · no · apps/browser/Cargo.toml is not reached and the session changes no feature
a11y-scope-summary#1.11 · no · tests/blitz-tests/Cargo.toml is not reached and the session adds tests but no target or dependency
a11y-scope-summary#1.12 · no · examples/assets/input.html is not reached and the session's fixture additions go into fixtures/mod.rs, not that page
a11y-scope-summary#2.1 · no · no root examples/*.rs file is reached, the one reached example, examples/seven_guis/tests/host_log.rs, sits outside that glob
a11y-scope-summary#2.2 · no · the session changes tests and CI scripts only and adds no ARIA, accesskit, role or screen-reader code, the report leaving the ARIA mapping unchanged
a11y-scope-summary#2.3 · no · the session changes tests and CI scripts only and adds no accessibility-handling code, the engine and stand source unchanged
a11y-scope-summary#2.4 · no · the session adds no keyboard, focus or ARIA handling and reaches no wpt runner file, the report leaving the keyboard and focus rules and Tab-order tables unchanged
