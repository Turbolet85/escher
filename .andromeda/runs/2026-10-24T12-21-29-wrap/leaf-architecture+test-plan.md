group: architecture+test-plan
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»

- leaf: .claude/docs/commands.md:53   class: stale
  leaf-text: «until upstream retires it, the floors unchanged»
  source: .andromeda/test-plan.md:408 «both `cargo llvm-cov` lines exclude dioxus-native-dom's `keyboard_compat.rs`, upstream's temporary keyboard-types bridge, until escher takes upstream's retirement of it»
  replace-with: «until escher takes upstream's retirement of it, the floors unchanged»

- leaf: .claude/docs/commands.md:65   class: omission
  leaf-text: «inner_html, mutations, outline»
  source: .andromeda/architecture.md:216 «`MULTI_DOC_N`, `MULTI_DOC_SEQ` and `MULTI_DOC_FLAT`, read by the `multi_doc` example alone»
  replace-with: «inner_html, mutations, multi_doc (reading `MULTI_DOC_N`, `MULTI_DOC_SEQ` and `MULTI_DOC_FLAT`), outline»
