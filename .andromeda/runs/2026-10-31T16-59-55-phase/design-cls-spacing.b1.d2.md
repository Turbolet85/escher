# Classified — design · spacing.b1

## Answers
spacing#1.1 · no · the rdme spacing tokens are an app's own stylesheet (apps/readme), which the task lists under Not needed
spacing#2.1 · no · browser urlbar, tab and menu spacing is browser chrome the list does not touch, listed under Not needed
spacing#2.2 · no · about:history page padding and list items are a browser page, not an engine default
spacing#2.3 · no · rdme markdown body width and padding are an app's own overrides, listed under Not needed
spacing#2.4 · no · the seven_guis card and Home padding is held in layout-templates and is not the popup or a design-system value
spacing#2.5 · no · the blitz-dom body margin is a different unit of default.css that the chunk does not edit, and the list's surface does not read it
spacing#2.6 · yes · the multiple select rows' padding 0 2px in default.css must keep holding, as the scope names `multiple` rows staying as built and the list's values sit beside them
spacing#3.1 · no · hspace and vspace margins on embed, img and object are in stylo.rs, which the chunk does not reach, and not a list surface
spacing#3.2 · no · body marginwidth and leftmargin mapping is in stylo.rs, which the chunk does not reach
spacing#3.3 · yes · the overlay scrollbar thumb geometry is what a long list's new nested scrolling box paints when it overflows, and the list's values are read against engine spacing
spacing#3.4 · no · vertical centring of single-line text inputs in node.rs concerns text inputs, not a select's label or list rows
spacing#3.5 · no · the outside list-item marker padding in render.rs rules over list-item markers, which the list's option rows do not paint
spacing#3.6 · no · the double text decoration gap in text.rs rules over text decoration, which the list does not paint
spacing#4.1 · no · grid and flex gaps in examples/assets are demonstration fixtures, which a new engine default does not read
spacing#4.2 · no · graphite example root variables are an example fixture's own values, listed under Not needed
spacing#4.3 · no · the hr example's margins and body padding are a demonstration fixture
spacing#4.4 · no · servo.css column gaps, container padding and button padding are an example fixture's own stylesheet
spacing#4.5 · no · the Preact TodoMVC .app width and list-item padding are an example page, listed under Not needed
