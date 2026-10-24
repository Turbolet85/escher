# Classified — architecture · established-decisions.b1

## Answers
established-decisions#1.1 · no · root Cargo.toml and deny.toml are byte-identical and every pin this entity names is unchanged; the lock-only tokio and windows-sys moves are no pin here
established-decisions#2.1 · no · root Cargo.toml is byte-identical, so the dev and named build profiles it lists are unchanged
established-decisions#3.1 · no · root Cargo.toml and .github are unchanged, and the workspace version and merge base 23b9b26a9 are unmoved
established-decisions#4.1 · yes · stale · wpt/runner/src/main.rs gained three lines (its mimalloc global allocator), so the cited lines 2-5 moved to 5-8
established-decisions#5.1 · yes · contradict · apps/browser's default features now include mimalloc, so the default list this entity gives is no longer complete
established-decisions#6.1 · yes · stale · the config.rs, stylo.rs and wpt main.rs citations moved, and the Parallel style threading that dioxus-native and the browser now resolve is not carried here
established-decisions#7.1 · yes · stale · the mutator.rs citations at 1280-1381 and the resolve.rs citation at 52-63 moved
established-decisions#8.1 · no · no approximation it lists changes, and none of its cited files is in the take's line shifts
established-decisions#9.1 · yes · stale · the stylo.rs and construct.rs citations moved, and layout/inline.rs is now a directory, so its text-indent and float citations point at a vanished path
established-decisions#10.1 · yes · stale · the node.rs and mutator.rs citations moved, and hit_inner and paint_tree.rs now skip nodes dropped since the last resolve, a hit fact this decision does not carry
established-decisions#11.1 · yes · contradict · the CARRY it names as owner, «Upstream sync ahead of polish and ship», is closed by this chunk, so the merge-surface owner moves to a residuals.md line (target next)
established-decisions#11.2 · yes · contradict · its clock merge surface is owned by that same closed CARRY, so the owner moves to a residuals.md line (target next)
established-decisions#11.3 · yes · contradict · its select merge surface is owned by that same closed CARRY, so the owner moves to a residuals.md line (target next)
established-decisions#12.1 · yes · stale · the construct.rs, paint_tree.rs and replaced.rs citations moved and layout/inline.rs is gone, and compute_replaced_layout now coerces natural height through aspect-ratio, a fact this entity does not carry
established-decisions#13.1 · yes · stale · the node/element.rs citations at 463-468, 497-501 and 716 moved
established-decisions#14.1 · yes · stale · the node/node.rs citations at 558-575 and 1424-1433 moved
established-decisions#14.2 · yes · stale · the mutator.rs citations at 751-768 and 827-853 moved
established-decisions#14.3 · yes · stale · the node/element.rs citation at 455-461 and the mutator.rs citations at 1017-1241 moved
established-decisions#15.1 · yes · stale · the config.rs citation at 76-95 and the node/element.rs citations at 79-80, 342-343 and 1026-1052 moved
established-decisions#16.1 · no · the take changes no window or shell code (its Parallel change sits in dioxus-native's launch and the browser), and no shell line this entity cites moves
established-decisions#17.1 · no · the paint crate and the password mask are untouched by the take
established-decisions#18.1 · yes · stale · the apps/browser/src/document_loader.rs citation at 206-254 moved
