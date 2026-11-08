@answer n=1 entity=input-validation#1.101 v=no
why: border-radius scaling in blitz-paint; the read paints nothing and the scope names no radius
@answer n=2 entity=input-validation#1.102 v=no
why: hidden inputs skipped by painting; the read paints nothing and reads no hidden-input state
@answer n=3 entity=input-validation#1.103 v=no
why: programmatic scroll clamp and no-op on a missing node; the read runs no scroll step and its off-screen refusal is geometry, not scrolling
@answer n=4 entity=input-validation#1.104 v=no
why: an open select list's own scroll is an engine rule for every document, which the read neither runs nor reads
@answer n=5 entity=input-validation#1.105 v=no
why: form method and enctype parsing; form submission is outside the read
@answer n=6 entity=input-validation#1.106 v=no
why: submitted-control filtering; form submission is outside the read
@answer n=7 entity=input-validation#1.107 v=no
why: CRLF normalisation of submitted names and values; submission is outside the read and no control value is carried
@answer n=8 entity=input-validation#1.108 v=no
why: file input accept is not applied as a filter; the read carries no file-input value and the scope names no accept
@answer n=9 entity=input-validation#1.109 v=no
why: NativeFormData::valid always returns true; form validity is not in the read's scope
@answer n=10 entity=input-validation#1.110 v=no
why: data-dioxus-id parsing in the Dioxus mutation path; the read keys on the HTML id, and the scope names no data-dioxus-id
@answer n=11 entity=input-validation#1.111 v=no
why: the event dispatch around runtime.handle_event; the read dispatches nothing and the chunk's code does not touch the dispatch path
@answer n=12 entity=input-validation#1.112 v=yes
why: the graph readers carry handler event names from the registry's DeclaredWrite entries, so the registry's no-log, no-unwrap-on-app-input and no-NodeId rules bind the read's graph path
@answer n=13 entity=input-validation#1.113 v=no
why: dangerous_inner_html applied with set_inner_html; a mutation path the read does not run and the scope does not name
@answer n=14 entity=input-validation#1.114 v=no
why: unsupported AttributeValue types ignored by the mutation writer; the scope names disabled and inert, not this general rule
@answer n=15 entity=input-validation#1.115 v=yes
why: the chunk's disabled and inert refusals read the markup attribute, and a falsy disabled or inert value is removed before the reader sees it
@answer n=16 entity=input-validation#1.116 v=no
why: Response status RangeError in blitz-vibey-script; script execution is outside the chunk
@answer n=17 entity=input-validation#1.117 v=no
why: fetch() method TypeError in blitz-vibey-script; script fetch is outside the chunk
@answer n=18 entity=input-validation#1.118 v=no
why: selector SyntaxError in the script DOM; the read parses no selector and engine selector validation is outside the chunk
@answer n=19 entity=input-validation#1.119 v=no
why: scroll option TypeError checks in the script DOM; script scroll arguments are outside the chunk
@answer n=20 entity=input-validation#1.120 v=no
why: width and height reflection in the script DOM; the chunk reads no such attribute
@answer n=21 entity=input-validation#1.121 v=no
why: timer delay and clear-id handling in the script runtime; the chunk runs no timer
@answer n=22 entity=input-validation#1.122 v=no
why: CSS.registerProperty argument checks in the script runtime; the chunk registers no property
@answer n=23 entity=input-validation#1.123 v=no
why: style setProperty and cssText drop rules in the script DOM; the chunk writes no style
@answer n=24 entity=input-validation#1.124 v=no
why: CSSStyleDeclaration property accessors in the script runtime; the chunk uses no script accessor
@answer n=25 entity=input-validation#1.125 v=no
why: native node-argument TypeError and replaceChild checks in the script DOM; the chunk calls no such method
