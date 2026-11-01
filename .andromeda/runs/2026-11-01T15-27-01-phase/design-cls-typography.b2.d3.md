# Classified — design · typography.b2

## Answers
typography#3.4 · yes · line-height mapping to Parley is an engine text mapping value the scope reads against construct.rs and inline/build.rs
typography#3.5 · yes · the open select list's row height and select_row_layout clip is what the select list's row-by-row re-read and select_popup tests measure
typography#3.6 · yes · letter and word spacing resolved against font size is a text mapping value the scope reads against construct.rs and inline/build.rs
typography#3.7 · yes · text locale from _x_lang is set through stylo.rs:1217-1219, a conflict surface, and is a text mapping value
typography#3.8 · yes · text-align, word break, wrap and white-space mappings are text mapping values the scope reads against construct.rs and inline/build.rs
typography#3.9 · no · ch and ic metric measurement in font_metrics.rs is in no reached file, named surface or need the scope states
typography#3.10 · yes · the initial 16px root font-size is an engine type default the scope reads against the resolved default.css
typography#3.11 · no · list marker glyphs in list.rs are in no reached file, named need or colour, type, spacing, depth or radius default
typography#3.12 · yes · font-embolden strength and hinting rule in blitz-paint text.rs, a conflict surface, and the font-embolden gate runs through select_popup_painted
typography#3.13 · yes · RunPaint synthesize_bold in stroke_text_as, which stays through a resolution in text.rs, and the select group header witness at select_popup_painted.rs
typography#3.14 · yes · text-decoration thickness and offset painting in text.rs, a conflict surface that e5929f9e2 and f76777766 can change
typography#4.1 · no · body and code font-family values of example fixtures other than examples/seven_guis, which the scope names no change to
typography#4.2 · no · graphite example font values, an example fixture other than examples/seven_guis that the scope does not change
typography#4.3 · no · gosub, servo-new-reduced and pseudo example fonts, fixtures other than examples/seven_guis that the scope does not change
typography#4.4 · no · servo example Fira font stack, an example fixture other than examples/seven_guis that the scope does not change
typography#4.5 · no · custom-widget and Preact example font stacks, example fixtures the scope names no change to
typography#4.6 · no · text-decoration example fixture, an example asset the scope does not name; its painting rule binds through typography#3.14
typography#5.1 · no · a search record of no font-family declarations in listed s08 files, stating no rule the chunk's code or tests must honour
typography#5.2 · no · a search record of font-family declarations over listed s09 files, stating no rule the chunk must honour
typography#5.3 · no · a search record locating font-family outside fixtures in blitz-vibey-script tests/dom.rs, a file not reached, so no rule for the chunk
typography#5.4 · no · a recorded absence of a project type scale, which the chunk defines none of (Not needed)
