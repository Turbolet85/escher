# security-plan · picked accounting

## Picked
threat-model-summary#1.1 · not touched · its claims stand; only line digits in moved files (document_loader.rs, element.rs) shift, which is the citation sweep's job
threat-model-summary#2.1 · not touched · javascript is still a non-default browser feature (default gains only mimalloc); the Cargo.toml line shift is the citation sweep's
input-validation#1.2 · proposed · the new root example multi_doc reads MULTI_DOC_N with parse().ok() and a default, which this numeric-fallback row can carry
input-validation#1.39 · not touched · the inline SVG parse claim stands; construct.rs's moved digits are the citation sweep's
input-validation#1.41 · not touched · the CSS declaration claim stands; element.rs's shifted digits are the citation sweep's
input-validation#1.45 · not touched · the bgcolor parser is unchanged; stylo.rs's shifted digits are the citation sweep's
input-validation#1.46 · not touched · the dimension parser is unchanged; stylo.rs's shifted digits are the citation sweep's
input-validation#1.47 · not touched · the SVG width/height reading is unchanged; stylo.rs's shifted digits are the citation sweep's
input-validation#1.48 · not touched · the border/margin parse is unchanged; stylo.rs's shifted digits are the citation sweep's
input-validation#1.51 · not touched · the ol start parse is unchanged; construct.rs's moved digits are the citation sweep's
input-validation#1.52 · not touched · attr_parsed and disabled-by-presence stand; element.rs's shifted digits are the citation sweep's
input-validation#1.53 · not touched · the disabled presence reading is unchanged; element.rs's shifted digits are the citation sweep's
input-validation#1.54 · not touched · the autofocus reading is unchanged; mutator.rs's shifted digits are the citation sweep's
input-validation#1.56 · not touched · the canvas src check is unchanged; mutator.rs's shifted digits are the citation sweep's
input-validation#1.62 · not touched · inert's reads stand (the dropped-node skip is in the child loops of hit_inner); node.rs's shifted digits are the citation sweep's
input-validation#1.63 · not touched · the range rules are unchanged (construct.rs only re-imports RangeInputData); mutator.rs's shifted digits around the new class arm are the citation sweep's
input-validation#1.65 · not touched · the select rules are unchanged; element.rs and mutator.rs's shifted digits are the citation sweep's
input-validation#1.67 · proposed · compute_replaced_layout now coerces a replaced element's natural height through its aspect-ratio, keeping is_usable_ratio and .max(0.0)
input-validation#1.68 · not touched · the finite inline-box height stands; the move from inline.rs to inline/layout.rs (path and digits) is named as the citation sweep's
input-validation#1.102 · proposed · attr_test.rs's added reads include a get_node(node_id).unwrap() panic path in the upstream-only runner
api-security#2.18 · not touched · outbound HTTP is unchanged; the browser Cargo.toml line shift is the citation sweep's
dependency-security#2.4 · not touched · a dated record of the 2026-10-18 sync, whose four Taffy-main commits were true as measured then; the 2026-10-24 sync gets a new record instead
dependency-security#2.5 · proposed · the 2026-10-24 sync's dependency record (tokio 1.54.1, mimalloc edge, twelve re-pointed edges, six Taffy-main waiting) follows this one
secret-management#2.2 · proposed · the new example multi_doc reads three non-secret env values outside the searched set
secret-management#2.4 · not touched · the chunk adds no env or secret read to this slice's files (the new env reads are the example's)
secret-management#2.5 · not touched · the chunk adds no env or secret read to this slice's files
secret-management#2.6 · not touched · the chunk adds no env or secret read to this slice's files
secret-management#2.8 · not touched · the chunk adds no env or secret read to this slice's files
secret-management#2.9 · not touched · the dated search and its one match stand; the chunk's only new env reads are in examples/multi_doc.rs
secret-management#2.10 · not touched · the chunk adds no env or secret read to this slice's files
error-handling#2.1 · not touched · the unimplemented paths stand; stylo.rs's shifted digits are the citation sweep's, and multi_doc's panic! belongs to the Apps and examples panic paths
logging-monitoring#1.1 · not touched · the sink, its allowlist and its readings are unchanged (escher-telemetry untouched, sink re-check holds)
logging-monitoring#1.2 · proposed · the new example multi_doc writes its output to stdout with one println!
