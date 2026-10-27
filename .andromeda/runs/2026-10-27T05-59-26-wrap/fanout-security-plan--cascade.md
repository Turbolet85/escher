proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/secret-management.md
    near: 33
    section: "## Secret Management"
    change: "- searched: `/home/|api[_-]?key|authorization:|bearer ` and two e-mail domains (case-insensitive) over `docs/capabilities.md` and `.github/scripts/test_capability_statement.py` (0 hits); the pin reads no environment variable; re-searched at 2026-10-27-readme-and-records, after the statement's run paragraph named the 2026-10-25 cold-agent live run's reach through the operator's own login: `/home/|api[_-]?key|authorization:|bearer ` (case-insensitive) over `README.md`, `docs/capabilities.md` and the pin (0 hits, the chunk's security probe, entry 7)"
    sidecar: "secret-management#2.13 — the absence record's search extended to this chunk's re-search over README.md, docs/capabilities.md and the pin (0 hits of the four terms, entry 7), after the statement named the operator's own login for the 2026-10-25 run (2026-10-27-readme-and-records)"
    rationale: "report Changes › Files: docs/capabilities.md:290-295 restated to the 2026-10-25 run reaching the model provider through the operator's own login, and README.md gained 31 lines; report Outcome (security): entry 7 green, 0 hits of `/home/|api[_-]?key|authorization:|bearer ` over the two documents and their pin (gate `grep -c -i -E`, last line 0) — the absence still holds on the changed files and now covers README.md; the e-mail-domain half was not re-run per the report, so it stays as first recorded"
    basis: .andromeda/security-plan-detail/secret-management.md:33
note: no New facts were listed; the body's Development storage line (security-plan.md:290) still holds — the 2026-10-25 run used the same login, no new secret source — so no body edit is proposed.
