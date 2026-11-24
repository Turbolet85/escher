# Fan-out results — 2026-11-23-circle-drawer-on-a-canvas (wrap 2026-11-24T02-04-06-wrap)

architecture: add 5 · set 2 · retire 0 · none 0 · ops-architecture.md
security-plan: add 1 · set 0 · retire 0 · none 1 · ops-security-plan.md
design-system: add 6 · set 0 · retire 0 · none 0 · ops-design-system.md
layout-templates: add 5 · set 2 · retire 0 · none 1 · ops-layout-templates.md
test-plan: add 5 · set 0 · retire 0 · none 0 · ops-test-plan.md
obs-plan: add 1 · set 0 · retire 0 · none 1 · ops-obs-plan.md
a11y-plan: add 4 · set 0 · retire 0 · none 0 · ops-a11y-plan.md

Homes (check 2): the click synthesis on a custom-widget target — architecture decision/widget-click-synthesis (the engine decision) and security-plan decision/custom-widget-click-synthesis (what crosses), each named by the plan's expected amendments; canvas text on whole device pixels — test-plan decision/canvas-text-on-whole-device-pixels; the click point against the content box — architecture decision/widget-click-point-content-box; the absent keyboard path into a canvas — a11y-plan absence/canvas-keyboard-path, itself then read not_master type 9 by fit, so no master holds it: the route's «Canvas keyboard path» entry does; Circle Drawer's circles painted on a canvas — layout-templates decision/circle-drawer-circles-painted-on-canvas (the surface), a11y-plan decision/circle-drawer-canvas-pixels-only (what an assistive technology reads); canvas content crossing as shot pixels — security-plan decision/canvas-content-crosses-as-shot-pixels, standing.

Boundary widening (check 1, rul-izni57qv): read not met for the click synthesis — a custom widget is set only by app code (a Rust `CustomWidgetAttr`), the pointer events already reached it, and no carrier, verb, field, cause, exit, log field or span is added; it is a direct engine change for every document behind `custom-widget`, ruled by the founder at F2. No answer card.

Disproved claims (check 6): the plan's «three lock lines» — no master site (sweep `lockdeps` control unfired), absorbed by plan revision 1 (`.andromeda/runs/2026-11-23T22-07-35-phase`), DISPOSED; the plan's centre `350,200` — no master site, the plan's implementation note alone, DISPOSED (the content-box fact lands as architecture decision/widget-click-point-content-box).

