# architecture · picked accounting · group 8 of 8

## Picked
existing-scopes#1.4 · not touched · it lists blitz-dom's crate-root modules and the report's two new modules leave that list whole: `select_list` sits under `node` and `lib.rs` gains only the re-export at 108-111
existing-scopes#1.7 · proposed · `document.rs` now holds the open list's state and readers at 761-1089 and `hit_with_scrollbar` answers the list first, an interaction area this unit does not name
existing-scopes#1.8 · not touched · `accessibility.rs` is still tree building with its one new `set_expanded` write at 238-242, a contract's rule (F17) and no change to what this unit says
existing-scopes#1.10 · not touched · `scrolling.rs` still holds into-view scrolling and the visible-region reader, the report's two row arms at 669-672 and 754-756 being standard-contracts' rules (F15)
existing-scopes#1.11 · not touched · `stylo.rs` still participates in Servo styling, the `size` hint at 1124-1139 being established-decisions' F6 and no change to this unit's conversions
existing-scopes#1.14 · proposed · the new `node::select_list` module joins the node unit's list, carrying F27
existing-scopes#1.17 · proposed · `render` gains the child module `render::select_list`, painted last in `paint_scene`
existing-scopes#1.23 · proposed · the snapshot module's re-exports gain `is_select`
existing-scopes#1.31 · proposed · `stand_keyboard` 21 to 23, Enter and Space on a single select retired, `STATE_ROWS` 18 to 20, `stand_act_*` fifteen to sixteen and three new checks to name
existing-scopes#3.2 · proposed · `mod session_common;` readers 31 to 32 and `mod fixtures;` readers 28 to 31 by the report's `mod-decl`, which reached no wording here
