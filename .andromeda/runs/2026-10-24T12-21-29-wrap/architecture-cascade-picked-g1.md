# architecture · picked accounting · group 1 of 4

## Picked
design-philosophy#2.1 · not touched · the providers, blitz-net, the script fetcher and the Dioxus bridge are unchanged; `config.rs`'s moved line numbers are the citation sweep's
design-philosophy#3.1 · not touched · no spec- or engine-cited behaviour is added or retired; the moved `stylo.rs` and `replaced.rs` cite digits are the citation sweep's
design-philosophy#4.1 · proposed · `Node` now carries the recorded `out_of_flow` flag and `ElementData` the class atoms, and hit testing skips nodes dropped since the last resolve
design-philosophy#5.1 · proposed · the new root example `multi_doc` times `resolve` over several documents beside the paint benchmark
stack-and-technologies#1.5 · not touched · taffy stays the registry "0.15"; the two git Taffy bumps were left out and its out-of-flow role is unchanged
stack-and-technologies#1.11 · proposed · tokio resolves to 1.54.1 in the lock, past RUSTSEC-2026-0386, the manifest's "1.42" unchanged
stack-and-technologies#1.16 · proposed · every `Parallel` traversal on Stylo's pool is now taken under the process-wide `PARALLEL_TRAVERSAL_LOCK`, and thin-vec holds `ElementData::classes`
stack-and-technologies#1.18 · proposed · mimalloc is now in the browser's `default` features, no longer optional only
stack-and-technologies#1.20 · proposed · the WPT runner gains `mimalloc` "0.1.48" as a direct dependency and its global allocator
stack-and-technologies#1.21 · proposed · the flake's rust-bin pin moved 1.90.0 → 1.91.0, retiring the recorded drift from `rust-version`
