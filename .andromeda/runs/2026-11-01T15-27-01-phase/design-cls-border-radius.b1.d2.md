# Classified — design · border-radius.b1

## Answers
border-radius#1.1 · no · browser chrome radii sit in apps/browser/assets/browser.css, which the scope names as Not needed
border-radius#1.2 · no · browser about-page radii sit in apps/browser/assets/about-*.css, which the scope does not name (Not needed)
border-radius#1.3 · yes · the seven_guis stand's card, button, input, task-card, tag and select radii are among the fixture's recorded values that the scope keeps accurate
border-radius#1.4 · no · the transparent example card radius is a fixture value of an example other than examples/seven_guis (Not needed)
border-radius#1.5 · yes · the blitz-dom button radius at default.css:108 is in a predicted conflict surface, and the scope reads recorded engine radius defaults against the resolved sheet
border-radius#1.6 · no · the checkbox frame radius is painted in form_controls.rs, which the chunk does not reach, and it is not a sheet value
border-radius#1.7 · no · the scrollbar thumb radius is painted in render.rs, which the chunk does not reach; the scroll needs cover scroll_into_view and visible_region, not the thumb
border-radius#2.1 · yes · the re-measured screenshot pixels and snapshot sizes paint per-corner radii scaled to device pixels in render.rs
border-radius#2.2 · no · the inset() clip-path TODO sits in clip_path.rs, which the chunk does not reach, and the scope names no clip-path change or check that uses inset()
border-radius#2.3 · yes · the border attribute mapping lives in stylo.rs, a predicted conflict surface, so a conflict resolution can move it
border-radius#3.1 · no · the example fixtures in examples/assets and examples/transforms.rs are not reached, and examples other than seven_guis are not named by the scope
border-radius#3.2 · no · the google, graphite, hr and input fixture values are of examples other than seven_guis, and the scope names no change to them (Not needed)
border-radius#3.3 · no · the inline-flex-transform fixture and the transforms example are not reached, and the scope names no change to examples other than seven_guis (Not needed)
border-radius#3.4 · no · the servo, shadow and preact fixture values are of examples the scope does not name (Not needed)
border-radius#4.1 · no · a past search record over s05 files, stating no border-radius rule the chunk's code or tests must honour
border-radius#4.2 · no · a past search record over s06 files with no border-radius handling found, stating no rule the chunk must honour
border-radius#4.3 · no · a past search record over s07 slice files, stating no radius rule the chunk must honour
border-radius#4.4 · no · a past search record over s08 files with no match, stating no rule the chunk must honour
border-radius#4.5 · no · a past search record over s10 slice files, stating no radius rule the chunk must honour
border-radius#4.6 · no · a past search record over s11 files with only a touch-point radius match, stating no rule the chunk must honour
