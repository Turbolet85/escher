# 1A tree — escher-0.2.0

All epochs are at depth 1 (epoch → chunks), so there is no sub-block tier.

## Epoch 1 — Foundation
- Handler splits: escher's range and activation arms out of `handle_click`, `handle_pointerdown` and `handle_key_or_input_event`, with `parse_float` split into steps (source: requirements v020-01; audit proposals M1; intent §1)
- File splits: escher's inline test modules out of `document.rs` with upstream's body untouched, and `mcp.rs` / `schema.rs` split along their seams (source: requirements v020-01; audit proposals M2 · M3; intent §1)
- Fast-failing mutants: the six timeout-only mutants each fail a test fast, and the score holds at 0 survivors (source: requirements v020-02; audit proposals §Mutation tier; test-plan §10)

## Epoch 2 — Engine debts
- Upstream sync ahead of engine debts (source: intent §Principles "Kept from 0.1.0"; arch §Project Intent upstream-sync line; test-plan §9)
- Engine debts: nine recorded deferrals, each taken to the founder as a fork, then fixed and proven, pinned as a limit, or kept (source: requirements v020-03; intent §2; residuals L15–L18 · L20–L22 · L25 · L28; a11y-plan §1; security-plan §Threat Model Summary)

## Epoch 3 — Determinism and traces
- Upstream sync ahead of determinism and traces (source: intent §Principles)
- One virtual clock: every time a headless run reads, the scrollbar fade included, comes from one clock the driver alone moves (source: requirements v020-06; intent §5; residual L19; security-plan §API Security, the settle row; test-plan §9, the engine-features row)
- Session recording: a driver session through the CLI or MCP kept as a replayable check (source: requirements v020-04; intent §3; security-plan §Threat Model Summary, the stdio and IPC vectors; layout-templates §Surface: cli)
- Recorded-check replay: a recorded check run in CI as ordinary test input, failing on a behaviour change (source: requirements v020-04; intent §3; test-plan §2 · §9)

## Epoch 4 — Visual lint and real-app readiness
- Upstream sync ahead of visual lint and real-app readiness (source: intent §Principles)
- Visual lint: one command for overlap, clipping, truncated text, off-screen, zero size and failed contrast, each by stable id with its measured reason (source: requirements v020-05; intent §4; a11y-plan §6; obs-plan §4)
- Load wait: a command meeting a load in flight waits it out within a stated bound (source: requirements v020-07; intent §6; residual L10; security-plan §API Security, the settle and request-timeout rows)
- Select by id: a `select` driven by id and read in the snapshot (source: requirements v020-08; intent §6; residual L11; a11y-plan §1)
- Large screen in parts: a screen over the snapshot budget read in parts an agent walks (source: requirements v020-09; intent §6; test-plan §6, the snapshot budget reading)

## Epoch 5 — Polish & ship
- Upstream sync ahead of polish and ship (source: intent §Principles)
- Stand requirement sweep: every 0.2.0 capability proven headless on the stand or a fixture (source: intent §Principles; vision "What 0.2.0 done means"; test-plan §10)
- Capability statement: what escher can do and its stated limits, written where Viola's frontend planning reads it (source: requirements v020-10; intent §Principles "escher leads Viola")
- Cold-agent bar: a fresh agent over 0.2.0's new capabilities, judged against the founder's bar from 0.1.0's baseline (source: requirements v020-11; intent §7; residual L9; test-plan §6; security-plan §Threat Model Summary, the cold-agent carriage)

18 chunks against the Phase 0 estimate of 17, within ±20%. Foundation has 3 against an estimate of 2, because the
audit's two structural proposals and its mutation finding are separate units.
