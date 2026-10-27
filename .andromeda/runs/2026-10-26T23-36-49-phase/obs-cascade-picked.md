# obs · picked accounting

## Picked
telemetry-strategy#1.1 · not in scope · escher-driver's ungated `tracing` dependency and its compile-in to the two binaries, which the chunk's crates and manifests leave as they are
telemetry-strategy#2.1 · carried · the scope has the two touched tests pass in both builds the workspace runs, the build split this entity states
telemetry-strategy#3.3 · not in scope · standing S1 · a rewritten test function installs no subscriber, reads no env var and prints nothing, so the census reads 0 over its file
span-trace-coverage#1.10 · not in scope · standing S2 · a change adds no log site, span or span field
span-trace-coverage#2.1 · carried · `lint.rs` is edited and the scope names no span change: the lint stays inside its call's `command` span, recording `verb` alone and opening none of its own, its findings never fielded
span-trace-coverage#2.2 · not in scope · a hosted part read and `host_log`'s fourth test, neither reached
span-trace-coverage#3.1 · not in scope · the `tool_call` span lives in `mcp.rs`, which the chunk does not edit, and the listing test reads `tools/list`, which opens no span
span-trace-coverage#3.2 · not in scope · the cold-agent pipe's span counts; `scripts/cold-agent.sh` is not touched
log-coverage#6.1 · carried · the lint's closed-span line holds no finding, id, bound or count, which the factored `findings` must keep
log-coverage#9.3 · not in scope · standing S1 · a rewritten test function installs no subscriber, reads no env var and prints nothing, so the census reads 0 over its file
pii-scrubbing-compliance#1.1 · not in scope · the sink's scrub and the needle readings of `stand_act_spans`, `host_log` and `mcp_log`, which the chunk neither changes nor re-runs
ci-integration#1.3 · carried · the chunk re-runs the `fast` and `doc` legs, whose merged output `ci-leg.sh` writes to `target/ci-logs/{leg}.log`
ci-integration#1.7 · not in scope · the recording directory; the recorder, the replay and the committed checks are not edited
