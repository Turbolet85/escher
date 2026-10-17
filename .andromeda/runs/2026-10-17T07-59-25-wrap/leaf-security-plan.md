group: security-plan
read: .claude/docs/security-summary.md · 56 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 32 · «- 2026-10-15: A listener or env read that only a test makes »

- leaf: .claude/docs/security-summary.md:42   class: omission
  leaf-text: «its drop owned by a `.andromeda/residuals.md` line (target: next); memmap2»
  source: .andromeda/security-plan.md:237 «RUSTSEC-2026-0367, RUSTSEC-2026-0368 and RUSTSEC-2026-0369 (zip 2.4.2, each fixed in zip 9.0), since 2026-10-16-fast-failing-mutants, reached only through `anyrender_serialize 0.8.0` ← `browser`»
  replace-with: «its drop owned by a `.andromeda/residuals.md` line (target: next); since 2026-10-16-fast-failing-mutants zip 2.4.2's RUSTSEC-2026-0367, RUSTSEC-2026-0368 and RUSTSEC-2026-0369 (each fixed in zip 9.0, no fixed release reachable) reach it only through `anyrender_serialize 0.8.0` ← `browser`, which never extracts to disk, and are ignored by ID on the founder's «Игнор по ID (Рекомендую)» (2026-10-17), their drop owned by a `.andromeda/residuals.md` line (target: next); memmap2»

- leaf: .claude/rules/security.md:26   class: omission
  leaf-text: «its drop owned by a `.andromeda/residuals.md` line; memmap2»
  source: .andromeda/security-plan.md:237 «RUSTSEC-2026-0367, RUSTSEC-2026-0368 and RUSTSEC-2026-0369 (zip 2.4.2, each fixed in zip 9.0), since 2026-10-16-fast-failing-mutants, reached only through `anyrender_serialize 0.8.0` ← `browser`»
  replace-with: «its drop owned by a `.andromeda/residuals.md` line; zip 2.4.2's RUSTSEC-2026-0367 · RUSTSEC-2026-0368 · RUSTSEC-2026-0369 (fixed only in zip 9.0, reached only through `anyrender_serialize 0.8.0` ← `browser`, no fixed release reachable) are ignored by ID on the founder's «Игнор по ID (Рекомендую)» (2026-10-17), their drop owned by a `.andromeda/residuals.md` line; memmap2»
