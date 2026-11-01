# Classified — design · spacing.b1

## Answers
spacing#1.1 · no · the rdme spacing tokens in apps/readme/assets/github-markdown.css; no file of the readme app is named by the scope
spacing#2.1 · no · browser urlbar, tab, menu and suggestion padding in apps/browser/assets/browser.css; no browser app file is named by the scope
spacing#2.2 · no · about:history padding and list gaps in apps/browser/assets/about-history.css; no browser app file is named by the scope
spacing#2.3 · no · rdme markdown body width and padding in apps/readme/assets/blitz-markdown-overrides.css; no readme app file is named by the scope
spacing#2.4 · yes · points at the seven_guis stand card and Home padding, which the scope's fixture-accuracy line for examples/seven_guis names; the value itself lives in layout-templates
spacing#2.5 · yes · the blitz-dom default body margin of 8px sits in default.css, a conflict surface, and the recorded engine default spacing is read against the resolved sheet
spacing#2.6 · yes · the default padding of multiple-select rows sits in default.css, a conflict surface, read against the resolved sheet
spacing#2.7 · yes · the multiple-select size hint in select_list.rs and stylo.rs (the `size` hint the scope names) and the overflow rule in default.css, all reached through the conflict surfaces
spacing#3.1 · yes · the hspace and vspace margin mapping sits in stylo.rs, a conflict surface reached whole
spacing#3.2 · yes · the body marginwidth, marginheight, leftmargin and topmargin mapping sits in stylo.rs, a conflict surface reached whole
spacing#3.3 · no · overlay scrollbar thumb geometry in node/scrollbar.rs, a component the chunk uses; the scope names neither scrollbar.rs nor a scrollbar behaviour, and no measured screenshot is stated to read it
spacing#3.4 · yes · the open select list's MAX_ROWS and engine constants (BORDER_WIDTH, ROW_PADDING_X, ROW_PADDING_Y, GROUP_INDENT) are the open list the scope re-reads row by row, and sit in select_popup.rs, a file the chunk edits
spacing#3.5 · yes · place's sizing and placement of the open select list is the open list the scope re-reads row by row through the list's merge surface
spacing#3.6 · yes · single-line text input vertical centering sits in node/node.rs, a conflict surface reached whole
spacing#3.7 · no · outside list marker padding in blitz-paint render.rs; render.rs is not reached, and the scope's text painting change is confined to text.rs and the named decoration properties
spacing#3.8 · yes · the double text decoration's second-line offset sits in blitz-paint text.rs, a conflict surface, and the scope names decoration painting there
spacing#4.1 · no · grid and flex gaps in examples/assets fixtures; fixture values of examples other than examples/seven_guis are not named by the scope
spacing#4.2 · no · graphite fixture root variables in examples/assets/graphite.html; fixture values of examples other than examples/seven_guis are not named by the scope
spacing#4.3 · no · hr margin and body padding in examples/assets/hr.html; fixture values of examples other than examples/seven_guis are not named by the scope
spacing#4.4 · no · servo.css column gap, container and button padding; fixture values of examples other than examples/seven_guis are not named by the scope
spacing#4.5 · no · Preact TodoMVC and reference page layout in examples/preact; fixture values of examples other than examples/seven_guis are not named by the scope
