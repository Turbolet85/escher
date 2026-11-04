@answer n=1 entity=integration-test-strategy#1.1 v=no
why: blitz-dom pseudo-element flow classification is a layout behaviour the scope does not name; the four links are read through the Dioxus runtime and the bridge, not layout
@answer n=2 entity=integration-test-strategy#2.1 v=no
why: blitz-vibey-script's DOM tests cover the script engine, which the probe does not run; the probe is over the Dioxus runtime
@answer n=3 entity=integration-test-strategy#3.1 v=no
why: a radio's change event in the script engine's DOM, which the Dioxus probe does not touch
@answer n=4 entity=integration-test-strategy#4.1 v=no
why: URLSearchParams in the script engine, outside the Dioxus runtime the probe reads
@answer n=5 entity=integration-test-strategy#5.1 v=no standing=S6
why: the no-sleep, no-wall-clock check convention, which the probe owes as its own rule (time moves only through the virtual clock); the driver Session settle and load-wait behaviour are not called by the probe
@answer n=6 entity=integration-test-strategy#5.2 v=no
why: the session host and its socket, which the probe never goes through (process-free reach on the headless stand)
@answer n=7 entity=integration-test-strategy#5.3 v=no
why: the session host's stderr and escher's sink, which the probe neither installs nor runs; its own value masking is its own work, not this entity's rule
@answer n=8 entity=integration-test-strategy#6.1 v=no
why: a select list opened and chosen through the session host's log, a surface the probe does not drive
@answer n=9 entity=integration-test-strategy#7.1 v=no
why: a hosted CLI click on a select list through a held host, which the probe does not use
@answer n=10 entity=integration-test-strategy#8.1 v=no
why: stand_act_select's multiple-select driver reads, a select the probe does not drive through the driver
@answer n=11 entity=integration-test-strategy#9.1 v=no
why: stand_act_popup's list reads through a held session on the driver, which the probe does not use
@answer n=12 entity=integration-test-strategy#10.1 v=yes
why: harness press is the key path the probe's key reads rest on (the scope names press), and its no-panic and focus-stays guarantee binds; the select-list detail does not
@answer n=13 entity=integration-test-strategy#11.1 v=no
why: painted frames and pixel reads, which no probe verdict reads and which the scope does not name
@answer n=14 entity=integration-test-strategy#12.1 v=no
why: render-blocking stylesheet loading, a resource behaviour the scope does not name and the probe does not read
@answer n=15 entity=integration-test-strategy#13.1 v=no standing=S1
why: a unit test sits in a #[cfg(test)] mod tests inside its source file, the convention the probe's cfg(test) placement owes; the apps' integration-test search reaches no file the probe adds