Expected amendments (check 5): architecture click synthesis — ops-architecture.md decision/widget-click-synthesis; security-plan — ops-security-plan.md decision/custom-widget-click-synthesis; architecture contract/headless-stand/boot — ops-architecture.md; layout-templates stand-lints-clean-at-boot and circle-canvas — ops-layout-templates.md; test-plan decision/canvas-shots-cross-os-by-digest gains Circle Drawer — raised by main, ops-main-test-plan.md; obs-plan §6 silent checks — not carried: the standing rows already bind them (the proposer's @row_none type 5), no row moves.

ops-architecture.md
decision/click-focuses-button · apply — check 1: no rule; the report's click synthesis runs no click walk and clears no focus, so the row's exception list must name a widget target · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-23, inputs#I2: «A: движок создаёт click (Recommended)»
contract/headless-stand/boot · apply — check 5: the plan's expected amendment; Circle Drawer boots in process, served by no role (F1 A) · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-23, inputs#I2: «A: не сейчас (Recommended)»
decision/widget-click-synthesis · apply — check 5: the plan's expected amendment, settled by the founder's F2 A, not a boundary widening (above) · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-23, inputs#I2: «A: движок создаёт click (Recommended)»
decision/canvas-text-whole-pixel-glyph-origins · reject — check 2: the fact's home is test-plan decision/canvas-text-on-whole-device-pixels
decision/widget-click-point-content-box · apply — check 1: no rule; a measured fact the report carries (Decisions & corrections), no other master's home
decision/canvas-placement-at-click-centre · apply — check 1: no rule; the stated limit until `drag` (v040-12), on F4 A, state provisional · word: delegate escher overseer, 2026-11-23, inputs#I2
absence/canvas-keyboard-path · reject — check 2: the fact's home is a11y-plan absence/canvas-keyboard-path

ops-security-plan.md
decision/custom-widget-click-synthesis · apply — check 5: the plan's expected amendment, the founder's F2 A, not a boundary widening (above) · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-23, inputs#I2: «A: движок создаёт click (Recommended)»

ops-design-system.md
decision/canvas-text-glyph-origins-whole-pixels · reject — check 2: the fact's home is test-plan decision/canvas-text-on-whole-device-pixels
decision/widget-element-click-synthesis · reject — check 2: the fact's home is architecture decision/widget-click-synthesis
decision/circle-drawer-circles-painted · reject — check 2: the fact's home is layout-templates decision/circle-drawer-circles-painted-on-canvas
contract/custom-widget-element/paint-box · reject — check 2: the fact's home is architecture decision/widget-click-point-content-box
absence/canvas-keyboard-path · reject — check 2: the fact's home is a11y-plan absence/canvas-keyboard-path
decision/circle-drawer-canvas-colours-in-code · apply — check 1: D-design-tokens reads `hardcoded✗` on the canvas colours; the report states them as the removed CSS held them, an exception the palette records

ops-layout-templates.md
decision/stand-lints-clean-at-boot · apply — check 5: the plan's expected amendment; Circle Drawer reads lint 0 at boot in both modes (the report's Outcome)
contract/stand-element-ids/circle-canvas · reject — check 7: not_master type 4 — the element kind, role and name retell how the canvas is built; the standing promise (a stable author id of the canvas) stays true, so nothing follows
decision/circle-drawer-circles-painted-on-canvas · apply — check 1: no rule; the circles painted from one `data-circles` attribute, on the delegate's word, state provisional · word: delegate escher overseer, 2026-11-23, inputs#I2
decision/widget-content-placed-against-content-box · reject — check 2: the fact's home is architecture decision/widget-click-point-content-box
decision/canvas-text-glyph-origins-whole-pixels · reject — check 2: the fact's home is test-plan decision/canvas-text-on-whole-device-pixels
decision/circle-drawer-diameter-label · reject — check 7: not_master type 4 — how the widget paints its label, no reason stated; the code holds it (F3 A stays the delegate's word in the report)
decision/widget-element-click-synthesis · reject — check 2: the fact's home is architecture decision/widget-click-synthesis

ops-test-plan.md
decision/canvas-text-on-whole-device-pixels · apply — check 1: no rule; the whole-pixel snap and the stated limit «Canvas text across hosts» of the operator pass, the delegate's word, state provisional · word: delegate escher overseer, 2026-11-24, inputs#I5
decision/widget-element-click-reaches-app · reject — check 2: the fact's home is architecture decision/widget-click-synthesis
contract/stand-boot/boot-circle-drawer · apply — check 1: no rule; one item per stand boot beside the standing `stand::boot*` items, served by no role (F1 A) · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-23, inputs#I2: «A: не сейчас (Recommended)»
decision/canvas-circles-read-from-attribute-and-pixels · reject — check 2: the fact's home is layout-templates decision/circle-drawer-circles-painted-on-canvas
decision/click-point-against-content-box · reject — check 2: the fact's home is architecture decision/widget-click-point-content-box

ops-obs-plan.md
decision/canvas-content-crosses-as-shot-pixels · reject — check 2: the fact's home is security-plan decision/canvas-content-crosses-as-shot-pixels

ops-a11y-plan.md
decision/widget-click-synthesis · reject — check 2: the fact's home is architecture decision/widget-click-synthesis
decision/circle-drawer-canvas-pixels-only · apply — check 1: no rule; what an assistive technology reads of the canvas (F5 A), state provisional · word: delegate escher overseer, 2026-11-23, inputs#I2
absence/canvas-keyboard-path · reject — check 7: not_master type 9 — an owed item the route holds («Canvas keyboard path», a later entry)
absence/canvas-paint-contrast-reading · reject — check 7: not_master type 5 — a coverage inventory, no statement of a missing capability; the sidecar holds it

ops-main-test-plan.md
decision/canvas-shots-cross-os-by-digest · apply — check 5: the plan's expected amendment, Circle Drawer's canvas element shot joining the table on the operator's P5 direction (route WATCH `cry-vnddnt4v`)
