# escher

An agent-first UI framework, built on Blitz.

[![CI status](https://github.com/Turbolet85/escher/actions/workflows/ci.yml/badge.svg)](https://github.com/Turbolet85/escher/actions/workflows/ci.yml)

## What escher is

escher is a UI framework whose screens an AI agent can see, drive and verify headlessly, with no display and no outside driver. Today every element of the demo app carries a stable id that stays the same across a re-render, a remount and a restart. The screen snapshot, the agent driver and its command-line and MCP tools are being built (see Plans).

## Why it exists

An agent with no prior context should learn the framework from the tool alone, then drive and check a UI headlessly. Today a screen is readable only as an HTML string, a debug print or a screenshot, and its elements have no stable id to aim at.

## Built on Blitz

escher is a fork of [Blitz](https://github.com/DioxusLabs/blitz), the modular HTML/CSS rendering engine by DioxusLabs and its contributors. Upstream changes are merged in at the end of each stage of work, so escher stays close to Blitz and its own changes stay additive. All credit for the engine goes to the Blitz authors.

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

The driver's MCP surface, the help it serves about itself and its screenshot are still to come.

## Plans

Version 0.1.0 is built in six stages:

- **Epoch 1 — Foundation** — the inherited engine building and tested on the fork's CI, a headless 7GUIs demo stand, and a test harness an agent can run.
- **Epoch 2 — Element identity** — a stable id for every element, kept across re-renders, remounts and restarts, and carried into the accessibility tree.
- **Epoch 3 — Observation model** — a compact, semantic snapshot of the screen, and the exact diff after each action.
- **Epoch 4 — Driver core** — acting on elements by id, waiting until the UI has settled, and explaining every refused action.
- **Epoch 5 — Agent surfaces** — a command line, MCP tools, built-in help and schemas, and screenshots with no display.
- **Epoch 6 — Polish & ship** — keyboard, contrast and accessibility checks on the stand, quality gates, and a test where a fresh agent learns escher from the tool alone.

Beyond 0.1.0: a graph of components, handlers and state; virtual time; fuller accessibility names and states; visual lint; turning traces into tests; automatic exploration; handler coverage; component isolation.

## License

escher is dual licensed under the MIT and Apache 2.0 licenses: see [`LICENSE-MIT`](LICENSE-MIT) and [`LICENSE-APACHE`](LICENSE-APACHE).

The `stylo_taffy` crate is additionally licensed under MPL 2.0, so it is triple licensed under MIT, Apache 2.0 and MPL 2.0.

Unless you explicitly state otherwise, any contribution you intentionally submit for inclusion in escher is dual licensed as MIT and Apache 2.0 (and MPL 2.0 if submitted to the `stylo_taffy` crate), without any additional terms or conditions.

## Author / Contact

escher is written by [Turbolet85](https://github.com/Turbolet85), open to AI-related work. Contact: [turbolet85@gmail.com](mailto:turbolet85@gmail.com).
