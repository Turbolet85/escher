# architecture · picked accounting · group 8 of 8

## Picked
existing-scopes#1.4 · not touched · the report adds no module and its only added `#[cfg(test)]` module is blitz-shell's, so the module list and `document`'s two child modules still hold on the reached `document.rs`
existing-scopes#1.6 · not touched · the reached `form.rs` still holds form owner reset and submission, the report adding one read at `:281` and two tests in its standing module, and `iframe.rs` and `query_selector.rs` are not reached
existing-scopes#1.7 · not touched · the report adds select readers, a `select_cursor` field and a wider `transformed_bounds` to the reached `document.rs`, and hit testing, hover, focus, active, scrollbar, geometry and text selection all still stand there, so only digits move
existing-scopes#1.8 · not touched · the reached `accessibility.rs` still builds the tree, the report moving only what it writes, and the entity states no write or limit
existing-scopes#1.10 · not touched · the reached `scrolling.rs` still holds the visible-region reader with its signature kept, and the entity states no untransformed limit, so only digits move
existing-scopes#1.18 · not touched · the report retires a synthesis in `runtime/events.rs` and adds one test to `tests/dom.rs`, and the entity names only modules and test files, each still standing
existing-scopes#1.19 · proposed · `event` now holds blitz-shell's first `#[cfg(test)]` module, F22's second half
existing-scopes#1.23 · proposed · the `snapshot` module's re-exports gain `closed_select_of` and `is_multiple_select`
existing-scopes#1.31 · proposed · test counts and clauses move at lines 102, 104, 106, 112, 113 and 116, the pin renamed and inverted, the focus-fixup limit retired and Cells reading 677
existing-scopes#2.1 · proposed · `stand_cells_served` moves 4 to 5 and `stand_load_page` 2 to 3
existing-scopes#3.2 · proposed · fixture readers move 24 to 28, `session_common` staying 31
existing-scopes#3.5 · proposed · the three tests now run both layout modes, ending upstream's exception, and F22's new targets stand beside it
