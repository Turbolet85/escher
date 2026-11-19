@row_add n=1 row=decision/devtools-inspector-opt-in area=surface-desktop-native fact=F1 detector=D-layout-surface
name: Devtools inspector opt-in
rule: The browser app's default build carries no devtools; the devtools inspector, a Chrome DevTools Protocol server on 127.0.0.1, is built only by the devtools feature and is opened from a toolbar item or the Chrome devtools shortcuts.
state: provisional
basis: fct-wakes6es (apps/browser default features, no devtools, W1), fct-xcqgt2ri (the CDP listener on 127.0.0.1, devtools feature only, no default feature set turns it on), fct-xfqezngb (the toolbar's open-devtools item), fct-c7jstd66 (the Chrome devtools shortcuts)
because: The merge brings a new user-facing surface, the devtools inspector with its toolbar item and shortcuts, held opt-in by the W1 ruling still provisional for ratification, and no row of the desktop surface says so.
@row_add n=2 row=decision/programmatic-focus-fires-focus-events area=surface-desktop-native fact=F2 detector=none
name: Programmatic focus fires focus events
rule: A programmatic focus change fires blur, focusout, focus and focusin.
except: The inert fix-up and the driver's select focus move focus silently and fire none of the four.
state: provisional
basis: fct-xcqgt2ri (set_focus_to and clear_focus queue blur, focusout, focus, focusin; set_focus_silently for the driver's focus_select, clear_focus_silently for the inert fix-up), fct-trpvqtky (focus events, W5)
because: Programmatic focus now fires the four focus events with two named silent exceptions, a convention ruled by W5 and still provisional, which no row of the desktop surface states.
