# Task — arch · 2026-10-24-stand-requirement-sweep

## The chunk
A sweep that re-proves, without re-claiming, each of 0.2.0's requirements v020-01 to v020-09 headless on a 7GUIs stand task or on a minimal fixture beside it, and records per requirement the task or fixture and the check that proves it; a requirement with no such check gets one.
The record follows 0.1.0's stated table in the stand requirement sweep check (rows joined to it or a table beside it), every named test checked to exist; v020-01 and v020-02 name their instrument witnesses (code-audit size and complexity, the host-run cargo-mutants tier) as they stand.
Every witness the record names runs inside the stand-repeat CI leg at the chunk's end (the leg's selection widened, or the proof carried by a stand check); the coverage leg's per-crate floors read at or over their floors, possibly re-read, raised or with blitz-dom added.
The stderr of the escher-session host and its mcp role is measured id-free for 0.2.0's new driver paths — the lint verb, snapshot's part argument and --record recording — at trace under both builds, by the host_log and mcp_log checks or beside them.
New fixture checks assert the visual lint's untested readings (a clipped or off-screen node holding text also reads truncated) and the text-extent reader's stated limits (RTL and vertical text as LTR lines, a transformed box at its client rect) in both layout modes; a reading judged a defect becomes a founder's fork, and only then does the lint or the engine reader change.

## Reached files
- tests/blitz-tests/tests/stand_requirement_sweep.rs
- tests/blitz-tests/tests/stand_lint.rs
- tests/blitz-tests/tests/text_extent.rs
- tests/blitz-tests/tests/common/mod.rs
- tests/blitz-tests/tests/session_common/mod.rs
- tests/blitz-tests/tests/fixtures/mod.rs
- tests/blitz-tests/Cargo.toml · merge surface
- .github/scripts/test_blitz_tests_targets.py
- examples/seven_guis/tests/host_log.rs
- examples/seven_guis/tests/mcp_log.rs
- examples/seven_guis/tests/common/mod.rs
- .github/scripts/ci-leg.sh · conflict surface
- .github/scripts/test_ci_workflows.py
- .github/scripts/coverage_floor.py
- packages/escher-driver/src/lint.rs
- packages/blitz-dom/src/text_extent.rs
- escher-0.2.0/verification-matrix.json

