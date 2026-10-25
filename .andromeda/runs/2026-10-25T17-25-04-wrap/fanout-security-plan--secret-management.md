proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/secret-management.md
    near: 15
    section: "## Secret Management"
    change: "Extend the Development item after line 15's run-by-run account with one clause: since 2026-10-25-cold-agent-bar the pipe's live run over six apps (counter, flight booker, timer, CRUD, cells, the load page) fired exactly once through the same operator Claude Code login (`claude` 2.1.308 on the dev host, the run's `client_version` the same) — no new secret source and no auth library — on the founder's «Запускать сейчас (Рекомендую)» (inputs#I6) within his «Один запуск, до $3 (Рекомендую)» (inputs#I3), its live child now run as `mcp --record` into the owner-only `target/cold-agent/record/` (call content, no credential), and the pipe's env census (`ANTHROPIC_` among its patterns over `scripts/cold-agent.sh`) re-read 0 (as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)."
    sidecar: "secret-management: the 2026-10-25 cold-agent live run recorded as a third use of the operator's own login — claude 2.1.308, no new secret source, env census 0."
    rationale: "The report's F15 records a new live run through the same login (report §Changes F15, Dev-tool versions `claude` 2.1.308, Decisions «Запускать сейчас (Рекомендую)»), and its gates re-read the pipe's `ANTHROPIC_` census at 0; line 14 names each earlier live run with its client version and the founder's word, and the sweep marks line 14 no-change (the login and masking unchanged), so the new run is new detail, not a correction. D-security-auth holds: same login, no API key, no off-spec secret source."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:175
    fact: F15
note: line 15's «the committed live evidence of both runs holds no key …» is left unextended — the report states `hygiene: clean` and a pipe census of 0 for the 2026-10-25 evidence but no credential scan of `evidence/live-*`, so a third-run credential claim is unmeasured; D-security-input and D-security-deps fire nothing in this category (no dependency added; no input surface here).
