@disposition n=1
row: held fct-73ojmpsr apps/browser/Cargo.toml:14 — refused at 2026-10-24T12-21-29-wrap, not touched since «default = ["hyb…»
disposition: holds — stands (default features still list "cookies" and "cache")
@disposition n=2
row: held fct-4kkbivro packages/blitz-dom/src/layout/table.rs:535-539 — refused at 2026-10-13T15-50-37-wrap, not touched since «let Some(di…»
disposition: holds — stands (the let-else on display_style logs at info and returns)
@disposition n=3
row: changed fct-7ni72vcj packages/dioxus-native-dom/src/dioxus_document.rs:297 → ? — rewritten «self.vdom.render_immediate…»
disposition: claim false — raised at Validate (DioxusDocument::poll no longer holds inner.borrow_mut() across vdom.render_immediate: render_immediate now writes into self.buffered_mutations at line 304 before the borrow at line 305, and the mutations are replayed after the render, so a task polled during render does not meet a held borrow and the stated RefCell already borrowed panic no longer follows from this code)
@disposition n=4
row: held fct-d2yejoup tests/blitz-tests/tests/stand_snapshot.rs:1-5 — refused at 2026-10-25T17-25-04-wrap, not touched since «//! The sna…»
disposition: holds — stands (the module doc still states each node carries its stable element id)
