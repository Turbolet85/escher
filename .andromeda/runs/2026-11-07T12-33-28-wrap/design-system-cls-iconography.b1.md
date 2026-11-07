@answer n=1 entity=iconography#1.1 v=no
why: the cursor rule lives in packages/blitz-dom/src/document.rs, which the session's change does not reach, and the task marks cursors untouched
@answer n=2 entity=iconography#1.2 v=no
why: the CursorIcon keyword map and shell cursor handling are in blitz-dom and blitz-traits, which the session does not reach, and the task marks cursor rules untouched
@answer n=3 entity=iconography#1.3 v=no
why: favicon_url in packages/blitz-dom/src/document.rs is not reached by the dioxus-native-dom bridge change
@answer n=4 entity=iconography#2.1 v=no
why: the cursor fixture under examples/assets is not among the files the session reaches, and no cursor rule changed
@answer n=5 entity=iconography#2.2 v=no
why: the inline SVG octicon and google fixtures are unchanged and not reached by the mutation buffer or MountedData change
@answer n=6 entity=iconography#2.3 v=no
why: the Font Awesome fixtures are unchanged and not reached by the session's dioxus-native-dom change
@answer n=7 entity=iconography#2.4 v=no
why: the graphite sprite and data-URI fixtures are unchanged and not reached by the session's change
@answer n=8 entity=iconography#2.5 v=no
why: the currentColor svg fixtures and svg_native example are unchanged and not reached by the session's change
@answer n=9 entity=iconography#3.1 v=no
why: the session brings no icon handling or icon text, and none of its reached files holds icon handling, so the absence still holds
@answer n=10 entity=iconography#3.2 v=no
why: the session brings no icon sets or icon handling, so the absence over the listed s11 files still holds
