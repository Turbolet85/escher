@answer n=1 entity=input-validation#1.101 v=no
why: form method and enctype parsing in blitz-dom is not reached and the chunk edits no form path
@answer n=2 entity=input-validation#1.102 v=no
why: submitted-control selection is form submission, which the scope excludes
@answer n=3 entity=input-validation#1.103 v=no
why: CRLF normalization of form names and values is form submission, which the scope excludes
@answer n=4 entity=input-validation#1.104 v=no
why: the file input accept handling in blitz-dom pointer.rs is not reached by the buffered render
@answer n=5 entity=input-validation#1.105 v=no
why: NativeFormData validity is named as not needed and events.rs is not edited by this chunk
@answer n=6 entity=input-validation#1.106 v=yes
why: the scope names data-dioxus-id usize parsing in dioxus_document.rs and the buffered replay must keep it
@answer n=7 entity=input-validation#1.107 v=yes
why: the scope names dangerous_inner_html applied with set_inner_html and the replay must keep that in call order
@answer n=8 entity=input-validation#1.108 v=yes
why: the scope names unsupported attribute value types being ignored and the replay into MutationWriter must keep that
@answer n=9 entity=input-validation#1.109 v=yes
why: the scope names falsy boolean attributes being removed and the buffer must carry values so the replay still clears them
@answer n=10 entity=input-validation#1.110 v=no
why: Response status checks in blitz-vibey-script fetch.js are not reached and the bridge runs no document scripts
@answer n=11 entity=input-validation#1.111 v=no
why: fetch method checks in blitz-vibey-script fetch.js are not reached and the bridge runs no document scripts
@answer n=12 entity=input-validation#1.112 v=no
why: the script selector API in blitz-vibey-script is not reached by the Dioxus bridge
@answer n=13 entity=input-validation#1.113 v=no
why: scroll option checks in the script scrolling.rs are a different path from the MountedData scroll operations and are not reached
@answer n=14 entity=input-validation#1.114 v=no
why: width and height reflection in blitz-vibey-script attributes.rs is not reached
@answer n=15 entity=input-validation#1.115 v=no
why: script timers in blitz-vibey-script are not reached by the Dioxus bridge
@answer n=16 entity=input-validation#1.116 v=no
why: CSS.registerProperty in blitz-vibey-script is not reached by the Dioxus bridge
@answer n=17 entity=input-validation#1.117 v=no
why: script style setProperty and cssText in blitz-vibey-script are not reached and the chunk edits no style path
@answer n=18 entity=input-validation#1.118 v=no
why: script CSS property accessors in blitz-vibey-script are not reached by the Dioxus bridge
@answer n=19 entity=input-validation#1.119 v=no
why: script node argument checks in blitz-vibey-script are not reached by the Dioxus bridge
@answer n=20 entity=input-validation#1.120 v=no
why: geometry.js conversions are script-side and not reached by the buffered render
@answer n=21 entity=input-validation#1.121 v=no
why: the script URLSearchParams bootstrap is not reached by the Dioxus bridge
@answer n=22 entity=input-validation#1.122 v=no
why: the script selection API in blitz-vibey-script is not reached by the Dioxus bridge
@answer n=23 entity=input-validation#1.123 v=no
why: sqlite persistence in apps/browser is not reached, as the scope names
@answer n=24 entity=input-validation#1.124 v=no
why: stored-URL re-parsing in apps/browser persistence is not reached, as the scope names
@answer n=25 entity=input-validation#1.125 v=no
why: blitz-dom serialize.rs is not edited and the buffered render writes no serialized output
