@row_add n=1 row=name/binary/todomvc-native area=occupied-resources rec=1,2
class: binary
name: todomvc_native
@row_add n=2 row=name/other/cold-agent-sh area=occupied-resources rec=1,2
class: other
name: cold-agent.sh
@row_add n=3 row=name/other/cold-agent-ps1 area=occupied-resources rec=1,2
class: other
name: cold-agent.ps1
@row_add n=4 row=name/server/escher area=occupied-resources rec=1,2
class: server
name: escher
@row_add n=5 row=decision/rustdoc-warnings-gate area=infrastructure-patterns rec=3,4,5
name: Workspace rustdoc warnings gate
rule: Rustdoc warnings are errors, and the docs job documents every workspace library crate under that setting, with no output-filename collision.
except: the browser binary blitz is not documented, so the lib blitz alone writes its docs
@row_add n=6 row=decision/changed-set-write-sites area=cross-cutting-patterns rec=6,7
name: Where the changed set is written
rule: The changed set is written at five kinds of site: mutations whose condition holds, focus or checked changes, typed text input, range moves and select choices that change a value, and a select list opening or closing; with no list open, never by creation, hover, scroll, resize, load, animation or layout.
why: ratified as the engine half of the document-core changed-set item
@row_add n=7 row=overview/project-intent area=project-intent rec=8,9
text: escher is a fork of Blitz, a Rust library workspace for an HTML/CSS engine (high-level APIs for rendering HTML, a core DOM abstraction whose primary entry point is BaseDocument, a Document trait so wrappers around BaseDocument can all be driven by blitz-shell), being turned into an agent-first UI framework: a headless driver (CLI and MCP), stable ids for every element, a compact semantic snapshot of the screen, proven on an internal 7GUIs stand. Development is agent-driven: the framework's own driver is meant to become the harness.
@row_add n=8 row=decision/fmt-and-clippy-gates area=inherited-defaults rec=10,11,5
name: Format and clippy gates
rule: `cargo fmt --all --check` and workspace clippy with warnings denied run, then, only when that line is green, upstream's blitz-dom clippy line, each as its CI leg script leg in CI and on the dev host.
