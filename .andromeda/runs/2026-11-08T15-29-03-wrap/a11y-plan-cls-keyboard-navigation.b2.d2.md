@answer n=1 entity=keyboard-navigation#2.10 v=no
why: its open-state rule lives in blitz-dom and open_state.rs, none of which the session reached
@answer n=2 entity=keyboard-navigation#2.11 v=no
why: its checkbox and radio click rule lives in blitz-dom pointer code and harness_smoke.rs, not reached by the session
@answer n=3 entity=keyboard-navigation#2.12 v=no
why: its change-event rule lives in blitz-dom pointer code and checkable_change_event.rs, not reached by the session
@answer n=4 entity=keyboard-navigation#2.13 v=no
why: its range-input click rule lives in blitz-dom pointer code, which the session did not change
@answer n=5 entity=keyboard-navigation#3.1 v=no
why: its text-input key rule lives in blitz-dom text.rs, which the session did not change
@answer n=6 entity=keyboard-navigation#3.2 v=no
why: its range-input key rule lives in blitz-dom keyboard.rs, which the session did not change
@answer n=7 entity=keyboard-navigation#3.3 v=no
why: its single-select key rule lives in blitz-dom keyboard.rs and select.rs, which the session did not change
@answer n=8 entity=keyboard-navigation#3.4 v=no
why: its open-list key rule lives in blitz-dom keyboard.rs and select_popup.rs, which the session did not change
@answer n=9 entity=keyboard-navigation#3.5 v=no
why: its list choice path lives in blitz-dom keyboard.rs and pointer.rs, which the session did not change
@answer n=10 entity=keyboard-navigation#3.6 v=no
why: its list scrolling rule lives in blitz-dom scrolling.rs and select_popup.rs, which the session did not change
@answer n=11 entity=keyboard-navigation#3.7 v=yes mode=stale
why: docs/capabilities.md is a reached file and its cited lines 248-250 may have moved with the session's doc edits
@answer n=12 entity=keyboard-navigation#3.8 v=no
why: its multiple-select key rule lives in blitz-dom keyboard.rs and select.rs, which the session did not change
@answer n=13 entity=keyboard-navigation#3.9 v=no
why: its multiple-select scroll-into-view rule lives in blitz-dom keyboard.rs, which the session did not change
@answer n=14 entity=keyboard-navigation#3.10 v=no
why: its multiple-select Space and character rule lives in blitz-dom keyboard.rs, which the session did not change
@answer n=15 entity=keyboard-navigation#3.11 v=no
why: its disabled-row and Ctrl+A rule lives in blitz-dom keyboard.rs and select.rs, which the session did not change
@answer n=16 entity=keyboard-navigation#3.12 v=no
why: its Shift span rule lives in blitz-dom select.rs and select_interaction.rs, which the session did not change
@answer n=17 entity=keyboard-navigation#3.13 v=no
why: its optgroup rule lives in blitz-dom select.rs and form.rs, which the session did not change
@answer n=18 entity=keyboard-navigation#3.14 v=no
why: its multiple-write rule lives in blitz-dom mutator.rs, which the session did not change
@answer n=19 entity=keyboard-navigation#3.15 v=yes mode=stale
why: execute.rs is reached and its type arm now decides refusals through check_target before acting, so the cited lines 251-255 and 645-652 may have moved
@answer n=20 entity=keyboard-navigation#3.16 v=yes mode=stale
why: schema.rs is reached through the new actions row and the verb counts, so the cited lines 270-275 may have moved
@answer n=21 entity=keyboard-navigation#3.17 v=no
why: its Enter and Space activation rule lives in blitz-dom keyboard.rs and events/mod.rs, and the driver press path is not among the id-taking arms the session changed
@answer n=22 entity=keyboard-navigation#3.18 v=no
why: its macOS Backspace rule lives in blitz-dom text.rs, which the session did not change
@answer n=23 entity=keyboard-navigation#3.19 v=no
why: its copy rule lives in blitz-dom keyboard.rs, which the session did not change
@answer n=24 entity=keyboard-navigation#3.20 v=no
why: its implicit submission rule lives in blitz-dom keyboard.rs, which the session did not change
@answer n=25 entity=keyboard-navigation#3.21 v=no
why: its label click rule lives in blitz-dom pointer.rs, which the session did not change
