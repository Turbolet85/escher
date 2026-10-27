# Classified — architecture · existing-scopes.b1

## Answers
existing-scopes#1.1 · no · packages/accesskit_xplat is not in the reached files and the platform modules it names are unchanged
existing-scopes#1.2 · no · packages/debug_timer is not in the reached files and its feature gate and timer lines are unchanged
existing-scopes#1.3 · no · packages/blitz-traits is not in the reached files so its module list at lib.rs:4-9 still holds
existing-scopes#1.4 · no · the crate's module list at lib.rs:37-87 and its document.rs test-module lines are unreached and no blitz-dom module file is added
existing-scopes#1.5 · no · cssom.rs and resolved_style.rs are not reached and the new CSS accessors are blitz-vibey-script's JS side, not this blitz-dom entity
existing-scopes#1.6 · no · form.rs, iframe.rs and query_selector.rs are not reached so their cited ranges still hold
existing-scopes#1.7 · no · document.rs is not reached so its cited hit, focus, scrollbar and geometry ranges still hold
existing-scopes#1.8 · no · accessibility.rs and debug.rs are not reached
existing-scopes#1.9 · no · events/mod.rs is not reached and its submodule list is unchanged
existing-scopes#1.10 · no · scrolling.rs and selection.rs are not reached and the scroll readers were re-read unchanged
existing-scopes#1.11 · yes · stale · stylo.rs is a merge surface that gained line 510 so its cited line 1 must be re-read
existing-scopes#1.12 · no · tree.rs, traversal.rs and url.rs are not reached
existing-scopes#1.13 · yes · stale · layout/construct.rs is a merge surface that gained lines at 313-317 which shifts the box-tree construction cited at construct.rs:388-634
existing-scopes#1.14 · no · node/element.rs, a merge surface, still defines ElementData (the session only adds flush_open_state to it) and node/mod.rs:3-15, :8 and :29-30 are not reached
existing-scopes#1.15 · no · blitz-html's lib.rs and html_sink.rs are not reached
existing-scopes#1.16 · no · blitz-net's lib.rs is not reached and the file: narrowing sits in blitz-vibey-script's fetch
existing-scopes#1.17 · no · blitz-paint was not reached
existing-scopes#1.18 · yes · add-to · F7: runtime/ is now a module tree with nine bootstrap scripts under runtime/bootstrap/ and the element bindings sit in dom/element/, both joining this entity's module list
existing-scopes#1.19 · yes · stale · lib.rs is a merge surface that gained lines at 90, 93-98 and 104-106 so the module declarations cited at lib.rs:11-18 must be re-read
existing-scopes#1.20 · no · packages/blitz/src/lib.rs is not reached and its re-exports are unchanged
existing-scopes#1.21 · no · stylo_taffy is not reached and Stylo 0.22.0 and Taffy 0.15 stay
existing-scopes#1.22 · no · dioxus-native's lib.rs and Cargo.toml are not reached and the manifests are byte-identical to the base
existing-scopes#1.23 · yes · stale · dioxus_document.rs is a conflict surface that gained a test at 725-759 ahead of the cited declaration at dioxus_document.rs:818-820
existing-scopes#1.24 · no · blitz-test-harness is not reached and the session's new test target lives in blitz-tests
existing-scopes#1.25 · no · escher-telemetry has an empty diff
