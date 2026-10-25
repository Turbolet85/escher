group: architecture+security-plan+test-plan-1
read: .claude/docs/services/escher-driver.md · 63 · «- The command line, through the real `escher-session` binary»

- leaf: .claude/docs/services/escher-driver.md:35   class: omission
  leaf-text: «a later entry ADDS its verbs to `VERBS` at its own promotion, never states a second table»
  source: .andromeda/architecture-detail/standard-contracts.md:262 «the capability-statement pin — `docs/capabilities.md` and README.md held to the driver schema's tables: `CapabilityStatementTest`, four cases in stdlib `unittest`, reads the verb names from `VERBS` (`schema.rs`)»
  replace-with: «a later entry ADDS its verbs to `VERBS` at its own promotion, never states a second table — and the CI-script pin `test_capability_statement.py` (the `ci-scripts` leg) reads the verb names from `VERBS`, the cause names from `Cause::name`'s match arms (`refusal.rs`) and the lengths of `SESSION_ERRORS` (`error.rs`), `NODE_FIELDS` and `EXIT_STATUSES`, holding `docs/capabilities.md` (every verb and cause a code span, the five counts stated in words) and README.md («the {word} driver commands as tools») to those tables»
