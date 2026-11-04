@extract_section name=relevance
partial — the probe renders no new UI, token or visual; only the plan's harness-facing facts (font loading, the virtual clock, MountedData-driven scroll, the agent-facing CLI output) bind its measured runs on the stand
@extract_section name=constraints
- Font-dependent readings on the stand require `blitz-dom` built with the `system-fonts` feature for the blitz-tests crate, else text measures 0x0 and an assertion passes vacuously (per design-system §Typography, Loading; fct-6tyrmhic)
- A driver `screenshot` whose font context resolves no `sans-serif` family is refused `fonts-unavailable`; a probe that captures a shot must have one, never treat that refusal as a reading (per design-system §Typography, Loading; fct-kjaf2qti)
- Settle never advances the `VirtualClock` and `click`/`type`/`press` leave it where it was; only the driver's `advance` moves it (and the app's own time with it). A link (3)/(4) reading of state written by a timer- or task-driven handler must account for that clock (per design-system §Motion, Animation runtime; fct-za7xpowz, fct-vjxch6lk)
- The driver's load wait moves no clock: a served task's timer or animation does not run while an acting step waits on a load (per design-system §Motion, Animation runtime; fct-rxggiikl)
- The stand's Cells grid arrow-key handler calls `MountedData::scroll_to(ScrollBehavior::Instant)` synchronously; whether that path, or a task spawned from a key handler, meets the `RefCell already borrowed` panic at HEAD is research's question (per design-system §Motion detail, This project's values; fct-z2ugu37c)
- `escher-session` / the driver CLI and its `mcp` role keep stdout JSON-only, no colour or ESC byte, exit class `0/1/2/3`; the probe adds nothing to that surface (per design-system §Surface: cli, Component Patterns; fct-or2earye)
@extract_section name=patterns
- Into-view scrolling through the driver is instant in every box and the viewport, made inside one settled step, so a probe reading after a key or scroll step needs no `advance` for scroll motion (per design-system §Motion, This project's values; fct-tnwqvola)
- After a driver `scroll` the overlay scrollbar stays shown until an `advance` passes its fade (~700 ms of the virtual clock), and a `click` in its band lands on the scrollbar, so in a Cells probe the `advance` comes before the `click` (per design-system §Motion detail, This project's values; fct-dxtawno3)
- Animation and transition event data are `unimplemented!()` in dioxus-native-dom; link (2)'s listener-map reading should name those event kinds as unreadable, not as missing (per design-system §Motion detail, Animation runtime; fct-5d7ppgng)
- The dioxus-native surface is launched through `dioxus_native::launch` with `WindowAttributes`; the windowed path is read, not run, for this probe (per design-system §Surface: desktop-native; fct-kdmcvdbg)
@extract_section name=anti_patterns
- Do not draw a verdict from a font- or layout-dependent assertion run without `system-fonts`: it passes vacuously (per design-system §Typography, Loading; fct-6tyrmhic)
- Do not print colour, an ESC byte, or any probed value (a component name, handler, signal value) on the driver's stdout (per design-system §Surface: cli, Component Patterns; fct-or2earye)
@extract_section name=bindings
- design §Typography (system-fonts) ↔ tests harness: the blitz-tests crate's feature set decides whether stand readings in both layout modes are real
- design §Motion (VirtualClock, settle) ↔ tests harness / driver: when a handler's state write becomes readable for links (3) and (4)
- design §Motion detail (MountedData::scroll_to in the Cells handler) ↔ engine (`packages/dioxus-native-dom/src/events.rs`, `dioxus_document.rs` `poll`): the MountedData hazard brought to the founder as a fork
- design §Surface: cli ↔ obs / driver contract: nothing crosses, stdout stays JSON-only
@extract_section name=acceptance
- Every probe check that reads text or layout runs with `blitz-dom`'s `system-fonts` feature on for blitz-tests, and none passes on a 0x0 measure (per design-system §Typography)
- No probe step expects state to move under `click`/`type`/`press`/settle alone; any timer- or animation-driven reading follows an explicit `advance` (per design-system §Motion)
- The driver's stdout stays JSON-only with no colour/ESC byte and no probed value in it; no new field is added (per design-system §Surface: cli)
