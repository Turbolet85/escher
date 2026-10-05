# a11y-plan — archived amendment originals

Writer = wrap P7 and registry.py migrate --apply · read by NO loop skill · cold history, never cited for current truth.

## Registry migration (U35) — 2026-10-05

<!-- U35 · a11y-plan.md · ## 3. A11y Assertion Harness Contract · sha256 edb45a3869329f733f4bf457bc3203f0359be3629755b3166033a33515d7e8b8 -->

## 3. A11y Assertion Harness Contract

**A11y testing tool pick:** accessibility assertions are Rust tests in blitz-tests over the AccessKit tree.
- Tests call `document.build_accessibility_tree()` after `resolve` and match `tree_update.nodes` with `test_that` matchers on `role()` and `is_hidden()` (tests/blitz-tests/tests/accessibility_hidden.rs:9-26)
- An `assert_role(html, element_id, expected)` helper maps the element's `NodeId` to an AccessKit `NodeId(node_id.as_u64())` and compares roles (tests/blitz-tests/tests/accessibility_roles.rs:39-72)
- An `unknown_tags(html)` helper lists the `html_tag` of nodes with `Role::Unknown` (tests/blitz-tests/tests/accessibility_roles.rs:16-37)
- The test harness builds blitz-dom with the accessibility feature (packages/blitz-test-harness/Cargo.toml:15)
- accesskit_xplat ships a doc-comment usage example, not tests (packages/accesskit_xplat/src/lib.rs:13-86)

**Focus management test harness:**
- The harness exposes focused and hovered node queries (packages/blitz-test-harness/src/inspect.rs:107-115)

**Keyboard test harness:** keyboard-event and Tab-order navigation tests are observed absent — see §5 (Test harness pattern).

**Contrast verification harness:** contrast checks are observed absent — see §6.

**CI integration:** accessibility checks in CI are observed absent — see §9.

**Observed absent (assertion searches):**
- accessibility tree or assertion tooling · searched: `accesskit|AccessKit` over the 32 s03 slice files
- accessibility assertions · searched: `#\[(tokio::)?test\]` over packages/accesskit_xplat/** and `aria-|role` over the 86 s04 slice files (role hits only in github-markdown.css)
- tests of the accessibility tree · searched: `#\[test\]` over packages/blitz-dom/src/accessibility.rs
- accessibility assertions in tests · searched: `accesskit|aria|role` over the 16 listed s08 files (only commented accesskit lines matched)
- accessibility assertions in tests · searched: `aria-|aria[A-Z]|\brole\b` over the 32 s10 slice files
- accessibility assertions in tests · searched: `aria|role|accesskit` over the 21 listed s11 files (matches only in crate docs and unrelated comments)

> NOT YET MEASURED — the structured violation JSON schema / log format, the WCAG criteria mapping of the existing tests and a screen-reader test pattern: no slice recorded them (s01, s02, s06, s07 and s13 recorded the a11y assertion harness as out of slice), and the test plan (§3) and obs plan (§3, §6 Log Coverage) leave the log format unmeasured.

### Bootstrap phases (derive for route / setup-project)

- **contrast-verification-harness-setup** — contrast checks are observed absent; recorded in §6 Visual Design Verification.
- **a11y-ci-gate-wire** — accessibility checks in CI are observed absent; recorded in §9 CI Integration.

---
