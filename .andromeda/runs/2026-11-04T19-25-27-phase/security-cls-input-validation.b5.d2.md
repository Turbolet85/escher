@answer n=1 entity=input-validation#1.101 v=no
why: form method and enctype parsing belongs to form submission, which the task drops as not needed (item 8), and the probe submits no form
@answer n=2 entity=input-validation#1.102 v=no
why: submitted-control selection is form submission, dropped by the task as not needed (item 8)
@answer n=3 entity=input-validation#1.103 v=no
why: CRLF normalization applies at form submission, which the probe never runs
@answer n=4 entity=input-validation#1.104 v=no
why: the accept filter on the file-picker click path is not reached; the file input value masking rule (task item 4) is a different rule
@answer n=5 entity=input-validation#1.105 v=no
why: NativeFormData::valid is a form-event validity rule outside the MountedData coordinate that the events.rs re-read checks, and the probe reads listeners, not form validity
@answer n=6 entity=input-validation#1.106 v=yes
why: the probe's node-to-component link reads data-dioxus-id, which the task names as its element-id read, so the probe must handle a parse failure yielding no element id
@answer n=7 entity=input-validation#1.107 v=no
why: dangerous_inner_html is a mutation the probe neither makes nor reads
@answer n=8 entity=input-validation#1.108 v=no
why: the ignored-unsupported-type rule covers attribute value types the scope does not name; data-dioxus-id is a text attribute and the probe makes no such write
@answer n=9 entity=input-validation#1.109 v=no
why: falsy boolean-attribute removal covers attribute names the scope does not name, and the chunk neither reads nor writes them
@answer n=10 entity=input-validation#1.110 v=no
why: the Response status RangeError is a JS fetch API check; the probe fetches nothing and runs no document scripts (task not_needed items 2 and 3)
@answer n=11 entity=input-validation#1.111 v=no
why: the fetch method check is a JS API check; the probe fetches nothing and runs no document scripts (task not_needed items 2 and 3)
@answer n=12 entity=input-validation#1.112 v=no
why: the JS selector SyntaxError is a document-script API; the harness query helpers are a different component, and the probe's valid-selector rule is the task's own item 8, not this entity
@answer n=13 entity=input-validation#1.113 v=no
why: scroll option checks are JS API checks for document scripts, which the task drops (not_needed item 3)
@answer n=14 entity=input-validation#1.114 v=no
why: width and height reflection is a JS API check for document scripts, which the task drops (not_needed item 3)
@answer n=15 entity=input-validation#1.115 v=no
why: timer delay and clear-id checks are JS API checks for document scripts, which the task drops (not_needed item 3)
@answer n=16 entity=input-validation#1.116 v=no
why: CSS.registerProperty is a JS API check for document scripts, which the task drops (not_needed item 3)
@answer n=17 entity=input-validation#1.117 v=no
why: style setProperty and cssText checks are JS API checks for document scripts, which the task drops (not_needed item 3)
@answer n=18 entity=input-validation#1.118 v=no
why: script CSS property accessors are reached only by page scripts, which the probe does not run
@answer n=19 entity=input-validation#1.119 v=no
why: node-argument type checks on JS native methods are JS API checks for document scripts, which the task drops (not_needed item 3)
@answer n=20 entity=input-validation#1.120 v=no
why: geometry.js conversions are JS API checks for document scripts, which the task drops (not_needed item 3)
@answer n=21 entity=input-validation#1.121 v=no
why: URLSearchParams and URL searchParams are JS API surfaces for page scripts, which the probe does not run (not_needed item 3)
@answer n=22 entity=input-validation#1.122 v=no
why: the document selection API is a JS API surface for page scripts, which the probe does not run (not_needed item 3)
@answer n=23 entity=input-validation#1.123 v=no
why: SQL bind parameters belong to the browser app's sqlite store, which the probe neither reads nor writes (not_needed item 1)
@answer n=24 entity=input-validation#1.124 v=no
why: stored-URL re-parsing on load belongs to the browser app's persistence, which the probe neither reads nor writes (not_needed item 1)
@answer n=25 entity=input-validation#1.125 v=no
why: text and attribute escaping in the serializer is a rule the scope never names, and the probe reads no serialized document output as a verdict
