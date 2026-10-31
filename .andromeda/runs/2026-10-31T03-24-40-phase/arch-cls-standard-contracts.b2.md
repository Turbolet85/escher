# Classified — arch · standard-contracts.b2

## Answers
standard-contracts#12.1 · yes · the shell's BlitzShellEvent set, BlitzShellProxy's NetWaker Poll and BlitzShellProvider's redraw route, which the new windowed redraw check reads
standard-contracts#13.1 · yes · BlitzShellProvider's request_redraw sends RequestRedraw through the BlitzShellProxy, the redraw route the new windowed redraw check reads
standard-contracts#14.1 · yes · text_contrast's WCAG reading, the input helpers that pump and never settle, settle's report of a load in flight and the incremental option the both-modes runs use, which the Cells and load-page rows and the pseudo_element run use
standard-contracts#15.1 · yes · tick and advance_time as the only movers of the shared VirtualClock, with text_contrast and settle never moving it, which the rows' timing relies on
standard-contracts#16.1 · yes · the pinned 800x600 Light viewport and options(incremental) with net_provider None, which boot_cells and boot_load run under
standard-contracts#17.1 · yes · the DejaVu Sans font_ctx every stand boot holds, boot_cells and boot_load included, which the rows lay out and read in
standard-contracts#18.1 · yes · boot_cells mounts Task::Cells in TaskShell, the page the Cells Tab-order, keyboard-activation and contrast rows run on
standard-contracts#18.2 · yes · boot_load's load-fetch Button and its load-done paragraph, which the load-page rows and the load-fetch contrast row run on
standard-contracts#18.3 · yes · LoadNet answers the stylesheet after LOAD_ANSWER_DELAY and is counted as a load in flight, which the load-page rows wait out through settle
standard-contracts#19.1 · no · escher-telemetry's sink, its field sets and its panic hook, which no fix edits (Not needed)
