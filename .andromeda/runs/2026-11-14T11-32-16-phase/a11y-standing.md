# a11y · standing conventions

- S1 · each new stand check states its expected roles, names and screen once in the shared stand expectation tables and asserts against them · screen-reader-support · 7-screen-reader-support
- S2 · each new stand check is repeated three rounds and runs in the slow-tier accessibility leg that gates CI · ci-integration · 9-ci-integration
  decision/stand-contrast-asserted · SC 1.4.3 asserted on the stand — SC 1.4.3 is asserted on every measured non-exempt stand row, in both layout modes, on the four lean tasks, the load page and Cells, and the check runs in the workspace test leg, per package and in the a11y leg. · guard: stand_contrast
- S3 · each new driver argument or answer reads through the one snapshot model and its text form, never a second reader · screen-reader-support · 7-screen-reader-support
