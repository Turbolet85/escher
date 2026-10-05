# Tests validation — route draft

## Insert
- Between `Headless stand` and `Cold-agent run pipe`: **"Stand test contract — one agent-invocable command each to boot, run, check status, clean up and read JSON-line logs of stand checks and blitz-tests"** (epoch: `Epoch 1 — Foundation`)
  Reason: Per test-plan §3 Test Harness Contract, the 5-command contract (boot/run/status/cleanup/logs, log format) is NOT YET MEASURED and no chunk sets it up, yet every feature chunk from Epoch 2 on has to be runnable through the harness by an agent.
- Between `Stand requirement sweep` and `Cold-agent test`: **"Quality gates — coverage floor on driver crates and flakiness budget for stand checks, enforced on fork CI"** (epoch: `Epoch 6 — Polish & ship`)
  Reason: Per test-plan §10 Quality Gates, there is no threshold or flakiness budget (NOT YET MEASURED), and the Epoch 1 coverage leg (§9) only reports coverage without gating on it.

## Rewrite
- `Fork CI reached`: "cached builds, fast checks apart from slow" → "cached builds, fast checks apart from slow, test results and WPT diff uploaded as artifacts"
  Reason: Per test-plan §9 CI Integration, only the WPT workflow uploads an artifact, and test-job artifact upload is an expected CI-integration bootstrap item.
- `Headless stand`: "mounted with no display, booted fresh per check" → "mounted with no display at a fixed viewport, bundled fonts, no live network, booted fresh per check"
  Reason: Per test-plan §7 (fixed-viewport seeds, fixtures loading remote hosts at render time) and §8 (net-provider stand-ins), stand fixtures need deterministic inputs, or the later id-persistence, diff and pixel checks become flaky.
- `Settle detection`: "delayed stand update covered" → "delayed stand update covered on virtual time, no real-time sleeps"
  Reason: Per test-plan §2, script timer tests sleep in real time, while §3 records virtual time without a timer thread as built for test runners, which settle checks need to stay deterministic.
- `Headless screenshot`: "identical pixels on repeat" → "identical pixels on repeat; pixel checks fail, never skip, when fonts are missing"
  Reason: Per test-plan §2 and §8, font-dependent tests skip at runtime with `eprintln!` when no font is available, so a screenshot check could pass without asserting anything.
