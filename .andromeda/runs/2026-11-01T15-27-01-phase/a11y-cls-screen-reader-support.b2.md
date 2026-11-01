# Classified — a11y · screen-reader-support.b2

## Answers
screen-reader-support#9.2 · yes · default.css is a predicted conflict surface, so the `dir` attribute-selector bidi rule at lines 14-36 binds through the blitz-dom default sheet's conflict
screen-reader-support#9.3 · no · html lang records over examples/assets fixtures the chunk's named files do not include; fixture lang records are not needed
screen-reader-support#9.4 · no · lang="en" in the servo and preact fixtures, outside the reached files; fixture lang records are not needed
screen-reader-support#10.1 · no · observed-absent lang record from a past search of the s02 fixtures, which the chunk's named files do not include
screen-reader-support#11.1 · no · sr-only tooltip in the github fixture; a fixture text-alternative record over a file the chunk does not reach
screen-reader-support#11.2 · no · google fixture aria-labels; a fixture text-alternative record over a file the chunk does not reach
screen-reader-support#11.3 · no · google fixture aria-hidden and aria-atomic; a fixture record over a file the chunk does not reach
screen-reader-support#11.4 · no · servo-new burger aria-hidden spans; a fixture record over a file the chunk does not reach
screen-reader-support#11.5 · no · servo.html image alt text; a fixture record over a file the chunk does not reach
screen-reader-support#11.6 · no · servo_header_reduced.html image without alt; a fixture record over a file the chunk does not reach
screen-reader-support#12.1 · yes · node.rs is a predicted conflict surface, so focus enabling IME and setting its cursor area to the content box, and blur disabling it, at node.rs:771-801 bind through the node.rs conflict
screen-reader-support#12.2 · yes · construct.rs and dioxus_document.rs are predicted conflict surfaces, so the create_text_editor rule of IME on for an already-focused input at construct.rs:860-872 and dioxus_document.rs:741-775 binds, pinned by input_focused_on_mount_enables_ime
screen-reader-support#12.3 · no · node/text.rs is not a reached file, and the IME commit, preedit and disable handling it states is not named in the scope
screen-reader-support#13.1 · no · observed-absent record of screen-reader or accessibility API integration over past s06 searches; the chunk adds no platform adapter, and the absence record is not needed
