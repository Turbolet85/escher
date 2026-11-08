@account n=1 entity=design-philosophy#1.1 v=carried
why: the graph reader and the signal registry live in dioxus-native-dom, the headless Dioxus renderer, with no blitz-dom engine change
@account n=2 entity=design-philosophy#2.1 v=carried
why: the graph reads the VirtualDom that DioxusDocument integrates, through the document the bridge already holds
@account n=3 entity=design-philosophy#3.1 v=not_in_scope
why: spec-cited web semantics and browser mirroring; the graph read cites no spec and changes no role mapping or web behaviour
@account n=4 entity=design-philosophy#4.1 v=carried
why: the graph names nodes of a held instance, so its node identities must stop resolving when a node drops and stay off the slotmap key
@account n=5 entity=design-philosophy#5.1 v=not_in_scope standing=S1
why: integration tests live under blitz-tests and run headless through the real dispatch pipeline, as every new test keeps
@account n=6 entity=stack-and-technologies#1.1 v=not_in_scope
why: edition 2024 and rust-version 1.91.0 of every crate; a re-check every change owes, the chunk moving no edition or MSRV
@account n=7 entity=stack-and-technologies#1.2 v=not_in_scope
why: the workspace version and in-repo pins; a re-check every change owes, the chunk moving no version or pin
@account n=8 entity=stack-and-technologies#1.10 v=carried
why: the graph is keyed on the agent-visible AccessKit node, whose type comes from the workspace's accesskit 0.25
@account n=9 entity=stack-and-technologies#1.13 v=carried
why: links 3 and 4 read Dioxus 0.7.3's dioxus-core and dioxus-signals APIs, with no fork
@account n=10 entity=stack-and-technologies#1.14 v=carried
why: the graph read's recorded step is read back by escher-driver's serde_json, which only reads and never writes an answer
@account n=11 entity=stack-and-technologies#1.20 v=not_in_scope
why: blitz-tests' dev-dependency list; a re-check every change owes, the graph's tests reaching seven_guis, escher-driver and dioxus-native-dom it already lists
@account n=12 entity=stack-and-technologies#1.22 v=not_in_scope standing=S3
why: every change passes fmt, clippy and rustdoc through the ci-leg.sh legs of the GitHub Actions workflows
@account n=13 entity=stack-and-technologies#2.1 v=not_in_scope
why: the dev-host cognitive-complexity reading of every function; a re-check every change owes
