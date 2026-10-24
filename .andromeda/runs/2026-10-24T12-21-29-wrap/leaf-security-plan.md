group: security-plan
read: .claude/docs/security-summary.md · 56 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 32 · «- 2026-10-15: A listener or env read that only a test makes »

- leaf: .claude/rules/security.md:9   class: stale
  leaf-text: «env reads are `WPT_DIR`, `PAINT_TREE_BENCH_HTML`, `HOME`, `CARGO_MANIFEST_DIR` and `RUST_LOG` (escher-telemetry's filter) only»
  source: .andromeda/security-plan.md:298 «`MULTI_DOC_N`, `MULTI_DOC_SEQ` and `MULTI_DOC_FLAT`, read by the root example `multi_doc` alone through two `std::env::var` call sites»
  replace-with: «env reads are `WPT_DIR`, `PAINT_TREE_BENCH_HTML`, `HOME`, `CARGO_MANIFEST_DIR`, `RUST_LOG` (escher-telemetry's filter) and `MULTI_DOC_N` · `MULTI_DOC_SEQ` · `MULTI_DOC_FLAT` (the root example `multi_doc` alone) only»
