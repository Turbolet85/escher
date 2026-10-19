- line: .andromeda/obs-plan.md:69
  old: «the reader takes the first word `mcp` alone as a role,»
  new: «the reader takes the first word `mcp` as a role, alone or with one `--record <dir>`, whose directory is checked before `serve_mcp` serves (a refusal: status 3, its fixed message on stderr, stdout empty),»
- line: .andromeda/obs-plan.md:70
  old: «`serde_json` reading requests and writing nothing,»
  new: «`serde_json` reading requests and kept checks and writing nothing,»
- line: .andromeda/obs-plan.md:70
  old: «for the five `stand_session_*` checks,»
  new: «for the six `stand_session_*` checks,»
- line: .andromeda/obs-plan.md:70
  old: «for the module twenty-three checks read — four of the five `stand_session_*`,»
  new: «for the module twenty-four checks read — five of the six `stand_session_*`,»
- line: .andromeda/obs-plan.md:70
  old: «so `host_log` is no longer the one check that reads a sink-installing binary at `trace`; the seven share the module»
  new: «so `host_log` is no longer the one check that reads a sink-installing binary at `trace`; `recording` spawns the same binary on `start <app> --session <dir> --record <dir>` and in its `mcp --record <dir>` role, reading its answers and its kept checks into its assertions only; the eight share the module»
- line: .andromeda/obs-plan.md:114
  old: «the `start` command's host spawn, and the host's idle expiry, which like a `stop` writes no record»
  new: «the `start` command's host spawn, the recording — `Session::recording_into`, `Session::recording_failed`, the recorder, `Check::from_bytes` and the wire's `recording-failed` reply — and the host's idle expiry, which like a `stop` writes no record»
- line: .andromeda/obs-plan.md:183
  old: «`mcp` followed by anything else usage (status 2, stdout empty), the usage line gaining `; mcp role: mcp` after the host role;»
  new: «`mcp` followed by anything but one `--record <dir>` usage (status 2, stdout empty), as is a `--record` anywhere else, twice or with no value once the call passed `validate`, the usage line gaining `; mcp role: mcp [--record <dir>]` after the host role, with `start <app> [--record <dir>]` and `host role: serve <app> --session <dir> [--record <dir>]`; a recording directory open to its group or others ends `start` with status 3 and nothing spawned and `mcp --record` with status 3, its fixed message on stderr and stdout empty, the two recording session errors ending status 3 like the rest;»
- line: .andromeda/obs-plan.md:188
  old: «its wire (`v2`) carries `hello`, `stop` and `call`,»
  new: «its wire (`v3`) carries `hello`, `stop` and `call` — a call after which the host's recording ended answered by the one fixed reply `recording-failed` —,»
