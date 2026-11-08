@answer n=1 entity=keyboard-navigation#2.10 v=no
why: the details and dialog open state lives in blitz-dom and open_state.rs, none of which the session reaches
@answer n=2 entity=keyboard-navigation#2.11 v=no
why: checkbox and radio click toggling lives in blitz-dom pointer.rs and harness_smoke.rs, neither reached by the session
@answer n=3 entity=keyboard-navigation#2.12 v=no
why: the checkbox and radio change event lives in blitz-dom pointer.rs and checkable_change_event.rs, neither reached by the session
@answer n=4 entity=keyboard-navigation#2.13 v=no
why: a range input's click focus lives in blitz-dom pointer.rs, which the session does not reach
@answer n=5 entity=keyboard-navigation#3.1 v=no
why: text input keys live in blitz-dom text.rs, which the session does not reach
@answer n=6 entity=keyboard-navigation#3.2 v=no
why: range input keys live in blitz-dom keyboard.rs, which the session does not reach
@answer n=7 entity=keyboard-navigation#3.3 v=no
why: single select keys live in blitz-dom keyboard.rs and select.rs, neither reached by the session
@answer n=8 entity=keyboard-navigation#3.4 v=no
why: the open list's key match lives in blitz-dom keyboard.rs and select_popup.rs, neither reached by the session
@answer n=9 entity=keyboard-navigation#3.5 v=no
why: choosing a list row lives in blitz-dom keyboard.rs, pointer.rs and select_popup.rs, none reached by the session
@answer n=10 entity=keyboard-navigation#3.6 v=no
why: list scrolling lives in blitz-dom scrolling.rs and select_popup.rs, neither reached by the session
@answer n=11 entity=keyboard-navigation#3.7 v=yes mode=stale
why: docs/capabilities.md is a reached file cited at lines 233-235 and the session edited it, so the citation may have moved
@answer n=12 entity=keyboard-navigation#3.8 v=no
why: multiple select keys live in blitz-dom keyboard.rs and select.rs, neither reached by the session
@answer n=13 entity=keyboard-navigation#3.9 v=no
why: scroll_into_view on a multiple select lives in blitz-dom keyboard.rs and select_popup.rs, neither reached by the session
@answer n=14 entity=keyboard-navigation#3.10 v=no
why: Space and character keys on a multiple select live in blitz-dom keyboard.rs and select_interaction.rs, neither reached by the session
@answer n=15 entity=keyboard-navigation#3.11 v=no
why: disabled rows and the unmodelled Ctrl+A on a multiple select live in blitz-dom keyboard.rs and select.rs, neither reached by the session
@answer n=16 entity=keyboard-navigation#3.12 v=no
why: Shift anchoring on a multiple select lives in blitz-dom select.rs and select_interaction.rs, neither reached by the session
@answer n=17 entity=keyboard-navigation#3.13 v=no
why: optgroup option ordering and disabled rows live in blitz-dom select.rs and form.rs, neither reached by the session
@answer n=18 entity=keyboard-navigation#3.14 v=no
why: multiple select writes and removals live in blitz-dom mutator.rs and select_interaction.rs, neither reached by the session
@answer n=19 entity=keyboard-navigation#3.15 v=yes mode=stale
why: execute.rs is a reached file cited at lines 242-246 and 572-579, and the session's graph verb changed the driver's closed sets, so those citations may have moved although the no-verb claim still holds
@answer n=20 entity=keyboard-navigation#3.16 v=yes mode=stale
why: schema.rs is a reached file cited at lines 270-275 and the session changed the verb and field sets there, so the citation may have moved
@answer n=21 entity=keyboard-navigation#3.17 v=yes mode=add-to
why: the Dioxus bridge's event path now sets a SignalRegistry dispatch around handle_event for the Enter and Space Click that reaches the app's onclick, and F6 (use_graph_state with DeclaredWrite, a write confirmed only once its handler's event is dispatched through its element) joins this dispatch account
@answer n=22 entity=keyboard-navigation#3.18 v=no
why: macOS Backspace lives in blitz-dom text.rs, which the session does not reach
@answer n=23 entity=keyboard-navigation#3.19 v=no
why: copy with no text input focused lives in blitz-dom keyboard.rs, which the session does not reach
@answer n=24 entity=keyboard-navigation#3.20 v=no
why: implicit form submission lives in blitz-dom keyboard.rs, which the session does not reach
@answer n=25 entity=keyboard-navigation#3.21 v=no
why: label click default actions live in blitz-dom pointer.rs, which the session does not reach
