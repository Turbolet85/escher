group: security-plan
read: .claude/docs/security-summary.md · 56 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 31 · «_This section is owned by `/andromeda-wrap-session`. setup-p»

- leaf: .claude/docs/security-summary.md:16   class: stale
  leaf-text: «still owed: no test of ours covers it, and a check of ours is due before 0.1.0 ships (route entry "Quality gates")»
  source: .andromeda/security-plan.md:102 «since 2026-10-14-quality-gates blitz-tests' `font_face_sniffed_source` does — offline, every request held by a hand provider, a font context holding no family, both layout modes»
  replace-with: «since 2026-10-14-quality-gates witnessed by blitz-tests' `font_face_sniffed_source` (offline, both layout modes: a hint-less `data:` source answered with a `wOF2` body registers, a body no sniff matches registers nothing)»

- leaf: .claude/docs/security-summary.md:42   class: stale
  leaf-text: «The dependency audit's reach — paste / memmap2 advisories in optional chains unseen by cargo-deny's resolved graph → "Quality gates".»
  source: .andromeda/security-plan.md:237 «memmap2 is kept out by the `unsound` default, not by pruning: it is reported, `error[unsound]`, under `unsound = "all"` in a copied config, which 0.1.0 does not take»
  replace-with: «The dependency audit's reach — since 2026-10-14-quality-gates the leg checks the whole workspace's graph (`--workspace --all-features`): paste (RUSTSEC-2024-0436, unmaintained) reaches it and is ignored by ID, its drop owned by a `.andromeda/residuals.md` line (target: next); memmap2 (RUSTSEC-2026-0186, unsound) is kept out by the `unsound` default, not by pruning, a measured limit owned by a `.andromeda/residuals.md` line (target: next).»

- leaf: .claude/docs/security-summary.md:43   class: stale
  leaf-text: «Licence compliance — upstream's `[licenses]` table sits in `deny.toml`, run by no leg; one reading, `licenses ok`; whether the fork gates on it is open, for the founder → "Quality gates".»
  source: .andromeda/security-plan.md:268 «the audit leg runs it on every push since 2026-10-14-quality-gates — `licenses` over the whole workspace's graph, reading `licenses ok` on the dev host and in CI#38692629940»
  replace-with: «Licence compliance — upstream's `[licenses]` table in `deny.toml` is run by the audit leg on every push since 2026-10-14-quality-gates, over the whole workspace's graph (`licenses ok` on the dev host and in CI#38692629940): the fork gates on licences, the open question decided by the founder (2026-10-14).»

- leaf: .claude/rules/security.md:9   class: omission
  leaf-text: «`CARGO_MANIFEST_DIR` and `RUST_LOG` (escher-telemetry's filter) only.»
  source: .andromeda/security-plan.md:315 «`TMPDIR`, through `std::env::temp_dir()` — on unix that variable, else `/tmp` — in escher-driver's unix-only client unit test `client::imp::tests`»
  replace-with: «`CARGO_MANIFEST_DIR` and `RUST_LOG` (escher-telemetry's filter) only, plus `TMPDIR` through `std::env::temp_dir()` in escher-driver's unix-only client unit test (test code only, not secret).»

- leaf: .claude/rules/security.md:14   class: stale
  leaf-text: «No test of ours covers it yet — a check of ours is owed before 0.1.0 ships (route entry "Quality gates").»
  source: .andromeda/security-plan.md:102 «since 2026-10-14-quality-gates blitz-tests' `font_face_sniffed_source` does — offline, every request held by a hand provider, a font context holding no family, both layout modes»
  replace-with: «Since 2026-10-14-quality-gates blitz-tests' `font_face_sniffed_source` covers it — offline, both layout modes: a hint-less `data:` source answered with a `wOF2` body registers, a body no sniff matches registers nothing.»

- leaf: .claude/rules/security.md:26   class: stale
  leaf-text: «Its reach is cargo-deny's resolved graph, which misses the paste and memmap2 advisories in optional chains — still review new crates by hand.»
  source: .andromeda/security-plan.md:237 «On the whole-workspace graph paste reaches it — `advisories FAILED` with `error[unmaintained]: paste` when its ignore is removed through a copied config, `advisories ok` with it»
  replace-with: «Since 2026-10-14-quality-gates it checks advisories and licences over the whole workspace's graph (`--workspace --all-features`): paste (RUSTSEC-2024-0436) reaches it and is ignored by ID, its drop owned by a `.andromeda/residuals.md` line; memmap2 (unsound) is kept out by the `unsound` default, a measured limit owned by a `.andromeda/residuals.md` line — still review new crates by hand.»

- leaf: .claude/rules/security.md:27   class: stale
  leaf-text: «`deny.toml` also holds upstream's `[licenses]` table, which no leg runs — one reading, `licenses ok`; whether the fork gates on it is an open question on the route entry "Quality gates".»
  source: .andromeda/security-plan.md:268 «the audit leg runs it on every push since 2026-10-14-quality-gates — `licenses` over the whole workspace's graph, reading `licenses ok` on the dev host and in CI#38692629940»
  replace-with: «`deny.toml` also holds upstream's `[licenses]` table, which the audit leg runs on every push since 2026-10-14-quality-gates over the whole workspace's graph (`licenses ok`) — the fork gates on licences, decided by the founder (2026-10-14).»
