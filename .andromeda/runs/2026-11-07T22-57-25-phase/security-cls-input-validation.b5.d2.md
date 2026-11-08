@answer n=1 entity=input-validation#1.101 v=no
why: method and enctype parsing is form submission, which the scope puts out of reach (not_needed 8)
@answer n=2 entity=input-validation#1.102 v=no
why: submitted-control selection is form submission, which the scope puts out of reach (not_needed 8)
@answer n=3 entity=input-validation#1.103 v=no
why: CRLF normalization of names and values before submission is form submission, which the scope puts out of reach (not_needed 8)
@answer n=4 entity=input-validation#1.104 v=no
why: the accept attribute filters the file dialog, which the scope excludes (not_needed 8), and it does not touch the file input value
@answer n=5 entity=input-validation#1.105 v=no
why: NativeFormData validity in dioxus-native is named as out of reach (not_needed 8)
@answer n=6 entity=input-validation#1.106 v=yes
why: a data-dioxus-id that fails to parse yields no element id, so the graph reader and element_id must take the option path and not panic (task items 3 and 12)
@answer n=7 entity=input-validation#1.107 v=no
why: set_inner_html for dangerous_inner_html is the bridge write path, which the chunk leaves unchanged and the scope does not name
@answer n=8 entity=input-validation#1.108 v=no
why: skipping unsupported AttributeValue types is the bridge write path, which the chunk leaves unchanged; the graph reads dynamic_attrs, not the written attribute
@answer n=9 entity=input-validation#1.109 v=no
why: the falsy boolean attribute clear is an in-process write path of the bridge that the chunk leaves unchanged, and the scope names no boolean attribute behaviour
@answer n=10 entity=input-validation#1.110 v=no
why: Response status checking is a JS API argument check in blitz-vibey-script, which the scope puts out of reach as script execution (not_needed 3)
@answer n=11 entity=input-validation#1.111 v=no
why: fetch method checking is a JS API argument check in blitz-vibey-script, which the scope puts out of reach as script execution (not_needed 3)
@answer n=12 entity=input-validation#1.112 v=no
why: selector SyntaxError handling is a DOM method in blitz-vibey-script, which the scope puts out of reach as script execution (not_needed 3)
@answer n=13 entity=input-validation#1.113 v=no
why: scroll option checking is a JS API argument check in blitz-vibey-script, which the scope puts out of reach as script execution (not_needed 3)
@answer n=14 entity=input-validation#1.114 v=no
why: width and height reflection is a JS API behaviour in blitz-vibey-script, which the scope puts out of reach as script execution (not_needed 3)
@answer n=15 entity=input-validation#1.115 v=no
why: timer delay and clear handling is a JS API behaviour in blitz-vibey-script, which the scope puts out of reach as script execution (not_needed 3)
@answer n=16 entity=input-validation#1.116 v=no
why: CSS.registerProperty checking is a JS API behaviour in blitz-vibey-script, which the scope puts out of reach as script execution (not_needed 3)
@answer n=17 entity=input-validation#1.117 v=no
why: style setProperty and cssText handling is a JS API behaviour in blitz-vibey-script, which the scope puts out of reach as script execution (not_needed 3)
@answer n=18 entity=input-validation#1.118 v=no
why: CSS property accessors on CSSStyleDeclaration are a script-side realization in blitz-vibey-script, which the scope puts out of reach as script execution (not_needed 3)
@answer n=19 entity=input-validation#1.119 v=no
why: native node argument checks are a JS API behaviour in blitz-vibey-script, which the scope puts out of reach as script execution (not_needed 3)
@answer n=20 entity=input-validation#1.120 v=no
why: geometry.js conversions are a JS API behaviour in blitz-vibey-script, which the scope puts out of reach as script execution (not_needed 3)
@answer n=21 entity=input-validation#1.121 v=no
why: the URLSearchParams bootstrap is script execution in blitz-vibey-script, out of reach (not_needed 3), and its engine change is already ratified, not a new widening
@answer n=22 entity=input-validation#1.122 v=no
why: document selection handling is a JS API behaviour in blitz-vibey-script, which the scope puts out of reach as script execution (not_needed 3)
@answer n=23 entity=input-validation#1.123 v=no
why: sqlite SQL value binding is the browser app's history persistence, which the scope puts out of reach (not_needed 1)
@answer n=24 entity=input-validation#1.124 v=no
why: stored URL re-parsing is the browser app's history persistence, which the scope puts out of reach (not_needed 1)
@answer n=25 entity=input-validation#1.125 v=yes
why: the scope names Snapshot::to_text but leaves open whether its text passes through blitz-dom's HTML serializer, so whether node text inside style or script reaches it unescaped is a fork that binds
