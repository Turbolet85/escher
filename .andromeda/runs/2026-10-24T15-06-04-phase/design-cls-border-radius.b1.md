# Classified — design · border-radius.b1

## Answers
border-radius#1.1 · no · browser chrome corner radii; the chunk touches no browser screen, the browser chrome being out of scope
border-radius#1.2 · no · browser about-page radii; no browser or about page is in the chunk
border-radius#1.3 · no · the stand's own card, button, input, task-card and tag radii; the chunk changes no stand markup or CSS and no witness reads a corner shape
border-radius#1.4 · no · the transparent example's card radius; examples/transparent is not reached and no chunk check reads it
border-radius#1.5 · no · blitz-dom's default button radius in its default.css, not reached; the lint and text_extent read geometry and colour, not corners
border-radius#1.6 · no · blitz-paint's checkbox frame radius, not reached; the chunk changes no engine painting and names no checkbox
border-radius#1.7 · no · blitz-paint's scrollbar thumb radius, not reached; the scroll witnesses assert offsets and into-view, not thumb paint
border-radius#2.1 · no · blitz-paint's per-corner radius resolution, not reached; the chunk changes no engine painting and its screenshot witnesses read no corner value
border-radius#2.2 · no · blitz-paint's inset clip-path radius TODO, not reached; the chunk's clip witnesses rest on overflow and contain: paint, not clip-path
border-radius#2.3 · no · blitz-dom's img, object and image-input border mapping, not reached; the chunk's fixtures are text and boxes with no such element
border-radius#3.1 · no · radius examples under examples/assets, not reached; the chunk's new fixture pages depend on no corner shape
border-radius#3.2 · no · google, graphite, hr and input example radii under examples/assets, not reached; no chunk check reads them
border-radius#3.3 · no · the inline-flex-transform and transforms example button radius, not reached; the chunk's transformed-box fixture is a new page with no corner shape
border-radius#3.4 · no · servo, shadow, Preact and reference page radii, not reached; no chunk check reads them
border-radius#4.1 · no · no radius token in the 15 s05 files; the chunk adds none, so holding the absence is a re-check every change owes
border-radius#4.2 · no · no radius, box-shadow or z-index in the 17 listed s06 files; the chunk reads no elevation, so holding the absence is a re-check every change owes
border-radius#4.3 · no · no radius in the 8 s07 slice files; the chunk adds none, so holding the absence is a re-check every change owes
border-radius#4.4 · no · no radius in the 16 listed s08 files; the chunk adds none, so holding the absence is a re-check every change owes
border-radius#4.5 · no · no radius definition in the 32 s10 slice files; the chunk adds none, so holding the absence is a re-check every change owes
border-radius#4.6 · no · only a touch-point radius in the 21 listed s11 files, no corner shape; the chunk drives pointer input, not touch, and reads no elevation
