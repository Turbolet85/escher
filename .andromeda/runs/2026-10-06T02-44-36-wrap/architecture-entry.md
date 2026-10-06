
## 2026-10-06-headless-stand — headless stand surface, HarnessOptions fields, seven_guis edges, disabled keyed two ways
**Section:** §Stack and Technologies → Testing · §Established Decisions → DOM semantics · §Conventions → Manifests · §Standard Contracts → Test harness · Headless stand (new bullet) · Dioxus DOM bridge · §Occupied Resources → Names · §Infrastructure Patterns → Deployment model · §Existing Scopes → blitz-tests · seven_guis
**Change:**
- Test harness: `HarnessOptions` has eight public fields — `font_ctx` → `DocumentConfig.font_ctx` and `incremental` → `DocumentConfig.incremental` added, `Default` leaving both `None`.
- New Headless stand contract: native-only `seven_guis::stand` — `LeanTask` (Counter, FlightBooker, Timer, Crud), pinned 800 × 600 · scale 1.0 · Light, `font_ctx()` (bundled DejaVu Sans, system fonts off), `options(incremental)` (offline, `net_provider: None`), `boot` / `boot_timer` (fresh `VirtualDom` per boot, TaskShell via `app::task_in_shell`, a `TimerTicks` context); `app::Task` public, `TaskShell` private; crate-root `DEJAVU_SANS` shared with the wasm entry; `TimerTicks` replaces the timer's 100 ms delay when in context. No telemetry, env read or `blitz_net` in the stand.
- Dioxus DOM bridge: was "falsy `checked` clears"; now a falsy `checked` or `disabled` clears, every other boolean attribute is still written as `"false"`.
- DOM semantics: was "`disabled` read as a parsed boolean"; now focusability parses it as a bool while the DISABLED state (`:disabled`) and the click target key on presence — `disabled="false"` matches `:disabled` yet stays focusable.
- Names / Testing / Existing Scopes: `seven_guis` is a `[workspace.dependencies]` path entry with default features; edges blitz-tests → seven_guis (dev) and seven_guis → blitz-test-harness, blitz-traits (native); native `dioxus-native` features `system-fonts` + `woff`; the `stand` module and the stand checks + falsy-`disabled` test rows.
- Manifests: the `default-features = false` convention gains the seven_guis exception (dioxus-native refuses to compile with no renderer).
- Deployment model: seven_guis' DejaVu bytes are the shared crate-root const, compiled on native too.
**Why:** the headless stand chunk added the stand boot, the harness options it needs and the crate edges; the falsy-`disabled` engine fix is a widening on the delegate overseer's word, 2026-10-06, PROVISIONAL (upstreamable). Trap: a presence read of a Dioxus boolean attribute other than `checked` / `disabled` still reads `"false"` as set.
**Ref:** .andromeda/runs/2026-10-06T02-44-36-wrap/
