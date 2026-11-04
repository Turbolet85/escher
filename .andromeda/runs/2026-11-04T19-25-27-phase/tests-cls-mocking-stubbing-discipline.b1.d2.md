@answer n=1 entity=mocking-stubbing-discipline#1.1 v=no
why: blitz-dom's ShellProvider fakes in mutator.rs and document.rs test the engine's redraw count, which the chunk does not reach and the scope does not name
@answer n=2 entity=mocking-stubbing-discipline#1.2 v=no
why: RecordingShell in interaction_state_teardown.rs is another check's IME shell double; the scope names no shell or IME call
@answer n=3 entity=mocking-stubbing-discipline#1.3 v=no
why: select_popup's CountingShell counts redraws for a select list the probe never opens; the scope names no redraw count
@answer n=4 entity=mocking-stubbing-discipline#1.4 v=no
why: CountedWakes is blitz-shell's unit double for the event loop proxy, and the windowed path is read from code only with no wake path named in the scope
@answer n=5 entity=mocking-stubbing-discipline#1.5 v=no
why: RecordingNetProvider in link_rel_attribute.rs is another check's URL recorder, and no boot the probe uses names it
@answer n=6 entity=mocking-stubbing-discipline#1.6 v=no
why: ManualNetProvider is upstream's file-private stylesheet double in render_blocking_stylesheet.rs, which no boot the probe uses names
@answer n=7 entity=mocking-stubbing-discipline#1.7 v=no
why: HeldNet is the stand_settle and fixtures network double; the probe's served task boots LoadNet through boot_load and never reads HeldNet
@answer n=8 entity=mocking-stubbing-discipline#1.8 v=no
why: LateNet is stand_load_wait's own in-file stand-in with a 200 ms answer; the probe reads LoadNet, not LateNet
@answer n=9 entity=mocking-stubbing-discipline#1.9 v=yes
why: boot_load carries LoadNet through options with every other field kept, and the served-page in-flight reading and the offline rule of the other boots rest on it (scope item 2)
@answer n=10 entity=mocking-stubbing-discipline#1.10 v=no
why: the Scripted Document and Held NetProvider are settle.rs unit-test doubles; the scope names settle's semantics, not these doubles
@answer n=11 entity=mocking-stubbing-discipline#1.11 v=no
why: blitz-net's test-local Handler is a unit test of the net crate, which the probe does not reach
@answer n=12 entity=mocking-stubbing-discipline#1.12 v=no
why: WptNetProvider serves the local WPT directory, and the chunk runs no WPT suite
@answer n=13 entity=mocking-stubbing-discipline#1.13 v=no
why: MapFetcher is the blitz-vibey-script test double; the probe is over the Dioxus runtime, not the script engine
@answer n=14 entity=mocking-stubbing-discipline#1.14 v=no
why: DummyNavigationProvider is the WPT runner's navigation double, outside the probe's scope
@answer n=15 entity=mocking-stubbing-discipline#1.15 v=no
why: the no-op Net, navigation and shell providers are blitz-traits defaults that the offline Counter and CRUD boots pick up by their own code; the scope names no call that carries them, and the served boot's offline rule is carried by the LoadNet entry
@answer n=16 entity=mocking-stubbing-discipline#1.16 v=yes
why: RecordingHandler is what dispatch_recorded installs, and scope item 5 names that a recorded name is not a Dioxus handler run, which bears on link (2) and the click reading
@answer n=17 entity=mocking-stubbing-discipline#1.17 v=no
why: NoopEventHandler is scrollbar_drag's EventDriver double, a check the probe does not reach
@answer n=18 entity=mocking-stubbing-discipline#1.18 v=no
why: Probe and SizedProbe are blitz-dom Widget stubs in custom_widget_layout, not Dioxus nodes the probe reads
@answer n=19 entity=mocking-stubbing-discipline#1.19 v=no
why: NullScenePainter is a paint-tree timing stand-in, and the paint benches are not reached by the chunk
@answer n=20 entity=mocking-stubbing-discipline#1.20 v=no
why: WPT virtual time drives the WPT runner's timers, which the chunk does not reach
@answer n=21 entity=mocking-stubbing-discipline#1.21 v=yes
why: TimerTicks is the stand's time source that every headless boot provides in context, and scope item 11 names it, so the probe's no-sleep time reads ride on it
@answer n=22 entity=mocking-stubbing-discipline#1.22 v=no
why: debug_timer's zero-cost dummy timer is an instrumentation switch the probe's verdicts do not read
@answer n=23 entity=mocking-stubbing-discipline#1.23 v=no
why: the WPT JS harness replacements are WPT-runner files outside the chunk
@answer n=24 entity=mocking-stubbing-discipline#1.24 v=no
why: worker publication capture belongs to apps/browser, which the chunk does not reach
@answer n=25 entity=mocking-stubbing-discipline#1.25 v=no
why: the synchronous nucleo no-op notify is an apps/browser double, and the chunk reaches no browser crate
