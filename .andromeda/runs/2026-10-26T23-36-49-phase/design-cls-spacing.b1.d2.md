# Classified — design · spacing.b1

## Answers
spacing#1.1 · no · the rdme app's spacing tokens; no app stylesheet is reached and no stand screen loads an rdme page
spacing#2.1 · no · the reference browser's chrome padding; no app stylesheet is reached and no stand screen is a browser page
spacing#2.2 · no · about:history's page and list padding; the browser's internal page, which the chunk does not reach
spacing#2.3 · no · the rdme markdown body's max-width and padding; the rdme app is not reached
spacing#2.4 · yes · the counter card's 24px 32px pad and 16px gap lay out the counter screen whose boot reading (0 findings) stand_lint re-reads unedited; the app home pad at app.rs:191 is on no booted stand screen
spacing#2.5 · yes · the flight booker's select padding sets the client rect its options read in the boot snapshot, the one overlap stand_lint re-reads on its select's options
spacing#2.6 · yes · the blitz-dom default body margin of 8px lays out the stand screens stand_lint re-reads at boot, unless a stand stylesheet resets it
spacing#3.1 · no · hspace and vspace apply only to embed, img, object, marquee and image inputs, and no booted stand screen carries one
spacing#3.2 · no · the legacy body margin attributes; no stand document sets them, so no boot reading reads them
spacing#3.3 · no · overlay scrollbar thumb geometry is paint-side and takes no layout space, so no boot reading reads it
spacing#3.4 · yes · the single-line text inputs on the CRUD and flight booker screens are vertically centered in the content box whose text box text_extent reads at boot
spacing#3.5 · no · outside list marker padding is paint-side (render.rs), and the lint never paints
spacing#3.6 · no · the double decoration's line gap is paint-side (text.rs), and the lint never paints
spacing#4.1 · no · example pages' grid and flex gaps; examples/assets pages are not stand screens and no boot reading reaches them
spacing#4.2 · no · the graphite example page's custom properties; examples/assets is not a stand screen
spacing#4.3 · no · the hr example page's margins; examples/assets is not a stand screen
spacing#4.4 · no · servo.css column gap and button padding; examples/assets is not a stand screen
spacing#4.5 · no · the preact TodoMVC and reference page widths; examples/preact is not a stand screen
