proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/dependency-security.md
    near: 22
    section: ## Dependency Security
    change: "Add after line 22 a new row: - The upstream sync read at `ebacd68bd` took eleven commits, `d63ed2c8c` · `6587e7e79` · `e6f4615ec` · `a7ea9631b` · `b5b163450` · `7fc9ee3af` · `2963b65a2` · `cee6b8731` · `53e9d81c6` · `6fee0db25` · `efdc5472a`, by no-commit cherry-picks of upstream's text on the founder's word, sixteen left out with their measured reasons, the merge base staying `23b9b26a9`: `Cargo.toml`, `Cargo.lock`, `deny.toml` and every crate manifest byte-identical to the tree before it, so no package enters, leaves or moves; the pins stay — Stylo \"0.22.0\", Taffy the registry \"0.15\", Dioxus \"0.7.3\", winit `=0.31.0-beta.3`, no git dependency; the audit leg reads `advisories ok` and `licenses ok`, every standing ignore encountered and none added — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/lock.md"
    sidecar: "dependency-security: the per-sync lock record gains the 2026-10-27 sync's row — eleven upstream commits cherry-picked, every manifest, the lock and deny.toml byte-identical, audit green with no ignore added"
    rationale: "Report Changes, Dependencies (line 22: no package enters, leaves or moves, the manifests, lock and deny.toml byte-identical, the pins listed), Symbols / APIs (line 12: the eleven shas), Counts `upstream` (line 34: `dependency-security.md:22` extended, the per-sync lock record) and `lockrec` (line 44), Outcome (lines 117-118: audit advisories ok, licenses ok, no ignore added); the task's F6"
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:22
    fact: F6
note: Not mine and left for their passes — F2 (FetchError's new variant; no picked error-handling entity carries it, and the body's FetchError line security-plan.md:336 lists its messages), F5 (the second test-only TMPDIR read; the body's security-plan.md:302 the report's `tmpdir` row marks changed) and F1; the report's `lockrec` row also extends dependency-security.md:9, :20 and :21, which this group's picks do not hold; the body's Pinning line security-plan.md:234 still holds for this cherry-pick that entered no package.
