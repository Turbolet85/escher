@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/dioxus-native-dom.md lines=44
last: - `.andromeda/architecture.md` · `.claude/docs/services/blit
@leaf_read leaf=.claude/docs/stack.md lines=59
last: 4. Commit as `chore: bump {crate} to {version}`.
@leaf_read leaf=.claude/docs/workflow.md lines=47
last: A reference-based CLAUDE.md: a thin index, with depth in `.c
@leaf_edit n=1 leaf=.claude/docs/services/dioxus-native-dom.md line=14 class=omission
leaf-text: whose first hook provides a `SignalRegistry` context at `ScopeId::ROOT`.
source: fct-srb7slak «and writes the ungated reached record (`DioxusDocument::record_reached`, `reached_handlers`, `ReachedHandler`) only where it was switched on, inert elsewhere at one `has_context` lookup per dispatch»
replace-with: whose first hook provides a `SignalRegistry` context at `ScopeId::ROOT`; ungated too, the reached record (`DioxusDocument::record_reached`, `reached_handlers`, `ReachedHandler`), written only where it was switched on.
@leaf_edit n=2 leaf=.claude/docs/services/dioxus-native-dom.md line=22 class=omission
leaf-text: while a page with no registered state dispatches as before;
source: fct-srb7slak «and writes the ungated reached record (`DioxusDocument::record_reached`, `reached_handlers`, `ReachedHandler`) only where it was switched on, inert elsewhere at one `has_context` lookup per dispatch»
replace-with: while a page with no registered state dispatches as before; it writes the reached record (`record_reached`, `reached_handlers`) only where that was switched on, inert elsewhere at one `has_context` lookup per dispatch;
@leaf_edit n=3 leaf=.claude/docs/services/dioxus-native-dom.md line=3 class=tree
leaf-text: Primary source: `.andromeda/architecture.md` §Standard Contracts Dioxus DOM bridge.
source: fct-srb7slak «**Dioxus DOM bridge:** DOM events reach the vdom through `data-dioxus-id`»
replace-with: Primary source: `doc:architecture` §Standard Contracts Dioxus DOM bridge.
@leaf_edit n=4 leaf=.claude/docs/services/dioxus-native-dom.md line=44 class=tree
leaf-text: - `.andromeda/architecture.md` · `.claude/docs/services/blitz-test-harness.md`
source: fct-srb7slak «**Dioxus DOM bridge:** DOM events reach the vdom through `data-dioxus-id`»
replace-with: - `doc:architecture` · `.claude/docs/services/blitz-test-harness.md`
@leaf_edit n=5 leaf=.claude/docs/stack.md line=3 class=tree
leaf-text: Mirrors `.andromeda/architecture.md` §Stack and Technologies
source: fct-d4plx3kh «The stack: a Rust 2024 Cargo workspace (rust-version 1.91.0) on Servo Stylo, html5ever and xml5ever, Taffy, Parley with ICU4X, anyrender over Vello and Skia, winit, AccessKit, Boa, Dioxus 0.7»
replace-with: Mirrors `doc:architecture` §Stack and Technologies
@leaf_edit n=6 leaf=.claude/docs/stack.md line=53 class=tree
leaf-text: See `.andromeda/architecture.md` §Established Decisions
source: fct-bd2samn6 «**[Dependency pinning]** the parser crates match stylo's web_atoms version, skrifa matches parley and vello and svgtypes usvg»
replace-with: See `doc:architecture` §Established Decisions
@leaf_edit n=7 leaf=.claude/docs/stack.md line=56 class=tree
leaf-text: Amend `.andromeda/architecture.md` §Stack
source: fct-d4plx3kh «The stack: a Rust 2024 Cargo workspace (rust-version 1.91.0) on Servo Stylo, html5ever and xml5ever, Taffy, Parley with ICU4X, anyrender over Vello and Skia, winit, AccessKit, Boa, Dioxus 0.7»
replace-with: Amend `doc:architecture` §Stack
@leaf_edit n=8 leaf=.claude/docs/workflow.md line=35 class=tree
leaf-text: check `.andromeda/architecture.md` §Established Decisions and §Occupied Resources
source: fct-uu6lok3l «**Network ports and listeners:** no served network port, TCP or UDP, blitz-net's tests alone binding TCP on loopback»
replace-with: check `doc:architecture` §Established Decisions and §Occupied Resources
