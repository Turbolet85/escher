# Classified — design · border-radius.b1

## Answers
border-radius#1.1 · no · browser chrome radii; the browser app's chrome and stylesheet are not reached (Not needed)
border-radius#1.2 · no · browser about-page radii; the browser app's stylesheet is neither a stand app nor a served page of this chunk (Not needed)
border-radius#1.3 · no · a pointer to stand radii held in layout-templates, not design-system values; the stand checks read focus, colour and overlap, never a radius
border-radius#1.4 · no · an example card's radius; examples/transparent is neither a stand app nor a served page, and no reached file cites it
border-radius#1.5 · no · the blitz-dom default button radius in default.css; default.css is not reached and no debt names a button radius, so the engine-change rule does not reach it
border-radius#1.6 · no · the engine checkbox frame radius in form_controls.rs; native checkedness reads the toggled, checked and pressed values, not the frame's paint, and the file is not reached
border-radius#1.7 · no · the engine scrollbar thumb radius in render.rs; the scrolled-box need is bounds and hit agreement, which a thumb's half-thickness radius does not change, and the file is not reached
border-radius#2.1 · no · the per-corner radius resolution in render.rs, which the chunk paints through and leaves unchanged; the popup fork's radius is an engine-painted P4 value, not a computed border-radius
border-radius#2.2 · no · the inset() clip-path TODO in clip_path.rs; no debt names clip-path and the file is not reached
border-radius#2.3 · no · the border attribute on img, object and image inputs in stylo.rs; no debt touches that mapping and the file is not reached
border-radius#3.1 · no · radius shapes in example fixtures under examples/assets; neither a stand app nor a served page, and the chunk's new fixtures name no radius
border-radius#3.2 · no · radius values in the google, graphite, hr and input example fixtures; neither a stand app nor a served page of this chunk
border-radius#3.3 · no · the `.button` radius in the inline-flex-transform fixture and transforms example; the transformed-box proof is a pan/zoom fixture, and its checks read visible region and client rect, not a radius
border-radius#3.4 · no · radius values in the servo.css, shadow, Preact and reference-page examples; neither a stand app nor a served page of this chunk
border-radius#4.1 · no · an absence search over the 15 s05 files; a record, not a rule the chunk's code or tests must honour
border-radius#4.2 · no · an absence search of radius, shadow and z-index handling over the 17 s06 files; a record, not a rule the chunk's code or tests must honour
border-radius#4.3 · no · an absence search of radius values over the 8 s07 slice files; a record, not a rule the chunk's code or tests must honour
border-radius#4.4 · no · an absence search of radius values over the 16 s08 files; a record, not a rule the chunk's code or tests must honour
border-radius#4.5 · no · an absence search of radius definitions over the 32 s10 slice files; a record, not a rule the chunk's code or tests must honour
border-radius#4.6 · no · an absence search of radius, shadow, elevation and z-index over the 21 s11 files; a record, not a rule the chunk's code or tests must honour
