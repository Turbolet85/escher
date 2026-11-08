@answer n=1 entity=input-validation#1.101 v=no
why: border radius scaling is a blitz-paint value rule the read never paints or reads
@answer n=2 entity=input-validation#1.102 v=no
why: hidden inputs not being painted is a paint rule the read's actionable list and refusals do not rule over
@answer n=3 entity=input-validation#1.103 v=no
why: scroll clamping and the missing-node scroll are step effects the read never runs
@answer n=4 entity=input-validation#1.104 v=no
why: an open select list's own scroll is an engine scroll behaviour the read does not reach and the scope does not name
@answer n=5 entity=input-validation#1.105 v=no
why: method and enctype parsing belongs to form submission, which the read does not run
@answer n=6 entity=input-validation#1.106 v=no
why: the submitted-controls exclusion is a submission rule, not the disabled refusal the read names
@answer n=7 entity=input-validation#1.107 v=no
why: CRLF normalization applies at form submission, which the read never performs
@answer n=8 entity=input-validation#1.108 v=no
why: the accept filter belongs to the native file dialog, which is not needed for this chunk, and the read carries no file value
@answer n=9 entity=input-validation#1.109 v=no
why: form data validity is a submission-side stub outside the read's answer
@answer n=10 entity=input-validation#1.110 v=no
why: data-dioxus-id parsing is a bridge mutation rule, and the read's stable id is an author key or a path, not this id
@answer n=11 entity=input-validation#1.111 v=no
why: the registry dispatch around handle_event is an event path the read never reaches, since the read dispatches nothing
@answer n=12 entity=input-validation#1.112 v=yes
why: the registry's stable element ids, declared event names and its no-log, no-print, no-env and no-unwrap rule on app input are the UI graph state the read reports, which item 12 names
@answer n=13 entity=input-validation#1.113 v=no
why: dangerous inner html is a mutation-writer rule the read neither produces nor reads
@answer n=14 entity=input-validation#1.114 v=no
why: ignoring unsupported attribute value types is a generic mutation rule that does not name disabled or inert, and the readers consume the attribute as written
@answer n=15 entity=input-validation#1.115 v=yes
why: a falsy disabled or inert value is cleared rather than written, so the disabled and inert readers the chunk keeps must see that attribute as absent
@answer n=16 entity=input-validation#1.116 v=no
why: the Response status range is a JS API argument rule outside the read
@answer n=17 entity=input-validation#1.117 v=no
why: the fetch method rule is a JS API argument rule outside the read
@answer n=18 entity=input-validation#1.118 v=no
why: the unparseable-selector SyntaxError is a JS API rule outside the read
@answer n=19 entity=input-validation#1.119 v=no
why: scroll option enumerations are a JS API rule outside the read
@answer n=20 entity=input-validation#1.120 v=no
why: width and height reflection is a JS attribute rule outside the read
@answer n=21 entity=input-validation#1.121 v=no
why: timer delay and clear-id rules are a JS runtime rule outside the read
@answer n=22 entity=input-validation#1.122 v=no
why: CSS.registerProperty validation is a JS runtime rule outside the read
@answer n=23 entity=input-validation#1.123 v=no
why: style setProperty and cssText validation is a JS style rule outside the read
@answer n=24 entity=input-validation#1.124 v=no
why: CSS property accessors are a script-to-DOM realization outside the read
@answer n=25 entity=input-validation#1.125 v=no
why: the native node-argument TypeError is a JS DOM rule outside the read
