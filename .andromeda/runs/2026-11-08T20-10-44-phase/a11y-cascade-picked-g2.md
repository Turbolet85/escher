@account n=1 entity=keyboard-navigation#1.1 v=carried
why: focusability is an input of the one actionable rule that `DioxusDocument::actionable_nodes` and the `actions` answer read, which the owed replay test re-asserts
@account n=2 entity=keyboard-navigation#1.9 v=carried
why: a disabled control losing focusability moves the actionable set the replayed `actions` answer is compared on
@account n=3 entity=keyboard-navigation#1.10 v=carried
why: the presence-keyed disabled gates decide that a click on a disabled form control reaches no handler, so the coverage read must not count it exercised
@account n=4 entity=keyboard-navigation#1.11 v=not_in_scope
why: the select list's disabled-row and closing behaviour is a component the chunk only drives, a re-check every change owes
@account n=5 entity=keyboard-navigation#1.12 v=not_in_scope
why: the file input's inner button tabindex is engine mutator behaviour the scope never names, a re-check every change owes
@account n=6 entity=keyboard-navigation#1.14 v=not_in_scope
why: the FOCUS and FOCUSRING element states are focus painting the chunk changes and reads nowhere
@account n=7 entity=keyboard-navigation#2.2 v=not_in_scope
why: focus clearing on a non-interactive click is engine pointer behaviour the scope never names; the chunk changes no focus
@account n=8 entity=keyboard-navigation#2.4 v=not_in_scope
why: a button click's focus and submit is engine behaviour the chunk only drives; the scope names no focus outcome
@account n=9 entity=keyboard-navigation#2.5 v=not_in_scope
why: a select click's focus and list opening is a component the chunk only uses, unnamed by the scope
@account n=10 entity=keyboard-navigation#2.7 v=not_in_scope
why: a multiple select's row click is a component the chunk only uses, unnamed by the scope
@account n=11 entity=keyboard-navigation#2.8 v=not_in_scope
why: the double-click window on the document clock is engine pointer behaviour the scope neither names nor re-measures
@account n=12 entity=keyboard-navigation#2.11 v=not_in_scope
why: checkbox and radio click focus is engine behaviour the chunk only drives, a re-check every change owes
@account n=13 entity=keyboard-navigation#2.13 v=not_in_scope
why: range input click focus and value is engine behaviour the chunk only drives, a re-check every change owes
@account n=14 entity=keyboard-navigation#3.1 v=not_in_scope
why: text input key handling is engine key behaviour; the chunk adds no key behaviour
@account n=15 entity=keyboard-navigation#3.2 v=not_in_scope
why: range input key stepping is engine key behaviour; the chunk adds no key behaviour
@account n=16 entity=keyboard-navigation#3.3 v=not_in_scope
why: single select keys and typeahead are engine key behaviour; the chunk adds no key behaviour
