@answer n=1 entity=input-validation#1.1 v=no
why: examples' CLI URL argument parsing; the examples' argv is not reached by the chunk
@answer n=2 entity=input-validation#1.2 v=no
why: examples' numeric CLI arguments; no argv grammar change in the chunk
@answer n=3 entity=input-validation#1.3 v=no
why: screenshot example's output filename; the example is not reached
@answer n=4 entity=input-validation#1.4 v=no
why: preact_script example's path; the example is not reached
@answer n=5 entity=input-validation#1.5 v=no
why: agent-run.sh verb and selection grammar; the scripts and the CI workflows are not reached
@answer n=6 entity=input-validation#1.6 v=no
why: cold-agent.sh verb and task allowlist; the pipe is not reached
@answer n=7 entity=input-validation#1.7 v=no
why: apt-install.sh package and option grammar; no CI script or workflow is edited
@answer n=8 entity=input-validation#1.8 v=no
why: coverage_floor.py floor and lcov arguments; the coverage leg is not changed
@answer n=9 entity=input-validation#1.9 v=no
why: cold-agent check reply and its replay; the driver and the pipe are not reached
@answer n=10 entity=input-validation#1.10 v=no
why: driver session socket request grammar; the chunk adds no listener, verb or carrier and does not touch the driver crate
@answer n=11 entity=input-validation#1.11 v=no
why: driver select-list highlighted row, which crosses no carrier; the driver is not touched
@answer n=12 entity=input-validation#1.12 v=no
why: driver state directory path and mode; the driver is not touched
@answer n=13 entity=input-validation#1.13 v=no
why: driver recording directory path and mode; the driver is not touched
@answer n=14 entity=input-validation#1.14 v=no
why: Check::from_bytes reader of kept checks; the chunk writes, reads and changes no kept check
@answer n=15 entity=input-validation#1.15 v=no
why: driver session label grammar; the driver is not touched
@answer n=16 entity=input-validation#1.16 v=no standing=S1
why: the driver span fields hold fixed words, bools and counts only, the convention S1 lists; the driver's command schema and executor are not reached
@answer n=17 entity=input-validation#1.17 v=no
why: escher-session command line argv reader; the command line is not reached
@answer n=18 entity=input-validation#1.18 v=no
why: MCP stdio server framing and tool arguments; the MCP server is untouched
@answer n=19 entity=input-validation#1.19 v=no
why: browser urlbar parsing; apps/browser is not reached
@answer n=20 entity=input-validation#1.20 v=no
why: navigation method and scheme limit in apps/browser and dioxus-native's link_handler; the chunk changes no link or navigation path
@answer n=21 entity=input-validation#1.21 v=no
why: href resolution and unparseable-href handling in blitz-dom's pointer events; the replay passes href through the same set_attribute call and the chunk clicks nothing new
@answer n=22 entity=input-validation#1.22 v=no
why: DocumentUrl parse error handling in blitz-dom; the chunk changes no document URL handling
@answer n=23 entity=input-validation#1.23 v=no
why: URL fragment lossy decode in blitz-dom scrolling; the chunk's MountedData scroll operations do not match fragments
@answer n=24 entity=input-validation#1.24 v=no
why: file: URL path handling in the net provider; the buffered render fetches nothing
@answer n=25 entity=input-validation#1.25 v=no
why: data: URL decoding in the net provider; the buffered render fetches nothing
