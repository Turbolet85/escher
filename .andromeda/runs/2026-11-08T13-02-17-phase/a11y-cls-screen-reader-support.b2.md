@answer n=1 entity=screen-reader-support#6.1 v=no
why: the HTML tag name of element nodes is not named by the read's scope, which lists ids, verbs and causes over the snapshot model
@answer n=2 entity=screen-reader-support#6.2 v=yes
why: the snapshot's tree focus falls back to the Window node u64::MAX, which the read must not list as a node, and the read consumes that focus
@answer n=3 entity=screen-reader-support#6.3 v=no
why: a test's stated goal about navigation is not a rule the read's code or tests must honour
@answer n=4 entity=screen-reader-support#7.1 v=no
why: the accesskit_xplat event routing is the platform adapter, which the chunk builds and reads nothing of
@answer n=5 entity=screen-reader-support#8.1 v=no
why: the adapter's creation from the window handle is the platform adapter, which the chunk builds and reads nothing of
@answer n=6 entity=screen-reader-support#8.2 v=yes
why: the AccessKit tree and author_id the read consumes exist only behind the accessibility feature, so the new code and its stand proof must sit inside that feature
@answer n=7 entity=screen-reader-support#9.1 v=no
why: the lang attribute and its inheritance are language behaviour, which the read does not reach
@answer n=8 entity=screen-reader-support#9.2 v=no
why: the default stylesheet's dir attribute rules are direction behaviour, which the read does not reach
@answer n=9 entity=screen-reader-support#9.3 v=no
why: the fixtures that set an html lang attribute are language facts the read does not reach
@answer n=10 entity=screen-reader-support#9.4 v=no
why: pages declaring lang en are language facts the read does not reach
@answer n=11 entity=screen-reader-support#10.1 v=no
why: the absence of html lang in the other s02 fixtures is a language fact the read does not reach
@answer n=12 entity=screen-reader-support#11.1 v=no
why: the sr-only tooltip element in a fixture is a text alternative that is not actionable, so the read does not list it
@answer n=13 entity=screen-reader-support#11.2 v=no
why: the descriptive aria-labels on the google links are names, and the read lists ids not names
@answer n=14 entity=screen-reader-support#11.3 v=no
why: the aria-hidden decoration and aria-atomic on suggestion options are not named by the read's visibility rule or its scope
@answer n=15 entity=screen-reader-support#11.4 v=no
why: the aria-hidden bar spans of the burger are not named by the read's visibility rule or its scope
@answer n=16 entity=screen-reader-support#11.5 v=no
why: the alt text of fixture images is a text alternative the read does not reach
@answer n=17 entity=screen-reader-support#11.6 v=no
why: the missing alt attribute on a fixture image is a text alternative the read does not reach
@answer n=18 entity=screen-reader-support#12.1 v=no
why: IME enabling on focusing a text input is input method behaviour, which the read does not reach since it runs no step
@answer n=19 entity=screen-reader-support#12.2 v=no
why: IME being on for a text input focused before its editor exists is input method behaviour, which the read does not reach
@answer n=20 entity=screen-reader-support#12.3 v=no
why: IME commit, preedit and disable events are input method behaviour, which the read does not reach
@answer n=21 entity=screen-reader-support#13.1 v=no
why: the absence of accessibility integration in the listed s06 files is a survey record that states no rule the read must honour
