@answer n=1 entity=input-validation#1.1 v=no
why: the session changed only the Dioxus render path in dioxus-native-dom, and this row's URL argument parsing in examples/screenshot.rs and paint_bench.rs is not reached
@answer n=2 entity=input-validation#1.2 v=no
why: the numeric-argument fallbacks in examples/paint_bench.rs, screenshot.rs and multi_doc.rs are outside the reached files
@answer n=3 entity=input-validation#1.3 v=no
why: the output filename rule in examples/screenshot.rs is outside the reached files and the session did not change it
@answer n=4 entity=input-validation#1.4 v=no
why: the preact_script canonicalize-and-panic rule in examples/preact_script.rs is outside the reached files and unchanged
@answer n=5 entity=input-validation#1.5 v=no
why: agent-run.sh and its test-file stem check are untouched, and the probe file it names as an existing stem still exists, so the claim holds
@answer n=6 entity=input-validation#1.6 v=no
why: cold-agent.sh, its verb and task allowlist and its stand prompt are untouched by the session
@answer n=7 entity=input-validation#1.7 v=no
why: apt-install.sh and the ci.yml calls it checks are untouched, and no CI step or leg moved
@answer n=8 entity=input-validation#1.8 v=no
why: coverage_floor.py and the leg's three floors are unchanged, and the green CI run at e26f1a207 shows dioxus-native-dom still meets its floor
@answer n=9 entity=input-validation#1.9 v=no
why: the cold-agent check reader in cold-agent.sh is untouched and reads no Dioxus render output
@answer n=10 entity=input-validation#1.10 v=no
why: the driver session socket and its request grammar are in escher-driver, which the session did not touch
@answer n=11 entity=input-validation#1.11 v=no
why: the select-list highlighted row is in no carrier the session changed, and escher-driver is untouched
@answer n=12 entity=input-validation#1.12 v=no
why: the driver state directory rules are in escher-driver, which the session did not touch
@answer n=13 entity=input-validation#1.13 v=no
why: the driver recording directory rules are in escher-driver, which the session did not touch
@answer n=14 entity=input-validation#1.14 v=no
why: the recorded check reader is in escher-driver and recording.rs, neither reached by the session
@answer n=15 entity=input-validation#1.15 v=no
why: the driver session label check is in escher-driver's session.rs, which the session did not touch
@answer n=16 entity=input-validation#1.16 v=no
why: the driver command schema and validate are in escher-driver, which the session did not touch
@answer n=17 entity=input-validation#1.17 v=no
why: the escher-session command line and the session host in examples/seven_guis are in escher-driver and the stand, neither reached by the session
@answer n=18 entity=input-validation#1.18 v=no
why: the MCP server is in escher-driver's mcp modules, which the session did not touch
@answer n=19 entity=input-validation#1.19 v=no
why: the browser urlbar parsing in apps/browser/src/nav.rs is outside the reached files and unchanged
@answer n=20 entity=input-validation#1.20 v=no
why: the link handler in packages/dioxus-native/src/link_handler.rs is not a reached file, and navigation method and scheme rules are not on the MountedData render-borrow path the session changed
@answer n=21 entity=input-validation#1.21 v=no
why: link href resolution in blitz-dom's pointer.rs is outside the reached files and unchanged
@answer n=22 entity=input-validation#1.22 v=no
why: DocumentUrl parsing in blitz-dom's url.rs is outside the reached files and unchanged
@answer n=23 entity=input-validation#1.23 v=no
why: the URL fragment decode in blitz-dom's scrolling.rs is outside the reached files and unchanged
@answer n=24 entity=input-validation#1.24 v=no
why: file: URL handling in blitz-net is outside the reached files and that crate was not touched
@answer n=25 entity=input-validation#1.25 v=no
why: data: URL decoding in blitz-net is outside the reached files and that crate was not touched
