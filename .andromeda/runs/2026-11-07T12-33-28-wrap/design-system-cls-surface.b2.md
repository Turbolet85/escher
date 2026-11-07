@answer n=1 entity=surface#6.1 v=no
why: the macOS titlebar note is window chrome and names no Dioxus render path or MountedData call
@answer n=2 entity=surface#6.2 v=no
why: the transparent example's window size and shell close are unchanged by the bridge change
@answer n=3 entity=surface#6.3 v=no
why: the compositing alpha mode in config.rs is not on the render path the session changed
@answer n=4 entity=surface#7.1 v=yes mode=add-to
why: F1 joins this desktop-native platform note list, since render-time user code on a dioxus-native surface can now call MountedData set_focus, scroll_to, scroll and the rect queries without panicking
@answer n=5 entity=surface#7.2 v=no
why: the scrollbars feature forwarding to blitz-paint is a paint feature the session did not touch
@answer n=6 entity=surface#7.3 v=no
why: the select list's keys, wheel and scroll move rows inside blitz-paint, which the session did not change and which is not a MountedData call
@answer n=7 entity=surface#8.1 v=no
why: the mobile urlbar font size and padding are CSS values the session did not move
@answer n=8 entity=surface#9.1 v=no
why: the mobile frame class driving urlbar sizing is styling, not a Dioxus render-path or MountedData statement
@answer n=9 entity=surface#10.1 v=no
why: the Android back button navigation is untouched by the render-buffer change
@answer n=10 entity=surface#11.1 v=no
why: the mobile screenshot default file name is capture behaviour the session did not touch
@answer n=11 entity=surface#12.1 v=no
why: the iOS and Android build-only CI targets are workflow facts the session did not move
@answer n=12 entity=surface#13.1 v=no
why: the WPT runner result colours are CLI tokens the session did not touch
@answer n=13 entity=surface#13.2 v=no
why: the WPT runner bright-black printing is CLI output the session did not touch
@answer n=14 entity=surface#13.3 v=no
why: the OSC 8 hyperlink wrapping in the WPT runner is CLI output the session did not touch
@answer n=15 entity=surface#14.1 v=no
why: the paint_bench printed lines are example CLI output the session did not change
@answer n=16 entity=surface#14.2 v=no
why: the screenshot printed lines are example CLI output the session did not change
@answer n=17 entity=surface#15.1 v=no
why: escher-session's JSON-only stdout, exit classes and MCP replies are driver output the session did not change
@answer n=18 entity=surface#15.2 v=no
why: the six-task list in session_host.rs is unchanged by the bridge change
@answer n=19 entity=surface#16.1 v=no
why: the bundled DejaVu fonts and the WASM font context are font paths the session did not touch
@answer n=20 entity=surface#17.1 v=no
why: the wasm_hello canvas is focusable by its tabindex attribute, not by a MountedData set_focus call, so its claim still holds
