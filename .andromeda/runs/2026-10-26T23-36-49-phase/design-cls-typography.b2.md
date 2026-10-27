# Classified — design · typography.b2

## Answers
typography#3.3 · no · the font-variant feature mapping is blitz-dom Parley shaping, in no reached file and named by no Need (Not needed: engine text mapping as values)
typography#3.4 · no · the line-height mapping and its normal-equals-1.2-times-font-size resolution are blitz-dom layout, in no reached file and named by no Need (Not needed: engine text mapping as values)
typography#3.5 · no · letter and word spacing resolving against font size is blitz-dom's Parley mapping, in no reached file and named by no Need (Not needed: engine text mapping as values)
typography#3.6 · no · the text locale from lang is blitz-dom's Parley mapping, in no reached file and named by no Need (Not needed: engine text mapping as values)
typography#3.7 · no · the alignment, break and white-space mapping is blitz-dom's Parley mapping, in no reached file and named by no Need (Not needed: engine text mapping as values)
typography#3.8 · no · the ch and ic font metrics are blitz-dom font reads, in no reached file and named by no Need
typography#3.9 · yes · rem resolves against the root font size, which sets the computed font size that the lint's contrast threshold reads through Harness::text_contrast (the Need's 3.0 or 4.5 ratio), so the factoring must keep that size input unchanged
typography#3.10 · no · list marker glyphs and their layout are blitz-dom list code, in no reached file and named by no Need
typography#3.11 · no · font emboldening is a blitz-paint raster effect behind a Cargo feature, and the lint's readings come from layout and computed styles, not paint
typography#3.12 · no · text-decoration thickness and the overline and line-through positions are blitz-paint values, which no reached file or Need reads
typography#4.1 · no · the sans-serif body and monospace code-label stacks are example-page styles under examples/assets, outside the reached files and named by no Need
typography#4.2 · no · the graphite example's Inter Variable and Bona Nova stacks and breakpoint sizes are example-page styles under examples/assets, outside the reached files and named by no Need
typography#4.3 · no · the gosub, servo-new-reduced-1 and pseudo example font stacks are under examples/assets, outside the reached files and named by no Need
typography#4.4 · no · the servo example's Fira font stacks and its Google Fonts load are under examples/assets, outside the reached files and named by no Need
typography#4.5 · no · the custom-widget, Preact and reference page stacks are examples under examples/, outside the reached files and named by no Need
typography#4.6 · no · the text-decoration fixture page under examples/assets is outside the reached files and named by no Need
typography#5.1 · no · the font-family absence over the 16 s08 files holds, since the refactor adds no font-family declaration or identifier to its reached files
typography#5.2 · no · the font-family absence over the 32 s09 files holds, since the refactor adds no font-family declaration or identifier
typography#5.3 · no · the font-family absence outside test fixtures over the 32 s10 files holds, since the refactor adds no font-family declaration
typography#5.4 · no · the absence of a project type scale or font tokens holds, since the refactor adds none; the fonts-unavailable Need reads the engine's font context, not a repository token
