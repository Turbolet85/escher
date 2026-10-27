# design · Typography — category read

## Nothing in scope
Nearest: (.andromeda/design-system-detail/typography.md:35) governs the driver `screenshot`'s `fonts-unavailable` refusal when an app's font context resolves no `sans-serif` family (`BaseDocument::can_paint_text`); this chunk does not reach it: it refactors `lint.rs::findings` and two test functions under the complexity ceiling, and it leaves `screenshot`, the font contexts, the stand's bundled DejaVu Sans and every type value unchanged. The lint's own `fonts-unavailable` refusal order is held by the scope, not by this category, which states the rule for `screenshot` alone.
