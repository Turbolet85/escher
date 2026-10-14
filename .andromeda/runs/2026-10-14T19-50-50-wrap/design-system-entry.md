
## 2026-10-14-stand-a11y-assertions — the stand's accent one step darker, SC 1.4.3 met
**Section:** §Color Palette → Core Colors (the `seven_guis accent` row)
**Change:** the accent is `#3a5ce5`, hover `#2a4cd3`, active `#1f3fc1` in every task CSS that declares it, its non-text uses moving with it and its three `rgba` uses at `rgba(58, 92, 229, …)`, alpha kept; was `#4a6cf7` / `#3a5ce5` / `#2a4cd3`. White button text on the five accent controls reads 5.47, a pass (was 4.39, a fail); on the hover shade 6.83 and on the active shade 8.27, reached states of `stand_contrast`.
**Why:** the operator's F5, "One step darker, stand-wide", by question dialog — a stand change and no engine change, the hex literals staying where each task declares them (the token set is NOT YET MEASURED).
**Ref:** .andromeda/runs/2026-10-14T19-50-50-wrap/
