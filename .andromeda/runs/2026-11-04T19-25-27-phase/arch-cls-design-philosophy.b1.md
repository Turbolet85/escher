@answer n=1 entity=design-philosophy#1.1 v=no
why: the modular-crates values statement is general architecture, and the chunk adds no crate and reads the bridge only through its public API.
@answer n=2 entity=design-philosophy#2.1 v=yes
why: DioxusDocument's handle_ui_event and poll are the calls the harness's input pumping and poll(None) drive, and the offline stand boot relies on the no-op provider defaults.
@answer n=3 entity=design-philosophy#3.1 v=no
why: its cited browser-spec behaviours (forms, ARIA, paint, networking, scrollbars) are not named in the scope, so the probe's reads do not depend on them.
@answer n=4 entity=design-philosophy#4.1 v=yes
why: the incremental-versus-non-incremental identical-layout oracle is the rule the both-layout-modes run checks, and versioned NodeIds are what the probe's element_id reads carry.
@answer n=5 entity=design-philosophy#5.1 v=yes
why: the harness runs headless with synthesized input through the real dispatch pipeline, which is the Harness the probe pumps input through and polls.
