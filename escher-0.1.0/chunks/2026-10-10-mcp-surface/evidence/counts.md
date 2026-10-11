# Counts — 2026-10-10-mcp-surface

The counts the wrap restates (plan step 15), each with the command that read it. Measured 2026-10-11 on the tree
the first full gate pass ran (trail `.andromeda/runs/2026-10-11T07-48-16-implement/`), each artifact written after
that pass began.

- **Workspace tests** — `bash .github/scripts/ci-leg.sh fast` (gate entry 15), then a parse of every `test result:`
  line of `target/ci-logs/test.log` (written 08:05:56Z): **170** result lines (base 167; three new test targets),
  **805** passed (base 791; +14 = 9 escher-driver unit · 2 `mcp_server` · 1 `mcp_log` · 2 `stand_mcp_tools`),
  **0** failed, **11** ignored.
- **`run stand`** — `bash scripts/agent-run.sh run stand` (gate entry 14), its `run.end` in
  `target/agent-run/events.jsonl` (08:04:26Z): **127** passed · **0** failed · **6** ignored · cargo exit 0 ·
  outcome passed, over **33** files (`run.start.files`; base 125 · 0 · 6 over 32).
- **escher-driver unit tests** — `cargo test -p escher-driver --locked` (gate entry 1): **59** passed (base 50: the
  `mcp` module's 9 tests; the cli module's tables grow 61 → 64 rows and 17 → 19 rows inside their standing tests).
- **seven_guis integration targets** — `ls examples/seven_guis/tests/*.rs | wc -l`: **7** (base 5: + `mcp_server`,
  `mcp_log`).
- **The `mcp` role's stderr at `info`**, one flow (`initialize`, `start crud`, `snapshot`, `type` into `crud-name`,
  `click crud-create`, `stop`) fed on stdin to `target/debug/escher-session mcp` with `RUST_LOG=info`:
  - built alone (`cargo build --locked -p seven_guis --bin escher-session`): exit 0 · stdout **6** lines · stderr
    **9** lines (1 install line · **5** `span="tool_call"` · **3** `span="command"`);
  - built with the workspace (`cargo build --workspace --locked`): exit 0 · stdout **6** · stderr **10** — the same
    nine and one `INFO blitz_dom::document … message=[redacted]` line;
  - the binary was rebuilt per-package afterwards.
