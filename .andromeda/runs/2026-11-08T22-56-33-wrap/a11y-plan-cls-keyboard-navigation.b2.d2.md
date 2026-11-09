@answer n=1 entity=keyboard-navigation#2.10 v=no
why: the open-state rule lives in blitz-dom files (element.rs, mutator.rs, stylo.rs) and open_state.rs, none of which the session reaches
@answer n=2 entity=keyboard-navigation#2.11 v=no
why: the checkbox and radio click rule lives in blitz-dom pointer.rs and harness_smoke.rs, which the session does not reach
@answer n=3 entity=keyboard-navigation#2.12 v=no
why: the checkable change dispatch lives in blitz-dom pointer.rs and checkable_change_event.rs, which the session does not reach
@answer n=4 entity=keyboard-navigation#2.13 v=no
why: the range click focus rule lives in blitz-dom pointer.rs, which the session does not reach
@answer n=5 entity=keyboard-navigation#3.1 v=no
why: the text input key rule lives in blitz-dom text.rs, which the session does not reach
@answer n=6 entity=keyboard-navigation#3.2 v=no
why: the range input key rule lives in blitz-dom keyboard.rs, which the session does not reach
@answer n=7 entity=keyboard-navigation#3.3 v=no
why: the single select key and typeahead rule lives in blitz-dom keyboard.rs and select.rs, which the session does not reach
@answer n=8 entity=keyboard-navigation#3.4 v=no
why: the open list key rule lives in blitz-dom keyboard.rs and select_popup.rs, which the session does not reach
@answer n=9 entity=keyboard-navigation#3.5 v=no
why: the choose and close rule lives in blitz-dom keyboard.rs and pointer.rs, which the session does not reach
@answer n=10 entity=keyboard-navigation#3.6 v=no
why: the list scrolling rule lives in blitz-dom scrolling.rs and select_popup.rs, which the session does not reach
@answer n=11 entity=keyboard-navigation#3.7 v=yes mode=stale
why: the session reaches docs/capabilities.md, which carries this list thumb line at 262-264, and the report does not show that line holds after the change
@answer n=12 entity=keyboard-navigation#3.8 v=no
why: the multiple select key rule lives in blitz-dom keyboard.rs and select.rs, which the session does not reach
@answer n=13 entity=keyboard-navigation#3.9 v=no
why: the multiple select scroll into view rule lives in blitz-dom keyboard.rs and select_popup.rs, which the session does not reach
@answer n=14 entity=keyboard-navigation#3.10 v=no
why: the multiple select Space and character rule lives in blitz-dom keyboard.rs and select_interaction.rs, which the session does not reach
@answer n=15 entity=keyboard-navigation#3.11 v=no
why: the multiple select disabled row rule lives in blitz-dom keyboard.rs and select.rs, which the session does not reach
@answer n=16 entity=keyboard-navigation#3.12 v=no
why: the multiple select Shift span rule lives in blitz-dom select.rs and select_interaction.rs, which the session does not reach
@answer n=17 entity=keyboard-navigation#3.13 v=no
why: the optgroup option rule lives in blitz-dom select.rs and form.rs, which the session does not reach
@answer n=18 entity=keyboard-navigation#3.14 v=no
why: the multiple write record rule lives in blitz-dom mutator.rs, which the session does not reach
@answer n=19 entity=keyboard-navigation#3.15 v=no
why: the driver type verb rule cites execute.rs and stand_act_select.rs, neither reached, and the session moves no verb, argument, result field or error set
@answer n=20 entity=keyboard-navigation#3.16 v=no
why: the press Ctrl limit cites schema.rs, which the session does not reach, and the usage line it moved sits in cli.rs, which this entity does not cite
@answer n=21 entity=keyboard-navigation#3.17 v=yes mode=add-to
why: the stand's Enter and Space click reading gains F2, that stand_handler_coverage asserts a keyboard activation exercises its control's click handler and a refused step exercises nothing
@answer n=22 entity=keyboard-navigation#3.18 v=no
why: the macOS Backspace rule lives in blitz-dom text.rs, which the session does not reach
@answer n=23 entity=keyboard-navigation#3.19 v=no
why: the copy with no text input focused rule lives in blitz-dom keyboard.rs, which the session does not reach
@answer n=24 entity=keyboard-navigation#3.20 v=no
why: the implicit form submission rule lives in blitz-dom keyboard.rs, which the session does not reach
@answer n=25 entity=keyboard-navigation#3.21 v=no
why: the label click rule lives in blitz-dom pointer.rs, which the session does not reach
