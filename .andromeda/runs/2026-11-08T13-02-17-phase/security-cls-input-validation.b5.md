@answer n=1 entity=input-validation#1.101 v=no
why: border-radius scaling is paint geometry in blitz-paint, which the read neither reaches nor re-measures
@answer n=2 entity=input-validation#1.102 v=no
why: skipping paint for hidden inputs is a painting rule, and the read runs no paint
@answer n=3 entity=input-validation#1.103 v=no
why: scroll-offset clamping and the no-op scroll to a missing node are scroll writes the read never performs, and the off-screen refusal does not depend on them
@answer n=4 entity=input-validation#1.104 v=no
why: an open select list's own scroll, wheel and clamp is not one of the read's covered, off-screen or disabled causes
@answer n=5 entity=input-validation#1.105 v=no
why: form method and enctype parsing belongs to submission, outside the read's refusals and reply
@answer n=6 entity=input-validation#1.106 v=no
why: disabled controls are excluded from submission, a rule about form submission rather than the read's disabled cause
@answer n=7 entity=input-validation#1.107 v=no
why: CRLF normalization of submitted names and values is form submission, and the read carries no control value
@answer n=8 entity=input-validation#1.108 v=no
why: the accept attribute not filtering file inputs is a file-dialog rule, and the read carries no file input value
@answer n=9 entity=input-validation#1.109 v=no
why: NativeFormData validity is form validation, and no validity cause appears among the read's refusals
@answer n=10 entity=input-validation#1.110 v=no
why: parsing data-dioxus-id is the mutation path's element mapping, and the read's ids come from the UI graph, not from this parse
@answer n=11 entity=input-validation#1.111 v=no
why: the registry dispatch around handle_event is a call's dispatch path, and the read dispatches nothing and takes its refusals from action_point
@answer n=12 entity=input-validation#1.112 v=yes
why: the registry's DeclaredWrite entries, a stable id or prefix with an event name without on, are the graph state and handler event names the read reports, so their forms and the no-log, no-panic rule bind the read's graph code
@answer n=13 entity=input-validation#1.113 v=no
why: dangerous_inner_html is a mutation write path, and the read writes nothing
@answer n=14 entity=input-validation#1.114 v=no
why: ignoring unsupported AttributeValue types is a generic mutation rule that names neither disabled nor inert, the attributes the read's refusal reads
@answer n=15 entity=input-validation#1.115 v=yes
why: the falsy-value clear for disabled and inert decides whether the attribute readers see the disabled and inert state the read's disabled cause depends on
@answer n=16 entity=input-validation#1.116 v=no
why: the Response status range check is a JS API argument rule in blitz-vibey-script, outside the Rust driver read
@answer n=17 entity=input-validation#1.117 v=no
why: the fetch method check is a JS API argument rule, and the read loads nothing
@answer n=18 entity=input-validation#1.118 v=no
why: the SyntaxError on an unparseable selector is a JS API rule, and the read parses no selector
@answer n=19 entity=input-validation#1.119 v=no
why: ScrollBehavior and ScrollLogicalPosition enumerations are JS API argument rules, outside the read
@answer n=20 entity=input-validation#1.120 v=no
why: width and height reflection is a JS API rule, outside the read
@answer n=21 entity=input-validation#1.121 v=no
why: timer delay and clear-id handling is a JS API rule, outside the read
@answer n=22 entity=input-validation#1.122 v=no
why: CSS.registerProperty argument checks are a JS API rule, outside the read
@answer n=23 entity=input-validation#1.123 v=no
why: setProperty and cssText handling of invalid declarations is a JS style rule, outside the read
@answer n=24 entity=input-validation#1.124 v=no
why: CSS property accessors on CSSStyleDeclaration are a JS script rule, outside the read
@answer n=25 entity=input-validation#1.125 v=no
why: TypeError on a non-node this or argument is a JS node-argument rule, outside the read
