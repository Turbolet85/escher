group: architecture+security-plan+test-plan+obs-plan+a11y-plan
read: CLAUDE.md · 151 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:110   class: omission
  leaf-text: «so the screens' ids, names, values, snapshot text and diffs at rest — read back strictly»
  source: .andromeda/architecture-detail/established-decisions.md:172 «so a kept check holds the screens' ids, names, values, snapshot text, diffs and a lint's findings at rest, never logged and never named by a tool call»
  replace-with: «so the screens' ids, names, values, snapshot text, diffs and a lint's findings at rest — read back strictly»

- leaf: CLAUDE.md:88   class: omission
  leaf-text: «the coverage leg's per-crate floor `.github/scripts/coverage_floor.py` (its tests `.github/scripts/test_coverage_floor.py`) |»
  source: .andromeda/test-plan.md:219 «test_capability_statement.py pins `docs/capabilities.md` to the driver schema's tables — every verb and refusal cause named as a code span»
  replace-with: «the coverage leg's per-crate floor `.github/scripts/coverage_floor.py` (its tests `.github/scripts/test_coverage_floor.py`) · the capability-statement pin `.github/scripts/test_capability_statement.py` (`docs/capabilities.md`'s verb and cause names and its five counts, and README.md's count of the driver commands as tools, held to the driver schema's tables) |»

- leaf: CLAUDE.md:80   class: omission
  leaf-text: «`.andromeda/architecture.md` §Standard Contracts (Driver session) · `.andromeda/security-plan.md` §Input Validation |»
  source: .andromeda/architecture-detail/project-intent.md:27 «the CI-script pin `.github/scripts/test_capability_statement.py` holds its verb and cause names and its five counts — verbs, refusal causes, session errors, node keys, exit statuses —»
  replace-with: «`.andromeda/architecture.md` §Standard Contracts (Driver session) · `.andromeda/security-plan.md` §Input Validation · the capability statement `docs/capabilities.md` (its verb and cause names and its five counts held to these tables by `.github/scripts/test_capability_statement.py`, its other claims kept by hand) |»
