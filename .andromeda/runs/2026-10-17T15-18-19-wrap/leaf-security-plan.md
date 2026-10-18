group: security-plan
read: .claude/docs/security-summary.md · 56 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 32 · «- 2026-10-15: A listener or env read that only a test makes »

- leaf: .claude/docs/security-summary.md:18   class: retired
  leaf-text: «numeric args with defaults; `bump` validates target + semver; `scripts/agent-run.sh`»
  source: .andromeda/security-plan.md:244 «`toml_datetime` 0.6.11, `toml_edit` 0.22.27 and `toml_write` 0.1.2 leave with the bump crate»
  replace-with: «numeric args with defaults; `scripts/agent-run.sh`»

- leaf: .claude/docs/security-summary.md:42   class: omission
  leaf-text: «owned by a `.andromeda/residuals.md` line (target: next); memmap2 (RUSTSEC-2026-0186, unsound)»
  source: .andromeda/security-plan.md:238 «RUSTSEC-2026-0370 (zvariant 5.15.0), RUSTSEC-2026-0371 (zbus_names 4.3.4) and RUSTSEC-2026-0372 (zvariant_derive 5.15.0), since 2026-10-17-upstream-sync-engine-debts, each unmaintained»
  replace-with: «owned by a `.andromeda/residuals.md` line (target: next); since 2026-10-17-upstream-sync-engine-debts RUSTSEC-2026-0370 (zvariant 5.15.0), RUSTSEC-2026-0371 (zbus_names 4.3.4) and RUSTSEC-2026-0372 (zvariant_derive 5.15.0), each unmaintained and folded into zbus 6.0.0, reach it only through zbus 5.19 + atspi 0.29 under accesskit_unix 0.24.0 (the latest) ← accesskit_xplat ← blitz-shell, no fix reachable, and are ignored by ID on the operator's word under the playbook's External decay pattern (2026-10-17), dropped when accesskit_unix moves to zbus 6, owned by a `.andromeda/residuals.md` line (target: next); memmap2 (RUSTSEC-2026-0186, unsound)»

- leaf: .claude/docs/security-summary.md:13   class: omission
  leaf-text: «no carrier, verb, key name, field or span added — an engine change for every document and a boundary widening, ratified by the founder (2026-10-13, 2026-10-14).»
  source: .andromeda/security-plan.md:125 «`aria-description`'s own text is written into AccessKit's description field — free page text, unbounded, not a closed set»
  replace-with: «no carrier, verb, key name, field or span added — an engine change for every document and a boundary widening, ratified by the founder (2026-10-13, 2026-10-14); since the 2026-10-17 upstream sync the accessibility builder reads, as upstream wrote them, `aria-labelledby` (its targets' text the computed `label`, outranking a `<label>` and an `aria-label`, inside the ratified name crossing), `aria-expanded` · `aria-disabled` · `aria-level` (closed values inside the states class; `aria-disabled` moves no snapshot reading) and `aria-describedby` · `aria-description` — descriptions, `aria-description` free page text, unbounded, a fourth content class on the platform adapter crossing only (the snapshot reads no description, so no driver or MCP carrier holds it), a boundary widening by upstream merge ratified as upstream's behaviour by the founder (2026-10-17), with no log site, print or panic path.»

- leaf: .claude/rules/security.md:17   class: omission
  leaf-text: «and the snapshot. Keep a new attribute read to the same shape: a closed set, no text carried out.»
  source: .andromeda/security-plan.md:123 «since the 2026-10-17 upstream sync the names it carries include `aria-labelledby`'s computed text (the `aria-label` · `<label for>` row), and it carries a fourth content class, descriptions»
  replace-with: «and the snapshot. Since the 2026-10-17 upstream sync the builder also reads, as upstream wrote them, `aria-labelledby` (its targets' text the computed `label`, outranking a `<label>` and an `aria-label`, inside the ratified name crossing), `aria-expanded` · `aria-disabled` · `aria-level` (closed values inside the states class; `aria-disabled` moves no snapshot reading) and `aria-describedby` · `aria-description` — descriptions, `aria-description` free page text, unbounded, a fourth content class on the adapter crossing only (the snapshot reads no description, so no driver or MCP carrier, field, log field or span holds it), a boundary widening by upstream merge ratified as upstream's behaviour by the founder (2026-10-17). Keep any other new attribute read to the closed shape: a closed set, no text carried out.»

- leaf: .claude/rules/security.md:26   class: omission
  leaf-text: «their drop owned by a `.andromeda/residuals.md` line; memmap2 (unsound)»
  source: .andromeda/security-plan.md:238 «RUSTSEC-2026-0370 (zvariant 5.15.0), RUSTSEC-2026-0371 (zbus_names 4.3.4) and RUSTSEC-2026-0372 (zvariant_derive 5.15.0), since 2026-10-17-upstream-sync-engine-debts, each unmaintained»
  replace-with: «their drop owned by a `.andromeda/residuals.md` line; zvariant 5.15.0's RUSTSEC-2026-0370, zbus_names 4.3.4's RUSTSEC-2026-0371 and zvariant_derive 5.15.0's RUSTSEC-2026-0372 (unmaintained, folded into zbus 6.0.0, reached only through zbus 5.19 + atspi 0.29 under accesskit_unix 0.24.0 ← accesskit_xplat ← blitz-shell, no fix reachable) are ignored by ID on the operator's word under the playbook's External decay pattern (2026-10-17), dropped when accesskit_unix moves to zbus 6, owned by a `.andromeda/residuals.md` line; memmap2 (unsound)»
