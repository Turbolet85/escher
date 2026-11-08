# a11y · standing conventions

- S1 · a stand check that reads the tree or snapshot states the roles and names it expects once, in the shared module's tables, never a second role or name mapping · screen-reader-support
- S2 · a stand check asserts in both layout modes · §3 A11y Assertion Harness Contract
  (a11y-plan/keyboard-navigation fct-h7iayydu) keyboard-harness
  (a11y-plan/visual-design-verification fct-rwtottqu) visual-contrast
  (a11y-plan/visual-design-verification fct-4ahxhbqz) visual-contrast
- S3 · a new stand test file is named in the leg lists that run it (the a11y leg's file list, the stand-repeat leg's named --test list) where it belongs to them · ci-integration
  (a11y-plan/ci-integration fct-3xv5s7f6) ci
- S4 · a new log line or span from escher's own targets passes the obs allowlist, text, value, html and attrs redacted, ids and closed words only · §3 A11y Assertion Harness Contract
