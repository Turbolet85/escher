# tests · E2E Test Strategy — category read

## Carried
- (.andromeda/test-plan-detail/e2e-test-strategy.md:8) The command line's own Tab witness in `cli_commands` must stay in agreement with the stand's re-asserted boot Tab order: if the re-read moves the first stop of a lean task's boot sequence, this end-to-end reading moves with it, re-recorded with the change named, never left contradicting `common::tab_order`.
  > a line that is no command is a usage error, status `2`; Tab on a fresh boot moves focus to `back-btn` alone;
- (.andromeda/test-plan.md:286) The broad line for that witness: the command-line checks run the package's own `escher-session` binary as a process, unix only, so a Tab-order re-record that reaches `cli_commands` has to read green as that process run as well as in the in-process stand checks.
  > - **Command line (`escher-session`, unix only):** each driver command is run as a process of the package's own binary against a host its `start` command spawned — `cli_commands`, the two shell flows of `cli_flow` and `host_timer`;
- (.andromeda/test-plan.md:299) The chunk's re-assertions have to stay headless (the stand checks and the `escher-session` binary). It must not add a test that launches a window, because the windowed assistive-technology witness is outside this chunk's scope.
  > - No automated assertion in the example fixtures, and no test in the apps that launches a window — the UI is driven through the `escher-session` binary (Drivers per surface)

## Bindings
- **`cli_commands`' fresh-boot Tab reading ↔ the body's §3 stand tables**: the end-to-end `back-btn` first stop (.andromeda/test-plan-detail/e2e-test-strategy.md:8) and the shared `tab_order(task)` / `ENABLED_TAB_ORDER` tables `stand_keyboard` reads (.andromeda/test-plan.md:146) must say the same sequence. A boot-order change the re-read records moves both.
- **No windowed test ↔ the body's §1 owed adapter witness**: the E2E absence of a window-launching test (.andromeda/test-plan.md:299; .andromeda/test-plan-detail/e2e-test-strategy.md:19) stays as it is under this chunk. The platform-adapter reading it would need is owed to its residuals line (.andromeda/test-plan.md:48), outside this chunk.
