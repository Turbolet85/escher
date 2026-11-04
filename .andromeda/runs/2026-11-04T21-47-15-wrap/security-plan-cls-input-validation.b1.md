@answer n=1 entity=input-validation#1.1 v=no
why: the URL argument parsing lives in examples/screenshot.rs and examples/paint_bench.rs, which the session did not change
@answer n=2 entity=input-validation#1.2 v=no
why: the numeric-argument fallbacks in examples/paint_bench.rs, screenshot.rs and multi_doc.rs are untouched by the session
@answer n=3 entity=input-validation#1.3 v=no
why: the output filename rule at examples/screenshot.rs:175-183 is in an example the session did not change
@answer n=4 entity=input-validation#1.4 v=no
why: the preact_script path canonicalization in examples/preact_script.rs is unchanged
@answer n=5 entity=input-validation#1.5 v=no
why: agent-run.sh is unchanged, and the new dioxus_runtime_probe target still satisfies its existing-test-file rule, so the stated claim holds
@answer n=6 entity=input-validation#1.6 v=no
why: cold-agent.sh, its verb table and its stand-only task allowlist are unchanged by the session
@answer n=7 entity=input-validation#1.7 v=no
why: apt-install.sh and ci.yml's package calls are unchanged, the CI audit fix touching only Cargo.lock and deny.toml
@answer n=8 entity=input-validation#1.8 v=no
why: coverage_floor.py and its three fixed floors are unchanged, and the new test target lies outside the three floored crate directories
@answer n=9 entity=input-validation#1.9 v=no
why: the cold-agent pipe's check extraction and replay bounds in cold-agent.sh are unchanged
@answer n=10 entity=input-validation#1.10 v=no
why: the driver session socket grammar in packages/escher-driver is unchanged, no driver code having moved
@answer n=11 entity=input-validation#1.11 v=no
why: the select-list highlighted-row rule concerns the driver's snapshot and lint, which the session did not change
@answer n=12 entity=input-validation#1.12 v=no
why: the state-directory checks in packages/escher-driver/src/host.rs are unchanged
@answer n=13 entity=input-validation#1.13 v=no
why: the recording-directory checks in packages/escher-driver/src/session.rs and recording.rs are unchanged
@answer n=14 entity=input-validation#1.14 v=no
why: the recorded-check reader in recording.rs and replay.rs and the committed checks under tests/blitz-tests/checks/ are unchanged, the session adding only a test target
@answer n=15 entity=input-validation#1.15 v=no
why: the session label check in packages/escher-driver/src/session.rs is unchanged
@answer n=16 entity=input-validation#1.16 v=no
why: the command schema in packages/escher-driver/src/command.rs is unchanged and VERBS stays twelve
@answer n=17 entity=input-validation#1.17 v=no
why: the escher-session argv reader in packages/escher-driver/src/cli.rs and the stand's session_host.rs are unchanged
@answer n=18 entity=input-validation#1.18 v=no
why: the MCP server in packages/escher-driver/src/mcp.rs and mcp/ is unchanged
@answer n=19 entity=input-validation#1.19 v=no
why: the urlbar parsing in apps/browser/src/nav.rs is unchanged, the session moving no app code
@answer n=20 entity=input-validation#1.20 v=no
why: the method and scheme limit in apps/browser/src/nav.rs and link_handler.rs is unchanged
@answer n=21 entity=input-validation#1.21 v=no
why: the href resolution in packages/blitz-dom/src/events/pointer.rs is unchanged
@answer n=22 entity=input-validation#1.22 v=no
why: the DocumentUrl parsing in packages/blitz-dom/src/url.rs is unchanged
@answer n=23 entity=input-validation#1.23 v=no
why: the fragment decoding in packages/blitz-dom/src/scrolling.rs is unchanged
@answer n=24 entity=input-validation#1.24 v=no
why: the file: URL handling in packages/blitz-net/src/lib.rs is unchanged, and the lock moved only pulldown-cmark and zip, neither read by it
@answer n=25 entity=input-validation#1.25 v=no
why: the data: URL handling in packages/blitz-net/src/lib.rs and the data_url crate are unchanged, the lock moving only pulldown-cmark and zip
