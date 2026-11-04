@answer n=1 entity=input-validation#1.101 v=no
why: form method and enctype parsing on submission is a navigation surface the probe does not drive (not_needed)
@answer n=2 entity=input-validation#1.102 v=no
why: submitted-control filtering on form submission is outside the probe, which submits no forms
@answer n=3 entity=input-validation#1.103 v=no
why: CRLF normalization of names and values at form submission is never reached by the probe's four links
@answer n=4 entity=input-validation#1.104 v=no
why: the accept filter on file-input clicks is not read by the probe, which opens no file picker, and file-input value masking is a different rule
@answer n=5 entity=input-validation#1.105 v=no
why: NativeFormData validity is a handler-data reading the probe does not make, and events.rs is re-read only at its line-162 hazard, so this is a re-check every change owes
@answer n=6 entity=input-validation#1.106 v=yes
why: the probe reads element ids through data-dioxus-id and try_element_to_node_id, so a failed usize parse that yields no element id is a value its node-to-component read must handle
@answer n=7 entity=input-validation#1.107 v=no
why: set_inner_html for dangerous_inner_html is a render path the probe neither passes nor reads, and the scope names no such behaviour
@answer n=8 entity=input-validation#1.108 v=no
why: ignored unsupported attribute value types affect only mutations the probe does not craft, and the scope names no such read
@answer n=9 entity=input-validation#1.109 v=no
why: falsy boolean attribute clearing is a mutation-writer behaviour the probe neither reads nor passes, and no new probe test asserts boolean attributes
@answer n=10 entity=input-validation#1.110 v=no
why: Response status RangeError is a script-runtime API the probe never runs, since it fetches nothing and runs no document scripts (not_needed)
@answer n=11 entity=input-validation#1.111 v=no
why: fetch method checks belong to the script runtime the probe does not run, and it fetches nothing (not_needed)
@answer n=12 entity=input-validation#1.112 v=no
why: the JS querySelector SyntaxError is a script API, and the harness's invalid-selector panic is a separate helper the scope already states, so this entity does not bind
@answer n=13 entity=input-validation#1.113 v=no
why: scroll option enumerations are script-API argument checks outside the probe's runtime and reads
@answer n=14 entity=input-validation#1.114 v=no
why: width and height reflection is a script-side attribute reflection the probe neither reads nor passes
@answer n=15 entity=input-validation#1.115 v=no
why: timer delay and clear-id checks belong to the script runtime, which the probe does not run
@answer n=16 entity=input-validation#1.116 v=no
why: CSS.registerProperty argument checks belong to the script runtime, which the probe does not run
@answer n=17 entity=input-validation#1.117 v=no
why: setProperty and cssText handling are script-side style writes the probe does not make
@answer n=18 entity=input-validation#1.118 v=no
why: CSSStyleDeclaration accessors are a script-side surface the probe neither runs nor reads
@answer n=19 entity=input-validation#1.119 v=no
why: native-node argument TypeErrors are script-API checks the probe does not reach
@answer n=20 entity=input-validation#1.120 v=no
why: geometry.js conversions are script-side dictionary checks the probe does not reach
@answer n=21 entity=input-validation#1.121 v=no
why: URLSearchParams and searchParams live-binding checks are a script-side surface the probe does not run
@answer n=22 entity=input-validation#1.122 v=no
why: document selection setBaseAndExtent checks are a script-side API outside the probe's four links
@answer n=23 entity=input-validation#1.123 v=no
why: sqlite bind parameters are the browser's at-rest persistence, which the probe neither reads nor writes (not_needed)
@answer n=24 entity=input-validation#1.124 v=no
why: stored-URL re-parse on load is the browser's persistence path, which the probe does not touch (not_needed)
@answer n=25 entity=input-validation#1.125 v=no
why: text and attribute escaping in serialized output is a path the probe neither reads nor serializes, so no link reaches it
