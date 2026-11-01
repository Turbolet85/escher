proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 15
    section: "## Surface: desktop-native › ### Tokens (platform-specific)"
    change: "New list item after line 15: - A single select's open list paints last, past the blend layers `render_element` pushes since `mix-blend-mode` and `isolation` paint, `paint_select_list`'s call holding no hunk; this is a reading of the call, no test paints a blended element to a frame (packages/blitz-paint/src/render.rs:485-506; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)"
    sidecar: "surface.md, desktop-native-tokens: the open select list paints last, past the blend layers that now paint; read, with no frame test."
    rationale: "The report's design criterion reads the list painting last, past the blend layers, `paint_select_list`'s call holding no hunk (report.md:147), and `render_element` newly pushes a viewport-wide blend layer (report.md:15); the Coverage row marks no frame test of blending (report.md:104), and line 15 holds the overlay's path with no word on its order against these layers."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:147
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 19
    section: "## Surface: desktop-native › ### Tokens (platform-specific)"
    change: "New list item after line 19: - A focused select whose anonymous block was dropped when its container lost its box, by `hidden`, by an inline `display:none` or by CSS alone, keeps focus and takes a key with no panic, and the key still moves its choice (packages/blitz-dom/src/document.rs:1493-1503; tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:136-160; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)"
    sidecar: "surface.md, desktop-native-tokens: the select's state past a dropped anonymous block; focus stays, no panic, a key still moves its choice."
    rationale: "The panic's fix gives the select a state lines 14-22 do not hold: focus stays and the key still acts, for `hidden`, `style=display:none` and CSS alone (report.md:23), with a key still moving a select's choice past its dropped block (report.md:114); the limit's own sentence is a11y-plan §5's (report.md:58), and this item records the component's state only."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:114
note: D-design-tokens holds, every Coverage `tokens` flag reading n/a (report.md:102-108), and no passage of this file describes a Retired item as standing (report.md:33).
