@answer n=1 entity=input-validation#1.1 v=no
why: the session's change reaches only dioxus-native-dom's render path and one probe test, not the examples' URL argument parsing
@answer n=2 entity=input-validation#1.2 v=no
why: the numeric-argument parsing in the examples sits outside the files the session reached
@answer n=3 entity=input-validation#1.3 v=no
why: the output filename rule in examples/screenshot.rs sits outside the files the session reached
@answer n=4 entity=input-validation#1.4 v=no
why: the preact_script canonicalize rule in examples/preact_script.rs sits outside the files the session reached
@answer n=5 entity=input-validation#1.5 v=no
why: scripts/agent-run.sh is not among the session's reached files, and its verb and selection rule is unchanged
@answer n=6 entity=input-validation#1.6 v=no
why: scripts/cold-agent.sh is not among the session's reached files, and its verb, task and prompt rules are unchanged
@answer n=7 entity=input-validation#1.7 v=no
why: apt-install.sh and ci.yml's package calls are unchanged, since no CI step or leg moved
@answer n=8 entity=input-validation#1.8 v=no
why: the floors 97, 83 and 90 and coverage_floor.py are unmoved, and CI#42371051720 is green 17/17 at e26f1a207, so the dioxus-native-dom floor of 83 still holds on the changed crate
@answer n=9 entity=input-validation#1.9 v=no
why: the cold-agent check's extraction and replay rules live in the pipe and the driver, neither of which the session reached
@answer n=10 entity=input-validation#1.10 v=no
why: the driver session socket is not reached, and its wire grammar and bounds are unchanged
@answer n=11 entity=input-validation#1.11 v=no
why: the highlighted-row rule is a driver carrier the session did not reach, and the driver is unchanged
@answer n=12 entity=input-validation#1.12 v=no
why: the state-directory rule lives in escher-driver, which the session did not reach
@answer n=13 entity=input-validation#1.13 v=no
why: the recording-directory rule lives in escher-driver's recording code, which the session did not reach
@answer n=14 entity=input-validation#1.14 v=no
why: the kept-check reader lives in escher-driver's recording and replay code, which the session did not reach
@answer n=15 entity=input-validation#1.15 v=no
why: the session-label rule lives in escher-driver's session module, which the session did not reach
@answer n=16 entity=input-validation#1.16 v=no
why: the driver command schema and its validate are unchanged, the driver not being reached by the session
@answer n=17 entity=input-validation#1.17 v=no
why: the escher-session argv reader lives in escher-driver's cli.rs, which the session did not reach
@answer n=18 entity=input-validation#1.18 v=no
why: the MCP server lives in escher-driver's mcp module, which the session did not reach
@answer n=19 entity=input-validation#1.19 v=no
why: apps/browser's urlbar parsing is not among the files the session reached
@answer n=20 entity=input-validation#1.20 v=no
why: apps/browser's nav.rs and dioxus-native's link_handler.rs are not reached, and link navigation sits outside the render path
@answer n=21 entity=input-validation#1.21 v=no
why: href resolution in blitz-dom's pointer.rs is not reached, since blitz-dom was not touched
@answer n=22 entity=input-validation#1.22 v=no
why: blitz-dom's DocumentUrl parsing is not reached, since blitz-dom was not touched
@answer n=23 entity=input-validation#1.23 v=no
why: fragment decoding in blitz-dom's scrolling.rs is not reached, since blitz-dom was not touched
@answer n=24 entity=input-validation#1.24 v=no
why: blitz-net's file: URL handling is not reached, since blitz-net was not touched
@answer n=25 entity=input-validation#1.25 v=no
why: blitz-net's data: URL handling is not reached, since blitz-net was not touched
