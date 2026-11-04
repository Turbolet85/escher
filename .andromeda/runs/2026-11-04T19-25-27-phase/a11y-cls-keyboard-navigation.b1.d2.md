@answer n=1 entity=keyboard-navigation#1.1 v=no
why: focusability and the Tab-walk candidate set are a component rule the probe only uses, and the task marks focusability not needed
@answer n=2 entity=keyboard-navigation#1.2 v=no
why: the inert focus fixup at flush is marked not needed, and the probe makes no node inert
@answer n=3 entity=keyboard-navigation#1.3 v=no
why: the focused-control-keeps-focus limit concerns shown-or-inert controls the chunk does not make
@answer n=4 entity=keyboard-navigation#1.4 v=no
why: the residual line owns the inert focus limit and its SC 2.4.3 re-assertion, not the click reading the probe answers
@answer n=5 entity=keyboard-navigation#1.5 v=yes
why: the probe's press can move focus onto a stand control by Tab, the focus its Enter or Space dispatch needs
@answer n=6 entity=keyboard-navigation#1.6 v=no
why: Tab with an open select list runs the select's choice, and select list behaviour is marked not needed
@answer n=7 entity=keyboard-navigation#1.7 v=no
why: focus held on a select while its list is open, and select list behaviour is marked not needed
@answer n=8 entity=keyboard-navigation#1.8 v=no
why: cached focusability recomputed on attribute change; the probe sets no tabindex, href or disabled
@answer n=9 entity=keyboard-navigation#1.9 v=no
why: disabled state toggling and button focusability; disabled-ness is marked not needed and the probe sets no disabled
@answer n=10 entity=keyboard-navigation#1.10 v=yes
why: Enter and Space activation is the dispatch the scope names, and its presence-based disabled gate decides whether the Counter or CRUD handler runs
@answer n=11 entity=keyboard-navigation#1.11 v=no
why: a disabled, inert or hidden select's list behaviour is marked not needed
@answer n=12 entity=keyboard-navigation#1.12 v=no
why: the file input's generated inner button; the chunk names no file input
@answer n=13 entity=keyboard-navigation#1.13 v=no
why: autofocus on flush; the chunk names no autofocus and reaches focus by Tab or click
@answer n=14 entity=keyboard-navigation#1.14 v=no
why: the FOCUS and FOCUSRING element states are styling state; the probe reads focus from the accessibility tree
@answer n=15 entity=keyboard-navigation#1.15 v=no
why: :focus and :focus-within styling matches; focus ring styling is marked not needed
@answer n=16 entity=keyboard-navigation#1.16 v=no
why: focus and focusin bubbling; the probe reads no focus event
@answer n=17 entity=keyboard-navigation#2.1 v=no
why: focus reset on removing the focused node; the chunk's probe removes no focused node
@answer n=18 entity=keyboard-navigation#2.2 v=yes
why: a CRUD row click falls to the unmatched focus clear, which the probe's arrow-key path on the listbox must account for
@answer n=19 entity=keyboard-navigation#2.3 v=no
why: closing an open select list by Escape, click or scroll; select list behaviour is marked not needed
@answer n=20 entity=keyboard-navigation#2.4 v=yes
why: a click on a stand button focuses it and the Tab walk continues from it, and a CRUD row click clears focus, both on the probe's click path
@answer n=21 entity=keyboard-navigation#2.5 v=no
why: click on a select and its typeahead; select list behaviour is marked not needed
@answer n=22 entity=keyboard-navigation#2.6 v=no
why: pointer highlight of an open select list's row; select list behaviour is marked not needed
@answer n=23 entity=keyboard-navigation#2.7 v=no
why: click on a multiple select's row; the CRUD list is a listbox, not a multiple select
@answer n=24 entity=keyboard-navigation#2.8 v=no
why: two clicks on one point as a double click still focus and dispatch each click, and word selection needs a text input the probe does not type into
@answer n=25 entity=keyboard-navigation#2.9 v=no
why: activating a details summary; the chunk names no details or summary element
