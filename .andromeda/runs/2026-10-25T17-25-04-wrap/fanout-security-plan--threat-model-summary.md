proposals:
  - detector: D-new-detail
    severity: escalate
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 37
    section: "## Threat Model Summary"
    change: "after line 37, inside the same boundary-mcp-stdio block, one new continuation line: ' Since 2026-10-25-cold-agent-bar the pipe''s second live run (the founder''s one run of 2026-10-25, `claude` 2.1.308, the operator''s own login) is a further carriage: through the client it carried the screens of six stand apps — the lean four, Cells'' contents read in parts and the load page — the flight booker''s select with its options and value, a lint''s findings and one shot as an `image` block, over 66 tool calls (0 wrong, $0.8244), to the model provider, and the model''s reply returned as the agent-written check the pipe parses and replays as before (the Input Validation cold-agent check row) — a boundary widening on the founder''s F3 «Один запуск, до $3 (Рекомендую)» (inputs#I3) and his word on the run «Запускать сейчас (Рекомендую)» (inputs#I6, relayed verbatim) — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md'"
    sidecar: "security-plan Threat Model Summary: the MCP stdio crossing records the 2026-10-25 cold-agent live run's carriage of the six-app stand's screens (Cells in parts, the select's options and value, a lint's findings, one shot) to the model provider, a boundary widening on the founder's F3 and I6."
    rationale: "Report F15 (`live-run`, stale at threat-model-summary.md:35) and Expected amendments → security-plan §Threat Model Summary / §Surfaces: the live run carried the stand's new screens (Cells' contents, the select's options and value, a lint's findings, a shot) on «Запускать сейчас (Рекомендую)» (inputs#I6) within F3 «Один запуск, до $3 (Рекомендую)» (inputs#I3); Outcome v020-11 gives 66 calls, 0 wrong, lint 1, screenshot 1, part_reads 2, six tasks met; Dev-tool versions give `claude` 2.1.308. The file records only the 2026-10-15 run (43 calls, no screenshot). A new line, since line 35 is already near the 2 000-char entity bound."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:175
    fact: F15
  - detector: D-new-detail
    severity: escalate
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 35
    section: "## Threat Model Summary"
    change: "in line 35, 'Since 2026-10-15-cold-agent-test the cold-agent pipe''s one live run is a second carriage' becomes 'Since 2026-10-15-cold-agent-test the cold-agent pipe''s 0.1.0 live run was a second carriage'; the rest of the line (its 43 tool calls, no `screenshot`, the founder's labels and its as-measured cite) kept as the dated reading of that run"
    sidecar: "security-plan Threat Model Summary: the 2026-10-15 cold-agent run is restated as 0.1.0's live run, no longer the pipe's one live run."
    rationale: "Report F15 marks threat-model-summary.md:35 stale: the pipe now has two live runs (0.1.0's of 2026-10-15, 4 tasks, 43 calls; the founder's of 2026-10-25, 6 tasks, 66 calls), so 'one live run is' no longer holds; the sweep of the claim the primary's new record retires."
    basis: .andromeda/security-plan-detail/threat-model-summary.md:35
    dependent-of: D-new-detail
note: a boundary widening (F15, the report's expected answer card, playbook «Boundary widening» .andromeda/playbook.md:36) rides both proposals and must go to the founder's card before apply; threat-model-summary.md:25's `examples/seven_guis/src/session_host.rs:34-56` is an F1 citation row whose new range the report's New text does not state, left to the P2 citation sweep; D-security-input, D-security-auth and D-security-deps find nothing in this file (no new external input in its vectors, the login unchanged, no dependency).
