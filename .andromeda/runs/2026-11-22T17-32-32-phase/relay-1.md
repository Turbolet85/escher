# Relay 1 — the operator's word for a plan revision of 2026-11-22-epoch-2-corrective

Given in the `/andromeda-phase` invocation, 2026-11-22, by the operator (his own word, not a delegate's), verbatim:

> A plan revision of the pending chunk 2026-11-22-epoch-2-corrective, nothing else changes. CI#45043508073 on HEAD 40ec4749e read red on Dependency audit alone: a new advisory, RUSTSEC-2026-0592 (url — a stack overflow in Url::origin() on nested blob: URLs; solution: cargo update -p url to >= 2.5.9). Measure it on the host first (the audit leg at HEAD), then add to the plan a lock-only update of url — the manifest unchanged, the lock move measured and named —, its gate entry (the audit leg green), the guard's allow for Cargo.lock, and re-run only the baselines the change touches. Stop on the revised review card.

Read: the stop is this word. `/andromeda-implement` has not run on the plan (no `*-implement` run dir under
`.andromeda/runs/`), so no step stands done.
