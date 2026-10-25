# Classified — design-system · iconography.b1

## Answers
iconography#1.1 · no · the cursor rule lives in blitz-dom's document.rs, which the change does not reach, and the report lists Iconography as untouched with no cursor rule changed
iconography#1.2 · no · the keyword-to-cursor map and the shell's cursor set live in blitz-dom, blitz-traits and blitz-shell, none reached, and no Cargo file is
iconography#1.3 · no · favicon_url lives in blitz-dom's document.rs, which the change does not reach, and the report lists favicons as untouched
iconography#2.1 · no · the 36-cursor fixture (examples/assets/cursor.html) is not a reached file
iconography#2.2 · no · the octicon and Google inline SVG fixtures are not reached files
iconography#2.3 · no · the Font Awesome fixtures are not reached files, and the report names no icon font or icon class
iconography#2.4 · no · the graphite sprite-atlas fixture is not a reached file
iconography#2.5 · no · the currentColor SVG fixture, examples/svg_native.rs and the Preact fixture are not reached files
iconography#3.1 · no · the report adds no icon asset or icon handling to any reached file, so the no-match search over the listed s08 files still holds
iconography#3.2 · no · the report adds no icon set to any reached file, so the no-match search over the listed s11 files still holds
