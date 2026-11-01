# Classified — design · border-radius.b1

## Answers
border-radius#1.1 · no · browser chrome radii in apps/browser/assets/browser.css, an app file the scope does not name
border-radius#1.2 · no · about-page radii in apps/browser/assets/about-*.css, app files the scope does not name
border-radius#1.3 · yes · the scope names the examples/seven_guis fixture's recorded values, and this entity records that stand's card, button, input, task-card, tag and select radii, which the cells.rs resolution must keep accurate
border-radius#1.4 · no · the transparent example's card radius in examples/transparent, an example fixture the scope does not change
border-radius#1.5 · yes · the blitz-dom default button radius in packages/blitz-dom/assets/default.css, a conflict surface whose recorded radius default is read against the resolved sheet
border-radius#1.6 · no · the checkbox frame radius is a paint-time formula in blitz-paint form_controls.rs, a component the chunk only uses, not a default sheet value and not named by the scope
border-radius#1.7 · no · the scrollbar thumb radius is a paint-time formula in blitz-paint render.rs, a component the chunk only uses, with no named behaviour or re-measured value reading it
border-radius#2.1 · yes · per-corner radius resolution and device-pixel scaling in blitz-paint render.rs is what the chunk's re-measured screenshot pixels read for every rounded box
border-radius#2.2 · no · the inset() clip-path TODO in clip_path.rs; no named surface paints inset clip-path and no re-measured value reads it
border-radius#2.3 · yes · the img/object border attribute mapping lives in stylo.rs, a predicted conflict surface, so its rule binds under a take
border-radius#3.1 · no · radius fixtures in examples/assets, example fixtures the scope does not change
border-radius#3.2 · no · radius fixtures in examples/assets for google, graphite, hr and input, other examples the scope does not change
border-radius#3.3 · no · the .button radius in examples/assets/inline-flex-transform.html and examples/transforms.rs, other examples the scope does not change
border-radius#3.4 · no · radius fixtures in servo.css, shadow.html and the preact pages, other examples the scope does not change
border-radius#4.1 · no · absence search record over s05 files the scope does not name; states no value the chunk reads
border-radius#4.2 · no · absence search record over s06 files; states no value the chunk reads
border-radius#4.3 · no · absence search record over s07 slice files; states no value the chunk reads
border-radius#4.4 · no · absence search record over s08 files (no match); states no value the chunk reads
border-radius#4.5 · no · absence search record over s10 slice files; states no value the chunk reads
border-radius#4.6 · no · absence search record over s11 files with only a touch-point match; states no value the chunk reads
