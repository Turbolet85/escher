# design extract

## Relevance
partial: the chunk designs no surface, but a take carrying paint, default-sheet, hit, scroll or clock commits lands on engine defaults and behaviours this plan records, and they must stand after it.

## Constraints
- A conflict in `packages/blitz-dom/assets/default.css` keeps the select rules this plan records (single-choice box, `multiple` rows, the chosen-row rule, the transparent `optgroup`) beside whatever upstream's `dee854c76` or `e5929f9e2` brings; whether the trial resolution does is research's question (per design-system §Surface: desktop-native → Tokens (platform-specific)).
- The open select list's colours stay `SelectListView` associated constants, with their values, and the engine gains no token; a take that moves `packages/blitz-dom/src/node/select_list.rs` or the sheet resolves to that (per design-system §Color Palette → Engine-painted defaults).
- `fe33b778b`'s narrower restyle invalidation must not narrow the full recascade a colour-scheme change requires; whether it does is research's question, per candidate shape (per design-system §Color Palette → Color model and scheme (engine)).
- A conflict in `packages/blitz-paint/src/text.rs` keeps `stroke_text_as`'s `RunPaint` contract and the embolden strengths; any caller a take adds or edits there, `ac6f9f582`, `f76777766` and `e5929f9e2` being the candidates, adds no synthesized bold unless it means to; which of them reach that function is research's question (per design-system §Typography → Engine text mapping).
- The conflicts in `packages/blitz-dom/src/node/node.rs`, the `hit_inner` hunk of `0980cce5f` and the culling of `e42ee1f25` keep this plan's hit stops at a clipping padding box, the non-invertible answer, `visible_region`'s reading and the select list's being painted last and answered first; `6fca6308d`'s `mix-blend-mode` and `isolation` work may change the stacking-root and atomic-effect conditions, and a change there is recorded, not silent (per design-system §Depth Strategy → Engine depth behavior).
- A conflict in `packages/blitz-dom/src/scrolling.rs` or `examples/seven_guis/src/tasks/cells.rs` keeps into-view scrolls instant in nested boxes and in the Cells grid, the driver's `scroll` instant, and the select list's own scroll written at once (per design-system §Motion → This project's values).
- The take keeps the single `VirtualClock` reading: settle never advances it, only the driver's `advance` moves it, and the load wait holds it still (per design-system §Motion → Animation runtime).

## Patterns to follow
- An engine-painted default is a fixed constant or a default-sheet rule, stated as such, with no token invented for it (per design-system §Color Palette → Engine-painted defaults).
- An engine overlay reaches the screen through `paint_scene` alone, painted after the root and before the devtools overlays, and hit-tested before the page's walk (per design-system §Depth Strategy → Engine depth behavior).
- A paint path that varies a run's look passes it through `RunPaint`; every other caller passes `RunPaint::default()` (per design-system §Typography → Engine text mapping).
- blitz-tests build `blitz-dom` with `system-fonts`, so a test file a take carries, under the new font-embolden gate too, measures real text (per design-system §Typography → Loading).

## Anti-patterns to avoid
- Moving the select list's colours into `default.css`, or adding a token or custom property, while resolving a sheet conflict (per design-system §Color Palette → Engine-painted defaults).
- Counting a font-dependent assertion as green on a build that resolves no fonts, where it passes vacuously, or painting a shot with its text skipped in place of the `fonts-unavailable` refusal (per design-system §Typography → Loading).
- Letting a print from the taken delta reach `escher-session` or `mcp` stdout, or adding colour, an ESC byte or a terminal test to them (per design-system §Surface: cli → Component Patterns).

## Contract bindings
- design ↔ a11y: the chosen-row pair, the select list's plain pair and the standing contrast rows bind to a11y-plan §6 Visual Design Verification; a take carrying `6fca6308d`, `ac6f9f582` or `e5929f9e2` may need `Harness::text_contrast`'s `None` conditions re-read (design-system §Color Palette → Core Colors).
- design ↔ a11y: the motion line of this domain's focus guide finds no mandate here, as design-system §Motion → Reduced motion records handling absent; this chunk adds no reduce-motion check from design, and the a11y target it re-proves does not name SC 2.3.3.
- design ↔ obs: stdout's JSON-only rule on `escher-session` and `mcp` meets the entry's sink re-check over the taken delta's prints (obs-plan §6 Log Coverage; design-system §Surface: cli → Component Patterns).
- design ↔ tests: the font-embolden local gate of the folded `cry-twswdbpc` proves the embolden strengths and the group header's weight (design-system §Typography → Engine text mapping); its build needs `system-fonts` (design-system §Typography → Loading).
- design ↔ layouts: the stand's accent, its Cells focus outline and other stand colours are held in layout-templates §Surface: desktop-native; a `cells.rs` conflict answers there for them, here only for the scroll behaviour.

## Acceptance criteria contributions
- `cargo test -p blitz-tests --locked --no-fail-fast --test select_popup_painted` passes, and passes again with `--features blitz-paint/font-embolden` (per design-system §Typography → Engine text mapping).
- After the take, `SelectListView`'s four constants read the values recorded, `default.css` holds no rule for the open list, and the `multiple` chosen-row rule still reads its recorded pair (per design-system §Color Palette → Core Colors).
- `hit_clipped_at_scrolling_box`, `hit_non_invertible_transform`, `visible_region_transformed`, `paint_order`, `select_popup` and `stand_act_popup` pass, in both layout modes where they run so; a moved number is re-measured and recorded, not loosened (per design-system §Depth Strategy → Engine depth behavior).
- `scroll_into_view_nested`, `stand_settle`, `stand_virtual_clock` and `stand_act_timer` pass with their assertions unedited, unless a change is recorded by the same re-measure (per design-system §Motion → Animation runtime).
