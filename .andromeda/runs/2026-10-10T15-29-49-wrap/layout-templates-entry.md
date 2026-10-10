
## 2026-10-10-scrolling-box-bounds-and-hit — what a scrolled list reads, and Home's scrolling box
**Section:** §Surface: desktop-native → IA notes (the scrolling-boxes bullet) · §Surface: desktop-native → Primary screens (the seven_guis Home and TaskShell bullet)
**Change:**
- IA notes: gains what `crud-list` reads once its content is scrolled — the same four figures, unnamed in that scroll's diff, and a driver `click` naming the list lands on the row at its centre at 12 Creates and at 14. Nothing the bullet said is retired: the last row is still refused `off-screen` until a `scroll` brings it in.
- Primary screens: "Home is a centered 640px column of task cards" now says the column stands inside a box that scrolls — at the stand's 800 × 600 viewport, Home unscrolled, `#home` is 600 high and holds six of the seven cards; the seventh, Cells, lies at 615 to 684, below the box, and shows and is hit only once scrolled into view. The windowed stand is not measured.
**Why:** the bounds reader and the hit were fixed in the engine, so the list's own readings became true ones. The Home reading was found when a standing check that clicked the Cells card without scrolling went red on the fixed hit: the master had not said that the cards do not all show. No stand markup, id, class or style moved.
**Ref:** .andromeda/runs/2026-10-10T15-29-49-wrap/
