# Escher

An agent-first UI framework, built on Blitz.

[![CI status](https://github.com/Turbolet85/escher/actions/workflows/ci.yml/badge.svg)](https://github.com/Turbolet85/escher/actions/workflows/ci.yml)

## What Escher is

Escher is a UI framework whose screens an AI agent can see, drive and verify headlessly, with no display and no outside driver. Every element of the demo app carries a stable id that stays the same across a re-render, a remount and a restart. An agent reads the screen as a compact snapshot of ids, roles, names and states, a large screen in parts, acts on an element by its id, a `select` included, and gets back exactly what changed. The driver runs as a command line and as MCP tools, describes itself through `help`, takes screenshots with no display, and lints the screen for six visual defects without changing it. Every headless run reads one virtual clock that only the driver moves, an acting command waits a bounded time for a page load in flight, and a driver session can be recorded as a check that replays.

## Why it exists

An agent with no prior context should learn the framework from the tool alone, then drive and check a UI headlessly. Today a screen is readable only as an HTML string, a debug print or a screenshot, and its elements have no stable id to aim at.

## Built on Blitz

Escher is a fork of [Blitz](https://github.com/DioxusLabs/blitz), the modular HTML/CSS rendering engine by DioxusLabs and its contributors. Upstream changes are merged in at the end of each stage of work, so Escher stays close to Blitz and its own changes stay additive. All credit for the engine goes to the Blitz authors.

## Try it

You need Rust 1.91 or newer, and on Linux the fontconfig development package (`libfontconfig1-dev`, or `fontconfig` on Arch).

Run the demo stand in a window:

```sh
cargo run --release -p seven_guis --bin seven_guis_native
```

Run its headless checks:

```sh
bash scripts/agent-run.sh boot
bash scripts/agent-run.sh run stand
```

These run the demo stand (the 7GUIs tasks) and its headless checks.

The agent driver has a command line. The `escher-session` binary starts a session on a stand task (`counter`, `flight-booker`, `timer`, `crud`, `cells` or `load`) and runs one driver command per call against it; each call answers one line of JSON on stdout and exits `0` when it ran, `1` when it was refused, `2` on a usage error and `3` on a session error:

```sh
cargo build -p seven_guis --bin escher-session
target/debug/escher-session start counter --session demo-session
target/debug/escher-session snapshot --session demo-session
target/debug/escher-session click --id counter-increment --session demo-session
target/debug/escher-session lint --session demo-session
target/debug/escher-session stop --session demo-session
```

An MCP client can drive the same apps: it spawns `target/debug/escher-session mcp` as its own child and gets the twelve driver commands as tools. [`docs/capabilities.md`](docs/capabilities.md) states what Escher can do and its limits. The driver describes itself: `target/debug/escher-session help` answers one line of JSON with every command, its arguments and its result, the refusal causes and session errors with their remedies, and the exit statuses; an MCP client gets the same from the `help` tool. The `screenshot` command returns the whole screen, or one element of it by `--id`, as a PNG, and an MCP client receives it as an image.

Since 0.2.0 the driver also does these:

- `lint` reads the screen for six defects (overlap, clipped, truncated, off-screen, zero-size and contrast) and answers each as a stable id, a closed defect word and numbers, changing nothing.
- `snapshot --part` reads a screen over the 10,000-byte text budget in parts, runs of whole lines that join to the whole text.
- `type` on a `select` chooses the first option whose label starts with the text.
- The five acting commands wait on a page load in flight for up to `LOAD_WAIT`, 5 s of wall time.
- `advance` moves the one virtual clock every headless document reads by exactly `--ms`.
- `start --record <dir>` keeps the session's answered calls as one owner-only check that replays through the driver; the two committed Counter checks under `tests/blitz-tests/checks/` are the one public exception.

## Status

### 0.1.0 — done and verified

Version 0.1.0 was built in six stages:

- **Epoch 1 — Foundation** — the inherited engine building and tested on the fork's CI, a headless 7GUIs demo stand, and a test harness an agent can run.
- **Epoch 2 — Element identity** — a stable id for every element, kept across re-renders, remounts and restarts, and carried into the accessibility tree.
- **Epoch 3 — Observation model** — a compact, semantic snapshot of the screen, and the exact diff after each action.
- **Epoch 4 — Driver core** — acting on elements by id, waiting until the UI has settled, and explaining every refused action.
- **Epoch 5 — Agent surfaces** — a command line, MCP tools, built-in help and schemas, and screenshots with no display.
- **Epoch 6 — Polish & ship** — keyboard, contrast and accessibility checks on the stand, quality gates, and a test where a fresh agent learns Escher from the tool alone.

All 17 of its capabilities are verified headlessly on the 7GUIs stand. A fresh agent given only the MCP tools drove the four lean tasks in 43 calls, with 0 wrong. The version-end audit's mutation testing left 0 surviving mutants of 1,558.

### 0.2.0 — done and verified

Version 0.2.0 was built in five stages:

- **Epoch 1 — Foundation** — the 0.1.0 audit's findings closed: the longest engine handlers and driver files split, and the six mutants that only timed out made to fail a test fast.
- **Epoch 2 — Engine debts** — upstream Blitz merged in, then each of the nine engine debts 0.1.0 recorded fixed and proven, pinned as a limit, or kept.
- **Epoch 3 — Determinism and traces** — one virtual clock for every headless run, so a repeated screenshot is identical, and a driver session recorded as an owner-only check that replays as an ordinary test.
- **Epoch 4 — Visual lint and real-app readiness** — shared fixture pages, the `lint` command that finds overlapping, clipped, truncated, off-screen and zero-size elements and low contrast, a bounded wait for a page load in flight, a `select` driven by id, and a large screen read in parts.
- **Epoch 5 — Polish & ship** — every new capability proven headlessly on the stand or a fixture beside it, the keyboard and contrast checks re-run, [a statement of what Escher can do and its limits](docs/capabilities.md), and a fresh agent driving six apps.

All 11 of its capabilities are verified. A fresh agent given only the twelve MCP tools drove six apps (the four lean tasks, Cells and a load page) in 66 calls, with 0 wrong, under a bar of at most 5 % wrong. The version-end audit's mutation testing caught 53 of 53 mutants in the engine's DOM crate; the demo stand's run went unscored, 12 of its 15 mutants unviable, and none of the 5 in upstream's WPT runner was caught.

### 0.3.0 — planned, in six stages

- **Epoch 1 — Foundation** — the functions the 0.2.0 audit found over the complexity ceiling brought under it, and this README and the project's records brought up to date.
- **Epoch 2 — Engine debts** — upstream Blitz merged in, then each of the engine's recorded debts fixed and proven, pinned as a limit, or kept: among them HTML's focus fixup for a focused node made inert, a native checkbox's or radio's checked state reaching the platform adapter, the lint reading a select's options as overlapping, a target inside a transformed scrolling box, the `select` limits, and keyboard and contrast checks for the pages served beside the four lean tasks.
- **Epoch 3 — The UI graph** — a probe of what the Dioxus runtime exposes, then one read that gives, for the current screen, each node's component, its handlers, the state they write and the nodes that read it; on it, a command listing what an agent can do on the screen now and why not, and one listing the handlers no recorded check reached.
- **Epoch 4 — Determinism and exploration** — a run's randomness from one seed, bundled fonts so every host reads one state, a session started in a given fixture state, and automatic breadth-first exploration that keeps each path as a replayable check.
- **Epoch 5 — Errors and explanation** — the panic, error or handler log an action caused returned with its answer, why an element sits where it does at its size, and one component rendered alone with given props.
- **Epoch 6 — Polish & ship** — every new capability proven headlessly, the accessibility checks re-run, the capability statement brought to 0.3.0, and a fresh agent driving the new capabilities.

Later: opt-in OpenTelemetry export, the platform accessibility adapter witnessed with a real assistive technology, and the next MCP protocol revision.

## License

Escher is dual licensed under the MIT and Apache 2.0 licenses: see [`LICENSE-MIT`](LICENSE-MIT) and [`LICENSE-APACHE`](LICENSE-APACHE).

The `stylo_taffy` crate is additionally licensed under MPL 2.0, so it is triple licensed under MIT, Apache 2.0 and MPL 2.0.

Unless you explicitly state otherwise, any contribution you intentionally submit for inclusion in Escher is dual licensed as MIT and Apache 2.0 (and MPL 2.0 if submitted to the `stylo_taffy` crate), without any additional terms or conditions.

## Author / Contact

Escher is written by [Turbolet85](https://github.com/Turbolet85), open to AI-related work. Contact: [turbolet85@gmail.com](mailto:turbolet85@gmail.com).
