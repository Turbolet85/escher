@row_add n=1 row=decision/canvas-content-crosses-as-pixels area=pii-scrubbing-compliance fact=F1 detector=D-obs-pii
name: Canvas content crosses only as pixels in a shot
rule: A canvas's app-painted content crosses only as pixels in a harness or driver shot, whole or element-cropped, on every ratified carrier, the model provider on a cold-agent live run included; no span, log field, cause, verdict field or kept check carries it, a kept check keeping no pixels.
why: a boundary widening, ratified; in process only for now, no served app holding a canvas
basis: fct-o5thr6xa, fct-h53hgvcx, fct-oxxunjkv
because: The chunk paints an app's widget into the harness and driver shots and the crossing was ratified for every carrier, so obs-plan's PII section must say where a canvas's content may leave and where it never does.
@row_add n=2 row=decision/pixel-check-failures-name-mode-row-pixel area=3-observability-harness-contract fact=F2 detector=none
name: Pixel check failures name a mode, a row and a pixel
rule: A stand check reading pixels names in its failure message only a layout mode, a row and the RGBA of the pixel it read, never page text or a control value.
why: a pixel's RGBA is a measured number, no page text or control value
guard: stand_canvas_shot
basis: fct-ktuxg7dw
because: The chunk's new pixel-reading stand check widened its failure message to carry the pixel it read, so the harness contract must state what such a failure may print.
