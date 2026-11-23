@row_add n=1 row=decision/canvas-crosses-as-pixels area=established-decisions fact=F1 detector=none
name: Canvas content crosses only as pixels
rule: A canvas's app-painted content crosses the driver's carriers only as pixels in a screenshot, on every ratified carrier, with no new carrier, verb, field, exit, refusal cause, session error, log field or span.
why: ratified
basis: fct-o5thr6xa, fct-xx3c5d24, fct-h53hgvcx
because: The chunk paints an app's widget into harness and driver shots with no verb, field, cause, exit, log field or span added, and no standing row states how a canvas's content crosses the carriers.
@row_add n=2 row=decision/harness-paints-custom-widgets area=established-decisions fact=F2 detector=none
name: Harness turns on blitz-paint's custom-widget feature
rule: blitz-test-harness names blitz-paint's custom-widget feature, so a harness or driver shot paints an app's widget in every build selection reaching the harness.
guard: stand_canvas_shot
basis: fct-xkqgbvsv, fct-o5thr6xa
because: The harness's blitz-paint dependency now names custom-widget, and no standing row states which crate turns that feature on for headless shots.
