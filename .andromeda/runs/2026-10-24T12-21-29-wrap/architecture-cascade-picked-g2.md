# architecture · picked accounting · group 2 of 4

## Picked
established-decisions#4.1 · not touched · its backends are unmoved; the shifted `wpt/runner/src/main.rs` and `Cargo.toml` line numbers are the citation sweep's
established-decisions#5.1 · proposed · the browser's default feature list gains `mimalloc` (Dependencies, `apps/browser/Cargo.toml` added 14)
established-decisions#6.1 · not touched · no document default moved; `config.rs`'s doc-comment edits shift its citations alone, the citation sweep's
established-decisions#7.1 · not touched · no loading rule moved; the `mutator.rs` line shifts are the citation sweep's
established-decisions#9.1 · not touched · no unsupported feature moved (Stylo stays 0.22.0); `layout/inline.rs` → `inline/layout.rs` is a path re-point the citation sweep owns
established-decisions#10.1 · not touched · its scrolling and hit-stop claims stay true; the dropped-node hit skip is carried on [DOM semantics] (#13.1), the `node.rs` shifts the citation sweep's
established-decisions#11.1 · proposed · the merge surface's owner CARRY on «Upstream sync ahead of polish and ship» closes with this chunk, moving to the new residuals line (inputs#I5 row 2)
established-decisions#11.2 · proposed · the clock surface's owner CARRY moves to the same residuals line
established-decisions#11.3 · proposed · the select model's owner CARRY moves to the same residuals line
established-decisions#12.1 · proposed · a replaced element's natural height now follows `aspect-ratio`, and the layout pass reads the recorded `Node::out_of_flow` flag
established-decisions#13.1 · proposed · a hit test now skips nodes dropped since the last resolve (upstream #1244)
established-decisions#14.1 · not touched · the inert test still answers first in `hit_inner`; only its line numbers shift, the citation sweep's
established-decisions#14.2 · not touched · no focus or `FOCUS_WITHIN` behaviour moved
established-decisions#14.3 · not touched · the select rules are unmoved; the `class` arm added to `set_attribute` shifts `mutator.rs` lines alone
established-decisions#15.1 · not touched · no parsing rule moved; the `config.rs` and `element.rs` shifts are the citation sweep's
established-decisions#18.1 · not touched · blitz-vibey-script is not among the changed files
established-decisions#19.1 · proposed · `dioxus_native::launch` now builds its document with `StyleThreading::Parallel`
established-decisions#20.1 · proposed · mimalloc becomes the browser's default global allocator, and its loader and toolbar documents resolve `Parallel`
established-decisions#21.1 · proposed · the WPT runner gains mimalloc as its global allocator
conventions#1.1 · not touched · no lint, fmt or doc rule moved; the `construct.rs` line shift is the citation sweep's
conventions#3.1 · not touched · no documentation convention moved; the `stylo.rs` line shifts are the citation sweep's
conventions#4.1 · not touched · the TODO convention stands; the `layout/inline.rs` cites re-point with the citation sweep
conventions#6.1 · not touched · the gating convention stands; the `construct.rs` and `layout/mod.rs` shifts are the citation sweep's
conventions#7.1 · not touched · the conventions stand; the `inline.rs`, `stylo.rs` and `node.rs` cites re-point with the citation sweep
conventions#8.1 · not touched · the new `hit_test_dropped_nodes` target follows the one-file-per-behaviour convention, and `all.rs` stays `test = false`
