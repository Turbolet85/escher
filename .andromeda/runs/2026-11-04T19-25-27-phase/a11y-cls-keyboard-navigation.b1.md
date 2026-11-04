@answer n=1 entity=keyboard-navigation#1.1 v=yes
why: the CRUD listbox's arrow-key path needs it in the Tab walk, and this rule states the tabindex and disabled reading that makes a node focusable
@answer n=2 entity=keyboard-navigation#1.2 v=no
why: the probe makes no node inert, and the inert focus fixup is the not-needed focusability item
@answer n=3 entity=keyboard-navigation#1.3 v=no
why: the limit needs a focused control that stops showing, which the probe neither hides nor reads hidden content for
@answer n=4 entity=keyboard-navigation#1.4 v=no
why: it names where the limit is owned in residuals and the deferred list, a planning pointer the probe does not edit
@answer n=5 entity=keyboard-navigation#1.5 v=yes
why: the press keys may move focus by Tab or Shift+Tab to reach the focused button and the CRUD list before Enter, Space or arrow keys
@answer n=6 entity=keyboard-navigation#1.6 v=no
why: Tab with an open select list, and the probe reads no select list
@answer n=7 entity=keyboard-navigation#1.7 v=no
why: focus held on a select while its list is open, and the probe reads no select list
@answer n=8 entity=keyboard-navigation#1.8 v=no
why: cached focusability recomputes only on attribute writes, and the probe writes no attribute path
@answer n=9 entity=keyboard-navigation#1.9 v=no
why: disabling a form control is an attribute write the probe never makes, and removal of focusability by disabled is not needed
@answer n=10 entity=keyboard-navigation#1.10 v=yes
why: Enter and Space activation of the focused button reads the disabled attribute's presence, and the scope names that Enter and Space dispatch
@answer n=11 entity=keyboard-navigation#1.11 v=no
why: a disabled, inert, hidden or removed open select list, and the probe reads no select list
@answer n=12 entity=keyboard-navigation#1.12 v=no
why: the file input's generated inner button, a control the scope does not name
@answer n=13 entity=keyboard-navigation#1.13 v=no
why: autofocus on flush, which the scope names no stand control for, and the probe's focus comes from its press and click path
@answer n=14 entity=keyboard-navigation#1.14 v=yes
why: the scope's focused button is carried by the FOCUS element state that focusing sets, which the probe's focus read depends on
@answer n=15 entity=keyboard-navigation#1.15 v=no
why: style matching of :focus and :focus-visible, and the probe reads no style or focus ring
@answer n=16 entity=keyboard-navigation#1.16 v=no
why: bubbling of focus and focusin events, which the probe's handlers do not listen to
@answer n=17 entity=keyboard-navigation#2.1 v=no
why: removing a focused node resets focus, and the scope names no delete or removal write, only Counter increments and CRUD selection
@answer n=18 entity=keyboard-navigation#2.2 v=yes
why: a click that reaches no focusable interactive target clears focus, which a CRUD row click does, so the arrow-key path must refocus the list
@answer n=19 entity=keyboard-navigation#2.3 v=no
why: closing an open select list by Escape, outside click or scroll, and the probe reads no select list
@answer n=20 entity=keyboard-navigation#2.4 v=yes
why: a click on a stand button focuses it before Enter and Space, and a CRUD row or list click clears focus, which the probe's press path depends on
@answer n=21 entity=keyboard-navigation#2.5 v=no
why: a click on a select focuses it and drives its list, and the probe reads no select list
@answer n=22 entity=keyboard-navigation#2.6 v=no
why: the highlight over an open select list's rows, and the probe reads no select list
@answer n=23 entity=keyboard-navigation#2.7 v=no
why: the multiple select's row click arm, and the CRUD list is a listbox the scope names, not a multiple select
@answer n=24 entity=keyboard-navigation#2.8 v=no
why: a double click on one point selects a word in a text input, and each click still focuses and dispatches as one click does
@answer n=25 entity=keyboard-navigation#2.9 v=no
why: activating a details summary, which the chunk does not reach
