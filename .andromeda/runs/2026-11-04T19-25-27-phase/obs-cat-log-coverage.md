@carried n=1 label=obs-plan/log-coverage fact=fct-gxkjmvep
why: The probe reads through dioxus-native-dom's bridge; that crate's log sites must stand as recorded, the probe adding no site, field or level beside them.
quote: Every DOM mutation (assign_node_id, create_placeholder, create_text_node, append/insert/replace, remove_node, push_root, set_node_text, load_template, set_attribute) is logged at debug through `trace!` (packages/dioxus-native-dom/src/mutation_writer.rs:141-227; packages/dioxus-native-dom/src/mutation_writer.rs:327; packages/dioxus-native-dom/src/mutation_writer.rs:410)
@carried n=2 label=obs-plan/log-coverage fact=fct-7dblu2wz
why: The MountedData hazard sits on the mounted-event path in dioxus-native-dom; the probe's code, and each fix option the reading brings to the founder, must keep that path free of any env read, print, `tracing` site or wall-clock read.
quote: no site added: the restyle narrowing, `NetHandler::error` and the failed-stylesheet path, blitz-paint's `blend` module and its blend layer, `init_stylo_prefs`, `is_inline_with_only_block_children` and its paint skip, dioxus-native-dom's mounted-event guard and the `deallocate_anonymous_block` fix add no env read, print, `tracing` site or wall-clock read (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/sink.md)
@carried n=3 label=obs-plan/log-coverage fact=fct-2yjtpnwu
why: Any probe check added under tests/blitz-tests or examples/seven_guis must pass the same silence census and read 0, its readings going into assertions only.
quote: observed absent — a print, an env read, a subscriber or a listener in the five checks `visible_region_transformed`, `hit_non_invertible_transform`, `open_state`, `checkable_change_event` and `text_range_no_change_event` · searched: the silence census `println!|eprintln!|print!|dbg!|std::env|env::var|escher_telemetry|tracing_subscriber|UnixListener|TcpListener|bind\(` over those files, exit 1, reading 0
@carried n=4 label=obs-plan/log-coverage fact=fct-jok55ozp
why: The chunk must leave the driver's span set and the log checks as they stand: 2 `info_span!` sites, eight `command` fields, and `host_log` and `mcp_log` at 0 needles under both builds.
quote: escher-driver holds 2 `info_span!`, `stand_act_spans` eight fields, and `host_log` and `mcp_log` read 0 needles at `trace` under both builds
@carried n=5 label=obs-plan fact=fct-6pasqyd4
why: Where the MountedData panic is measured under a binary that installs the sink, it must be logged only through this existing ERROR event with its payload redacted; no option may add a panic log site.
quote: ERROR at target `escher_telemetry::panic`, message `panic`, fields `panic.file`, `panic.line`, `panic.column`, `panic.payload` (redacted), once per panic (packages/escher-telemetry/src/panic.rs:11-21)
@carried n=6 label=obs-plan fact=fct-lrj2spd7
why: The panic's raw message (`RefCell already borrowed`) gets past the scrub through the chained std hook; the reading may quote that fixed message, but nothing the probe reads may enter it.
quote: Past escher's scrub: the chained std panic hook prints the raw panic message to stderr
@carried n=7 label=obs-plan fact=fct-dxerzkok
why: The bridge's debug logs already record text-node contents and attribute values; the probe's readings (component names, handlers, signal values) must not be added to them or to any other log, values masked and never logged.
quote: dioxus-native debug logs record text-node contents and attribute values
@carried n=8 label=obs-plan fact=fct-upbrcclf
why: The probe's measured runs and any log check they touch must state which build they ran under, because engine `tracing` is on only in the workspace build.
quote: a build that resolves the whole workspace (`cargo test --workspace`, `ci-leg.sh fast`, CI's test leg) turns `blitz-dom/tracing` on, while `cargo test -p blitz-tests` and a `-p seven_guis` build leave it off
@binding n=1
**dioxus-native-dom log sites ↔ the scope's «Nothing crosses» boundary**: the probe adds no log site, field or span to the bridge, and the mounted-event path keeps its no-site record (obs-plan/log-coverage fct-gxkjmvep), (obs-plan/log-coverage fct-7dblu2wz); its readings stay out of the bridge's content-bearing debug logs (obs-plan fct-dxerzkok)
@binding n=2
**Silence census ↔ the test plan's probe check**: a probe check in tests/blitz-tests reads 0 on the silence census, and the driver's span set and the `host_log` / `mcp_log` needles stay unchanged under both builds (obs-plan/log-coverage fct-2yjtpnwu), (obs-plan/log-coverage fct-jok55ozp), the build split stated (obs-plan fct-upbrcclf)
@binding n=3
**Panic logging ↔ §7 Error Capture and the founder's MountedData fork**: the hazard's panic is logged only by the existing redacted ERROR event while its raw message gets past the scrub; the fork's options are brought with this log reach stated and add no panic site (obs-plan fct-6pasqyd4), (obs-plan fct-lrj2spd7)
