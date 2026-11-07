@answer n=1 entity=mocking-stubbing-discipline#1.1 v=no
why: blitz-dom's own test shell counts redraw requests in blitz-dom's tests, a path the chunk's DioxusDocument render never reaches

@answer n=2 entity=mocking-stubbing-discipline#1.2 v=no
why: RecordingShell belongs to interaction_state_teardown, a test the chunk does not touch and the scope does not name

@answer n=3 entity=mocking-stubbing-discipline#1.3 v=no
why: select_popup's CountingShell reads select popup redraws, a path outside the Dioxus render the chunk changes

@answer n=4 entity=mocking-stubbing-discipline#1.4 v=no
why: CountedWakes is blitz-shell's event-loop wake counter, not reached by the DioxusDocument render or poll

@answer n=5 entity=mocking-stubbing-discipline#1.5 v=no
why: RecordingNetProvider is link_rel_attribute's URL recorder, a non-Dioxus test the scope does not name

@answer n=6 entity=mocking-stubbing-discipline#1.6 v=no
why: ManualNetProvider belongs to render_blocking_stylesheet, an HtmlDocument test the chunk does not reach

@answer n=7 entity=mocking-stubbing-discipline#1.7 v=yes
why: stand_settle reads NotSettled Loads through HeldNet, and the scope names settle's Loads reading, which the buffered poll must leave unmoved and the stand_* checks re-assert

@answer n=8 entity=mocking-stubbing-discipline#1.8 v=yes
why: LateNet's load answered inside LOAD_WAIT is read through settle and poll by stand_load_wait, a stand_* check the chunk re-asserts

@answer n=9 entity=mocking-stubbing-discipline#1.9 v=yes
why: LoadNet's in-flight load, counted because it does not override is_noop, is read by stand_load_page and the load-page click, stand_* checks the chunk re-asserts

@answer n=10 entity=mocking-stubbing-discipline#1.10 v=no
why: the Scripted Document and Held provider are settle's own unit-test doubles in blitz-test-harness, not the DioxusDocument poll the chunk changes

@answer n=11 entity=mocking-stubbing-discipline#1.11 v=no
why: blitz-net's test Handler covers the net crate's own fetch helper, which the chunk does not reach

@answer n=12 entity=mocking-stubbing-discipline#1.12 v=no
why: WptNetProvider serves the WPT runner's local directory, a path wholly outside the Dioxus bridge

@answer n=13 entity=mocking-stubbing-discipline#1.13 v=no
why: MapFetcher is blitz-vibey-script's script fetch double, and ScriptDocument is not a Dioxus document

@answer n=14 entity=mocking-stubbing-discipline#1.14 v=no
why: DummyNavigationProvider and its dummy base URL belong to the WPT runner, which the chunk does not touch

@answer n=15 entity=mocking-stubbing-discipline#1.15 v=yes
why: the ported test builds its document with DocumentConfig::default(), and the scope leaves open whether that config carries these no-op net, navigation and shell providers, a fork the new test's calls run through

@answer n=16 entity=mocking-stubbing-discipline#1.16 v=no
why: RecordingHandler records DOM event names dispatched by dispatch_recorded, while the mounted events the scope names are Dioxus events the chunk flushes and does not dispatch as DOM events

@answer n=17 entity=mocking-stubbing-discipline#1.17 v=no
why: NoopEventHandler is scrollbar_drag's event-driver stand-in, a test the chunk does not reach

@answer n=18 entity=mocking-stubbing-discipline#1.18 v=no
why: Probe and SizedProbe are custom_widget_layout's stub widgets, a layout test the chunk does not reach

@answer n=19 entity=mocking-stubbing-discipline#1.19 v=no
why: NullScenePainter is a paint-tree timing stand-in, and the chunk changes no paint path

@answer n=20 entity=mocking-stubbing-discipline#1.20 v=no
why: WPT virtual time belongs to the WPT runner's timers, which the Dioxus render path never uses

@answer n=21 entity=mocking-stubbing-discipline#1.21 v=yes
why: TimerTicks applies its ticks on the next pump or settle pass, and the stand_timer and stand_act_timer checks the chunk re-asserts read that pass through the buffered poll

@answer n=22 entity=mocking-stubbing-discipline#1.22 v=no
why: debug_timer's dummy timer is compile-time instrumentation, and the chunk adds no timing instrument or log

@answer n=23 entity=mocking-stubbing-discipline#1.23 v=no
why: the WPT JS harness replacement is WPT-only, outside the Dioxus document

@answer n=24 entity=mocking-stubbing-discipline#1.24 v=no
why: url_suggestions' capturing publish closure is a browser worker stand-in, not a Dioxus document

@answer n=25 entity=mocking-stubbing-discipline#1.25 v=no
why: the nucleo no-op notify closure belongs to apps/browser url suggestions, which the chunk does not reach
