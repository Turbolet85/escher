group: test-plan-2
read: .claude/rules/testing.md · 60 · «- 2026-10-19: Code compiled only off unix is unverified on t»
read: .claude/rules/verification-harness.md · 41 · «- 2026-10-06: A boot smoke bounded by `timeout N` exits 124 »

- leaf: .claude/rules/testing.md:17   class: omission
  leaf-text: «one `TestCase` per function.»
  source: .andromeda/test-plan-detail/test-data-fixtures.md:29 «no fixture file: `test_capability_statement.py` reads the files it pins by path from the repository root — `docs/capabilities.md`, `README.md` and escher-driver's `schema.rs`»
  replace-with: «one `TestCase` per function; a CI-script pin over repository text reads the files it pins by path from the repository root and writes nothing — `test_capability_statement.py` (`CapabilityStatementTest`, four tests) holds `docs/capabilities.md`'s verb and cause names and its five counts, and README.md's count of the MCP tools, to the driver schema's tables, with no fixture file beyond its inline `NUMBER_WORDS` table (one … twenty).»

- leaf: .claude/rules/testing.md:21   class: omission
  leaf-text: «regression tests cite the upstream issue/PR URL and the real page that exposed it.»
  source: .andromeda/test-plan-detail/test-strategy.md:28 «only its names and counts are held to the schema's tables, by the CI-script pin `test_capability_statement.py` (§4), so a check renamed or moved changes that section by hand»
  replace-with: «regression tests cite the upstream issue/PR URL and the real page that exposed it. `docs/capabilities.md` ends on `## Where each claim is proven`, naming the check behind each claim it makes — the stand checks among them — and only its names and counts are pinned, so a check renamed or moved changes that section by hand.»
