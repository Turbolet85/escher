# Classified — design-system · surface.b1

## Answers
surface#1.1 · no · the blitz sheet is still added once through DocumentConfig::default() (config.rs is unreached); the session changed only default.css rules past line 183, not the mechanism
surface#1.2 · no · the select border, padding, background and single-choice box rules at default.css 77-86 and 155-159 lie before the changed region past 183 and still hold; the list's colours are new color-palette rows, not this claim
surface#1.3 · no · the multiple select's block and nowrap row rules still hold; the session adds only line-height and overflow under the multiple select's size attribute, which the claim does not state
surface#1.4 · no · the multiple select's chosen-row rule still colours the chosen row as before; the list's highlight colours are constants on the open list only
surface#1.5 · no · the single select's transparent group rules at default.css 161-168 lie before the changed region past 183 and hold; the group label paints only in the open list
surface#1.6 · yes · contradict · the session paints the select's list over the page through paint_scene (F8, F13), Enter and Space open a single select's list (F14), a group's label paints (F3), and a multiple select's size is read (F7), so all four retired limits are now false
surface#1.7 · no · the window theme default lives in blitz-shell window.rs, which the session did not edit
surface#1.8 · no · the default viewport lives in blitz-traits shell.rs, which is unreached and unedited
surface#1.9 · no · the JS innerWidth, innerHeight and devicePixelRatio bootstrap lives in blitz-vibey-script, which is unreached
surface#2.1 · no · the macOS merged titlebar is an apps/browser rule, which the session did not edit
surface#2.2 · no · the shell provider's window controls live in blitz-shell, which is absent from the changed files
surface#3.1 · no · the custom widget's opacity 0.8 composite lives in examples/custom_widget.rs, which was not edited; the list's paint past opacity (F9) is a depth-strategy fact
surface#4.1 · no · the devtools overlays still paint on top of the list (paint_select_list runs before them), so the claim holds; blitz-traits devtools.rs is unreached
surface#5.1 · no · the browser title rule lives in apps/browser and blitz-shell, neither of which the session edited
surface#5.2 · no · rdme titling and the theme toggle live in apps/readme, which was not edited
surface#6.1 · no · the browser macOS titlebar lives in apps/browser/src/main.rs, which was not edited
surface#6.2 · no · the transparent example lives in examples/transparent, which was not edited
surface#6.3 · no · the alpha mode config lives in dioxus-native config.rs, which is unreached and unedited
surface#7.1 · no · the wasm32 canvas config lives in dioxus-native config.rs, which is unreached; the web surface was not edited
surface#7.2 · yes · add-to · F19, a scrolled list paints a thumb under feature scrollbars, which this forwarding line now also governs; the forwarding itself is unmoved and the manifests are unchanged
surface#8.1 · no · the mobile urlbar CSS lives in apps/browser, which was not edited
surface#9.1 · no · the mobile frame class lives in apps/browser, which was not edited
surface#10.1 · no · the Android back button lives in apps/browser/src/main.rs, which was not edited
surface#11.1 · no · mobile screenshot naming lives in apps/browser/src/capture.rs, which was not edited
surface#12.1 · no · the cited iOS and Android CI lines are in ci.yml, which is unreached, and the report names no mobile leg change
