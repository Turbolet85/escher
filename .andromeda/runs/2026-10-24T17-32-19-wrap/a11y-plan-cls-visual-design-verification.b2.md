# Classified — a11y-plan · visual-design-verification.b2

## Answers
visual-design-verification#7.4 · no · its zoom rule lives in packages/blitz-shell/src/window.rs, which no reached file names and the session does not change
visual-design-verification#7.5 · no · its rdme colour-scheme rule lives in apps/readme/assets CSS, outside the reached files and the session's change
visual-design-verification#7.6 · no · its colour-scheme declarations live in examples/preact HTML, outside the reached files and the session's change
visual-design-verification#7.7 · no · its inverted-colors image rule lives in packages/blitz-dom/assets/default.css, outside the reached files and the session's change
visual-design-verification#7.8 · no · its touch-action panning rule lives in packages/blitz-dom/src/events/pointer.rs, outside the reached files and the session's change
visual-design-verification#8.1 · yes · add-to · the inert_subtree test it cites now also runs in stand-repeat, through ci-leg.sh's fixed --test list (three rounds, zero-failure budget), its behaviour unmoved
visual-design-verification#8.2 · no · its visibility-hidden and collapse hit rule lives in packages/blitz-dom/src/node/node.rs, which the session does not reach, and it cites no test the session moves
visual-design-verification#8.3 · no · its pointer-events hit rule lives in packages/blitz-dom/src/node/node.rs, which the session does not reach, and it cites no test the session moves
visual-design-verification#8.4 · no · its scrollbar-width rule lives in packages/blitz-dom/src/node/scrollbar.rs and its test in tests/blitz-tests/tests/scrollbars.rs, neither reached nor in the stand-repeat change
visual-design-verification#8.5 · no · its devtools settings live in packages/blitz-traits/src/devtools.rs, which the session does not reach
