# Hover and active, read by `Harness::text_contrast` (implement step 1)

A scratch test (`tests/blitz-tests/tests/scratch_hover_active_probe.rs`, deleted after this reading) booted each task
through `seven_guis::stand::boot(task, stand::options(incremental))`. On the node's `layout_rect_of` centre it read
`text_contrast(node)` at boot, then after `Harness::move_mouse_to` (the harness's `hovered()` read the node in every
row), then after `Harness::mouse_down_at` at the same point. Run on the untouched tree, before the step-2 re-colour,
on 2026-10-14 (`cargo test -p blitz-tests --locked --test scratch_hover_active_probe -- --nocapture`, exit 0).

The rows are identical in both layout modes (`incremental` false and true):

| task | id | boot | hover | active |
|---|---|---|---|---|
| Counter | `counter-increment` | `#ffffff` on `#4a6cf7` 4.39 | `#ffffff` on `#3a5ce5` 5.47 | `#ffffff` on `#2a4cd3` 6.83 |
| FlightBooker | `flight-book` | `#ffffff` on `#4a6cf7` 4.39 | `#ffffff` on `#3a5ce5` 5.47 | `#ffffff` on `#2a4cd3` 6.83 |
| Timer | `timer-reset` | `#ffffff` on `#4a6cf7` 4.39 | `#ffffff` on `#3a5ce5` 5.47 | `#ffffff` on `#2a4cd3` 6.83 |
| Crud | `crud-create` | `#ffffff` on `#4a6cf7` 4.39 | `#ffffff` on `#3a5ce5` 5.47 | `#ffffff` on `#3a5ce5` 5.47 |
| Counter | `back-btn` | `#333333` on `#f0f0f0` 11.09 | `#333333` on `#e4e4e4` 9.94 | `#333333` on `#e4e4e4` 9.94 |

## Reading

- **Yes.** The reader reads the declared `:hover` background after `move_mouse_to` and the declared `:active` background
  after `mouse_down_at`, for every row. Step 5 therefore asserts the hover and active rows in `STATE_ROWS`.
- CRUD's buttons and `#back-btn` declare no `:active` rule (`crud.rs` `.crud-buttons button:hover:not(.btn-off)`,
  `app.rs` `#back-btn:hover`). A pressed button there keeps its hover background, as the active reading shows. Their
  active row reads the hover pair, which is what the cascade gives.
