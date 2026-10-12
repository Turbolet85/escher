# Counts — 2026-10-12-self-description

Step 19 of the plan. Each figure is a reading taken in this implement run
(`.andromeda/runs/2026-10-12T08-17-16-implement/`), on the tree its second full gate block read green.

| count | before | measured | read from |
|---|---|---|---|
| escher-driver unit tests | 59 | 66 passed · 0 failed · 0 ignored | gate entry 1, second firing (`cargo test -p escher-driver --locked`) |
| escher-driver re-exports | 41 | 45 (`describe`, `EXIT_STATUSES`, `ExitStatus`, `NodeFieldSpec` added) | the names of every `pub use` in `packages/escher-driver/src/lib.rs` |
| `tools/list` result | 2,984 bytes, 9 tools (reply line 3,018) | 8,844 bytes, 10 tools (reply line 8,878) | `initialize` then `tools/list` sent to `escher-session mcp`, `RUST_LOG` unset, stderr 0 bytes: before, the binary built from `bf973d33` in a scratch worktree (removed after); after, `target/debug/escher-session` of this tree |
| `run stand` `run.end` | 127 · 0 · 6 over 33 files | 131 passed · 0 failed · 6 ignored over 34 files | gate entry 14, second firing (`bash scripts/agent-run.sh run stand`); files are the distinct `file` values of its `test` events |
| workspace tests | 805 · 0 · 11 over 170 result lines | 821 passed · 0 failed · 11 ignored over 171 result lines | `target/ci-logs/test.log`, written 08:36:52Z by gate entry 15's second firing (`ci-leg.sh fast`) |
| checks declaring `mod session_common;` | the masters say seventeen | 18 | `grep -l 'mod session_common;' tests/blitz-tests/tests/*.rs` |

The listing grows by the tenth tool and by each tool's description carrying its result fields
(`{name} ({kind}): {help}`, joined by `; `), the cost the operator's Q4 answer accepted; the
result stays under the 1,048,576-byte bound.

The workspace's sixteen new tests: escher-driver 7, dioxus-native-dom 1, blitz-tests 4 (a new
`stand_diff` test, two in `stand_mcp_tools`, the new `stand_mcp_masked`), seven_guis 4 (two in
`cli_commands`, two in `mcp_server`). The new result line is `stand_mcp_masked`'s target.
