# Classified — arch · standard-contracts.b2

## Answers
standard-contracts#9.6 · yes · flush_open_state clears ElementState::OPEN on every element but details and dialog, so an open list held on that state is cleared by an `open` write; the chunk's open-state fork binds
standard-contracts#10.1 · yes · ShellProvider's default no-op request_redraw, which the redraw after a list opens or closes calls; the NetProvider, HtmlParserProvider and Viewport parts do not bind
standard-contracts#11.1 · no · blitz-net and blitz-html fetch and parse paths, which the list never touches (Not needed)
standard-contracts#12.1 · yes · paint_scene's signature is the overlay's paint entry, reached by every blitz-paint renderer, the screenshot and the windowed path
standard-contracts#13.1 · yes · BlitzShellProvider's request_redraw sends RequestRedraw through the proxy, the windowed redraw the list's change requests
standard-contracts#14.1 · yes · screenshot, screenshot_rect, text_contrast, settle and scroll_into_view are the harness readers and helpers the overlay, the contrast row and the driver's scroll use
standard-contracts#15.1 · yes · screenshot, screenshot_rect, text_contrast and settle never move the harness clock, which the list's reduced motion and typeahead read through DocumentConfig.clock
standard-contracts#16.1 · yes · boot(FlightBooker) with the pinned viewport and offline options is the stand the popup and the four stand tables run through
standard-contracts#17.1 · yes · the stand's DejaVu Sans font is what the list's painted text and the stand_contrast row measure at
standard-contracts#18.1 · no · boot_cells mounts Task::Cells, which holds no select the list opens
standard-contracts#18.2 · no · boot_load's load page, which the list does not use and the chunk does not change (Not needed)
standard-contracts#18.3 · no · LoadNet answers the load page's stylesheet fetch, and the list loads nothing (Not needed)
standard-contracts#19.1 · no · standing S8 · engine targets print only safe fields, the telemetry sink's allowlist scrub
</content>
</invoke>
