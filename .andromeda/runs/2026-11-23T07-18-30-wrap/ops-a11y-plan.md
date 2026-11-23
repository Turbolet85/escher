@row_add n=1 row=decision/canvas-content-leaves-only-as-pixels area=screen-reader-support fact=F1 detector=none
name: Canvas content leaves a session only as pixels
rule: An app's canvas content, painted from Rust through a custom widget, leaves a session only as pixels in a harness or driver shot; no snapshot field, carrier, cause or log field carries it.
why: ratified
guard: stand_canvas_shot
basis: fct-o5thr6xa, fct-h53hgvcx
because: The chunk paints an app's widget into harness and driver shots with no verb, field, cause, exit, log field or span added, so a screen reader's snapshot reads none of the canvas's content and the section must say so as its own decision beside the option-label row.
