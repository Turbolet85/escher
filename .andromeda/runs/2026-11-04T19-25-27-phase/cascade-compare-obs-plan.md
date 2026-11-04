@compare side=phase doc=obs-plan marker=2026-11-04-dioxus-runtime-probe e_items=42 held=12 lost_classify=5 lost_strong=2 beside=- cascade_only=1 listed=4 body=17 unplaced=0 held_ids=error-capture-reporting#1.4,log-coverage#8.1,log-coverage#8.9,pii-scrubbing-compliance#1.1,pii-scrubbing-compliance#2.1,span-trace-coverage#1.1,span-trace-coverage#1.10,span-trace-coverage#2.1,telemetry-strategy#1.1,telemetry-strategy#2.1,telemetry-strategy#3.2,telemetry-strategy#3.3 only_ids=span-trace-coverage#3.1
@compare_item n=1 part=lost how=classify entity=ci-integration#1.3 label=obs-plan/ci-integration fact=fct-b7ujkv3r
@carried n=1 label=obs-plan/ci-integration fact=fct-b7ujkv3r
why: A probe check run in a ci.yml leg must keep that leg's log free of user data, so the probe prints no component name, handler or signal value that could land in an uploaded failure log.
quote: each ci.yml leg's merged stdout+stderr, written by `ci-leg.sh` to `target/ci-logs/{leg}.log` (matrix: `target/ci-logs/matrix-{platform}.log`), truncated at the leg's start; uploaded only on failure as artifact `ci-log-{job id}` (`if-no-files-found: ignore`), kept 7 days, from `target/ci-logs/` alone; unscrubbed build output (no user data — §8)
@compare_item n=2 part=lost how=classify entity=ci-integration#1.6 label=obs-plan/ci-integration fact=fct-3pafqnmo
@carried n=4 label=obs-plan/ci-integration fact=fct-3pafqnmo
why: If a probe check starts a session, its state directory must sit under `target/tmp/`, `0700`, holding the socket alone, with no log, event or screen content written into it, and be uploaded by no CI leg.
quote: under `target/tmp/` — created `0700` by the host, holding the socket file `session.sock` alone while a session is up and nothing after `stop` or after the host's idle expiry, which ends it exactly as `stop` does; no log, event or screen content is written into it; local only, uploaded by no CI leg
@compare_item n=3 part=lost how=classify entity=ci-integration#1.9 label=obs-plan/ci-integration fact=fct-pkacqyww
@carried n=5 label=obs-plan/ci-integration fact=fct-pkacqyww
why: The probe must commit no kept check of its own runs: the two committed Counter checks stay the only committed ones.
quote: no kept check of the live run committed — the two committed Counter checks (below) the only committed ones
@carried n=5 run=2 label=obs-plan/ci-integration fact=fct-7467zgun
quote: `tests/blitz-tests/checks/counter-cli.json` and `counter-mcp.json`, two kept checks in git (mode `100644`, a public repository)
@compare_item n=4 part=lost how=classify entity=metric-coverage#1.1 label=obs-plan/metric-coverage fact=fct-5sph2bsu
@carried n=3 label=obs-plan/metric-coverage fact=fct-5sph2bsu
why: The probe reads `packages/dioxus-native-dom/` (its `poll` and `events.rs`) without editing anything, so the dioxus-native frame and phase timing row must stay exactly as it is: no new timing gate or feature, and no change to the `log-times` wiring.
quote: | Frame / phase timing | Timing logs gated by `log-frame-times`, `log-phase-times`, umbrella `log-times` | log features only | (Cargo.toml:265-268); (packages/dioxus-native/Cargo.toml:60-68) |
@compare_item n=5 part=lost how=classify entity=metric-coverage#1.9 label=obs-plan/metric-coverage fact=fct-wy742at6
@carried n=4 label=obs-plan/metric-coverage fact=fct-wy742at6
why: The one printed figure in `tests/blitz-tests/`, where a probe check would live, is the bench's markdown table; a probe check must not print one like it, because nothing it reads is printed and its readings go into assertions only.
quote: | `paint_tree_bench` test | Medians of `hit()`, render on a null backend, an incremental hover-only frame and a full non-incremental frame | printed as a markdown table |
@compare_item n=6 part=lost how=strong entity=pii-scrubbing-compliance#3.1 label=obs-plan/pii-scrubbing-compliance fact=fct-zofw4lw2
@carried n=2 label=obs-plan/pii-scrubbing-compliance fact=fct-zofw4lw2
why: Whatever Dioxus runtime records the probe trips (component, handler, signal reads) must stay dropped by the sink, and no allowlist change may admit them.
quote: a record from any target outside the two prefix sets — Stylo's `style::*`, `selectors::matching`, `dioxus_core::*` and `dioxus_signals::*` on both binaries
@compare_item n=7 part=lost how=strong entity=telemetry-strategy#3.1 label=obs-plan/telemetry-strategy fact=fct-qollkyhe
@carried n=5 label=obs-plan/telemetry-strategy fact=fct-qollkyhe
why: The probe adds no workspace crate or dependency, so no telemetry backend may enter the build files.
quote: observed absent — telemetry backends · searched: `sentry|opentelemetry|otel|prometheus|metrics` over .github/workflows/*.yml, Cargo.toml, Cross.toml, flake.nix, .github/scripts/*.py
@compare_item n=1 part=listed how=classify entity=log-coverage#9.12 label=obs-plan/log-coverage fact=fct-2yjtpnwu
@carried n=3 label=obs-plan/log-coverage fact=fct-2yjtpnwu
why: Any probe check added under tests/blitz-tests or examples/seven_guis must pass the same silence census and read 0, its readings going into assertions only.
quote: observed absent — a print, an env read, a subscriber or a listener in the five checks `visible_region_transformed`, `hit_non_invertible_transform`, `open_state`, `checkable_change_event` and `text_range_no_change_event` · searched: the silence census `println!|eprintln!|print!|dbg!|std::env|env::var|escher_telemetry|tracing_subscriber|UnixListener|TcpListener|bind\(` over those files, exit 1, reading 0
@compare_item n=2 part=listed how=strong entity=log-coverage#9.11 label=obs-plan/log-coverage fact=fct-jok55ozp
@carried n=4 label=obs-plan/log-coverage fact=fct-jok55ozp
why: The chunk must leave the driver's span set and the log checks as they stand: 2 `info_span!` sites, eight `command` fields, and `host_log` and `mcp_log` at 0 needles under both builds.
quote: escher-driver holds 2 `info_span!`, `stand_act_spans` eight fields, and `host_log` and `mcp_log` read 0 needles at `trace` under both builds
@compare_item n=3 part=listed how=classify entity=pii-scrubbing-compliance#1.2 label=obs-plan/pii-scrubbing-compliance fact=fct-v4kxqyim
@carried n=4 label=obs-plan/pii-scrubbing-compliance fact=fct-v4kxqyim
why: Any probe code added in or beside `packages/dioxus-native-dom/` (the bridge's `dioxus_document.rs`) must hold the same zero added-lines census: no log site, print or env read.
quote: the added lines hold no log site, print or env read, the added-lines census reading 0 (packages/dioxus-native-dom/src/dioxus_document.rs:31-45;
@compare_item n=4 part=listed how=strong entity=span-trace-coverage#2.4 label=obs-plan/span-trace-coverage fact=fct-xswdmupu
@carried n=1 label=obs-plan/span-trace-coverage fact=fct-xswdmupu
why: The probe must add no span, so escher-driver must still hold exactly its 2 `info_span!` sites and `command` its eight fields, and `stand_act_spans` must still pass.
quote: `packages/escher-driver` and `packages/escher-telemetry` are untouched, escher-driver keeps its 2 `info_span!` sites and `command` its eight fields (`stand_act_spans`)
@compare_item n=5 part=listed how=strong entity=span-trace-coverage#2.4 label=obs-plan/span-trace-coverage fact=fct-xswdmupu
@carried n=2 label=obs-plan/span-trace-coverage fact=fct-xswdmupu
why: The MountedData hazard sits on the mounted-event path. Any fix the founder chooses for it must leave that guard opening no span, and this chunk must not decide or land that fix.
quote: the mounted-event guard in `flush_queued_mounted_events` and `Harness::text_contrast`'s blend read open no span
