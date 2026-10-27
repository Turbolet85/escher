# security · Secret Management — category read

## Carried
- (.andromeda/security-plan.md:295) The chunk refactors a test in `packages/escher-driver/src/mcp_tests.rs`. That file's build-time `CARGO_MANIFEST_DIR` read is cited by line range in the env-read census. The read must stay the only env read in the file, and the cite must be re-pointed if factoring the test at line 399 moves those lines.
  > since 2026-10-19-session-recording in escher-driver's unit tests, which reach the workspace `target/tmp` through it (packages/escher-driver/src/mcp_tests.rs:633-634)
- (.andromeda/security-plan.md:301) The helpers or table walk that replace `lint.rs::findings` are escher-driver library code. They must add no env var read, so the library still reads no env var beyond `RUST_LOG`.
  > the escher-driver library's code and the `escher-session` binary read no env var beyond `RUST_LOG` through `escher_telemetry::init`
- (.andromeda/security-plan.md:292) None of the three refactored functions, and none of their new helpers, may read a secret. That keeps the census of secret reads in source at none.
  > **Never in code:** secret reads in source are observed absent in every slice that searched:

## Bindings
- **Env-read census (Secret Management) ↔ the body's Input Validation, driver command schema**: the census of env values read says escher-driver's library reads no env var beyond `RUST_LOG` (.andromeda/security-plan.md:301). The schema record says the same crate "installs no subscriber and reads no env var" (.andromeda/security-plan.md:79). A refactored `lint.rs` must leave both true.
- **Env-read census line cite ↔ the chunk's line moves in `mcp_tests.rs`**: the line range `mcp_tests.rs:633-634` (.andromeda/security-plan.md:295) follows the file's layout. Factoring `the_listing_is_the_twelve_verbs_of_the_table` (line 399) into a helper or table can shift it, and then the cite must be re-pointed at the chunk's wrap.
