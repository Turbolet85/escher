## 2026-10-07-driver-session — the `escher-session` host: a cli entry and a third consumer of the TaskShell mount
**Section:** §Surface: cli → Primary screens · §Surface: desktop-native → Primary screens
**Change:**
- cli: new entry `escher-session <counter|flight-booker|timer|crud> <state-dir>` — a closed two-argument argv; any other argv prints the usage line to stderr and exits 2 with nothing booted and no state created; exit 0 after its session is stopped, 1 with an error's fixed message; nothing on stdout; stderr lines carry `service.name=seven_guis`; unix only in 0.1.0.
- desktop-native: the seven_guis bullet names the session host as a third consumer of the `task_in_shell` mount, after the windowed app and the headless stand checks — it boots one lean task through `seven_guis::stand` only and adds no markup, id, class, style, wrapper or order; a held instance reads as a fresh boot and the mount and pinned viewport hold after commands.
**Why:** the chunk added the host binary; no screen, markup or viewport value changed.
**Ref:** .andromeda/runs/2026-10-07T06-51-52-wrap/
