# a11y · standing conventions

- S1 · A new accessibility or stand test file is listed in the a11y leg's file list, and a stand test that can flake is also named in the stand-repeat leg's three-round list. · ci-integration
  (a11y-plan/visual-design-verification fct-67ycdssr) visual-contrast
- S2 · A stand check asserts in both layout modes, from stated tables with their row counts asserted, and reads roles and names only from the shared module's tables. · a11y-strategy
  (a11y-plan/visual-design-verification fct-rwtottqu) visual-contrast
  (a11y-plan/visual-design-verification fct-4ahxhbqz) visual-contrast
- S3 · A check of tree-derived readings reads them off the built tree after resolve, the snapshot model being the only consumer, never the element. · screen-reader-support
