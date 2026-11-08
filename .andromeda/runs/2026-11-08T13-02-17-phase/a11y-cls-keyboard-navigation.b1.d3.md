@answer n=1 entity=keyboard-navigation#1.1 v=yes
why: the focusable rule (tabindex, natively focusable, inert and disabled controls excluded) decides the focusable part of the actionable set the read lists
@answer n=2 entity=keyboard-navigation#1.2 v=no
why: the focus hand-off at flush when a focused node turns inert changes focus, which the read neither lists nor changes
@answer n=3 entity=keyboard-navigation#1.3 v=no
why: a hidden focused control keeps focus, but a hidden control is out of the tree so the read never lists it
@answer n=4 entity=keyboard-navigation#1.4 v=no
why: residual res-6k56erh6 ownership and the SC 2.4.3 re-assertion are a residual row elsewhere, with no rule over the read
@answer n=5 entity=keyboard-navigation#1.5 v=no
why: Tab and Shift+Tab focus movement is keyboard dispatch, and the read dispatches no key
@answer n=6 entity=keyboard-navigation#1.6 v=no
why: Tab with an open list runs a keyboard close, and per-key keyboard handling is not reached by the read
@answer n=7 entity=keyboard-navigation#1.7 v=no
why: focus staying on an open select concerns the Tab walk, which the read does not run or move
@answer n=8 entity=keyboard-navigation#1.8 v=yes
why: the focusable part of the actionable set reads the cached focusability, which the scope's focusability rule decides and must follow tabindex and disabled changes
@answer n=9 entity=keyboard-navigation#1.9 v=yes
why: button, input, select and textarea disabling sets the disabled state that the read's disabled cause and focusable listing depend on
@answer n=10 entity=keyboard-navigation#1.10 v=yes
why: disabled-ness keyed by presence on button, input, select or textarea is the rule the read's disabled cause must follow
@answer n=11 entity=keyboard-navigation#1.11 v=yes
why: a disabled row changing nothing and a hidden, inert or disabled select opening nothing are owner rules the read's verbs and causes must agree with
@answer n=12 entity=keyboard-navigation#1.12 v=yes
why: the file input's generated inner button with tabindex -1 can be listed as a button, so its tabindex decides its focusable part
@answer n=13 entity=keyboard-navigation#1.13 v=no
why: autofocus on flush is not named by the scope and the read focuses nothing
@answer n=14 entity=keyboard-navigation#1.14 v=no
why: FOCUS and FOCUSRING element states are focus styling, and the read paints nothing
@answer n=15 entity=keyboard-navigation#1.15 v=no
why: focus and focus-within matching is paint, which the scope puts out as not needed
@answer n=16 entity=keyboard-navigation#1.16 v=no
why: focus and focusin bubbling is event dispatch the read does not run
@answer n=17 entity=keyboard-navigation#2.1 v=no
why: focus reset to the body on removing a focused node is focus restoration, which the scope puts out
@answer n=18 entity=keyboard-navigation#2.2 v=no
why: clearing focus on a click on a non-interactive area is pointer focus restoration, which the scope puts out and the read never clicks
@answer n=19 entity=keyboard-navigation#2.3 v=no
why: the list close paths (Escape, second click, outside click, scroll) are not named by the scope and the read closes nothing
@answer n=20 entity=keyboard-navigation#2.4 v=yes
why: its disabled gate (a disabled form control stops the click before the walk, a disabled ancestor div does not) is the rule the read's disabled cause must match
@answer n=21 entity=keyboard-navigation#2.5 v=yes
why: a row of an open list takes a click and an option of a closed single select is refused, which the owner-served verbs and causes must agree with
@answer n=22 entity=keyboard-navigation#2.6 v=yes
why: the release choosing the row under the pointer and the highlighted row being in no snapshot are what the owner-served row and its click must agree with
@answer n=23 entity=keyboard-navigation#2.7 v=yes
why: a multiple select's row click (plain, Ctrl or Meta, Shift, disabled row changing nothing) is the owner-served row behaviour the read must agree with
@answer n=24 entity=keyboard-navigation#2.8 v=no
why: double-click timing is put out as not needed, and the read performs no click
@answer n=25 entity=keyboard-navigation#2.9 v=no
why: summary and details toggling is no part of the scope, and the read's click verb comes from listeners through the shared path, not default actions
