# Classified — architecture · conventions.b1

## Answers
conventions#1.1 · no · its cited CI legs, workflows, engine crates, apps/browser and WPT runner are unchanged and unreached, so the fmt and clippy commands and their allows hold
conventions#2.1 · no · the root manifest, the lockfile and the cargo-deny configuration are unchanged, no crate was added or removed, and the manifests it cites are unreached
conventions#3.1 · no · its cited doc headers and doc comments sit in files the session did not reach, and no engine crate changed
conventions#4.1 · no · its cited TODO and FIXME markers sit in engine files the session did not reach, the session adds no code marker, and its open lint defect is pinned as measured in stand_lint
conventions#5.1 · no · its cited visibility and naming examples sit in engine crates, and no engine crate under packages/ changed
conventions#6.1 · no · its cited feature gates and ungated tracing sit in unreached crates, and the telemetry sink, the escher spans and no engine crate changed
conventions#7.1 · no · its cited unsafe blocks and borrow workarounds sit in engine files the session did not reach, and no engine crate changed
conventions#8.1 · yes · stale · the session adds stand_cells_served, which walks Cells through the driver and is absent from this list of Session checks, and reaches common/mod.rs and session_common/mod.rs, where it places shared helpers
conventions#9.1 · no · the escher-driver unit-test files it cites are unreached, and the driver's schema and wire are unchanged
conventions#9.2 · no · fixtures/mod.rs is unreached and still holds HeldNet, and the stand's new LoadNet lives in the stand module this claim does not cover
conventions#10.1 · no · its cited binaries and example CSS constants sit in unreached files, and the session names no change to a binary or to example CSS
conventions#11.1 · no · its cited HTML fixtures under examples/assets are unreached, and no HTML file is among the session's reached files
