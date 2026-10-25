proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 217
    section: "## Input Validation"
    change: "In the `aria-selected` · `aria-pressed` · `aria-checked` · `aria-invalid` row, replace «the witness owned by a `.andromeda/residuals.md` line (target: next) on the founder's F1 «Пока нет, перенести (Рекомендую)» (2026-10-14)» with «the witness owned by a `.andromeda/residuals.md` line read `re-carried:0.2.0` — owed at an active assistive technology, which this version does not reach — on the founder's F1 «Пока нет, перенести (Рекомендую)» (2026-10-14)»; nothing else on the line changes."
    sidecar: "security-plan Input Validation (control states row): the windowed AT witness's residuals owner «(target: next)» → `re-carried:0.2.0`, not reached in this version (atowner, 2026-10-25-capability-statement)"
    rationale: "The report's Spec claims disproved (pattern `atowner`) measures the AT witness's residuals line as `re-carried:0.2.0`, owed at an active assistive technology this version does not reach, and lists .andromeda/security-plan-detail/input-validation.md:217 @1129 among the three sites outside the pattern's reach, each changed (escher-0.2.0/chunks/2026-10-25-capability-statement/report.md)."
    basis: .andromeda/security-plan-detail/input-validation.md:217
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 194
    section: "## Input Validation"
    change: "In the `id` (stable element id) row, replace «that witness owned by a `.andromeda/residuals.md` line (target: next)» with «that witness owned by a `.andromeda/residuals.md` line read `re-carried:0.2.0` — owed at an active assistive technology, which this version does not reach»; the founder's F1 words and the rest of the line unchanged."
    sidecar: "security-plan Input Validation (stable element id row): the adapter's AT witness owner «(target: next)» → `re-carried:0.2.0`, not reached in this version (atowner sweep, 2026-10-25-capability-statement)"
    rationale: "Sweep of the file for the claim the `atowner` change retires: this clause names what the platform adapter hands an assistive technology and owns that witness by the same residuals line «(target: next)», which the report measures as `re-carried:0.2.0` (escher-0.2.0/chunks/2026-10-25-capability-statement/report.md, Spec claims disproved); the report's rows for this file (:193, :195 ×2, :210 no-change, :217 changed) do not list it."
    basis: .andromeda/security-plan-detail/input-validation.md:194
    dependent-of: D-new-detail
note: D-security-input, D-security-auth and D-security-deps hold (the report adds no external-input surface, auth, secret or dependency; `test_capability_statement.py` reads tracked files only); :194 was found by sweep and is outside the report's listed rows; :197's route-entry owner phrase is the report's carried-for-Validate no-change fact; :216 and :221 «(target: next)» own other residuals the report does not move.
