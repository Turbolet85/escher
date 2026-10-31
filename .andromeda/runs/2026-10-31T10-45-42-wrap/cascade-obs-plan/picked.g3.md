### log-coverage#8.5 · values · logs · ## 6. Log Coverage · lines 87–92 · stale
- **blitz-net**
  - info "Using cache dir" with field `path` (packages/blitz-net/src/lib.rs:57-58)
  - error "Failed to clear HTTP cache: {:?}" (packages/blitz-net/src/lib.rs:140-141)
  - warn "HTTP error status" with fields `url` and `status` (packages/blitz-net/src/lib.rs:219-224)
  - error "Fetching" with fields `url` and `error`, and info "Success fetching" with field `url`, in `fetch_with_callback` and `fetch_async` (packages/blitz-net/src/lib.rs:245-252; packages/blitz-net/src/lib.rs:266-273)
  - info "Fetching" with field `url`; info "Success fetching" with field `url`; and error "Error fetching" with fields `url` and `error` in `NetProvider::fetch` (packages/blitz-net/src/lib.rs:284-285; packages/blitz-net/src/lib.rs:310-315)
### log-coverage#8.6 · values · logs · ## 6. Log Coverage · lines 93–94 · stale
- **blitz-html**
  - Each collected HTML parse error is logged at error level on sink finish (packages/blitz-html/src/html_sink.rs:181-186)
### log-coverage#8.7 · values · logs · ## 6. Log Coverage · lines 95–97 · stale
- **blitz-shell, blitz-paint**
  - `tracing::error!` is logged for unsupported Ime, PointerSource, PointerKind, ButtonSource and MouseScrollDelta variants (packages/blitz-shell/src/convert_events.rs:41-45; packages/blitz-shell/src/convert_events.rs:78-82; packages/blitz-shell/src/convert_events.rs:94-98; packages/blitz-shell/src/convert_events.rs:112-116; packages/blitz-shell/src/convert_events.rs:140-144; packages/blitz-shell/src/window.rs:850-854)
  - `warn!` is logged for unimplemented image layer kinds and unsupported mask-mode luminance (packages/blitz-paint/src/render/background.rs:201-220; packages/blitz-paint/src/render/mask.rs:102-111)
### log-coverage#8.8 · values · logs · ## 6. Log Coverage · lines 98–102 · stale
- **blitz, blitz-vibey-script**
  - `launch_url` logs `tracing::info!` with "Launching" and the URL under the `tracing` feature (packages/blitz/src/lib.rs:48-49)
  - Recorded script errors are logged with a "blitz-vibey-script:" prefix under the `tracing` feature (packages/blitz-vibey-script/src/document.rs:264-267)
  - Uncaught JS errors are logged as "Uncaught JS error in" plus the source description (packages/blitz-vibey-script/src/runtime/events.rs:20-21)
  - JS console log, info, warn and error all map to one debug-level log call (packages/blitz-vibey-script/src/runtime.rs:34-50)
### log-coverage#8.9 · values · logs · ## 6. Log Coverage · lines 103–107 · stale
- **dioxus-native, dioxus-native-dom**
  - Every DOM mutation (assign_node_id, create_placeholder, create_text_node, append/insert/replace, remove_node, push_root, set_node_text, load_template, set_attribute) is logged at debug through `trace!` (packages/dioxus-native-dom/src/mutation_writer.rs:141-227; packages/dioxus-native-dom/src/mutation_writer.rs:327; packages/dioxus-native-dom/src/mutation_writer.rs:410)
  - Asset fetch success is logged at trace and failure at warn; fetches without a net provider are logged at warn (packages/dioxus-native/src/assets.rs:47-60)
  - A failure to open a URL is logged at error (packages/dioxus-native/src/link_handler.rs:14-15)
  - Injecting the document provider into windows is logged at debug (packages/dioxus-native/src/dioxus_application.rs:137-138)
### log-coverage#8.10 · values · logs · ## 6. Log Coverage · lines 108–110 · stale
- **tests/blitz-tests**
  - A `blitz-dom/log-phase-times` feature prints per-phase resolve timings (tests/blitz-tests/tests/paint_tree_bench.rs:223-224; tests/blitz-tests/tests/paint_tree_bench.rs:261)
  - Test diagnostics go to stdout/stderr via `println!`/`eprintln!` (tests/blitz-tests/tests/br_trailing_line.rs:46-50; tests/blitz-tests/tests/paint_tree_bench.rs:266)
