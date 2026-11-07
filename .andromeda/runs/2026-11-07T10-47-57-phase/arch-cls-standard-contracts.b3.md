@answer n=1 entity=standard-contracts#17.1 v=yes
why: boot and boot_timer build the document through the VirtualDom render path the chunk changes, and the stand_* witnesses run through them.
@answer n=2 entity=standard-contracts#18.1 v=yes
why: the bundled DejaVu Sans is the family and size the stand's snapshot bounds are re-measured at, a value the chunk's witnesses re-read.
@answer n=3 entity=standard-contracts#19.1 v=yes
why: boot_cells builds the Cells document whose snapshot text, parts and element ids the chunk re-verifies through the render path.
@answer n=4 entity=standard-contracts#19.2 v=yes
why: boot_load mounts the keyed load-fetch and load-done elements whose element ids and snapshot the chunk re-verifies.
@answer n=5 entity=standard-contracts#19.3 v=no
why: LoadNet is a network stand-in answering fetches from its own thread; the chunk changes no load path and its scope names no load behaviour.
@answer n=6 entity=standard-contracts#20.1 v=no
why: the telemetry sink, its targets and redaction are escher-telemetry's and untouched; the buffer's trace is governed by task item 11 (cfg gate, no mutation content), not by the sink allowlist.
@answer n=7 entity=standard-contracts#21.1 v=no
why: the driver session's verbs, refusals, settle wrapper and span are escher-driver's own code the chunk does not touch; the bridge facts it consumes are bound by the bridge entities.
@answer n=8 entity=standard-contracts#22.1 v=no
why: the recording's keep rules and check file are escher-driver code the chunk does not change.
@answer n=9 entity=standard-contracts#22.2 v=no
why: replay is escher-driver code running each step through Session::run, which the chunk does not change.
@answer n=10 entity=standard-contracts#22.3 v=no
why: the visual lint's readings and pins are escher-driver's own rules; the snapshot bounds they read are bound by the bridge snapshot entities.
@answer n=11 entity=standard-contracts#22.4 v=no
why: the load wait is session code over Harness::settle, which the chunk does not change.
@answer n=12 entity=standard-contracts#23.1 v=no
why: the overlap skip and the at-boot lint pins are lint's own rules; the option bounds they read are bound by standard-contracts#30.2.
@answer n=13 entity=standard-contracts#23.2 v=no
why: the load-page witness is a load-wait check over the harness, and nothing the buffered render changes reaches it.
@answer n=14 entity=standard-contracts#24.1 v=no
why: blitz-vibey-script JavaScript runs no page script in a Dioxus document and the chunk does not touch the script runtime.
@answer n=15 entity=standard-contracts#25.1 v=no
why: document_may_load governs script and file fetches, not the Dioxus render path.
@answer n=16 entity=standard-contracts#26.1 v=no
why: the CSS property accessors live on the script runtime, outside the Dioxus bridge.
@answer n=17 entity=standard-contracts#26.2 v=no
why: URLSearchParams is a script-side class, outside the Dioxus bridge the chunk changes.
@answer n=18 entity=standard-contracts#27.1 v=yes
why: the windowed dioxus-native launch renders through DioxusDocument and so takes the buffered render, and its launch surface and CreateHeadElement path are named in scope.
@answer n=19 entity=standard-contracts#28.1 v=yes
why: the bridge's event routing, boolean attributes, stable element ids, removed-node drop and snapshot, text and diff are what the buffer replays and the re-verified stand_* checks read.
@answer n=20 entity=standard-contracts#28.2 v=yes
why: Cells' keyboard calls MountedData::scroll_to and holds per-cell onmounted handles, which the buffered render must keep in mounted-event order.
@answer n=21 entity=standard-contracts#29.1 v=yes
why: the listener-attribute over-report is pinned by dioxus_runtime_probe.rs, one of the probe tests the chunk re-runs as witness.
@answer n=22 entity=standard-contracts#29.2 v=yes
why: the buffer records the listener calls dioxus-core 0.7.10 issues, so the bridge's event attribute must stay as core sends it.
@answer n=23 entity=standard-contracts#30.1 v=yes
why: is_masked_input is the predicate the snapshot value reader calls, and the chunk re-verifies the snapshot through the stand_* checks.
@answer n=24 entity=standard-contracts#30.2 v=yes
why: the closed select's option bounds and value readings are bridge snapshot readings the chunk re-verifies.
@answer n=25 entity=standard-contracts#30.3 v=yes
why: NodeState's seven fields and the thirteen node keys are snapshot shape the chunk must not change, since task item 10 bars an added node key.
