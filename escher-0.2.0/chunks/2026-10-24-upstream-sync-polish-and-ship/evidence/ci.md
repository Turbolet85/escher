# CI on the pushed take

- **Commit:** `34f6f051b81460d82837f9433dbe1faebaf49e56`, the operator pre-CI commit, pushed `a356f9086..34f6f051b` to `origin/build/escher-0.2.0`.
- **Run:** CI#40092092468 (push), completed/success: `verdict: green · checks 17/17 · wall 1210 s` (entry 38, `ci.py conclusion --sha HEAD --wait 1800`, polled 40× over 1207 s, from 2026-10-24T11:59:53Z). The Dependency audit job is green again (red on the base, CI#40081725681), and MSRV Build [Rust 1.91], Coverage report and Stand checks repeated are among the 17.

## Per-OS test tallies (entry 39, read 2026-10-24T12:20Z)
The job logs' libtest result lines summed per job (lines · passed · failed · ignored), against CI#39410865604 (the plan's P4 control):

| job | CI#39410865604 | CI#40092092468 | moved by |
|---|---|---|---|
| Test [default features] | 205 · 1098 · 0 · 12 | 206 · 1099 · 0 · 12 | +1 line, +1 passed: `hit_test_dropped_nodes` |
| Test (macos) | 186 · 1095 · 0 · 12 | 187 · 1096 · 0 · 12 | +1 line, +1 passed: `hit_test_dropped_nodes` |
| Coverage report | 186 · 1095 · 0 · 12 | 187 · 1096 · 0 · 12 | +1 line, +1 passed: `hit_test_dropped_nodes` |
| Test (windows) | 186 · 1051 · 0 · 9 | 187 · 1052 · 0 · 9 | +1 line, +1 passed: `hit_test_dropped_nodes` (not unix-gated) |
| Accessibility (a11y) tests | 6 · 43 · 0 · 0 | 6 · 43 · 0 · 0 | unmoved |
| Stand checks repeated | 168 · 684 · 0 · 21 | 168 · 684 · 0 · 21 | unmoved |

Each moved count is the one new upstream target, as forecast. No other count moved on any OS.
