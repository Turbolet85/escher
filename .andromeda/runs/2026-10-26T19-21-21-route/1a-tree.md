# 1A tree — escher-0.3.0

Depth 1 in every epoch (complexity-outline.md); no sub-blocks.

## Epoch 1 — Foundation
- Codebase health (source: requirements v030-01; intent §1; the Epoch 5 audit `.andromeda/runs/2026-10-25T21-43-01-code-audit/proposals.md` M1, not opened here — intent §1 quotes it)
- README and records (source: requirements v030-02, v030-16; intent §2, §Also carried; arch §Project Intent "Front page"; residual lines 41, 42; design-system §Core Colors accent row)

## Epoch 2 — Engine debts
- Upstream sync ahead of engine debts (source: requirements v030-03; intent §3; arch §Project Intent "Upstream sync"; residual lines 35, 36; security-plan §Dependency Security)
- Engine debts (source: requirements v030-04; intent §4; residual lines 31-34, 39, 40; arch §Established Decisions [Scrolling and input], [DOM semantics]; a11y-plan §2, §5)

## Epoch 3 — The UI graph
- Upstream sync ahead of the UI graph (source: intent §Principles "upstream is merged at each epoch boundary"; test-plan §9 per-OS tallies; obs-plan §8 the sink allowlist)
- Dioxus runtime probe (source: requirements v030-05; intent §5, §Principles "D2 first"; arch §Standard Contracts "Dioxus DOM bridge")
- The UI graph (source: requirements v030-05; intent §5; residual line 23)
- What can I do here (source: requirements v030-06; intent §6; residual line 12; arch §Standard Contracts "Dioxus DOM bridge" `unkeyed_actionable`)
- Handler coverage (source: requirements v030-07; intent §6; test-plan §2 "Recorded-check replay")

## Epoch 4 — Determinism and exploration
- Upstream sync ahead of determinism and exploration (source: intent §Principles)
- Seeded randomness (source: requirements v030-13; intent §9)
- Bundled fonts (source: requirements v030-14; intent §9; test-plan §9 "Fonts")
- Start in a state (source: requirements v030-08; intent §7; arch §Conventions "Tests" — the shared fixture boot and pages)
- Auto-exploration (source: requirements v030-09; intent §7; test-plan §2 "Recorded-check replay")

## Epoch 5 — Errors and explanation
- Upstream sync ahead of errors and explanation (source: intent §Principles)
- Errors with the action (source: requirements v030-10; intent §8; security-plan §Logging & Monitoring; obs-plan §7, §8)
- Layout explained (source: requirements v030-11; intent §9)
- A component in isolation (source: requirements v030-12; intent §9)

## Epoch 6 — Polish & ship
- Upstream sync ahead of polish and ship (source: intent §Principles)
- Stand requirement sweep (source: intent §Principles "the stand is the proof"; test-plan §9 Legs, §10 Coverage floors and Flakiness budget)
- A11y re-read (source: a11y-plan §5, §9)
- Capability statement (source: intent §Principles "escher leads Viola", "the API explains itself"; arch §Project Intent "Front page")
- Cold-agent bar (source: requirements v030-15; intent §10)
