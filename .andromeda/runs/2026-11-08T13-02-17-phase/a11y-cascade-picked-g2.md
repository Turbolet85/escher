@account n=1 entity=keyboard-navigation#1.1 v=carried
why: the focusability rule decides the focusable part of the actionable set the read lists, read through unkeyed_actionable
@account n=2 entity=keyboard-navigation#1.2 v=not_in_scope
why: the flush's focus fixup moves focus on a step, and the read runs no step and moves no focus
@account n=3 entity=keyboard-navigation#1.3 v=not_in_scope
why: a hidden focused control taking keys is key dispatch, and the read dispatches no key and lists no node outside the snapshot
@account n=4 entity=keyboard-navigation#1.7 v=not_in_scope
why: focus and the Tab walk with a select's list open are keyboard behaviour the read neither drives nor reports
@account n=5 entity=keyboard-navigation#1.8 v=not_in_scope
why: the engine's cached focusability recomputation keeps holding by its own code, a re-check every change owes
@account n=6 entity=keyboard-navigation#1.9 v=carried
why: which form controls can be disabled bounds where the read's disabled cause can stand
@account n=7 entity=keyboard-navigation#1.10 v=carried
why: the read's disabled cause follows the one presence keying of disabled-ness the call's refusal reads
@account n=8 entity=keyboard-navigation#1.11 v=carried
why: an owner-served row's or a select's verbs and causes must agree with a disabled row and a disabled, inert or hidden select acting on nothing
@account n=9 entity=keyboard-navigation#1.12 v=not_in_scope
why: the file input's inner button's tabindex is engine markup the read does not change, a re-check every change owes
@account n=10 entity=keyboard-navigation#2.4 v=not_in_scope
why: focus restoration on a pointer click is click behaviour, and the read performs no click
@account n=11 entity=keyboard-navigation#2.5 v=carried
why: a closed single select's option refused covered and an open list's row taking a click are the owner-served node's verbs and causes the read lists
@account n=12 entity=keyboard-navigation#2.6 v=not_in_scope
why: pointer-move highlight and press-release choice are pointer dispatch the read never performs
@account n=13 entity=keyboard-navigation#2.7 v=carried
why: a multiple select's row taking a click through its owner's arm is an owner-served node's verb the read lists
@account n=14 entity=keyboard-navigation#2.10 v=not_in_scope
why: the details and dialog open state is no verb, refusal cause or actionable node of the read
@account n=15 entity=keyboard-navigation#3.2 v=not_in_scope
why: range input key handling is key dispatch, and the read dispatches no key
@account n=16 entity=keyboard-navigation#3.3 v=not_in_scope
why: a closed select's key handling is key dispatch, and the read dispatches no key
@account n=17 entity=keyboard-navigation#3.5 v=carried
why: a click on an open list's row choosing it is the accepted click an owner-served row's listed verb must agree with
