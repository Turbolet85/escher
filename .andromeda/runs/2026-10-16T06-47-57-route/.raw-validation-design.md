# Design validation — route draft

## No suggestions
The draft covers the design domain. escher-0.2.0 is a brownfield version. Both design plans only record measurements of existing upstream code. Neither records any design intent (§Brand Identity, §Anti-Patterns and §Design Decisions Log in design-system say "NO RECORDED INTENT", and so does §Decisions Log in layout-templates). The draft also introduces no new surface and no token-consuming UI, so there are no missing bootstrap chunks to insert.
- **Bootstrap items present:**
  - §Color Palette / §Typography / §Spacing / §Border Radius / §Motion / §Iconography → no tokens-bundle chunk is expected. The values already live in shipped code (blitz `DEFAULT_CSS`, `apps/browser/assets/browser.css`, the stand's measured colours), and no chunk in the draft adds tokens or consumes them.
  - §Brand Identity → no recorded intent, so no brand wire is expected.
  - §Component Patterns → no new UI component primitives are in scope. The CLI/MCP answer-line pattern under §Surface: cli (one uncoloured JSON line, no ESC byte, exit class) is already enforced by the shared reader and `Speaker` checks. The new driver commands in "Visual lint", "Select by id" and "Large screen in parts" (Epoch 4) inherit that pattern.
- **Sequencing deps satisfied:**
  - Tokens before consumers → nothing in the draft consumes tokens.
  - Surface bootstrap before features → all four surfaces (desktop-native, mobile-native, cli, web-spa) already exist from upstream and 0.1.0.
  - "One virtual clock" (Epoch 3) comes before "Visual lint" (Epoch 4). That order matches design-system §Motion, which lists the time sources a deterministic shot depends on: the `FADE_DELAY`/`FADE_DURATION` scrollbar fade, 300 ms smooth scrolls and CSS transitions. The chunk's scope ("every time a headless run reads, scrollbar fade included") covers them.
- **Coverage:**
  - Per-surface (layout-templates §Surface: desktop-native, §Primary screens) → "Select by id", "Load wait" and "Large screen in parts" are proven "on a fixture". That keeps the stand's TaskShell mount and its stable author-id layout unchanged ("no class, style, wrapper or order" added).
  - §Surface: cli (§Primary screens `escher-session`) → the new verbs reach it through the existing driver CLI/MCP roles. "Capability statement" (Epoch 5) records the stated limits.
  - Layout templates for mobile-native and web-spa → untouched by 0.2.0, so no scaffold chunk is needed.
