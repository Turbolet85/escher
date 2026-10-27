group: architecture+layout-templates
read: .claude/docs/services/seven_guis.md · 44 · «- `.claude/rules/a11y.md` (the stand is in its paths) · `.cl»

- leaf: .claude/docs/services/seven_guis.md:24   class: omission
  leaf-text: «white button text on it 5.47), text `#1a1a1a`;»
  source: .andromeda/layout-templates.md:16 «the stand accent `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`) in every task CSS that declares it, on the page `#f5f5f5` with text `#1a1a1a`, its non-text uses taking the same shade»
  replace-with: «white button text on it 5.47, 6.83 on its hover shade `#2a4cd3` and 8.27 on its active shade `#1f3fc1`, each a reached colour state of `stand_contrast`; its non-text uses take the same shade, its three `rgba` uses reading `rgba(58, 92, 229, …)`), text `#1a1a1a`;»

- leaf: .claude/docs/services/seven_guis.md:24   class: omission
  leaf-text: «success `#ebf8ee` family.»
  source: .andromeda/layout-templates.md:17 «seven_guis stand depth — the cards use `0 2px 8px rgba(0, 0, 0, 0.08)` and the circle dialog uses 0.10»
  replace-with: «success `#ebf8ee` / `#68d391` / `#276749`; cards shadow `0 2px 8px rgba(0, 0, 0, 0.08)` (the circle dialog 0.10), cards pad 24px 32px with a 16px gap, Home pads 48px 32px 64px; radius untokenized — cards 8px, buttons 6px, inputs 4px, task cards 6px, tags 3px — the stand's colour, shadow, spacing and radius held on layout-templates' desktop-native surface, out of design-system's product surface (PROVISIONAL, owner F2).»

- leaf: .claude/docs/services/seven_guis.md:19   class: omission
  leaf-text: «`font_ctx()` (bundled DejaVu Sans, system fonts off)»
  source: .andromeda/architecture-detail/standard-contracts.md:127 «`font_ctx()` registers it for every generic family with system fonts off, on native; every stand boot holds it through `HarnessOptions.font_ctx`, `boot_cells` and `boot_load` included»
  replace-with: «`font_ctx()` (bundled DejaVu Sans, the stand's one font and its typography, registered for every generic family, system fonts off — every stand boot holds it through `HarnessOptions.font_ctx`, `boot_cells` and `boot_load` included)»
