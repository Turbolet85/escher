# Classified — a11y · screen-reader-support.b2

## Answers
screen-reader-support#9.2 · yes · default.css is a predicted conflict surface, so its `dir` bidi-isolation attribute-selector rules bind as part of the default sheet the checks read
screen-reader-support#9.3 · no · the 11 fixture html lang attributes sit in examples/assets, which no reached file names, and no check reads lang
screen-reader-support#9.4 · no · the lang="en" pages sit in examples/assets and examples/preact, neither reached, and no check reads them
screen-reader-support#10.1 · no · an absence record over the s02 fixtures, which the chunk's named files do not include
screen-reader-support#11.1 · no · the sr-only tooltip sits in a captured GitHub fixture the chunk does not reach
screen-reader-support#11.2 · no · the google fixture's aria-labels sit in examples/assets/google.html, which the chunk does not reach
screen-reader-support#11.3 · no · the google fixture's aria-hidden and aria-atomic sit in examples/assets/google.html, which the chunk does not reach
screen-reader-support#11.4 · no · the servo-new burger's aria-hidden spans sit in an unreached fixture
screen-reader-support#11.5 · no · the servo.html alt texts sit in an unreached fixture, and no check reads them
screen-reader-support#11.6 · no · the servo_header_reduced image without alt sits in an unreached fixture
screen-reader-support#12.1 · yes · node.rs is a predicted conflict surface, so the focus IME enable, the cursor area at the content box and the blur disable bind
screen-reader-support#12.2 · yes · construct.rs and dioxus_document.rs are conflict surfaces, so create_text_editor's IME enable for an input focused before its editor, pinned by input_focused_on_mount_enables_ime, binds
screen-reader-support#12.3 · no · node/text.rs (IME commit, preedit and disable application) is not a reached file, and no scope names IME event handling
screen-reader-support#13.1 · no · an absence record of accessibility-API integration over past-searched s06 files, stating no rule the chunk's code must meet
