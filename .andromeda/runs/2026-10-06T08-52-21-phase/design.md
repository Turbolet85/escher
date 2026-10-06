# design extract

## Relevance
partial. The chunk adds no visual surface. Its only design contact is the stand markup that the author-key rule may add to TaskShell chrome and the four lean tasks: any `id:`, key or escher attribute added there must leave the seven_guis as-built visual values unchanged.

## Constraints
- The seven_guis colour values are recorded per design-system §Color Palette (Core Colors "seven_guis accent" row; Surface Scale "seven_guis / todomvc page" row; Text Hierarchy "seven_guis text" row). Adding author keys to stand elements must not change any of them. Whether any stand style selects by `id`, so that a new or renamed `id:` could change a match, is research's question.
- The flight booker's invalid and success state colours are recorded per design-system §Color Palette §Semantic Colors (seven_guis invalid / success rows). A key added to a flight booker control must not move, drop or re-scope the styling that carries those states.
- The stand's typography is recorded per design-system §Typography (the "seven_guis" row: sans-serif, base 14px, home title 36px/700). Headless boot loads the bundled DejaVu Sans for every generic family, with system fonts off, per design-system §Typography **Loading**. The id work must not change the font context that `seven_guis::stand::boot` / `boot_timer` installs.
- The stand's spacing, radius and elevation values are recorded per design-system §Spacing (seven_guis cards/home padding), §Border Radius (seven_guis row) and §Depth Strategy (seven_guis card / circle-dialog shadows). Wrapper elements or restructured markup added for keys must not change them. The `[inferred]` component-path derivation must not need extra DOM wrappers that would.
- The windowed and wasm stand must keep rendering the same tree, per design-system §Surface: web-spa (seven_guis canvas on `#f5f5f5`) and §Surface: desktop-native (Dioxus apps launched through `dioxus_native::launch`). The author-key markup applies identically on both surfaces.

## Patterns to follow
- The stand styles live inline in the task and app Rust sources (the citations under design-system §Color Palette, §Spacing and §Border Radius point into `examples/seven_guis/src/{app.rs,tasks/*.rs}`). Put author keys in the same `rsx!` element declarations, as attributes, and leave the style blocks alone.
- The headless stand pins its font through `HarnessOptions.font_ctx` with `build_single_font_ctx(DEJAVU_SANS)`, per design-system §Typography **Loading**. Proof checks boot through that same path and build no font context of their own.

## Anti-patterns to avoid
- Do not use the stable id as a styling hook. If the author-key rule uses HTML `id:`, adding or renaming one must not create a new CSS match, and no new inline style or stylesheet rule comes with it (the stand's values per design-system §Color Palette / §Typography stay as recorded).
- Do not wrap elements in extra containers to make component paths unique among siblings. Wrappers shift the stand's recorded spacing, radius and shadow values (design-system §Spacing / §Border Radius / §Depth Strategy).

## Contract bindings
- design ↔ a11y: the stand's painted colours are a11y surface (SC 1.4.3, CLAUDE.md invariant). Leaving the design-system §Color Palette seven_guis values unchanged is what keeps the scope's "no painted-colour change" true. The flight booker's invalid/success states bind to a11y not-colour-alone (SC 1.4.1) through design-system §Color Palette §Semantic Colors.
- design ↔ tests: the pinned bundled font (design-system §Typography **Loading**) underlies the deterministic headless boot that the per-task `stand_*.rs` checks run on, under the test-plan harness.
- design ↔ layouts: the stand's screen tree (TaskShell chrome + task body) is owned by layout-templates. A DOM-ancestry component path, if P4 chooses one, reads that tree and must not reshape it.

## Acceptance criteria contributions
- (design) After the author-key markup lands, each lean task booted headlessly in TaskShell resolves the same computed accent, page background, text colour and flight booker invalid/success colours as before the chunk. No `color` / `background-color` / `border-color` difference on any pre-existing stand element (per design-system §Color Palette).
- (design) The chunk's diff to `examples/seven_guis/src/{app.rs,tasks/*.rs}` adds or changes only identity attributes. It adds or edits no style declaration (colour, font, padding, gap, radius or shadow) and no wrapper element (per design-system §Typography / §Spacing / §Border Radius / §Depth Strategy).
- (design) `seven_guis::stand::boot` / `boot_timer` still install the single bundled DejaVu Sans font context with system fonts off (per design-system §Typography **Loading**).
