moved .andromeda/architecture.md:61 apps/browser/Cargo.toml:45-52 → 46-53 «dioxus-native = { workspace = true, feature…» → holds
moved .andromeda/architecture.md:61 apps/browser/Cargo.toml:66 → 67 «nucleo = "0.5"» → holds
moved .andromeda/architecture.md:61 apps/browser/Cargo.toml:74 → 75 «tokio = { workspace = true, features = ["time", "…» → holds
moved .andromeda/architecture.md:61 apps/browser/Cargo.toml:77 → 78 «mimalloc = { version = "0.1.48", optional = true }» → holds
moved .andromeda/architecture.md:61 apps/browser/Cargo.toml:79-80 → 80-81 «[target.'cfg(not(any(target_os = "android",…» → holds
moved .andromeda/architecture.md:61 apps/browser/Cargo.toml:82-83 → 83-84 «[target.'cfg(target_os = "android")'.depend…» → holds
moved .andromeda/architecture.md:80 apps/browser/Cargo.toml:13 → 14 «default = ["hybrid", "cookies", "cache", "screens…» → holds
moved .andromeda/architecture.md:80 apps/browser/Cargo.toml:18-26 → 19-27 «vello = ["dioxus-native/vello"]» → holds
moved .andromeda/security-plan.md:175 apps/browser/Cargo.toml:13 → 14 «default = ["hybrid", "cookies", "cache", "scree…» → holds
moved .andromeda/security-plan.md:175 apps/browser/Cargo.toml:30-31 → 31-32 «cache = ["blitz-net/cache"]» → holds
moved .andromeda/security-plan.md:215 apps/browser/Cargo.toml:55 → 56 «blitz-net = { workspace = true, features = ["ht…» → holds
moved .andromeda/obs-plan.md:9 apps/browser/Cargo.toml:27-29 → 28-30 «log-times = ["log-frame-times", "log-phase-times…» → holds
moved .andromeda/obs-plan.md:9 apps/browser/Cargo.toml:36 → 37 «tracing = ["dioxus-native/tracing", "blitz-html/tracin…» → holds
moved .andromeda/obs-plan.md:39 apps/browser/Cargo.toml:36 → 37 «tracing = ["dioxus-native/tracing", "blitz-html/traci…» → holds
moved .andromeda/obs-plan.md:45 apps/browser/Cargo.toml:27-29 → 28-30 «log-times = ["log-frame-times", "log-phase-time…» → holds
moved .andromeda/obs-plan.md:245 examples/seven_guis/src/lib.rs:13 → 16 «console_error_panic_hook::set_once();» → holds
moved .andromeda/a11y-plan.md:15 apps/browser/Cargo.toml:13 → 14 «default = ["hybrid", "cookies", "cache", "screenshot…» → holds
moved .andromeda/a11y-plan.md:15 apps/browser/Cargo.toml:38 → 39 «accessibility = ["dioxus-native/accessibility"]» → holds

What was read, before the ask (2026-10-09, this wrap): every row at its citing line and at the lines its new number names.
- `apps/browser/Cargo.toml` whole, 84 lines (15 rows): each new number names the line the sentence describes — the default feature set at 14, the renderer features at 19-27, `log-times` and its two halves at 28-30, `cache` and `cookies` at 31-32, `tracing` at 37, `accessibility` at 39, the `dioxus-native` dependency with its six features at 46-53, `blitz-net` with `http2` at 56, `nucleo` at 67, `tokio` at 75, `mimalloc` at 78, the `rfd` target block at 80-81, the `android-activity` target block at 83-84. Every number moved by one: one line was added above line 13 after the citing lines were last written.
- `examples/seven_guis/src/lib.rs` 1-24 (1 row): `console_error_panic_hook::set_once();` stands at 16.
- Citing lines opened: architecture.md 61 · 80, security-plan.md 175 · 215, obs-plan.md 9 · 39 · 45 · 245, a11y-plan.md 15. No other number on them was checked against its file: the tool classes them `unmoved` and this read did not open their targets.
- Refused: none.
