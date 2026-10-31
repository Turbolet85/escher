# Classified — security · input-validation.b5

## Answers
input-validation#1.93 · no · dangerous_inner_html is an app-authored Dioxus mutation in mutation_writer.rs, a path the chunk does not reach and adds no inner HTML to
input-validation#1.94 · no · dropping unsupported AttributeValue types is the Dioxus mutation writer, which the chunk does not touch
input-validation#1.95 · yes · the removal of falsy values for disabled and inert on the Dioxus mutation path is what makes the chunk's presence read of those names on the select, its groups and options honour a false value
input-validation#1.96 · no · Response status RangeError is a JS API argument check in blitz-vibey-script, and the list adds no script path
input-validation#1.97 · no · fetch() method check is a JS fetcher rule, and the list fetches nothing
input-validation#1.98 · no · selector SyntaxError is a script DOM API rule, and the list takes no call-supplied selector
input-validation#1.99 · no · ScrollBehavior and ScrollLogicalPosition checks belong to the JS scrolling API, and the chunk adds no script scroll API
input-validation#1.100 · no · width and height reflection is a JS attribute parse, and the chunk reads size, not width or height
input-validation#1.101 · no · timer delay and clear-id checks are JS runtime rules, and the list adds no timer
input-validation#1.102 · no · CSS.registerProperty is a script API the chunk does not touch
input-validation#1.103 · no · setProperty and cssText drops are script style APIs, and default.css declarations take the CSS parsing rule rather than this path
input-validation#1.104 · no · CSS property accessors on CSSStyleDeclaration are a script API the chunk does not touch
input-validation#1.105 · no · TypeError on a non-node this or argument is a script node-API rule, and the chunk adds no script path
input-validation#1.106 · no · geometry.js conversions are JS dictionary and number rules, and the list's geometry is engine Rust code
input-validation#1.107 · no · document.getSelection is a script selection API, and the list paints option text and writes no text selection
input-validation#1.108 · no · sqlite params! binding is browser persistence, which the chunk does not reach
input-validation#1.109 · no · stored-URL reparse is browser persistence, outside the select
input-validation#1.110 · no · text and attribute escaping in serialize.rs is HTML serialization output, which the list's named carriers do not use
input-validation#1.111 · no · inline SVG well-formedness is a serialization rule the list does not touch
input-validation#1.112 · no · the harness query helper panics only on an unparseable selector, and the chunk's selectors are fixed valid literals such as select#flight-type
input-validation#1.113 · no · WPT timeout-quarantine entries, and the chunk does not edit the runner
input-validation#1.114 · no · WPT fuzzy ranges, and the chunk does not edit the runner
input-validation#1.115 · no · WPT checkLayout attribute parse, and the chunk does not edit the runner
input-validation#1.116 · no · WPT wrapper HTML escaping, and the chunk does not edit the runner
input-validation#1.117 · no · WPT script type rule, and the chunk does not edit the runner
