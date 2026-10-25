proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-strategy.md
    near: 9
    section: "## 2. A11y Strategy"
    change: "End the bullet's owner clause as «— and is owned by its `.andromeda/residuals.md` line, `re-carried:0.2.0` (the stand's platform accessibility adapter and its windowed witness), which escher-0.2.0's requirements owe at an active assistive technology this version does not reach — as measured at escher-0.2.0/chunks/2026-10-25-capability-statement/report.md», in place of «— and is owned by a `.andromeda/residuals.md` line (target: next)»; nothing else on the line moves."
    sidecar: "a11y-strategy §2 changed-set refresh: the windowed AT witness's owner reads its residuals line `re-carried:0.2.0`, unreached in 0.2.0, not «(target: next)» (2026-10-25-capability-statement, atowner)."
    rationale: "The report's Spec claims disproved by measurement (pattern `atowner`) names this site — `.andromeda/a11y-plan-detail/a11y-strategy.md:9`, its owner clause «and is owned by a `.andromeda/residuals.md` line (target: next)» — as changed: the residuals line reads `re-carried:0.2.0` and requirements owe the witness at an active assistive technology, which this version does not reach; Expected amendments lists a11y-plan §2's three category rows under the same fact."
    basis: .andromeda/residuals.md:14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-strategy.md
    near: 21
    section: "## 2. A11y Strategy"
    change: "Read «and the windowed witness, at an active assistive technology, is owned by its `.andromeda/residuals.md` line, `re-carried:0.2.0`, which this version does not reach, on the founder's F1 «Пока нет, перенести (Рекомендую)» (2026-10-14)» in place of «… is owned by a `.andromeda/residuals.md` line (target: next) on the founder's F1 …»; the headless half, the citations and the as-measured cite stand unchanged."
    sidecar: "a11y-strategy §2 ARIA state reads: the windowed AT witness's owner reads its residuals line `re-carried:0.2.0`, unreached in 0.2.0, not «(target: next)» (2026-10-25-capability-statement, atowner)."
    rationale: "The report's `atowner` row list names `.andromeda/a11y-plan-detail/a11y-strategy.md:21` as a changed site — the same disproved owner disposition as :9 (residuals line `re-carried:0.2.0`; escher-0.2.0/requirements.md:42-43 owes the witness «at an active assistive technology, which this version does not reach»)."
    basis: escher-0.2.0/requirements.md:42-43
    dependent-of: D-new-detail
note: "Neither scoped detector fires (the chunk adds a document and a CI-script test, no interactive UI element; no a11y violation or obs log schema changed); the two proposals carry the report's disproved `atowner` fact under D-new-detail for want of a detector that owns it, and line 20's «(target: next)» (the native checked-state fix, a different residuals line) is outside the report's pattern and left as it stands."
