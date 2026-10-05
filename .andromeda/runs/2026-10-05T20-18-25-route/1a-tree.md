# 1A tree — escher-0.1.0

## Epoch 1 — Foundation (depth 1)
- As-built baseline on this host (source: calibration §Adopted project; intent §Principles order step 1)
- Fork CI reached + fast feedback (source: intent §Principles; synthesis-protocol §Placement fast feedback; arch §Infrastructure Patterns CI/CD)
- Dependency-audit + coverage legs (source: security-plan §Bootstrap phases dep-audit-tooling-install + dep-security-ci-gate; test-plan §Bootstrap phases coverage-tooling-install)
- Headless 7GUIs stand boot (source: intent §Principles "the stand is the proof"; arch §Existing Scopes seven_guis; test-plan §3 Harness)
- Cold-agent run pipe reachability (source: intent §9 cold-agent test + §8 agent session; synthesis-protocol §Placement — the gate drives an agent client beyond base CI, so its failure surface grows with the command set)

## Epoch 2 — Element identity (depth 1)
- Stable id assignment: author key, else component path (source: requirements v010-01; intent §1; arch §Standard Contracts Dioxus DOM bridge, Node identity)
- Id persistence across re-render, remount, fresh process (source: v010-02; intent §1 OBSERVED remount re-clone)
- Accessibility-tree identity + stand roles/names (source: v010-03 a11y half; a11y-plan §2 Role derivation; arch §Standard Contracts `build_accessibility_tree`)

## Epoch 3 — Observation model (depth 1)
- Snapshot data model id · role · name · state · bounds (source: v010-04; intent §2)
- Snapshot state fidelity (source: v010-05; intent §2; a11y-plan §5 disabled states)
- Compact snapshot serialization (source: v010-04 "readable whole in one tool result")
- Change tracking + snapshot diff (source: v010-06; intent §3 OBSERVED `has_changes()` inverted, changed nodes never drained; a11y-plan §2 `changed_nodes`)

## Epoch 4 — Driver core (depth 1)
- Settle detection (source: v010-10; intent §5; arch §Infrastructure Patterns Runtime threading/event flow, JS runtime timers)
- Command + refusal schema (source: v010-14 verb set; v010-11 causes; intent §Principles "one schema")
- Act by stable id, settled, with diff (source: v010-09; v010-06; v010-10)
- Refusal detection: not found · stale · disabled · covered · off-screen (source: v010-11; arch §Established Decisions DOM semantics hit-test canonicalization; a11y-plan §6 Visibility and hit-testing)
- Driver diagnostics (source: obs-plan §Bootstrap phases otel-sdk-install + pii-scrubbing-wire; security-plan §Bootstrap phases logging-redaction-wire)

## Epoch 5 — Agent surfaces (depth 1)
- Driver CLI (source: v010-12; intent §7; design-system §Surface: cli)
- MCP surface (source: v010-13; intent §8; security-plan §API Security no served API + §Bootstrap phases auth-scaffolding-baseline → a local, listener-free surface)
- Self-description across CLI + MCP (source: v010-14; intent §Principles "the API explains itself")
- Headless screenshot (source: v010-07, v010-08; intent §4; arch §Standard Contracts Paint `paint_scene`)

## Epoch 6 — Polish & ship (depth 1)
- Stand a11y assertions + a11y CI leg (source: a11y-plan §Bootstrap phases contrast-verification-harness-setup + a11y-ci-gate-wire; a11y-plan §1 tier criteria, §5, §6, §9)
- Stand requirement sweep (source: intent §Principles "every requirement verified headless, by an agent, on the stand")
- Cold-agent test (source: v010-15; intent §9; intent §Principles "cold-agent test last")
