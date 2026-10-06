
## 2026-10-06-accessibility-tree-identity — stand inputs named by attributes alone
**Section:** §Surface: desktop-native → Primary screens (seven_guis Home and TaskShell) · citations re-pointed
**Change:**
- The six task inputs are named by attributes alone: `for` on the existing labels of `timer-duration`, `crud-filter`, `crud-name` and `crud-surname`; `aria-label` "Departure date" / "Return date" on `flight-start` / `flight-return-date`. No element, class, style or order changed.
- 4 citations re-pointed by the measured line map: 1 into `flight_booker.rs` (+1 from old line 82, +2 from old 88) and 3 into dioxus-native-dom `dioxus_document.rs` (+2 from old line 18).
**Why:** the accessibility-tree chunk needed a non-empty name on every stand input without touching the stand's structure; its markup probe confirmed the files differ from the base only by those attributes.
**Ref:** .andromeda/runs/2026-10-06T12-55-34-wrap/
