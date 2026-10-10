
## 2026-10-10-upstream-sync-agent-surfaces — the default link rule selects `a[href]`
**Section:** §Color Palette → Core Colors, the "blitz-dom default link" row
**Change:** the row's usage was "Default stylesheet link"; it now says the rule selects `a[href]` — an anchor with no `href` takes neither the colour nor the underline. The value, `rgb(0, 0, 238)`, is unchanged.
**Why:** upstream narrowed the default stylesheet's link rule from `a` to `a[href]`, as the HTML specification styles only links. No stand source holds an `a`, so no stand reading moves; no test is named for the rule.
**Ref:** .andromeda/runs/2026-10-10T01-56-50-wrap/
