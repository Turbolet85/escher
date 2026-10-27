# architecture · picked accounting · group 6 of 7

## Picked
occupied-resources#4.1 · proposed · blitz-vibey-script `tests/dom.rs` `temp_dir_with_files` (860-869) makes a directory under the system temporary directory and writes files into it, a test filesystem write beside `eab-{pid}` the entity does not register
occupied-resources#7.1 · not touched · the `js_console` target, the timer thread and the shell's statics stand; only the `js_console` citation moved to `runtime/events.rs`, the citation sweep's
occupied-resources#9.2 · not touched · `PARALLEL_TRAVERSAL_LOCK` is unchanged; `stylo.rs` gained only line 510, below the cited lines, and the claim holds
occupied-resources#10.1 · proposed · `TMPDIR` is now read at two test-only sites, the second in blitz-vibey-script `tests/dom.rs:863` (Counts `tmpdir`, `occupied-resources.md:78` changed)
cross-cutting-patterns#1.1 · not touched · `apps/browser/src/main.rs` gained only 192-193, below its cited lines, and no config surface moved (Schema / config: none)
cross-cutting-patterns#2.1 · not touched · tracing stays feature-gated; the uncaught-JS-error site moved whole to `runtime/events.rs:21`, so only a citation moved, and the subscriber installs are unchanged
cross-cutting-patterns#3.1 · proposed · the windowed shell's redraw requests now travel through its event loop, and the shell provider's IME enable is also called at a focused input's editor construction
cross-cutting-patterns#4.1 · not touched · the microtask drains and geometry flushes moved file with the runtime split and nothing was removed; `dioxus_document.rs` only gained a test, so the claims hold
cross-cutting-patterns#5.1 · not touched · the report re-read the changed-set inserts as unchanged; the `open` arms in `mutator.rs` set element state and add no changed-set site
project-intent#1.0 · proposed · upstream main is now read at `ebacd68bd`; eleven more commits were taken and sixteen left out, with the merge base unmoved (Counts `upstream`, `project-intent.md:9` changed)
project-intent#8.1 · not touched · crate manifests are byte-identical, and the crate docs at blitz-shell `lib.rs:3`/`:8` and blitz-vibey-script `lib.rs:8` sit above the added lines
