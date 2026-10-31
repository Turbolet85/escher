# Classified — arch · design-philosophy.b1

## Answers
design-philosophy#1.1 · yes · the overlay is painted by blitz-paint for every renderer and lives in the engine, not in a shell or platform entry, so the engine stays separately usable
design-philosophy#2.1 · yes · the list's redraw request goes through the ShellProvider the embedder supplies, and the chunk reads DocumentConfig's clock and ua_stylesheets, so no direct shell call
design-philosophy#3.1 · yes · the select's focus under SC 2.1.1 and 2.4.3, its input and change dispatch, the Enter and Space default actions and the SC 1.4.3 colour check are browser-semantic behaviour the chunk must match
design-philosophy#4.1 · yes · the open state's versioned NodeId with clear-or-retarget on removal, damage and redraw through doc.mutate() on open and close, and the topology-only paint tree bind the chunk's code
design-philosophy#5.1 · yes · the four stand tables, the headless harness's screenshot, text_contrast and settle, and the blitz-tests checks the chunk adds run without a window through the real dispatch pipeline
