@answer n=1 entity=input-validation#1.1 v=no
why: the session touched only the new blitz-tests target, Cargo.lock and deny.toml, and the URL argument parsing in examples/screenshot.rs and examples/paint_bench.rs is unchanged
@answer n=2 entity=input-validation#1.2 v=no
why: the numeric-argument fallbacks in examples/paint_bench.rs, examples/screenshot.rs and examples/multi_doc.rs are not in the change
@answer n=3 entity=input-validation#1.3 v=no
why: the output filename rule in examples/screenshot.rs is not in the change
@answer n=4 entity=input-validation#1.4 v=no
why: the preact_script canonicalize rule in examples/preact_script.rs is not in the change
@answer n=5 entity=input-validation#1.5 v=no
why: scripts/agent-run.sh is unmoved and the rule still holds on the changed tests directory, since the new dioxus_runtime_probe stem is one more existing name under the same rule
@answer n=6 entity=input-validation#1.6 v=no
why: scripts/cold-agent.sh, its task allowlist and its preconditions are not in the change
@answer n=7 entity=input-validation#1.7 v=no
why: apt-install.sh and ci.yml's package calls are unmoved, since the CI audit fix changed only Cargo.lock and deny.toml
@answer n=8 entity=input-validation#1.8 v=no
why: coverage_floor.py, its three floors and its lcov rules are unchanged, and the entity states no measured count the new test target moves
@answer n=9 entity=input-validation#1.9 v=no
why: the cold-agent check extraction and its replay rules in scripts/cold-agent.sh are not in the change
@answer n=10 entity=input-validation#1.10 v=no
why: no driver code moved, so the socket request grammar at wire version v3 and its bounds stand as written
@answer n=11 entity=input-validation#1.11 v=no
why: the select-list highlight rule is in the driver's measured report and no select or driver code moved
@answer n=12 entity=input-validation#1.12 v=no
why: the state-directory mode, path-length and socket rules are in escher-driver, which the session did not touch
@answer n=13 entity=input-validation#1.13 v=no
why: the recording-directory rules are in escher-driver's recording and session code, which the session did not touch
@answer n=14 entity=input-validation#1.14 v=no
why: Check::from_bytes and the committed checks under tests/blitz-tests/checks/ are unmoved, since the new file sits under tests/blitz-tests/tests/ and no check changed
@answer n=15 entity=input-validation#1.15 v=no
why: the session label rule in escher-driver's session.rs is not in the change
@answer n=16 entity=input-validation#1.16 v=no
why: the driver command schema holds twelve verbs and no driver code moved, so the validation rules stand
@answer n=17 entity=input-validation#1.17 v=no
why: the escher-session command-line reader in escher-driver's cli.rs and the stand's session_host.rs are not in the change
@answer n=18 entity=input-validation#1.18 v=no
why: the MCP server in escher-driver's mcp.rs and its framing are not in the change
@answer n=19 entity=input-validation#1.19 v=no
why: the browser urlbar parsing in apps/browser/src/nav.rs is not in the change
@answer n=20 entity=input-validation#1.20 v=no
why: the browser navigation method and scheme rule in apps/browser/src/nav.rs and packages/dioxus-native/src/link_handler.rs is not in the change
@answer n=21 entity=input-validation#1.21 v=no
why: the link href resolution in packages/blitz-dom/src/events/pointer.rs and url.rs is not in the change
@answer n=22 entity=input-validation#1.22 v=no
why: DocumentUrl parsing in packages/blitz-dom/src/url.rs is not in the change, and the url crate is unmoved since Cargo.lock changed only pulldown-cmark
@answer n=23 entity=input-validation#1.23 v=no
why: fragment percent-decoding in packages/blitz-dom/src/scrolling.rs is not in the change
@answer n=24 entity=input-validation#1.24 v=no
why: the file: URL path reading in packages/blitz-net/src/lib.rs is not in the change, and the url crate is unmoved
@answer n=25 entity=input-validation#1.25 v=no
why: the data: URL decoding in packages/blitz-net/src/lib.rs is not in the change, and the data_url crate is unmoved since Cargo.lock changed only pulldown-cmark
