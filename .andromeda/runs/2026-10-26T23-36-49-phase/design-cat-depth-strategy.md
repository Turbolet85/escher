# design · Depth Strategy — category read

## Nothing in scope
Nearest: (.andromeda/design-system-detail/depth-strategy.md:26) governs the engine's stacking paint order and the hit walk that stops at the padding box of a node paint clips. The chunk only factors `lint.rs::findings` and two table-asserting tests into helpers or table walks, with no change to behaviour, paint, hit-testing, z-index, shadow or any rendered surface. It leaves `blitz-dom`/`blitz-paint` untouched, adds no elevated UI and introduces no depth value, so nothing in this category reaches it.
