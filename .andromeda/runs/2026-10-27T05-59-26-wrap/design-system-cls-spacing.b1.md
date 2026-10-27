# Classified — design-system · spacing.b1

## Answers
spacing#1.1 · no · the rdme spacing tokens live in apps/readme's github-markdown.css, which the session does not change, so the value and its citation hold
spacing#2.1 · no · the browser urlbar, tab, menu and suggestion paddings live in apps/browser/assets/browser.css, which the session does not change
spacing#2.2 · no · the about:history paddings live in apps/browser/assets/about-history.css, which the session does not change
spacing#2.3 · no · the rdme markdown body max-width and padding live in apps/readme's override stylesheet, which the session does not change
spacing#2.4 · yes · add-to · the stand's card padding (move-set row 8) leaves design-system and is held in layout-templates' values · desktop-native-screens, per F4 (PROVISIONAL)
spacing#2.5 · no · the flight booker's select padding is an undecided stand line awaiting the operator's P2 dialog, so the session decides nothing on it and its citation holds
spacing#2.6 · no · the blitz-dom default body margin is an engine default in packages/blitz-dom/assets/default.css, which the session does not change
spacing#3.1 · no · the hspace and vspace margin mapping in packages/blitz-dom/src/stylo.rs is an engine rule the session does not change
spacing#3.2 · no · the body marginwidth, marginheight, leftmargin and topmargin mapping in packages/blitz-dom/src/stylo.rs is an engine rule the session does not change
spacing#3.3 · no · the overlay scrollbar thumb geometry in packages/blitz-dom/src/node/scrollbar.rs is untouched, and the session changes no scroll behaviour
spacing#3.4 · no · the single-line input vertical centring in packages/blitz-dom/src/node/node.rs is an engine rule the session does not change
spacing#3.5 · no · the outside list marker padding in packages/blitz-paint/src/render.rs is an engine rule the session does not change
spacing#3.6 · no · the double text decoration offset in packages/blitz-paint/src/text.rs is an engine rule the session does not change
spacing#4.1 · no · the grid and flex gaps and body padding live in examples/assets fixture pages that the session does not touch
spacing#4.2 · no · the graphite root spacing variables live in examples/assets/graphite.html, a fixture the session does not touch
spacing#4.3 · no · the hr margins live in examples/assets/hr.html, a fixture the session does not touch
spacing#4.4 · no · the servo.css gaps and padding live in examples/assets/servo.css, a fixture the session does not touch
spacing#4.5 · no · the Preact TodoMVC and reference page spacing lives in examples/preact, which the session does not touch
