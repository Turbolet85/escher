@task_prose part=session
The session moved the Taffy layout engine from the crates.io registry release 0.15.1 to upstream's git source at rev e90751fd (package version 0.15.0), the workspace's one git dependency, under the Taffy-only exception, with no patch section.
It took eight upstream blitz commits by no-commit cherry-pick, Taffy #1303 among them; the lock was the only conflict in each and was re-resolved; no other package moved.
The engine's one call that places a floated box now passes a caller-defined float id, the layout node's Taffy id, as its first argument.
The founder ratified the eight commits' layout changes for every document; they land ratified, not provisional.
Upstream's reversed-scroll-origin change (#1372) and the five later bumps behind it are left out for a separate route entry.
Measured: every test tally, the stand, stand-repeat, a11y, msrv, audit, doc, build, counter and wasm legs equal to before and green, CI green 17 of 17; no span, event, metric, log field, sink or harness surface was added, changed or retired.
@task_file n=1 path=Cargo.toml
what: the workspace manifest — the dependency pins of the engine crates, the taffy line among them, edited from a registry version to a git source and rev
@task_file n=2 path=Cargo.lock mark=conflict
what: the workspace lock — the resolved dependency set with sources and checksums; taffy's package edited to the git source, its checksum line gone; the cherry-picks' conflict resolved in it
@task_file n=3 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: blitz-dom's inline layout pass — the engine component that places floated boxes through Taffy; one call edited by a cherry-picked upstream commit
@task_item n=1 part=touches
A row that names Taffy as a crates.io registry version or names the engine dependency set as registry-only with checksums: Taffy is now a git source at e90751fd, the one git source in the lock.
@task_item n=2 part=touches
A row on engine tracing set by the build's invocation or behind a cargo feature, where it counts Taffy among the upstream crates whose tracing compiles in only with the tracing feature: Taffy's source moved to git, its eight features unchanged.
@task_item n=3 part=touches
A row on engine targets printing safe fields only or on the sink's target allowlist, where it rules over the layout engine's targets: the engine's layout component changed by one call and by the eight upstream commits.
@task_item n=4 part=touches
A row on the WPT report, WPT scores or WPT results published from upstream only, where it holds layout outcomes: the eight ratified commits change layout for every document, floats placement among them.
@task_item n=5 part=touches
A row on the snapshot, the snapshot and diff bounds or the shot, where it rules over the geometry layout produces: the ratified layout changes reach every document's boxes.
@task_item n=6 part=touches
The metric coverage overview naming blitz-dom's resolve phases and layout timings held in process: the layout pass behind them changed by one call; no timing or counter was added or removed.
@task_item n=7 part=touches
A row on CI on GitHub workflows or fork CI artifacts, where it holds a leg list or a count of checks: the run used the same legs, CI green 17 of 17, no leg added or removed.
@task_item n=8 part=touches
A row on the WASM console panic hook or the WASM build, where it rules over what the wasm build compiles: the wasm leg built green with the git Taffy.
@task_item n=1 part=untouched
The driver's command and tool_call spans, their fields, the refusal cause names and the tool_call endings: no span or field changed.
@task_item n=2 part=untouched
escher's telemetry sink, its log line format, scrub set, RUST_LOG handling, resource keys service.name and service.version, and the panic hook: none changed; the smoke still logged service.name.
@task_item n=3 part=untouched
The agent-run and cold-agent harness events, the cold-agent verdict and its counted verbs, and the kept check files: the harness and gate surface did not move.
@task_item n=4 part=untouched
escher-session's commands, exit codes, mcp and coverage roles, host wire, recording file and session lifecycle: no schema, config or public symbol changed.
@task_item n=5 part=untouched
The PII scrubbing and compliance rows, the select, optgroup and form-entry rows, and the state registry: no content handling changed.
@task_item n=6 part=untouched
The absences of a telemetry backend, metrics exporters, metric emission, OTel export and an error-reporting service: none was added.
@task_item n=7 part=untouched
The script error surface, the window error event and onerror, and the script diagnostics drain: the script runtime was not reached.
