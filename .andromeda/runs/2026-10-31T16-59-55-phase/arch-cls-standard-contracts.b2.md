# Classified — arch · standard-contracts.b2

## Answers
standard-contracts#9.6 · yes · a list's open state may be carried as `ElementState::OPEN` read by `:open`, and `flush_open_state` sets OPEN only on details/dialog and clears it on any other element on an `open` write, a fork over where open state lives
standard-contracts#10.1 · yes · `Viewport` `color_scheme` is the read that makes headless documents `ColorScheme::Light` for the new colour default, and `ShellProvider`'s no-op `request_redraw` default governs the redraw requested after a list opens or closes
standard-contracts#11.1 · no · blitz-html and blitz-net's `Provider` are Not needed: the list loads nothing and the chunk parses no HTML
standard-contracts#12.1 · yes · `paint_scene` and the overlay in paint through every blitz-paint renderer, and the password-input paint and caret click rules in blitz-paint render.rs and blitz-dom pointer.rs, which the chunk reaches whole
standard-contracts#13.1 · yes · `BlitzShellProvider.request_redraw` through the shell proxy carries the redraw requested after a list opens or closes, for the windowed path's redraw loop
standard-contracts#14.1 · yes · `screenshot`, `screenshot_rect`, `text_contrast` and `settle` are the harness calls the chunk's stand checks paint, measure and wait through, and `scroll_into_view` is the helper the driver's scroll calls
standard-contracts#15.1 · yes · the list's reduced-motion and typeahead reads go through `DocumentConfig.clock`, and `screenshot`, `text_contrast` and `settle` must never move the harness's time
standard-contracts#16.1 · yes · the four stand tables' new rows boot FlightBooker through `boot` with `options`, offline, and flight booker's `select#flight-type` is the stand's popup
standard-contracts#17.1 · yes · the new stand_contrast row is measured at the stand's DejaVu Sans through `font_ctx()`, the family and size it is measured at
standard-contracts#18.1 · no · boot_cells mounts Cells, a screen with no select, and the chunk's snapshot change touches only a select's options, so the Cells snapshot budget is not reached
standard-contracts#18.2 · no · boot_load's load page and its stylesheet fetch are not reached, the list loads nothing
standard-contracts#18.3 · no · `LoadNet` is the load page's network stand-in, which the list does not reach
standard-contracts#19.1 · no · standing S8 · a new log line carries no snapshot, snapshot text or diff, escher-driver logs no id, name or value, and engine targets print only safe fields
