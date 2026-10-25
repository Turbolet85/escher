proposals:
  - detector: D-new-detail
    severity: warning
    fact: F4
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 40
    section: "## 2. Test Strategy"
    change: "- **Claim-to-check statement (since 2026-10-25-capability-statement):** `docs/capabilities.md`, a document with no runtime surface, ends on `## Where each claim is proven`, which maps each claim it makes to the check that proves it (the stand checks inside `stand-repeat`: the 48 `stand_*` files, ten named blitz-tests targets and seven_guis' eight process checks); each limit it states as pinned names its check (the boolean model's two limits pinned at unit and engine level only, the cross-OS shot limit citing CI#39225375577), and a limit no check asserts is stated as asserted by no check (the `select` model's); its names and counts are held to the schema's tables by the CI-script pin `test_capability_statement.py` (§4) (docs/capabilities.md:172-174; docs/capabilities.md:180-181; docs/capabilities.md:188-192; docs/capabilities.md:300-301)"
    sidecar: "test-strategy gains the capability statement's claim-to-check section (2026-10-25-capability-statement)"
    rationale: "The report's Changes name the new `docs/capabilities.md` with its fifth section `## Where each claim is proven`, and its Outcome (tests) reads every pinned limit naming its check, the select's limits asserted by no check, the boolean model's two limits pinned at unit and engine level only and the cross-OS shot limit citing CI#39225375577 — met; New text lists docs/capabilities.md:172-174, 180-181, 188-192 and 300-301."
    basis: docs/capabilities.md:300-301
note: the new bullet takes its own `<!-- detail: procedure · strategy -->` … `<!-- /detail -->` block after line 42, as each neighbouring strategy entry does.
