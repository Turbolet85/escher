@row_add n=1 row=decision/app-widget-shows-in-shots area=test-strategy fact=F1 detector=D-tests-coverage
name: App widget painted from Rust shows in shots
rule: A widget an app paints from Rust shows in a harness shot and in a driver shot, whole and cropped to its element, in both layout modes, under each per-package test selection and in the workspace build.
guard: stand_canvas_shot
basis: fct-o5thr6xa, fct-lmyruqwd, fct-i35rm7mu, fct-h53hgvcx, fct-h4kquilq
because: The chunk adds the stand check that witnesses a Rust-painted widget in harness and driver shots per package and in the workspace build, and no row of the test strategy states that witness.
@row_add n=2 row=decision/harness-turns-on-custom-widget-paint area=ci-integration fact=F2 detector=none
name: Harness turns on custom-widget paint itself
rule: The headless test harness turns on blitz-paint's custom-widget feature itself, so a shot paints an app's widget in every per-package test selection that reaches the harness, not only in the workspace build.
guard: stand_canvas_shot
basis: fct-xkqgbvsv, fct-wqnem4ql
because: The harness's blitz-paint dependency now names custom-widget, so per-package selections and the workspace build no longer differ on whether a shot paints a widget, and no CI row states it.
@row_add n=3 row=decision/canvas-crosses-only-as-shot-pixels area=session-lifecycle fact=F3 detector=none
name: Canvas content crosses only as shot pixels
rule: An app's canvas content crosses to a caller only as pixels in a screenshot, with no carrier, field, exit, cause, session error, log field or span of its own, and its crossing is checked in process only.
guard: stand_canvas_shot
basis: fct-xx3c5d24, fct-h53hgvcx, fct-o5thr6xa, fct-sru74vhw
because: The chunk paints a widget's pixels into the harness and driver shots in process only and adds no carrier or field, a boundary decision no session row states.
@row_add n=4 row=decision/stand-failure-messages-hold-no-content area=unit-test-strategy fact=F4 detector=none
name: Stand check failure messages hold no content
rule: A stand check's failure message carries the layout mode, the row and, where the check reads one, a measured number such as the RGBA pixel it read, and never page text or a control value.
why: A pixel's RGBA is a measured number, no page text or control value.
basis: fct-ktuxg7dw
because: The canvas shot check's failure message now prints the pixel it read beside the layout mode and the row, a convention for what a stand failure may carry that no row states.
