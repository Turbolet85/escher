# Classified — a11y · screen-reader-support.b2

## Answers
screen-reader-support#9.2 · yes · default.css is a predicted conflict surface and the `dir` bidi-isolation attribute selectors (lines 14-36) live in it, so the rule binds through the conflict
screen-reader-support#9.3 · no · html lang records over examples/assets fixtures the chunk does not name; a fixture lang record, dropped by Not needed
screen-reader-support#9.4 · no · lang="en" in the servo and preact fixtures, files the chunk does not name; a fixture lang record, dropped by Not needed
screen-reader-support#10.1 · no · observed-absent lang search over the s02 fixtures, a past search the chunk's named files do not include, dropped by Not needed
screen-reader-support#11.1 · no · sr-only tooltip in the GitHub fixture, a fixture text-alternative record, dropped by Not needed
screen-reader-support#11.2 · no · google fixture aria-labels, a fixture text-alternative record over an unnamed file, dropped by Not needed
screen-reader-support#11.3 · no · google fixture aria-hidden and aria-atomic, a fixture text-alternative record over an unnamed file, dropped by Not needed
screen-reader-support#11.4 · no · servo-new burger aria-hidden spans, a fixture text-alternative record over an unnamed file, dropped by Not needed
screen-reader-support#11.5 · no · servo.html image alt text, a fixture text-alternative record over an unnamed file, dropped by Not needed
screen-reader-support#11.6 · no · servo_header_reduced image without alt, a fixture text-alternative record over an unnamed file, dropped by Not needed
screen-reader-support#12.1 · yes · focus enables IME and sets the cursor area in node.rs (771-784, 794-801), a predicted conflict surface, so the rule binds through that conflict
screen-reader-support#12.2 · yes · create_text_editor's IME enable sits in construct.rs and its pinning test input_focused_on_mount_enables_ime in dioxus_document.rs, both predicted conflict surfaces
screen-reader-support#12.3 · no · IME commit, preedit and DeleteSurrounding live in blitz-dom node/text.rs, which the chunk does not reach; the task's text.rs conflict is blitz-paint's
screen-reader-support#13.1 · no · observed-absent accessibility-API search over past s06 files the chunk does not name; the chunk adds no platform adapter
