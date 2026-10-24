# obs · Error Capture & Reporting — category read

## Nothing in scope
Nearest: (.andromeda/obs-plan.md:205) escher-telemetry's chaining panic hook in `escher-session`: one ERROR event at `escher_telemetry::panic`, `panic.payload` redacted, then std's default hook printing the raw message to stderr. That is the binary whose stderr the sweep measures for the new verbs (`lint`, `part`, `--record`). The chunk does not reach the hook: it neither changes the hook nor adds a panic path to the escher-session host or its `mcp` role, and a passing `host_log` or `mcp_log` run never fires it. The category file (.andromeda/obs-plan-detail/error-capture-reporting.md:8-13) records only that no error-reporting client or service exists, and the chunk adds none.
