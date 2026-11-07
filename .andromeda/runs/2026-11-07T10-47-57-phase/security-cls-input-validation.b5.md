@answer n=1 entity=input-validation#1.101 v=no
why: method and enctype parsing in blitz-dom form.rs; the chunk changes no form path and does not reach that file
@answer n=2 entity=input-validation#1.102 v=no
why: submitted-control filtering in blitz-dom form.rs; form submission is outside the chunk
@answer n=3 entity=input-validation#1.103 v=no
why: CRLF normalization of names and values in blitz-dom form.rs; the form submission path is not reached
@answer n=4 entity=input-validation#1.104 v=no
why: the accept attribute on file inputs in blitz-dom pointer.rs; the buffered render does not reach that file
@answer n=5 entity=input-validation#1.105 v=no
why: NativeFormData::valid in events.rs; the scope marks form validity as not needed and the chunk changes no form path
@answer n=6 entity=input-validation#1.106 v=yes
why: the scope names data-dioxus-id usize parsing as a kept input rule that the buffered replay must preserve
@answer n=7 entity=input-validation#1.107 v=yes
why: the scope names dangerous_inner_html applied with set_inner_html, which the replay into MutationWriter must keep in call order
@answer n=8 entity=input-validation#1.108 v=yes
why: the scope names ignoring unsupported attribute value types, which the replay into MutationWriter must keep in call order
@answer n=9 entity=input-validation#1.109 v=yes
why: the scope names removal of falsy boolean attributes, which the replay into MutationWriter must keep on both attribute paths
@answer n=10 entity=input-validation#1.110 v=no
why: Response status check in blitz-vibey-script fetch.js; the Dioxus bridge runs no document scripts and the chunk does not reach it
@answer n=11 entity=input-validation#1.111 v=no
why: fetch method check in blitz-vibey-script fetch.js; script execution is outside the chunk
@answer n=12 entity=input-validation#1.112 v=no
why: selector SyntaxError in blitz-vibey-script; the chunk changes no selector handling
@answer n=13 entity=input-validation#1.113 v=no
why: scroll option enumerations on the JS API in blitz-vibey-script; the chunk's MountedData scroll calls are a separate path this rule does not rule over
@answer n=14 entity=input-validation#1.114 v=no
why: width and height reflection in blitz-vibey-script attributes.rs; not reached by the chunk
@answer n=15 entity=input-validation#1.115 v=no
why: timer delay and id handling in blitz-vibey-script; not reached by the chunk
@answer n=16 entity=input-validation#1.116 v=no
why: CSS.registerProperty in blitz-vibey-script css.rs; not reached by the chunk
@answer n=17 entity=input-validation#1.117 v=no
why: style setProperty and cssText in blitz-vibey-script style.rs; not reached by the chunk
@answer n=18 entity=input-validation#1.118 v=no
why: CSS property accessors on CSSStyleDeclaration in blitz-vibey-script; not reached by the chunk
@answer n=19 entity=input-validation#1.119 v=no
why: node argument checks in blitz-vibey-script dom; not reached by the chunk
@answer n=20 entity=input-validation#1.120 v=no
why: geometry.js conversions in blitz-vibey-script; not reached by the chunk
@answer n=21 entity=input-validation#1.121 v=no
why: URLSearchParams in blitz-vibey-script bootstrap; not reached by the chunk
@answer n=22 entity=input-validation#1.122 v=no
why: document selection API in blitz-vibey-script; not reached by the chunk
@answer n=23 entity=input-validation#1.123 v=no
why: sqlite parameter binding in apps/browser persistence; apps/browser is not reached
@answer n=24 entity=input-validation#1.124 v=no
why: stored URL re-parse in apps/browser persistence; apps/browser is not reached
@answer n=25 entity=input-validation#1.125 v=no
why: text and attribute escaping in blitz-dom serialize.rs; the chunk neither edits nor calls serialization, and the scope names snapshot text as a witness value, not this escaping rule
