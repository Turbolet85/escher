# a11y · standing conventions

- S1 · every accessibility check stays a Rust test over the AccessKit tree, run headless · 3-a11y-assertion-harness-contract
  decision/a11y-assertions-as-rust-tests · Accessibility assertions as Rust tests over AccessKit — Accessibility assertions are Rust tests in blitz-tests that read the built AccessKit tree.
  decision/select-list-coverage-headless · Select list coverage headless — The select list's keyboard and focus coverage is headless, asserting SC 2.1.1 and 2.4.3 on it in both layout modes; the windowed list is read and not run. · guard: select_popup, stand_act_popup, stand_keyboard
- S2 · every test the take brings runs in its CI tier and the a11y leg stays a gating job of the slow tier · 9-ci-integration · ci-integration
