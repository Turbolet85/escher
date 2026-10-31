# Classified — design-system · surface.b1

## Answers
surface#1.1 · no · the DEFAULT_CSS sheet is still added once through DocumentConfig::default(), config.rs is unreached, and the session's default.css edits leave that wiring alone
surface#1.2 · no · the report's default.css edits sit past line 183, so the single-choice rules at lines 77-86 and 155-159 stand and the closed select still lays out as a box holding its chosen label
surface#1.3 · yes · stale · the session's default.css edits past line 183 fall inside this 161-199 span and add a size rule to multiple rows, so the citation moves while rows stay block and nowrap
surface#1.4 · yes · stale · the session's default.css edits past line 183 fall inside this 161-199 span, so the citation may have moved; the chosen-row colour rule is not named as changed
surface#1.5 · no · the single-choice group display rules at lines 161-164 and 166-168 sit before the report's edits past line 183 and stand unchanged
surface#1.6 · yes · contradict · the open list is painted over the page by the engine (F13), Enter and Space open it (F14), a group's label paints (F3) and a multiple select's size is read (F7), so all four retired limits are false
surface#1.7 · no · the initial window theme lives in blitz-shell, which the session did not reach, and the list adds no theme
surface#1.8 · no · the default viewport and scale rules live in blitz-traits, which is unreached, and the list is placed inside the viewport without changing any default
surface#1.9 · no · the JS window-size bridge lives in blitz-vibey-script, which is unreached, and the session changes none of its sizes
surface#2.1 · no · the browser tabstrip lives under apps/browser, which the session did not edit
surface#2.2 · no · the shell's titlebar provider lives in blitz-shell, which the session did not reach
surface#3.1 · no · the custom widget example is unedited, and the session edited no app or example source
surface#4.1 · no · devtools overlays still paint on top, since paint_select_list runs before them (render.rs:246-247), and the node-box overlay claim holds
surface#5.1 · no · the window title follows apps/browser and blitz-shell window.rs, neither of which the session reached
surface#5.2 · no · the rdme title and theme shortcut live under apps/readme, which the session did not edit
surface#6.1 · no · the macOS browser titlebar lives in apps/browser main.rs, which the session did not edit
surface#6.2 · no · the transparent example under examples/transparent is unedited
surface#6.3 · no · the compositing alpha setting lives in dioxus-native config.rs, which is unreached
surface#7.1 · no · the wasm32 canvas lives in dioxus-native config.rs, which is unreached, and the report ran no wasm build of the list
surface#7.2 · yes · add-to · the feature still forwards to blitz-paint, and under it a scrolled list paints a thumb that takes no hit and is not draggable (F19)
surface#8.1 · no · the mobile urlbar lives in apps/browser, which the session did not edit
surface#9.1 · no · the mobile frame class lives in apps/browser, which the session did not edit
surface#10.1 · no · the Android back button lives in apps/browser, which the session did not edit
surface#11.1 · no · mobile screenshot capture lives in apps/browser capture.rs, which the session did not edit
surface#12.1 · no · the claim lives in .github/workflows/ci.yml, which the session did not reach, and the report names no mobile leg
