@answer n=1 entity=mocking-stubbing-discipline#1.1 v=no
why: blitz-dom's own hand-written ShellProvider fakes in the engine's mutator and document code; the chunk's Dioxus render path and tests use no shell double.
@answer n=2 entity=mocking-stubbing-discipline#1.2 v=no
why: RecordingShell in interaction_state_teardown is an engine interaction test double the chunk does not reach.
@answer n=3 entity=mocking-stubbing-discipline#1.3 v=no
why: select_popup's CountingShell counts select-popup redraws, an unrelated engine check the chunk does not reach.
@answer n=4 entity=mocking-stubbing-discipline#1.4 v=no
why: CountedWakes is a blitz-shell event-loop test double; blitz-shell is neither edited nor on the Dioxus render path.
@answer n=5 entity=mocking-stubbing-discipline#1.5 v=no
why: RecordingNetProvider in link_rel_attribute is a single engine check not on the Dioxus render path.
@answer n=6 entity=mocking-stubbing-discipline#1.6 v=no
why: ManualNetProvider is upstream's file-private stand-in in render_blocking_stylesheet, which the chunk does not reach.
@answer n=7 entity=mocking-stubbing-discipline#1.7 v=yes
why: HeldNet is the stand_settle stand-in whose held loads carry settle's Loads busy reading, which the buffered poll must leave unchanged and the re-asserted stand_* checks witness.
@answer n=8 entity=mocking-stubbing-discipline#1.8 v=yes
why: LateNet answers stand_load_wait's loads inside LOAD_WAIT through the settle path, a stand_* check the chunk re-asserts, so its in-flight timing must hold under the buffered poll.
@answer n=9 entity=mocking-stubbing-discipline#1.9 v=yes
why: LoadNet is the stand's load-page stand-in that keeps a load in flight through settle, and stand_load_page is a stand_* check the chunk re-asserts over the buffered render path.
@answer n=10 entity=mocking-stubbing-discipline#1.10 v=no
why: settle.rs unit tests drive a scripted Document and a held NetProvider, not DioxusDocument; the buffered poll's answer is witnessed through the stand checks, not these doubles.
@answer n=11 entity=mocking-stubbing-discipline#1.11 v=no
why: blitz-net's test-local Handler covers blitz-net's own fetch tests, and the chunk does not reach blitz-net.
@answer n=12 entity=mocking-stubbing-discipline#1.12 v=no
why: WptNetProvider serves WPT files; WPT runs on upstream only and is not reached.
@answer n=13 entity=mocking-stubbing-discipline#1.13 v=no
why: blitz-vibey-script's MapFetcher stands in for a ScriptDocument, which is not a Dioxus document.
@answer n=14 entity=mocking-stubbing-discipline#1.14 v=no
why: DummyNavigationProvider is WPT-runner navigation and is not reached by the chunk.
@answer n=15 entity=mocking-stubbing-discipline#1.15 v=no
why: the no-op net, navigation and shell providers are not named by the scope; the chunk's ported test uses DocumentConfig::default() but neither calls nor chooses these providers.
@answer n=16 entity=mocking-stubbing-discipline#1.16 v=no
why: RecordingHandler records dispatched DOM event names, while the chunk's mounted-event flush is the document's own queue and the scope names neither dispatch_recorded nor DOM event names.
@answer n=17 entity=mocking-stubbing-discipline#1.17 v=no
why: NoopEventHandler in scrollbar_drag is an unrelated engine check's event driver the chunk does not reach.
@answer n=18 entity=mocking-stubbing-discipline#1.18 v=no
why: the Probe and SizedProbe custom widgets in custom_widget_layout are not on the Dioxus render path.
@answer n=19 entity=mocking-stubbing-discipline#1.19 v=no
why: NullScenePainter in paint_tree_bench is a timing stand-in; no paint path changes.
@answer n=20 entity=mocking-stubbing-discipline#1.20 v=no
why: WPT-runner virtual time is not reached; WPT runs on upstream only.
@answer n=21 entity=mocking-stubbing-discipline#1.21 v=yes
why: TimerTicks is provided by every headless-stand boot and its ticks apply on the next pump, the buffered poll path the re-asserted stand_* checks and the wall-time rule read through.
@answer n=22 entity=mocking-stubbing-discipline#1.22 v=no
why: debug_timer's dummy timer is not named by the scope and is not on the Dioxus render path.
@answer n=23 entity=mocking-stubbing-discipline#1.23 v=no
why: the WPT JS harness replacements belong to WPT, which the chunk does not reach.
@answer n=24 entity=mocking-stubbing-discipline#1.24 v=no
why: the apps/browser worker publication closure is in a browser module the chunk does not reach.
@answer n=25 entity=mocking-stubbing-discipline#1.25 v=no
why: the apps/browser nucleo notify closure is in a browser module the chunk does not reach.
