@answer n=1 entity=input-validation#1.101 v=no
why: form submission (method and enctype) is not_needed, item 8 of the task
@answer n=2 entity=input-validation#1.102 v=no
why: form submission's submitted-controls set is not_needed, item 8 of the task
@answer n=3 entity=input-validation#1.103 v=no
why: CRLF line endings on form submission are not_needed, item 8 of the task
@answer n=4 entity=input-validation#1.104 v=no
why: the file dialog's accept filter is not_needed, item 8; it is not the masked file value read
@answer n=5 entity=input-validation#1.105 v=no
why: NativeFormData validity is not_needed, item 8 of the task
@answer n=6 entity=input-validation#1.106 v=yes
why: the graph's element ids come from data-dioxus-id, so a failed parse yielding no id must reach the option-returning paths and be refused without a panic (items 3, 12, 13)
@answer n=7 entity=input-validation#1.107 v=no
why: dangerous_inner_html is a DOM write path the vdom walk does not read, and no scope item names it
@answer n=8 entity=input-validation#1.108 v=no
why: the writer's skip of unsupported attribute types sits off the vdom dynamic_attrs read path, and no masking or refusal rule rules over it
@answer n=9 entity=input-validation#1.109 v=no
why: the writer's falsy boolean attribute clear is an in-process DOM write the graph's vdom read does not call, and scope names no such behaviour
@answer n=10 entity=input-validation#1.110 v=no
why: JS API arguments in blitz-vibey-script are not_needed, item 3 of the task
@answer n=11 entity=input-validation#1.111 v=no
why: fetch() method checks in blitz-vibey-script are not_needed, item 3 of the task
@answer n=12 entity=input-validation#1.112 v=no
why: selector SyntaxError in the script runtime is not_needed, item 3 of the task
@answer n=13 entity=input-validation#1.113 v=no
why: scroll option checks in the script runtime are not_needed, item 3 of the task
@answer n=14 entity=input-validation#1.114 v=no
why: width and height reflection in the script runtime is not_needed, item 3 of the task
@answer n=15 entity=input-validation#1.115 v=no
why: timer argument handling in the script runtime is not_needed, item 3 of the task
@answer n=16 entity=input-validation#1.116 v=no
why: CSS.registerProperty in the script runtime is not_needed, item 3 of the task
@answer n=17 entity=input-validation#1.117 v=no
why: style setProperty and cssText in the script runtime are not_needed, item 3 of the task
@answer n=18 entity=input-validation#1.118 v=no
why: CSS property accessors on the script runtime are not_needed, item 3 of the task
@answer n=19 entity=input-validation#1.119 v=no
why: node argument checks in the script runtime are not_needed, item 3 of the task
@answer n=20 entity=input-validation#1.120 v=no
why: geometry.js conversions in the script runtime are not_needed, item 3 of the task
@answer n=21 entity=input-validation#1.121 v=no
why: URLSearchParams in the script runtime is not_needed, item 3 of the task, and it carries no log site or carrier of the graph
@answer n=22 entity=input-validation#1.122 v=no
why: document selection in the script runtime is not_needed, item 3 of the task
@answer n=23 entity=input-validation#1.123 v=no
why: sqlite persistence statements are not_needed, item 1 of the task
@answer n=24 entity=input-validation#1.124 v=no
why: stored URLs in sqlite persistence are not_needed, item 1 of the task
@answer n=25 entity=input-validation#1.125 v=yes
why: snapshot text is a bounded reply carrier, and scope leaves open whether it passes the serializer's text and attribute escaping, a fork over it
