proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/dependency-security.md
    near: 11
    section: "## Dependency Security"
    change: "After the RUSTSEC-2026-0386 record, add one line in the group's shape: RUSTSEC-2026-0414 (pulldown-cmark 0.13.4), since 2026-10-31-engine-debts, is fixed by the lockfile update to pulldown-cmark 0.13.5, in `Cargo.lock` alone, `deny.toml` unmoved; it was reached through `rdme`, which never sets `ENABLE_WIKILINKS`, so no reader was exposed (apps/readme/src/markdown/pulldown_cmark.rs:7-12); semver-compatible, no coupled pin, the operator's call and not put to the founder; read by the audit leg on 2026-10-31, `advisories ok` and `licenses ok` — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md."
    sidecar: "Audit tool gains RUSTSEC-2026-0414: pulldown-cmark 0.13.5, a lock-only fix with no reader exposed and `deny.toml` unmoved (2026-10-31-engine-debts)."
    rationale: "Report **Dependencies** and **Cross-project / external claims**: the bump is RUSTSEC-2026-0414's fix and no standing `pulldown` hit names the advisory (**Expected amendments**); the entity already lists the lock-only fixes for RUSTSEC-2026-0285 and RUSTSEC-2026-0386. No dependency is added, so D-security-deps holds."
    basis: Cargo.lock:6505
    fact: F13
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 9
    section: "## Error Handling"
    change: "Replace the clause «and `off-screen`'s that a scrolling box under a transform is read where it stands untransformed, so an element inside it can read `off-screen` though it shows» with «and `off-screen`'s, since 2026-10-31-engine-debts, that under a rotation a box's visible part reads as the bounding box of its rotated padding box, so an element in a corner it does not show can read in view»; its remedy, the cause set (eight) and the cause names stay unchanged, as the sentence already says."
    sidecar: "`off-screen`'s transformed-box clause retired: `visible_region` reads through transforms, and the meaning now states the rotation limit (2026-10-31-engine-debts)."
    rationale: "Report **Retired** names `off-screen`'s transformed-box clause; **Symbols / APIs (driver and stand)**, Fixed `'static` text moved, gives the new meaning with the remedy, eight causes and names unmoved; **Counts / qualifiers moved**, `visible-region`, lists `error-handling.md:9` as changed."
    basis: packages/escher-driver/src/refusal.rs:83-85
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 9
    section: "## Error Handling"
    change: "After «`type`'s help still says that it replaces and that on a select the text chooses the first option whose label starts with it», add: «, and that fixed literal moved at 2026-10-31-engine-debts to cover `type` on a `multiple` select, where the text chooses its match alone and a text opening on a space types nothing, still holding nothing a call supplies»."
    sidecar: "`type`'s help moved for a `multiple` select, still a fixed literal, with no verb, argument, field, cause or exit added (2026-10-31-engine-debts)."
    rationale: "Report **Symbols / APIs (driver and stand)**: F6 and Fixed `'static` text moved name `type`'s help as moved; **Counts / qualifiers moved**, `type-on-select`, lists `error-handling.md:9` as changed; **Deviations** 6c states the space-opening reading. The report does not quote the help's new wording, so the clause states the behaviour and that the literal moved, not its words."
    basis: packages/escher-driver/src/schema.rs:239-240
note: F12 (no picked entity holds mounted handles) and F14, F15 (no picked entity holds the CI tooling row) are left to the new-detail pass; the `pulldown-cmark` version row the report's Expected amendments names in dependency-security.md is not among this group's picked entities; error-handling.md line 10's provenance clause still names `off-screen`'s retired transformed-box sentence, left to the citation sweep; D-security-input, D-security-auth and D-security-deps fire no drift in these categories.