## Needs
- The test conventions: one behaviour per integration-test file opened by a `//!` doc, one test target per file in blitz-tests, both layout modes per scenario, the fixture asserted to produce its condition first, shared helpers in non-target modules, the fixture boot and hold, shared pages and held network stand-in stated once in the fixtures module, a page with one reader kept in its check's file.
- The single incremental pipeline: incremental and non-incremental layout identical, so every new fixture check runs in both modes.
- The CI contracts: the one leg runner `ci-leg.sh` shared by CI and the dev host, `coverage_floor.py` as the per-crate floor gate, and the stand-repeat leg's three rounds under a zero-failure budget.
- The CI infrastructure and job shape: the stand-repeat job needing the fast four, restoring the test job's `workspace-test` cache without saving and uploading its log on failure; the coverage job and its report upload; CI leg logs under `target/ci-logs/`.
- The test harness contract: `Harness` constructors, inspection and input helpers that pump, `settle`, `screenshot`, `text_contrast` and the shared `VirtualClock` that `tick` and `advance_time` move — the witnesses read through it.
- The headless stand contract: `seven_guis::stand` boots the four lean tasks offline in a pinned 800x600 Light viewport with one bundled font, and `timer_step` gives the Timer its ticks.
- The driver session contract: every call through `Session::run`, validated first; `Session::lint` reads the screen and changes nothing; the recording and `Check::replay`; `LOAD_WAIT`; the `command` and `tool_call` spans' closed fields — the surfaces the sweep re-proves and the stderr measurement exercises.
- The driver-session decision, indirectly through the lint: lint findings cross on every carrier, so changing what `Session::lint` returns moves a driver answer everywhere and is a boundary widening for the founder.
- The scrolling and geometry contracts the text-extent checks lean on: `text_extent` a read-only reader, `get_client_bounding_rect` reading an element where its transforms move it, `visible_region`'s untransformed-scroller limit, and the hit stopping at a clipping box's padding box.
- The document core contract: the document's clock through `DocumentConfig.clock`, `select_value`, `can_paint_text` and `layout_as_text_input` — witnessed by the virtual clock, select and password checks the record names.
- The DOM-semantics and scrolling-and-input decisions the named witnesses re-prove: `inert`, `disabled` read on form controls alone, the select model, `scroll_into_view` through every scrolling box, and that each is an engine change for every document — so any engine change the sweep proposes is the founder's fork.
- The events and nodes-and-widgets contracts behind the range-input, select and keyboard-activation witnesses: `input` then `change` on a select, a range input's default actions, Enter and released Space as a queued Click, `ElementData::is_disabled`, `RangeInputData`.
- The painting decision and paint contract behind the password caret witness: a password input painted masked in every document, its caret placed by the bullets.
- The Dioxus DOM bridge contract: stable element ids, the snapshot, its text parts within the budget, the diff and `is_masked_input` — witnessed by the stand snapshot checks.
- The telemetry bootstrap contract and the logging pattern: one stderr sink, foreign targets dropped at every level, engine targets printing only safe fields, escher fields with content redacted — the id-free stderr measurement reads against it.
- The `RUST_LOG` environment variable as the sink's filter, set to `trace` by the stderr measurement; no other env read added.
- The CLIs contract for `escher-session`: its verbs, `serve` host role, `mcp` role, `--record`, one JSON line on stdout and the four exit statuses — the carriers the stderr measurement drives.
- The filesystem resources: the session state directory and the recording directory, both caller-named and `0700`, and the two committed Counter checks as the one owner-only exception.
- The process-wide state: test binaries re-executing themselves as a session host on its Unix-domain socket, one session per host process.
- The renderer-backend decision and the cross-OS note: shots render through `VelloCpuImageRenderer` over white, and a check compared byte for byte across OSes holds no `screenshot` step.
- The default-features decision and config pattern: `accessibility` is default and gates the snapshot modules a witness reads.
- The stack's quality tiers: cargo-llvm-cov coverage with per-crate floors, and cargo-mutants under nextest's `mutants` profile as a host-only tier never in CI — v020-02's witness.
- The manifests convention for any new `[[test]]` target in blitz-tests' manifest: workspace dependencies consumed as `{ workspace = true }`, `publish = false`.
- The formatting, lint and documentation conventions every new check passes: fmt, clippy `-D warnings`, rustdoc `-D warnings`, `//!` module docs.
- The project intent and existing scopes: the stand is the proof with a minimal fixture beside it, and blitz-tests and seven_guis as the scopes the checks live in.

## Not needed
- The browser app and rdme: history database, navigation provider, URL suggester, about pages — no witness boots either.
- The WPT runner, its outputs and its upstream-only CI workflows — the sweep proves on the stand and fixtures, not WPT.
- JavaScript on Boa: script fetching, timers, the JS bootstrap — no stand task or fixture runs script.
- Dependency pinning, build profiles, release versioning, Nix packaging and cross-compilation — the chunk adds no dependency or profile.
- Networking through blitz-net, the outbound hosts and resource loading — the stand and fixtures boot offline.
- Shell and windowing, the shell loop, launch paths and the AccessKit platform adapter — the sweep is headless; the windowed witness is owned elsewhere.
- The stand's Tab order, Enter and Space tables and contrast rows in the a11y leg — owned by the later A11y re-read entry.
- The cold-agent pipe, the spawned client and the model provider host — v020-11 is the Cold-agent bar entry's.
- The capability statement for a consumer and the front-page README — v020-10 is the Capability statement entry's.
- HTML parsing, inline SVG, CSS approximations, unsupported features and table layout — no reading the sweep asserts rests on them.
- The deployment model and the publish-browser bundle job — upstream-only, never run on the fork.
- The upstream sync merge base and the waiting Stylo, Taffy and Dioxus versions — no sync in this chunk.
- The Unix socket listener's wire grammar and the MCP framing bounds — exercised as they stand, not changed.
- The example HTML fixtures under `examples/assets` and their form fixtures — the sweep's fixtures are Rust check pages in blitz-tests.
