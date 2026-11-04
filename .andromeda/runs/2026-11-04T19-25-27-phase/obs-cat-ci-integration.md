@carried n=1 label=obs-plan/ci-integration fact=fct-b7ujkv3r
why: A probe check run in a ci.yml leg must keep that leg's log free of user data, so the probe prints no component name, handler or signal value that could land in an uploaded failure log.
quote: each ci.yml leg's merged stdout+stderr, written by `ci-leg.sh` to `target/ci-logs/{leg}.log` (matrix: `target/ci-logs/matrix-{platform}.log`), truncated at the leg's start; uploaded only on failure as artifact `ci-log-{job id}` (`if-no-files-found: ignore`), kept 7 days, from `target/ci-logs/` alone; unscrubbed build output (no user data — §8)
@carried n=2 label=obs-plan fact=fct-5q6m2wwa
why: The probe must add no CI artifact: the fork's CI keeps uploading exactly its two artifacts, neither holding user data.
quote: the fork's CI uploads two artifacts — a leg's merged log on failure and the coverage report on success, each kept 7 days, neither holding user data.
@carried n=3 label=obs-plan fact=fct-4rkn3jky
why: Whatever state the probe's runs on the stand leave (a session or recording directory under `target/`) must stay local and be uploaded by no CI leg.
quote: Local state stays local, uploaded by no CI leg: the agent-run and cold-agent state under `target/`, a session's state directory (`0700`, its socket alone, no log, event or screen content) and a recording directory (`0700`, refused when open to its group or others, holding call content at rest, never a log or event line); two kept checks are committed in git, holding no typed text, host path or secret.
@carried n=4 label=obs-plan/ci-integration fact=fct-3pafqnmo
why: If a probe check starts a session, its state directory must sit under `target/tmp/`, `0700`, holding the socket alone, with no log, event or screen content written into it, and be uploaded by no CI leg.
quote: under `target/tmp/` — created `0700` by the host, holding the socket file `session.sock` alone while a session is up and nothing after `stop` or after the host's idle expiry, which ends it exactly as `stop` does; no log, event or screen content is written into it; local only, uploaded by no CI leg
@carried n=5 label=obs-plan/ci-integration fact=fct-pkacqyww
why: The probe must commit no kept check of its own runs: the two committed Counter checks stay the only committed ones.
quote: no kept check of the live run committed — the two committed Counter checks (below) the only committed ones
@carried n=5 run=2 label=obs-plan/ci-integration fact=fct-7467zgun
quote: `tests/blitz-tests/checks/counter-cli.json` and `counter-mcp.json`, two kept checks in git (mode `100644`, a public repository)
@binding n=1
**Per-leg CI log carries no user data ↔ the chunk scope's «nothing crosses» boundary and obs-plan §8**: a probe check under `tests/blitz-tests/` or `examples/seven_guis/` that runs in a CI leg must print nothing a probe reads (component names, handlers, signal values), so the failure-uploaded `ci-log-{job id}` keeps holding build output only (obs-plan/ci-integration fct-b7ujkv3r) (obs-plan fct-5q6m2wwa)
@binding n=2
**Local state and committed checks ↔ the stand checks' replay (`stand_replay`, seven_guis' freshness check)**: the probe's sessions and recordings stay under `target/`, uploaded by no leg, and add no file beside `counter-cli.json` and `counter-mcp.json` under `tests/blitz-tests/checks/` (obs-plan/ci-integration fct-3pafqnmo) (obs-plan/ci-integration fct-pkacqyww) (obs-plan fct-4rkn3jky)
