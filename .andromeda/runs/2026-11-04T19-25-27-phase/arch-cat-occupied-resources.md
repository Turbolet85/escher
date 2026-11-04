@carried n=1 label=architecture/occupied-resources fact=fct-crxfkvwh
why: The probe must add no served port and no second listener: the session socket must stay the tree's one served listener (the scope's «Nothing crosses»: no listener, port or carrier).
quote: **Network ports and listeners:** no served network port — no TCP or UDP port a binary serves; one local listener — the driver session's Unix-domain socket `{state_dir}/session.sock`, set to mode `0600` inside a state directory the caller names and `escher_driver::serve` creates `0700`, bound at the tree's one served listener site
@carried n=2 label=architecture/occupied-resources fact=fct-drud2scu
why: Link (2), node → handlers, must read the bridge's existing listener-count state as recorded here and must add no process-wide state of its own beside it.
quote: 64 event-handler count slots indexed by `DomEventKind` (packages/dioxus-native-dom/src/mutation_writer.rs:22-23; packages/dioxus-native-dom/src/mutation_writer.rs:36)
@carried n=3 label=architecture/occupied-resources fact=fct-drud2scu
why: The probe must emit under no new log target or span; the escher log target and its spans stay as recorded, and nothing the probe reads may be logged.
quote: log target `escher_driver`, under which escher-driver's command span is emitted at INFO since 2026-10-07-driver-command-spans and its `tool_call` span since 2026-10-10-mcp-surface
@carried n=4 label=architecture/occupied-resources fact=fct-drud2scu
why: A probe check that re-verifies the MountedData panic in a child process must follow the recorded self-re-execution form: test-only, a fixed argv, no socket or port.
quote: five blitz-tests integration binaries re-execute their own test binary under `cargo test`, the first three once each and the fourth and fifth twice
@carried n=4 run=2 label=architecture/occupied-resources fact=fct-drud2scu
quote: test-only, a fixed argv, no socket or port
@carried n=5 label=architecture/occupied-resources fact=fct-xzzhfdyl
why: Any scratch or state directory a probe check in `tests/blitz-tests/` or `examples/seven_guis/` makes must live under `target/tmp/`, local only and uploaded by no CI leg.
quote: under `CARGO_TARGET_TMPDIR`, i.e. `target/tmp/`, beside the recording checks' scratch directories and escher-driver's unit tests', which reach the workspace `target/tmp` through the compile-time `CARGO_MANIFEST_DIR`, local only and uploaded by no CI leg
@carried n=6 label=architecture/occupied-resources fact=fct-7quovrlq
why: The probe must read no new environment variable; the recorded set (with `RUST_LOG` the escher filter) stays unchanged.
quote: **Environment variables:** `RUST_LOG`, the tracing filter directives `escher_telemetry::init` reads through `EnvFilter::try_from_default_env`, defaulting to `warn` when unset or unparsable (packages/escher-telemetry/src/lib.rs:144)
@carried n=7 label=architecture fact=fct-x4u4skbo
why: The probe must add no workspace crate, binary or feature; its code must sit in existing crates (test or example code).
quote: **Names:** the workspace's crates, among them escher-telemetry and escher-driver, `publish = false` libraries; binaries `blitz`, `seven_guis_native`, `escher-session` and `todomvc_native`; crate features
@carried n=8 label=architecture/occupied-resources fact=fct-bw7ybeh6
why: A probe run on the Counter, or anything of it kept in the stand, must leave the two committed Counter checks byte-identical to a fresh recording.
quote: read by `stand_replay` through `Check::from_bytes`, compared byte for byte with a fresh recording by seven_guis `recording` in `rc-fresh` under `CARGO_TARGET_TMPDIR`, uploaded by no CI leg
@binding n=1
**Occupied Resources ↔ the chunk scope's «Nothing crosses» boundary**: the probe adds no listener, port, env var, workspace crate, log field or span, so the network, environment and names lines must stand unchanged at wrap; any resource the probe does land must be recorded in this category at wrap (architecture/occupied-resources fct-crxfkvwh; architecture/occupied-resources fct-7quovrlq; architecture fct-x4u4skbo)
@binding n=2
**Process-wide state (listener-count slots) ↔ Inherited Defaults and Project Intent (Dioxus "0.7.3")**: link (2)'s reading through `packages/dioxus-native-dom/src/mutation_writer.rs` must be made against Dioxus 0.7.3 with no pin move, patch or vendored copy (architecture/occupied-resources fct-drud2scu; architecture fct-656exhoj; architecture fct-7lmrueug)
@binding n=3
**Filesystem and self-re-executing test binaries ↔ Conventions → Tests**: a probe check must keep its scratch under `target/tmp/`, re-execute itself (if at all) with a fixed argv and no socket, and run in both layout modes where layout touches the reading (architecture/occupied-resources fct-xzzhfdyl; architecture/occupied-resources fct-drud2scu; architecture fct-wqndq65v)
@binding n=4
**Committed checks ↔ Standard Contracts → Driver session recording**: a Counter run must not change what `recording` compares byte for byte against `tests/blitz-tests/checks/` (architecture/occupied-resources fct-bw7ybeh6; architecture fct-elnqiowc)
