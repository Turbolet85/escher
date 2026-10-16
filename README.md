# Escher

An agent-first UI framework, built on Blitz.

[![CI status](https://github.com/Turbolet85/escher/actions/workflows/ci.yml/badge.svg)](https://github.com/Turbolet85/escher/actions/workflows/ci.yml)

## What Escher is

Escher is a UI framework whose screens an AI agent can see, drive and verify headlessly, with no display and no outside driver. Every element of the demo app carries a stable id that stays the same across a re-render, a remount and a restart. An agent reads the screen as a compact snapshot of ids, roles, names and states, acts on an element by its id, and gets back exactly what changed. The driver runs as a command line and as MCP tools, describes itself through `help`, and takes screenshots with no display.

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

The agent driver has a command line. The `escher-session` binary starts a session on a stand task and runs one driver command per call against it; each call answers one line of JSON on stdout and exits `0` when it ran, `1` when it was refused, `2` on a usage error and `3` on a session error:

```sh
cargo build -p seven_guis --bin escher-session
target/debug/escher-session start counter --session demo-session
target/debug/escher-session snapshot --session demo-session
target/debug/escher-session click --id counter-increment --session demo-session
target/debug/escher-session stop --session demo-session
```

An MCP client can drive the same apps: it spawns `target/debug/escher-session mcp` as its own child and gets the eleven driver commands as tools. The driver describes itself: `target/debug/escher-session help` answers one line of JSON with every command, its arguments and its result, the refusal causes and session errors with their remedies, and the exit statuses; an MCP client gets the same from the `help` tool. The `screenshot` command returns the whole screen, or one element of it by `--id`, as a PNG, and an MCP client receives it as an image.

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

### 0.2.0 — planned, in five stages

- **Epoch 1 — Foundation** — the version-end audit's findings closed: the longest engine handlers and driver files split, and the six mutants that only timed out made to fail a test fast.
- **Epoch 2 — Engine debts** — upstream Blitz merged in, then each of the nine engine debts 0.1.0 recorded fixed and proven, pinned as a limit, or kept.
- **Epoch 3 — Determinism and traces** — one virtual clock for every headless run, so a repeated screenshot is identical, and a driver session recorded as a check that replays as an ordinary test.
- **Epoch 4 — Visual lint and real-app readiness** — shared fixture pages, one command that finds overlapping, clipped, truncated, off-screen and zero-size elements and low contrast, a wait for a page load in flight, a `select` driven by id, and a large screen read in parts.
- **Epoch 5 — Polish & ship** — every new capability proven headlessly, the keyboard and contrast checks re-run, a statement of what Escher can do and its limits, and a fresh agent driving the new capabilities.

Later: a graph of components, handlers and state; layout explained; automatic exploration; handler coverage; component isolation.

## License

Escher is dual licensed under the MIT and Apache 2.0 licenses: see [`LICENSE-MIT`](LICENSE-MIT) and [`LICENSE-APACHE`](LICENSE-APACHE).

The `stylo_taffy` crate is additionally licensed under MPL 2.0, so it is triple licensed under MIT, Apache 2.0 and MPL 2.0.

Unless you explicitly state otherwise, any contribution you intentionally submit for inclusion in Escher is dual licensed as MIT and Apache 2.0 (and MPL 2.0 if submitted to the `stylo_taffy` crate), without any additional terms or conditions.

## Author / Contact

Escher is written by [Turbolet85](https://github.com/Turbolet85), open to AI-related work. Contact: [turbolet85@gmail.com](mailto:turbolet85@gmail.com).
