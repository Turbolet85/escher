group: architecture+test-plan+obs-plan
read: CLAUDE.md · 150 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:84   class: omission
  leaf-text: «`packages/escher-driver/src/{cli,json}.rs` · the binary»
  source: .andromeda/architecture.md:141 «Three of the eight sit in escher-driver beside `mcp::tests` and `schema::tests`, each moved out whole with its test paths unchanged: `cli::tests` in `cli_tests.rs`»
  replace-with: «`packages/escher-driver/src/{cli,json}.rs` (`cli`'s unit tests in `cli_tests.rs`) · the binary»

- leaf: CLAUDE.md:83   class: omission
  leaf-text: «`packages/escher-driver/src/{session,host,client,wire,error,recording,replay}.rs` · host»
  source: .andromeda/architecture.md:141 «`recording::tests` in `recording_tests.rs` (`#[cfg(all(test, unix))]`) and `recording::off_unix_tests` in `recording_off_unix_tests.rs`»
  replace-with: «`packages/escher-driver/src/{session,host,client,wire,error,recording,replay}.rs` (`recording`'s unit tests in `recording_tests.rs`, its off-unix one in `recording_off_unix_tests.rs`) · host»
