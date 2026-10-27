# design extract

## No domain coverage
A behaviour-preserving complexity refactor of `packages/escher-driver/src/lint.rs::findings` and two table-asserting tests. It adds or changes no rendered surface, colour, type, spacing, depth, radius, motion or icon. The lint's `contrast` reading and the `escher-session` stdout form (design-system §Surface: cli › Component Patterns) are held unchanged by the scope's own same-output boundary, so this plan adds no requirement beyond it.
