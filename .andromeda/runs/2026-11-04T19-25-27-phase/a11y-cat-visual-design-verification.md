@carried n=1 label=a11y-plan/visual-design-verification fact=fct-rwtottqu
why: The probe runs on the stand (examples/seven_guis/) and may keep test or example code there, so the stand's measured SC 1.4.3 record must still read its 17 control rows over the four lean tasks at boot, identical in both layout modes, with no row moved by anything the probe adds.
quote: The stand check `stand_contrast` records one row per `controls(task)` row over the four lean tasks at boot (800 × 600 · scale 1 · Light · DejaVu Sans · offline), identical in both layout modes — 17 rows.
@carried n=2 label=a11y-plan/visual-design-verification fact=fct-67ycdssr
why: Probe code kept on the stand must leave the driver's visual lint reading no finding at boot on the four lean tasks in both layout modes, `at_boot` still `[0, 0, 0, 0]`.
quote: The stand at boot reads no `contrast` finding on the four lean tasks in both layout modes, the nodes outside `stand_contrast`'s rows included — and no finding of any kind at boot, `at_boot` reading `[0, 0, 0, 0]`
@carried n=3 label=a11y-plan fact=fct-o6t6iprq
why: The body's broad line the stand rows serve: SC 1.4.3 must stay met on every measured non-exempt stand row after the probe, with the exempt rule unchanged.
quote: `stand_contrast` asserts three tables in both layout modes, a row exempt exactly where the snapshot's `enabled` reads `Some(false)`, and the load page's and Cells' ratios in tests of their own (`the_load_page_meets_its_ratios`, `cells_meets_its_ratios`), so SC 1.4.3 is met on every measured non-exempt stand row.
@binding n=1
**stand contrast record ↔ the chunk scope's surfaces (examples/seven_guis/, tests/blitz-tests/) and its boundary «the probe's code is test or example code that measures»**: whatever probe code the plan keeps on the stand or in blitz-tests must leave `stand_contrast`'s 17 rows, its tables and its exempt rule as measured (a11y-plan/visual-design-verification fct-rwtottqu), (a11y-plan fct-o6t6iprq)
@binding n=2
**visual lint at boot ↔ the chunk scope's boundary «Nothing crosses»**: the probe adds no finding, field or log; the stand at boot must still read `at_boot` `[0, 0, 0, 0]` in both layout modes (a11y-plan/visual-design-verification fct-67ycdssr)
