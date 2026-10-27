# arch · picked accounting · group 11 of 12

## Picked
existing-scopes#1.3 · not in scope · blitz-traits' module list lives in its `lib.rs`, which the take does not reach; `events.rs` and `shell.rs` take upstream's clipboard lines within the same modules, a re-check every change owes
existing-scopes#1.4 · carried · blitz-dom's module set, `clock` and `text_extent` among them, and `document.rs`'s two `cfg(test)` child modules must hold through a merge into `document.rs`
existing-scopes#1.5 · not in scope · `cssom.rs` and `resolved_style.rs` are not reached; upstream's CSSStyleDeclaration accessors land in blitz-vibey-script
existing-scopes#1.6 · not in scope · `form.rs`, `iframe.rs` and `query_selector.rs` are not reached by any commit of the take
existing-scopes#1.7 · carried · hit testing, overlay scrollbar and CSSOM View geometry in `document.rs`, a merge surface `0980cce5f` reaches
existing-scopes#1.8 · carried · its debug half only: `0980cce5f` edits the arguments of an existing `println!` in `debug.rs`, which the sink re-read must read; `accessibility.rs` is not reached
existing-scopes#1.9 · not in scope · the events submodule list lives in `events/mod.rs`, untouched by the delta; `driver.rs` and `pointer.rs` stay those submodules, a re-check every change owes
existing-scopes#1.10 · carried · `scrolling.rs` is a conflict surface holding escher's into-view scrolling and visible-region reader
existing-scopes#1.11 · carried · `stylo.rs` is a merge surface (`53e9d81c6`), and a git Stylo take moves the whole Stylo integration under it
existing-scopes#1.12 · not in scope · `tree.rs`, `traversal.rs` and `url.rs` are not reached by any commit of the take
existing-scopes#1.13 · carried · `construct.rs`, `inline/build.rs` (add/add) and `writing_mode.rs` are conflict surfaces, and `0980cce5f` moves the layout module's dispatch, replaced sizing and inline layout
existing-scopes#1.14 · carried · `node/node.rs` conflicts and `node/element.rs`, `node/text.rs` merge; the range module's privacy and `RangeInputData`'s export must hold through them
existing-scopes#1.15 · not in scope · blitz-html is not reached: no commit of the take edits it and its parser pins are unmoved past `f9cbb2756`
