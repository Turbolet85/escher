# P5 controls — 2026-10-15-cold-agent-test

Each entry fired through `gate.py run --plan escher-0.1.0/chunks/2026-10-15-cold-agent-test/plan.md --entry {n}
--project-root $TMPDIR/{scratchpad}/ctl`, over a `git clone --shared` of the project at HEAD
`01297a12af19946fe1d213cc8d549907ccb71ddb` (the clone's role: the control tree). Unmodified first — each read what it
read on the real tree — then with its input, the input restored after each pair. The lines as the tool printed them:

| entry | input | line |
|---|---|---|
| 1 | clone unmodified | `green · exit 0` |
| 3 | clone unmodified | `red · exit 0 ✗ (exit 1)` |
| 4 | clone unmodified | `green · exit 1` |
| 5 | clone unmodified | `green · exit 0` |
| 11 | clone unmodified | `red · exit 0 ✗ (exit 2)` |
| 1 | must-fail: `if then` appended to scripts/cold-agent.sh | `red · exit 0 ✗ (exit 2)` |
| 4 | must-fail: a line naming `os.environ` appended to scripts/cold-agent.sh | `red · exit 1 ✗ (exit 0)` |
| 3 | must-pass: scripts/cold_agent_stub.py removed | `green · exit 0` |
| 5 | must-fail: packages/escher-driver/src/mcp.rs edited | `red · last line '0' ✗ (last: '1')` |
| 5 | must-pass: scripts/cold-agent.sh edited only | `green · exit 0` |
| 11 | must-pass: evidence/live-verdict.json written by `json.dump(…, indent=2)` with `outcome` passed | `green · exit 0` |
| 11 | must-fail: the same file with `outcome` failed | `red · exit 0 ✗ (exit 1)` |

Entries 2 and 6 run suites this chunk rewrites (2) or extends (6): their control is /implement's red-before-green run.
