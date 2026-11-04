@answer n=1 entity=input-validation#1.101 v=no
why: form method and enctype parsing is form submission, which the task drops as not needed; the probe submits no form.
@answer n=2 entity=input-validation#1.102 v=no
why: submitted-control selection belongs to form submission, not to the four links; the probe submits nothing.
@answer n=3 entity=input-validation#1.103 v=no
why: CRLF normalization of names and values is a submission encoding the probe never produces.
@answer n=4 entity=input-validation#1.104 v=no
why: the file-input accept filter is a picker path the headless probe does not open; masking of file input values is the chunk's own task item.
@answer n=5 entity=input-validation#1.105 v=no
why: NativeFormData::valid is form validity at events.rs:327-330, outside the MountedData check re-read at events.rs:162 and off the listener-map click read.
@answer n=6 entity=input-validation#1.106 v=yes
why: the probe's node-to-component read goes through data-dioxus-id, and a parse failure yielding no element id must be handled by the probe's read (task items 1 and 2).
@answer n=7 entity=input-validation#1.107 v=no
why: the dangerous_inner_html set_inner_html path is not among the element-id reads the probe uses, and the scope names no innerHTML behaviour.
@answer n=8 entity=input-validation#1.108 v=yes
why: an id attribute of an unsupported AttributeValue type is ignored by mutation_writer.rs, so the HTML id author key the probe reads may be absent and must fall back to the path (task item 2).
@answer n=9 entity=input-validation#1.109 v=no
why: falsy boolean attribute removal is not read by any of the four links, and the scope names no boolean attribute read.
@answer n=10 entity=input-validation#1.110 v=no
why: the Response status RangeError is a fetch API argument check; the probe fetches nothing and runs no document scripts.
@answer n=11 entity=input-validation#1.111 v=no
why: the fetch method check is a JS API rule; the probe makes no fetch call (task items 2 and 3 of not needed).
@answer n=12 entity=input-validation#1.112 v=no
why: the SyntaxError on an unparseable selector is the script engine's document API; the probe's harness helpers are Rust and it passes only valid fixed selectors by its own scope.
@answer n=13 entity=input-validation#1.113 v=no
why: scroll option enumerations are JS API arguments the probe never passes.
@answer n=14 entity=input-validation#1.114 v=no
why: width and height reflection is JS element reflection, which the probe does not read.
@answer n=15 entity=input-validation#1.115 v=no
why: timer delay and clear-id rules belong to the script runtime; the probe's settles run no document timers.
@answer n=16 entity=input-validation#1.116 v=no
why: CSS.registerProperty is a script API the probe does not call.
@answer n=17 entity=input-validation#1.117 v=no
why: style setProperty and cssText filtering are script-side style writes the probe does not make.
@answer n=18 entity=input-validation#1.118 v=no
why: CSS property accessors are a script-to-DOM realization the probe does not use.
@answer n=19 entity=input-validation#1.119 v=no
why: native node-argument TypeErrors are script API checks, not a Rust read the probe makes.
@answer n=20 entity=input-validation#1.120 v=no
why: geometry.js conversions are script API argument checks the probe does not reach.
@answer n=21 entity=input-validation#1.121 v=no
why: URLSearchParams is a script API the probe does not run; the native parse function is not a probe read.
@answer n=22 entity=input-validation#1.122 v=no
why: document selection is a script API the probe does not drive.
@answer n=23 entity=input-validation#1.123 v=no
why: sqlite parameter binding is browser at-rest data, dropped as not needed.
@answer n=24 entity=input-validation#1.124 v=no
why: re-parsing stored URLs on load is the browser persistence path, dropped as not needed.
@answer n=25 entity=input-validation#1.125 v=no
why: serialization escaping belongs to the document serializer, which the probe does not call; masking of its output is the chunk's own task item.
