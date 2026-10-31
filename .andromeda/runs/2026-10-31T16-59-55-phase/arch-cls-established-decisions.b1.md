# Classified — arch · established-decisions.b1

## Answers
established-decisions#1.1 · no · dependency pins and patch blocks stay as they are; the chunk adds no dependency and the Stylo, Taffy and Dioxus bumps are out of scope
established-decisions#2.1 · no · the dev and named build profiles; the chunk adds no profile and its tests run under the existing test profile
established-decisions#3.1 · no · release versioning; the chunk ships no release and touches no version
established-decisions#4.1 · yes · the overlay paints through every blitz-paint renderer and the harness's screenshot, whose composite over opaque white after paint bears on how the overlay is painted
established-decisions#5.1 · no · default feature lists; the chunk adds no feature and its accessibility.rs edits stay behind the accessibility feature, gated as S7 already requires
established-decisions#6.1 · yes · headless documents read ColorScheme::Light, which the new default colours and the harness's text_contrast read for SC 1.4.3
established-decisions#7.1 · no · resource loading; the list loads nothing and the user-agent sheet is compiled in, not fetched
established-decisions#8.1 · yes · CSSOM geometry is snapped to 1/64px, which the chunk's reads of an option's client rect (snapshot bounds, visible_region, get_client_bounding_rect) carry
established-decisions#9.1 · yes · :focus-visible always matches false, so the list's focus styling cannot rest on it; pseudo-element content supports only strings, which bears on how a group label is painted
established-decisions#10.1 · yes · the client rect and hit stop at a clipping box's padding box under the overlay, and scroll_into_view writes each scrolling box on the target's chain, which the driver's readings of an option rely on
established-decisions#11.1 · yes · inert and the disabled reading on a form control alone bind option_disabled, is_disabled, the pointer and key gates, and the client rect read under transforms
established-decisions#11.2 · yes · visible_region's clipping through transforms, the non-invertible transform taking no hit, the focus fixup and the hit_inner textual conflict bind the overlay's hit testing and the driver's readings of an option
established-decisions#11.3 · yes · every engine time read goes through DocumentConfig.clock, which the list's reduced motion and typeahead must also use
established-decisions#11.4 · yes · the select model (Enter and Space open nothing, arrows and typeahead on a single select, apply_select_choice, size unread, no popup, a closed option refused covered) is what the chunk ends or restates
established-decisions#11.5 · yes · a multiple select's keys stay as 2026-10-31-engine-debts built them, which the chunk must keep holding
established-decisions#11.6 · yes · a multiple select's pointer, click_select_row, stays as built, which the chunk must keep holding
established-decisions#11.7 · yes · the windowed shell's redraw route carries the list's windowed path, which the chunk's shell and poll loop rely on
established-decisions#12.1 · yes · a main-button pointerdown right after a scroll in a shown scrollbar band dispatches nothing, so a driver click on an option after scroll_into_view needs an advance past the fade
established-decisions#13.1 · yes · paint-tree stacking (z-index auto level, hoisted non-zero contexts, out-of-flow boxes by containing block) and the damage bits bind the overlay's paint order and the redraw when a list opens or closes
established-decisions#14.1 · no · aspect-ratio of replaced elements; the list and the chunk's files involve no replaced element
established-decisions#14.2 · no · standing S2 · scenarios run in both layout modes, and every test runs for incremental in [false, true]
established-decisions#14.3 · no · IME at a focused text input's editor construction; the chunk adds no text input
established-decisions#15.1 · yes · disabled read by presence on a form control alone, the DISABLED state, and interaction state on removed nodes cleared, which option_disabled, is_disabled and the overlay's persistent open state bind
established-decisions#16.1 · yes · inert honoured by presence, hit_inner's answer for it, and the accessibility subtree leaving the tree and snapshot, which the chunk's inert contract and hit testing bind
established-decisions#16.2 · yes · the focus fixup under inert fires no blur or focusout, which the list's focus contract under inert binds