### log-coverage#8.11 · values · logs · ## 6. Log Coverage · lines 111–113 · stale
- **wpt/runner**
  - Log calls cover WPT_DIR setup, glob failures, net load errors, pending requests, skips, JS errors and missing harness results (wpt/runner/src/main.rs:473-478; wpt/runner/src/main.rs:298; wpt/runner/src/net_provider.rs:79; wpt/runner/src/net_provider.rs:205; wpt/runner/src/net_provider.rs:220; wpt/runner/src/test_runners/harness_test.rs:151-153; wpt/runner/src/test_runners/harness_test.rs:179)
  - Log messages are free-form format strings, not structured fields (wpt/runner/src/net_provider.rs:110; wpt/runner/src/test_runners/mod.rs:184)
### log-coverage#8.12 · values · logs · ## 6. Log Coverage · lines 114–115 · stale
- **CI publish builds**
  - Publish builds log at `CARGO_LOG: info` with `--verbose --trace`, on upstream `DioxusLabs/blitz` only — the `release-cli` job is repository-guarded (.github/workflows/publish-browser.yml:33; .github/workflows/publish-browser.yml:37; .github/workflows/publish-browser.yml:157)
### log-coverage#9.1 · absence · logs · ## 6. Log Coverage · lines 121–121 · stale
- observed absent — console logging calls · searched: `console\.` over the 21 s02 files
### log-coverage#9.2 · absence · logs · ## 6. Log Coverage · lines 122–122 · stale
- observed absent — structured logging · searched: `tracing::|log::|env_logger` over the 32 slice files (examples slice); `seven_guis_native` has since gained escher-telemetry's subscriber (§3)
### log-coverage#9.3 · absence · logs · ## 6. Log Coverage · lines 123–123 · stale
- observed absent — a log site, span, field, env read or listener in the 0.2.0 requirement record's test additions (`stand_requirement_sweep`'s `WITNESSES_020` and its leg check reading `.github/scripts/ci-leg.sh` through `include_str!`, `stand_lint`'s `reading_page` and its seventh test, `text_extent`'s `extent` helper and three limit tests, `stand_act_spans`' paged part read, the `fixtures` module's `many_items` and `ITEMS`, seven_guis `host_log`'s third test) · searched: the silence census over those files, reading 0; `packages/` unchanged, the `RUST_LOG=trace` on `host_log`'s client commands the registered `RUST_LOG` read (as measured at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md, its gate entry 8)
### log-coverage#9.4 · absence · logs · ## 6. Log Coverage · lines 124–124 · stale
- observed absent — a print, an env read, a subscriber or a listener in the eight checks the `a11y` leg runs beside its six earlier files (`accessibility_states`, `accessibility_label`, `accessibility_description`, `accessibility_merged_states`, `accessibility_active_descendant_unread`, `inert_subtree`, `select_interaction`, `stand_act_select`) and the two shared modules they read (`tests/common/mod.rs`, `tests/fixtures/mod.rs`) · searched: the silence census `println!|eprintln!|print!|dbg!|std::env|env::var|escher_telemetry|tracing_subscriber|UnixListener|TcpListener|bind\(` over those ten files, reading 0; `packages/` unchanged (as measured at escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md, its silence-census gate)
### log-coverage#9.6 · absence · logs · ## 6. Log Coverage · lines 126–126 · stale
- observed absent — a print or log site in blitz-vibey-script `tests/dom.rs`'s `style_property_accessors`, `remote_document_cannot_load_file_urls` and `file_document_can_load_sibling_file_urls` · searched: the fixed-sha census over the added `packages/*.rs` lines of the upstream take read at `ebacd68bd`, its one hit in the file a test-only env read — `temp_dir_with_files` reads `TMPDIR` through `std::env::temp_dir()`, test code only and not secret, the second test-only `TMPDIR` site beside escher-driver's client unit test (packages/blitz-vibey-script/tests/dom.rs:903; as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/sink.md)
