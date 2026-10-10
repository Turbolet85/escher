
## 2026-10-10-snapshot-states — the snapshot model reads selected, pressed and invalid; the driver's node carries them
**Section:** §Standard Contracts → Dioxus DOM bridge · §Standard Contracts → Driver session
**Change:**
- Dioxus DOM bridge: was "`NodeState` carries `enabled`, `checked`, `value` and `focused`"; now seven fields in the order `enabled` · `checked` · `value` · `selected: Option<bool>` · `pressed: Option<bool>` · `invalid: bool` · `focused`, no new public type. The three are the accessibility node's own, read at the one construction site and never parsed from the element: `is_selected()`; `toggled()` with only `Toggled::True` reading `Some(true)`; `invalid().is_some()` — a boolean model with two limits, a mixed toggle reading not pressed and a grammar or spelling error reading invalid, the node keeping the full value. On the stand each reading equals its accessibility node's.
- `Snapshot::to_text`: the fixed token order gains `selected` / `unselected` · `pressed` / `unpressed` · `invalid` (only when true) after `value=` and before `focused`.
- Measured sizes: the boot text was Counter 755 · FlightBooker 1269 · Timer 1259 · Crud 2034 bytes, now 755 · 1287 · 1259 · 2049; the hosted `snapshot` line was 832 · 1,375 · 1,368 · 2,187, now 832 · 1,393 · 1,368 · 2,202.
- Driver session: `NODE_FIELDS` was nine words (`[&str; 9]`), now twelve — `selected` · `pressed` · `invalid` between `value` and `focused`. In JSON, `parent`, `enabled`, `checked`, `value`, `selected` and `pressed` are left out when absent (was the first four); `invalid` is always written.
- The help of `changed` and of `snapshot`'s `text` were quoted as the words that stood before 2026-10-10-driver-cli; both are restated and quoted as they now read — each names the state words and says the two limits. The crate docs gain one sentence saying the same.
**Why:** the founder brought the three states into 0.1.0 as their own chunk (2026-10-10): a snapshot reads which control is selected, pressed or invalid from what the app states in its markup, and an agent learns the words and the limits from the tool.
**Kept:** a boolean model, not a tri-state `pressed` or a kind on `invalid`, and no help per node key — the operator's answers at the chunk's plan forks; the per-key help is owned on the route.
**Ref:** .andromeda/runs/2026-10-10T17-47-57-wrap/
