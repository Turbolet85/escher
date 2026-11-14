@task_prose part=chunk
Builds one exploration command over a held app (capability v030-09): from boot it reads the actions available on the screen and drives them breadth-first within a step budget, then returns a closed report.
Each step is a validated call through the driver's one run path; a refusal is reported by its closed cause and a panic inside a step is caught and reported as a closed word, its path kept, and neither stops the run.
Each explored path is left as an owner-only kept check, with no screenshot step, that replays green through check replay.
The kept-check step format gains a stated format version and can carry a verb with more than one target, as an addition. Verbs are read from the schema's verb table and the actions answer, never from a list of exploration's own.
Proven on the stand's served tasks in both layout modes, and on a fixture that panics in a handler and one that holds a refusing control.
@task_item n=1 part=needs
The exploration command itself: from boot it enumerates the available actions and drives them breadth-first, each path re-reached from a fresh session and the frontier ordered by path length, until the step budget is spent. It returns a closed report of each panic and refusal it met.
@task_item n=2 part=needs
The available actions come from the driver's own readers, never a hand list: the actions verb's answer (Session::actions, its target verbs and node fields) and the driver verb table VERBS. A verb exploration drives is one the table holds and the actions answer offers on a node, so a future verb (drag) is driven with no reshaping.
@task_item n=3 part=needs
Every explored step runs as a validated call through the one run path (Session::run), as Session::reach does. There is no second executor, call validation keeps its order and first failure, and a refused call records no ids.
@task_item n=4 part=needs
A refusal met on a step is caught and reported by its closed refusal cause (one of the eight causes, malformed faults included). This is reported and does not stop the exploration.
@task_item n=5 part=needs
A panic raised inside a step (a handler that panics) is caught and reported as a closed word, with the path that led to it kept. The exploration goes on.
@task_item n=6 part=needs
Each explored path is left as a kept check in the recorded-session format (Check), on the owner-only medium: a 0600 check file in a 0700 launcher-named recording directory. A group- or other-open directory is refused as recording refuses it, and recording removes only its own checks.
@task_item n=7 part=needs
Each kept check left by exploration replays green through Check::replay, which replays through the one run path. Its opening record, digest, size bounds and kept verbs hold as for any kept check, and the strict kept-check reader reads it.
@task_item n=8 part=needs
The kept-check step format is versioned: a kept check states its format version, and a step can carry a verb with more than one target. The two-target step is an addition to the format, not a redefinition, and existing kept checks still read and replay. The production of drag or any two-target verb stays out.
@task_item n=9 part=needs
No screenshot step in an explored path and no pixel at rest. An explored check holds no screenshot.
@task_item n=10 part=needs
Indirect, through the recording: an explored kept check keeps masking (password and file values masked), carries no NodeId or other process-local value, and is never logged.
@task_item n=11 part=needs
Exploration reads the default seed alone (DEFAULT_SEED), as every driver session does. It takes no seed argument and puts no seed on any carrier. A later seed argument is an addition to the command's arguments, with no rework.
@task_item n=12 part=needs
Navigation is a provisional boundary fork. An action that would navigate the document (a link, a form submission to another URL) is not driven, and exploration loads nothing new.
@task_item n=13 part=needs
The carriers stay as they are: the command is provisionally in process only. It adds no new verb or role to the command line, MCP or the session socket, because the validated verb set and the carriers widen only by ratification.
@task_item n=14 part=needs
Logging: no new log field carries content. A span, if any, is a driver span built by hand that holds fixed words and counts only, and the driver emits no event.
@task_item n=15 part=needs
Untrusted input (the step budget, the recording directory) has no panic path. It is refused as typed values through typed results.
@task_item n=16 part=needs
Proof: the stand's served tasks explored in both layout modes, a fixture whose handler panics, and a fixture holding a refusing control. The stand boots offline with pinned environment and its readings are the same on every CI OS.
@task_standing n=1
convention: Each new test asserts its fixture's condition first, carries a descriptive name, and an integration test file holds one behaviour
home: conventions
@task_standing n=2
convention: Each new scenario test runs in both layout modes
home: conventions
@task_standing n=3
convention: Each new fixture states its cases numbered, is stated once in the shared fixtures module, and uses relative URLs
home: conventions
@task_standing n=4
convention: Each new unit-test module sits in the source file's path child, at the file's end, keeping its test paths
home: conventions · existing-scopes
@task_standing n=5
convention: Each new expect or unwrap carries a justification; new error types are enums and fallible APIs return typed results
home: conventions · cross-cutting-patterns
@task_standing n=6
convention: Each new public item and module carries docs, with rustdoc warnings as errors
home: conventions
@task_standing n=7
convention: Each new driver span is built by hand and holds no content; the driver library stays without side channels
home: conventions · standard-contracts
@task_standing n=8
convention: Each new cross-OS check holds no screenshot step and reads the same on every CI OS
home: existing-scopes · standard-contracts
@task_standing n=9
convention: Each new driver test keeps its crate above the per-crate line coverage floor
home: standard-contracts
@task_item n=1 part=not_needed
The style, layout and paint engine (Stylo, Taffy, Parley, paint scene, damage and incremental layout): exploration drives actions through the driver and changes no layout or painting.
@task_item n=2 part=not_needed
The WPT runner, its suites, result kinds, statuses, outputs and the WPT diff-to-PR tooling: exploration is no WPT test.
@task_item n=3 part=not_needed
The browser app: its history store, about pages, toolbar, persistence and favicon cache. Exploration runs over held driver apps, not the browser.
@task_item n=4 part=not_needed
The JavaScript runtime (Boa, script globals, script fetcher, microtask drain): exploration drives Dioxus apps through the driver and touches no page script.
@task_item n=5 part=not_needed
Accessibility tree contracts (ARIA states, names, platform adapter, refresh): exploration reads the actions answer, not the accessibility tree.
@task_item n=6 part=not_needed
The select list machinery and DOM event dispatch default actions: exploration triggers them as any driven action does and changes none of their behaviour.
@task_item n=7 part=not_needed
The cold-agent and agent-run scripts, their events, exit codes, state directories and verdict reasons: exploration is a driver command, not an agent run.
@task_item n=8 part=not_needed
The package-install script and the Nix flake: exploration adds no system package and no dev-shell change.
@task_item n=9 part=not_needed
The lint verb and its findings, and the graph verb and UI graph: exploration neither reads nor changes them.
@task_item n=10 part=not_needed
Platform shells and targets (winit window shell, Android and iOS entry, wasm builds, window renderer choice): exploration is headless.
@task_item n=11 part=not_needed
Upstream sync rules (merge base, cherry-picks, upstream-only workflows, the upstream publish job): this chunk takes nothing from upstream.
@task_item n=12 part=not_needed
Networking (net provider, HTTP cache, per-host request cap, URL scheme routing): exploration loads nothing new and drives no navigation.
