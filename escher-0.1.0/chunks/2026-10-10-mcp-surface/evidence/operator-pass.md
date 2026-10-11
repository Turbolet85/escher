# Operator pass — 2026-10-10-mcp-surface

Run 2026-10-11 on the operator's instruction, each entry in plan order. Entries 18 and 20 were fired through
`gate.py run --plan escher-0.1.0/chunks/2026-10-10-mcp-surface/plan.md --operator {n}` (no run dir); entries 19 and 21
as written. Lines are copied without their headers.

## Entry 18 — the live trial (once), 08:09:58Z

```
operator entry 18 · history tripwire: none of 7 forms matched — not a clearance
 18 e2e         green · exit 0 · 10.28s · 697 B → 18.log · history: unmoved · cargo build --locked -p seven_guis --bin escher-session &&… (132 chars)
entries 21 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 20
```

The trial line is in `evidence/live-trial.md`:
`trial: client_exit 0 · isolated true · tool_calls 7 · refused 0 · errors 0 · end_state reached · protocol_version 2025-11-25 · discover sent`.

## Entry 19 — hygiene, before the pre-CI commit

```
hygiene: clean — read 47 (runs 38 · evidence 4 · inputs 5) · trails 14 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## The pre-CI commit

`36877a237318b73df6116b5899d9e0bb341f9068` — `chore(2026-10-10-mcp-surface): operator pre-CI commit, for the run this
chunk verdict reads`, the whole tree (`git add -A`, as the previous chunk's pre-CI commit did); the tree clean after.

## Entry 20 — the fast leg, the clean-tree guard and the push

```
operator entry 20 · history tripwire: git
 20 probe       green · exit 0 · 127.22s · 86169 B → 20.log · history moved: refs/remotes/origin/build/escher-0.1.0 69a399da→36877a23 · bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… (122 chars)
entries 21 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 20
```

The history move is the intended push. `git ls-remote origin build/escher-0.1.0`, read live after it:
`36877a237318b73df6116b5899d9e0bb341f9068`.

## Entry 21 — the CI read on the pushed sha — RED

```
36877a237318 verdict: red · checks 16/16 · first-fail +261 s Build wasm examples · runs CI#38124028789 in_progress/-
runs: CI#38124028789 push in_progress/-
failed 1: Build wasm examples (failure)
running 10: oldest Test (android) 33 s
run open: CI#38124028789 in_progress
```

Read at 2026-10-11T08:18:00Z, after 10 polls over 280 s; the run was still open. The failed job's log (job
114425250564), read and not kept: the job failed in its toolchain step, before any build — `rustup` could not download
`channel-rust-stable.toml` from static.rust-lang.org ("error reading a body from connection: connection reset"), then
`Process completed with exit code 1`. No source of this chunk was compiled by that job.

The pass stopped here on the operator's rule (stop on any red entry) and the red was reported.

## The rerun, and entry 21 again — GREEN

The operator confirmed the cause on their side (the wasm job died in `rustup` on a connection reset from
static.rust-lang.org before any compile) and directed the rerun: wait until the run reads completed, rerun the failed
job, fire entry 21 again on `36877a23`.

- The run read `completed failure` at 2026-10-11T08:29:10Z; its one failed job was `Build wasm examples`.
- `gh run rerun 38124028789 --failed -R Turbolet85/escher` — exit 0 at 08:29:21Z.
- Entry 21 fired again as written, HEAD `36877a237318b73df6116b5899d9e0bb341f9068`, read at 08:34:29Z:

```
36877a237318 verdict: green · checks 16/16 · attempt 2 · wall 1267 s · runs CI#38124028789 completed/success
runs: CI#38124028789 push completed/success
re-run: CI#38124028789 attempt 2 — each check is its latest attempt's; an earlier attempt's result is unread
checks: Accessibility (a11y) tests · Build [default features] · Build counter example · Build wasm examples · Clippy
  Coverage report · Dependency audit · Documentation · MSRV Build [Rust 1.91] · Rustfmt · Test (android) · Test (ios)
  Test (macos) · Test (windows) · Test CI scripts · Test [default features]
```

Attempt 1's red (`Build wasm examples`, the runner's toolchain download) stands in the record above; attempt 2 is the
reading this chunk's verdict takes. No other job of attempt 1 read `failure` when the run completed.
