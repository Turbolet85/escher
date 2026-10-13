# Stand contrast — the per-pair record as measured

Taken 2026-10-13 by `/andromeda-implement` (run `.andromeda/runs/2026-10-13T17-25-26-implement/`). The reading came from
one run of `cargo test -p blitz-tests --locked --test stand_contrast` with the record left empty and a temporary dump
test whose panic message carried the measured rows. That test was removed before the gates ran, so the committed file
prints nothing. The rows below are that reading. `tests/blitz-tests/tests/stand_contrast.rs` states the same 18 rows as
`RECORD`, and `every_stand_control_reads_its_recorded_pair` asserts them in both layout modes.

Boot: `seven_guis::stand::boot` (800 × 600, scale 1, Light, the bundled DejaVu Sans, offline), each lean task as it
mounts, at its boot state. Colours are computed, post-cascade styles. The background is resolved up the ancestor chain to
the first opaque one: every measured row stops at the control's own background.

| # | task | id | result | text | background | ratio | size px | weight | threshold | declared (stylesheet) |
|---|---|---|---|---|---|---|---|---|---|---|
| 0 | Counter | `back-btn` | pass | `#333333` | `#f0f0f0` | 11.09 | 13 | 400 | 4.5 | `#back-btn` `color: #333` · `background: #f0f0f0` (app.rs `SHELL_CSS`) |
| 1 | Counter | `counter-increment` | **fail** | `#ffffff` | `#4a6cf7` | 4.39 | 16 | 500 | 4.5 | `.counter-btn` `#ffffff` on `#4a6cf7` |
| 2 | FlightBooker | `back-btn` | pass | `#333333` | `#f0f0f0` | 11.09 | 13 | 400 | 4.5 | as row 0 |
| 3 | FlightBooker | `flight-one-way` | **fail** | `#ffffff` | `#4a6cf7` | 4.39 | 14 | 600 | 4.5 | `.type-btn-active` `#ffffff` on `#4a6cf7`, weight 600 (the active toggle at boot) |
| 4 | FlightBooker | `flight-return` | pass | `#444444` | `#fafafa` | 9.33 | 14 | 400 | 4.5 | `.type-btn` `color: #444` · `background: #fafafa` |
| 5 | FlightBooker | `flight-start` | pass | `#1a1a1a` | `#ffffff` | 17.40 | 15 | 400 | 4.5 | `.date-input` `#1a1a1a` on `#ffffff` |
| 6 | FlightBooker | `flight-return-date` | exempt-inactive | `#999999` | `#ececec` | 2.41 | 15 | 400 | (4.5) | `.date-input:disabled` `#999999` on `#ececec` (disabled at boot, one-way) |
| 7 | FlightBooker | `flight-book` | **fail** | `#ffffff` | `#4a6cf7` | 4.39 | 15 | 500 | 4.5 | `.flight-btn` `#ffffff` on `#4a6cf7` (enabled at boot) |
| 8 | Timer | `back-btn` | pass | `#333333` | `#f0f0f0` | 11.09 | 13 | 400 | 4.5 | as row 0 |
| 9 | Timer | `timer-duration` | no-text | — | — | — | — | — | — | range input, paints no text |
| 10 | Timer | `timer-reset` | **fail** | `#ffffff` | `#4a6cf7` | 4.39 | 16 | 500 | 4.5 | `.timer-reset-btn` `#ffffff` on `#4a6cf7` |
| 11 | Crud | `back-btn` | pass | `#333333` | `#f0f0f0` | 11.09 | 13 | 400 | 4.5 | as row 0 |
| 12 | Crud | `crud-filter` | pass | `#000000` | `#ffffff` | 21.00 | 14 | 400 | 4.5 | no colour in the task sheet: text inherited (no ancestor declares `color`, the initial black), background the UA `input { background-color: white }` |
| 13 | Crud | `crud-name` | pass | `#000000` | `#ffffff` | 21.00 | 14 | 400 | 4.5 | as row 12 |
| 14 | Crud | `crud-surname` | pass | `#000000` | `#ffffff` | 21.00 | 14 | 400 | 4.5 | as row 12 |
| 15 | Crud | `crud-create` | **fail** | `#ffffff` | `#4a6cf7` | 4.39 | 14 | 500 | 4.5 | `.crud-buttons button` `#ffffff` on `#4a6cf7` |
| 16 | Crud | `crud-update` | exempt-inactive | `#ffffff` | `#b0b8d8` | 1.96 | 14 | 500 | (4.5) | `.crud-buttons button.btn-off` `background-color: #b0b8d8`, text `#ffffff` (disabled at boot, nothing selected) |
| 17 | Crud | `crud-delete` | exempt-inactive | `#ffffff` | `#b0b8d8` | 1.96 | 14 | 500 | (4.5) | as row 16 |

The unrounded readings: 11.086817 (`back-btn`), 4.3939724 (the five fails), 9.33138, 17.404325, 2.4116313, 20.999998
(the CRUD inputs) and 1.9637561. Sizes read exactly 13.0, 14.0, 15.0 and 16.0 px, so rounding to whole px dropped nothing.

## Counts by verdict
- **pass:** 9 — `back-btn` ×4, `flight-return`, `flight-start`, `crud-filter`, `crud-name`, `crud-surname`
- **fail:** 5 — `counter-increment`, `flight-one-way`, `flight-book`, `timer-reset`, `crud-create`
- **exempt-inactive:** 3 — `flight-return-date`, `crud-update`, `crud-delete`
- **no-text:** 1 — `timer-duration`
- Total 18 rows. No text is large (13–16 px, weight 400–600), so every measured threshold is 4.5.

## The scope's hypothesis
It held as stated (scope.md, Boundaries; plan.md, Implementation notes). The five predicted controls read `#ffffff` on
`#4a6cf7` at 4.39 (4.394 by research's scratch computation), and each is `fail` against 4.5. Every other prediction held
too:
- `back-btn` 11.09 and `flight-return` 9.33, both pass;
- `flight-start` 17.40, pass;
- `flight-return-date` 2.41, and `crud-update` / `crud-delete` 1.96, all three exempt-inactive;
- `timer-duration` no-text;
- the CRUD inputs 21.00, pass. Their text inherits black and their background is the UA `white`, as predicted.

## Measured against declared
No measured colour differs from the value its stylesheet declares. The shorthands `#333` and `#444` read `#333333` and
`#444444`, the same colours. The three CRUD inputs declare no colour, and read the inherited initial black on the UA white.

## The painted witness
`the_painted_background_is_the_measured_background` takes the pixel inside each measured control's left padding (17
rows, all but `timer-duration`). In both layout modes, each pixel's RGB matches the measured background within 1 per
channel. Every such box has a left padding of at least 2 px, and no measured control holds focus.
