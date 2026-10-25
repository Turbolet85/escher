proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 14
    section: "## 6. Visual Design Verification"
    change: "In line 14's dated clause replace \"the `a11y` leg reads 43 tests over 6 targets, unmoved\" with \"the `a11y` leg then read 43 tests over 6 targets, unmoved — since 2026-10-24-a11y-re-read it reads 84 tests over 14 targets (§9), `stand_lint` not among them\", and extend the closing cite to \"— as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/report.md and its evidence/readings.md, the leg's 14 · 84 at escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md)\""
    sidecar: "visual-design-verification.md:14 — the a11y leg's 43 tests over 6 targets kept as dated (2026-10-19-visual-lint), its 84 over 14 since 2026-10-24-a11y-re-read added"
    rationale: "The report's Counts / qualifiers moved puts the a11y leg at six → fourteen files, 43 → 84 tests (`lines 14 passed 84 failed 0 ignored 0`), and its site sweep lists this line as Changed under `a11y-leg` and `a11y-tally`; Expected amendments carries `visual-design-verification.md:14` (the leg 84 tests over 14 targets). The 43 · 6 reading is true as dated, so it is kept and the new reading added; the fourteen named in Symbols / APIs exclude `stand_lint`."
    basis: escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md:16
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 14
    section: "## 6. Visual Design Verification"
    change: "Add a bullet after line 14: \"- Since 2026-10-24-a11y-re-read the stand's contrast record is re-asserted after 0.2.0's engine changes: `stand_contrast`'s 5 tests are green in the `a11y` leg (14 targets, 84 tests, §9), per package and in each of `stand-repeat`'s three rounds, with `controls`, `TEXT_ROWS` and `STATE_ROWS` unedited and nothing re-baselined, so SC 1.4.3 stays met on every measured non-exempt stand row (as measured at escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md and its evidence/re-read.md)\""
    sidecar: "visual-design-verification.md — new contrast-record bullet: stand_contrast re-asserted after 0.2.0 (2026-10-24-a11y-re-read), tables unedited, nothing re-baselined"
    rationale: "The report's Outcome (a11y criterion 3) states stand Tab order, Enter/Space and contrast re-read after 0.2.0: `stand_contrast` 5 green in the leg, in the per-package run of the fourteen and in each of `stand-repeat`'s three rounds, the guard showing `controls`, `TEXT_ROWS`, `STATE_ROWS` unedited and nothing re-baselined (evidence/re-read.md). This is a new SC 1.4.3 measurement for this category's visual-contrast group, which until now dates its record to 2026-10-13 and 2026-10-14."
    basis: escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md:67
note: D-a11y-surface and D-a11y-obs-schema do not fire (the report adds no interactive UI element and changes neither the a11y violation schema nor the obs log schema); line 13's "in the `a11y` leg (§9)" stays true (sweep no-change); the body's §6 lines 154-156 move nothing, but body §3 :53-55 and §9 :235 are other categories' sites.
