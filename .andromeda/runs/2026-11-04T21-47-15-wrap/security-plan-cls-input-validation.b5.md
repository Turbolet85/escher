@answer n=1 entity=input-validation#1.101 v=no
why: the session changes no packages/blitz-dom file, so the method and enctype rule at form.rs:80-111 and :373-382 still holds.
@answer n=2 entity=input-validation#1.102 v=no
why: the session changes no packages/blitz-dom file, so the submitted-controls rule at form.rs:224-240 still holds.
@answer n=3 entity=input-validation#1.103 v=no
why: the session changes no packages/blitz-dom file, so the CRLF normalization at form.rs:427-464 still holds.
@answer n=4 entity=input-validation#1.104 v=no
why: the session changes no packages/blitz-dom file, so the unfiltered accept attribute at pointer.rs:1050-1052 still holds.
@answer n=5 entity=input-validation#1.105 v=no
why: the session changes no packages/dioxus-native-dom file, so the NativeFormData::valid stub at events.rs:327-330 still holds even though the MountedData coordinates in events.rs are re-measured.
@answer n=6 entity=input-validation#1.106 v=no
why: the session re-measures dioxus_document.rs:295 and :297 but leaves the data-dioxus-id usize parse at :48-55 unchanged, as untouched item 4 records.
@answer n=7 entity=input-validation#1.107 v=no
why: mutation_writer.rs is not among the changed files, so the set_inner_html application at :430-431 still holds.
@answer n=8 entity=input-validation#1.108 v=no
why: mutation_writer.rs is not among the changed files, so the unsupported AttributeValue ignore at :306-308 still holds.
@answer n=9 entity=input-validation#1.109 v=no
why: mutation_writer.rs and dioxus_falsy_boolean_attrs.rs are unchanged, and the new dioxus_runtime_probe target does not touch the falsy boolean path.
@answer n=10 entity=input-validation#1.110 v=no
why: the session changes no packages/blitz-vibey-script file, so the Response status RangeError at fetch.js:38-39 still holds.
@answer n=11 entity=input-validation#1.111 v=no
why: the session changes no packages/blitz-vibey-script file, so the fetch method TypeError at fetch.js:81-83 still holds.
@answer n=12 entity=input-validation#1.112 v=no
why: the session changes no packages/blitz-vibey-script file, so the selector SyntaxError at document.rs:357-376 still holds.
@answer n=13 entity=input-validation#1.113 v=no
why: the session changes no packages/blitz-vibey-script file, so the scroll option TypeError checks at scrolling.rs still hold.
@answer n=14 entity=input-validation#1.114 v=no
why: the session changes no packages/blitz-vibey-script file, so the width and height reflection at attributes.rs:289-305 and :343-354 still holds.
@answer n=15 entity=input-validation#1.115 v=no
why: the session changes no packages/blitz-vibey-script file, so the timer delay and clear handling at timers.rs still holds.
@answer n=16 entity=input-validation#1.116 v=no
why: the session changes no packages/blitz-vibey-script file, and the lock moves only pulldown-cmark and zip, so the CSS.registerProperty rule at css.rs:81-131 still holds.
@answer n=17 entity=input-validation#1.117 v=no
why: the session changes no packages/blitz-vibey-script file, so the setProperty and cssText handling at style.rs still holds.
@answer n=18 entity=input-validation#1.118 v=no
why: the session changes no packages/blitz-vibey-script file, and the lock moves no Stylo crate, so the accessor set at style.js:26-48 and css.rs:8-18 still holds.
@answer n=19 entity=input-validation#1.119 v=no
why: the session changes no packages/blitz-vibey-script file, so the node-argument TypeError and replaceChild checks at mod.rs and node.rs still hold.
@answer n=20 entity=input-validation#1.120 v=no
why: the session changes no packages/blitz-vibey-script file, so the BigInt and dictionary rejection at geometry.js:27-46 still holds.
@answer n=21 entity=input-validation#1.121 v=no
why: the session changes no packages/blitz-vibey-script file, and the url_search_params bootstrap and its test at url_search_params.rs:61-78 are unchanged.
@answer n=22 entity=input-validation#1.122 v=no
why: the session changes no packages/blitz-vibey-script file, so the setBaseAndExtent argument checks at selection.rs:69-119 still hold.
@answer n=23 entity=input-validation#1.123 v=no
why: apps/browser/persistence is unchanged and the lock moves only pulldown-cmark and zip, so the params! bindings still hold.
@answer n=24 entity=input-validation#1.124 v=no
why: apps/browser/persistence is unchanged, so the re-parse and skip of unparsable stored URLs at lib.rs:184-188 still holds.
@answer n=25 entity=input-validation#1.125 v=no
why: the session changes no packages/blitz-dom file, so the unescaped raw-text elements in serialize.rs:138-159 and :182-196 still hold.
