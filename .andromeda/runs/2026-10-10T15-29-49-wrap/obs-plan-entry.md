
## 2026-10-10-scrolling-box-bounds-and-hit — three engine checks join the no-subscriber census
**Section:** §3 Observability Harness Contract → Logging stack (the headless-stand bullet)
**Change:** the census of checks that install no subscriber gains the engine's three scrolled-box checks — `scrolled_box_client_rect`, `hit_clipped_at_scrolling_box` and `scrolled_box_absolute_position` — none of which reads an env var or declares a shared module. No count in the bullet moves: 15 readers of `common/mod.rs`, five `stand_session_*`, eleven `stand_act_*`, sixteen of the seventeen session-holding checks.
**Why:** three new check files landed and the bullet names its members. The chunk added no log site: the engine's added lines hold no `tracing` call, print or panic, and the command span keeps its eight fields.
**Ref:** .andromeda/runs/2026-10-10T15-29-49-wrap/
