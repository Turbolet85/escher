# Classified — design · typography.b2

## Answers
typography#3.4 · yes · line-height normal as 1.2 × font size, mapped to Parley line heights, sets the inline text box height that text_extent reads, which the truncated reading of clipped text rests on
typography#3.5 · yes · Parley letter and word spacing resolved against font size is part of the text mapping text_extent reads, sizing the inline text box width the truncated and RTL readings measure
typography#3.6 · yes · the Parley locale from the computed lang is part of the text mapping the Needs name as text_extent reads it, shaping the text box the readings measure
typography#3.7 · yes · text-align and the break, wrap and white-space settings are Parley's line layout, which text_extent's content and inline text boxes and the truncated reading of text in a clipping box rest on
typography#3.8 · no · ch and ic unit metrics are font-metric unit resolution outside the text mapping and line layout the Needs name, and no fixture the chunk names uses a ch or ic length
typography#3.9 · no · rem and rlh root seeding is unit resolution in document.rs that the sweep neither changes nor asserts, its fixtures naming no rem length and the stand's CSS staying unchanged
typography#3.10 · no · list marker strings are list.rs's rule, which no Needs bullet names, and the stand's markup stays unchanged
typography#3.11 · no · font emboldening is a paint-time rule in blitz-paint, and the screenshot witness compares a shot only with the harness raster and another process's shot on one host, both painted by the same engine
typography#3.12 · no · text-decoration thickness and line positions are paint-time rules in blitz-paint, the sweep's Needs name no decoration and it changes no painting
typography#4.1 · no · the example pages' sans-serif and monospace stacks in examples/assets are outside the reached files, the stand and the new fixtures
typography#4.2 · no · graphite's Inter and Bona Nova type stack in examples/assets/graphite.html is an example page outside the reached files and the sweep
typography#4.3 · no · the gosub, servo-new-reduced and pseudo example pages are outside the reached files, the stand and the new fixtures
typography#4.4 · no · servo.css and servo.html are an example stylesheet and page outside the reached files, and the offline stand loads no Google Fonts
typography#4.5 · no · the custom-widget and Preact example type stacks are outside the reached files, and the sweep runs no windowed example
typography#4.6 · no · the text-decoration example page is outside the reached files, and the sweep adds no decoration reading
typography#5.1 · no · the absence of font-family declarations in the s08 files is a fact, not a rule, and holds on the reached files only as a re-check every change owes
typography#5.2 · no · the absence of font-family declarations in the s09 files is a fact, not a rule, and holds on the reached files only as a re-check every change owes
typography#5.3 · no · the only font-family match outside test fixtures is the test file packages/blitz-vibey-script/tests/dom.rs and the sweep's new pages are test fixtures, a re-check every change owes on the reached product files
typography#5.4 · no · the chunk defines no type scale or font token, which its Not needed list rules out, so the absence binds nothing it adds
