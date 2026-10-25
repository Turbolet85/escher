proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 73
    section: "## Stack and Technologies"
    change: "In the CI/CD Technology item's cold-agent client clause, after «and at 2.1.296 (as measured at escher-0.1.0/chunks/2026-10-15-cold-agent-test/evidence/operator-9-claude-version.txt)» add «and at 2.1.308 on 2026-10-25, the 0.2.0 live run's `client_version` the same (as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/live-run.md)» — still a host tool on the operator host only, a `run` precondition, no lockfile resolving it; the line stays under 2 000 chars"
    sidecar: "stack-and-technologies: the cold-agent client `claude` read at 2.1.308 on the dev host (2026-10-25-cold-agent-bar), the live run's client_version the same"
    rationale: "Report §Changes → Dev-tool versions: `claude` read 2.1.308 on the dev host at 2026-10-25 (operator entry 22, `claude --version`), the live run's `client_version` the same, no lockfile resolving it; §Outcome Gates: `claude --version` green (2.1.308) — `evidence/live-run.md`; F15 names Claude Code 2.1.308 for the founder's one live run. The category records each dated reading of this host tool (2.1.288, 2.1.296); this chunk's reading is new detail. Dependencies: none, so D-arch-decisions and D-arch-resources hold for this category (`LoadNet` uses blitz-traits, already a seven_guis dependency; no crate, library, runtime or package added)."
    basis: .andromeda/architecture-detail/stack-and-technologies.md:73
note: "stack-and-technologies.md:73's cite `scripts/cold-agent.sh:557-567` (the judge's stdlib imports) is a `citation` row of F14 (cold-agent.sh grew 127 lines in 25 ranges); the report states no new line for it, so it is left to the P2 citation sweep — no proposal here."
