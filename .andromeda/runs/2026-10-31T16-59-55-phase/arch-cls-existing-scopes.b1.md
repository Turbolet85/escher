# Classified — arch · existing-scopes.b1

## Answers
existing-scopes#1.1 · no · the accesskit_xplat platform adapters, which the chunk changes none of; it writes through the tree's builder only
existing-scopes#1.2 · no · the debug timer under feature `enable` is not read by any path the list touches
existing-scopes#1.3 · no · the crate's module list only; the chunk adds no module, and the event and NodeId types it uses already exist
existing-scopes#1.4 · yes · the chunk edits node/select, events and accessibility and reads clock and config, and the module list with its cfg(test) child modules places its code
existing-scopes#1.5 · no · stylesheet editing and getComputedStyle are not reached; the colour default is a compile-time sheet read through config
existing-scopes#1.6 · no · form owner reset, form submission, iframe loading and query selectors are untouched; the select's form data sits in the bridge
existing-scopes#1.7 · yes · hit testing under the overlay, hover, focus and active, and the CSSOM View geometry an open list's options read all sit here
existing-scopes#1.8 · yes · accessibility.rs is reached: the select's open state and option and group names are written by build_accessibility_tree; debug printing is not
existing-scopes#1.9 · yes · keyboard.rs and pointer.rs are reached, and the focus event order and the key and pointer gates for a list's options run through these submodules
existing-scopes#1.10 · yes · visible_region and scroll_into_view are the driver's readings of an option in an open list
existing-scopes#1.11 · yes · the UA colour default and the headless ColorScheme::Light reading resolve through Stylo's Device management and the sheet's styling
existing-scopes#1.12 · yes · the open state references nodes over versioned NodeIds and must be cleared or retargeted before a removed node is freed
existing-scopes#1.13 · yes · the open list's layout, damage on open and close, and the rule that state never references layout-generated boxes sit in layout
existing-scopes#1.14 · yes · node::select holds the crate-private select rules, the apply_select_choice path and the single select's choice rules
existing-scopes#1.15 · no · HTML parsing into the tree is not changed; the optgroup and label already parse into the tree
existing-scopes#1.16 · no · blitz-net loads nothing for the list, as the task's Not needed names
existing-scopes#1.17 · yes · blitz-paint lib.rs and render.rs are reached: paint_scene, the overlay's layer and the layer cap
existing-scopes#1.18 · no · the chunk adds no script surface, so Boa, script timers and the module loader are not reached
existing-scopes#1.19 · yes · the shell's redraw request, the redraw and poll loop and the platform tree refresh carry the list when a list opens or closes
existing-scopes#1.20 · no · the facade re-exports whole crates by module; the chunk adds no public type through a path of its own
existing-scopes#1.21 · no · the Stylo-to-Taffy wrapper and conversions are not reached; the overlay paints from resolved layout
existing-scopes#1.22 · yes · the windowed path shows the same list through the engine overlay and builds its documents Parallel through its config and renderer
existing-scopes#1.23 · yes · snapshot.rs is edited; element_id, actionable and the bridge's input and change events carry an open list's options
existing-scopes#1.24 · yes · screenshot, screenshot_rect, text_contrast and settle are checks the chunk re-measures, and none of them may move time
existing-scopes#1.25 · no · the chunk adds no log line, target or subscriber; the formatter's allowlist and field scrub are unchanged
