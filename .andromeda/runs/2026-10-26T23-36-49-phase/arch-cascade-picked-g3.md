# arch · picked accounting · group 3 of 7

## Picked
standard-contracts#1.1 · carried · `can_paint_text` is the reader behind the lint's `fonts-unavailable` refusal, which must stay before anything is read
standard-contracts#2.1 · not in scope · blitz-dom's document clock is only used; the lint reads and moves no clock, and the harness's time it must not move is held by its own entity
standard-contracts#6.1 · carried · the factored walk keeps reading bounds and the visible region through `get_client_bounding_rect` and `visible_region`, which the re-asserted `clipped` and `off-screen` rows read
standard-contracts#6.2 · carried · `text_extent` is the lint's read-only `truncated` reader, its one non-test caller `Session::lint`, which the factoring must keep read-only
standard-contracts#13.1 · carried · `Harness::text_contrast` is the lint's read-only `contrast` reader, its `None` rule and required ratio the inputs of the re-asserted findings
standard-contracts#14.1 · carried · `text_contrast` never moves the harness's `VirtualClock`, so the lint changes nothing on the held instance
standard-contracts#15.1 · carried · `stand_lint`'s re-asserted rows read the stand's pinned viewport, scale, colour scheme and bundled font
standard-contracts#16.2 · carried · the load page whose 0 findings at boot the unchanged lint walk must still read
standard-contracts#17.1 · not in scope · the telemetry bootstrap is not reached; the refactor adds no log site, span or field and the sink is untouched
