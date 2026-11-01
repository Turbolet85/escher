# Classified — security-plan · input-validation.b2

## Answers
input-validation#1.26 · no · blitz-shell net.rs is not in Reached files, and its silent return on a bad data URL still holds
input-validation#1.27 · no · blitz-vibey-script fetch.rs is not reached, and its InvalidData claim holds on it
input-validation#1.28 · no · the file: URL gate files (fetch, runtime/fetch, modules, document, lib, tests/dom.rs) are not reached
input-validation#1.29 · no · blitz-vibey-script document.rs is not reached, and the script type allow-list holds
input-validation#1.30 · no · blitz-html html_sink.rs is not reached, and scripting-disabled parsing holds
input-validation#1.31 · yes · add-to · a failed, empty or non-CSS stylesheet load now answers and no longer blocks render, which this fetched-stylesheet row must carry (net.rs is a merge surface)
input-validation#1.32 · yes · stale · net.rs is a merge surface and :552-555 sits past the added lines at :144-147
input-validation#1.33 · yes · stale · net.rs is a merge surface and :575-610, including the moved :576-579, sits past the added lines
input-validation#1.34 · no · apps/browser favicon.rs is not reached, and its decode check holds
input-validation#1.35 · yes · stale · net.rs is a merge surface and :314-335 sits past the added lines at :144-147
input-validation#1.36 · no · blitz-dom util.rs is not in Reached files, and the short-bytes and WOFF fallback claim holds
input-validation#1.37 · yes · add-to · F6: an unresolvable stylesheet href starts no load and no panic, which belongs beside this unresolvable-URL row (net.rs is a merge surface)
input-validation#1.38 · no · node/svg.rs is not reached, and the UTF-8 and DTD claim holds
input-validation#1.39 · no · node/svg.rs is not reached, and the SVGZ detection holds
input-validation#1.40 · no · node/svg.rs is not reached, and the viewBox checks hold
input-validation#1.41 · yes · stale · layout/construct.rs is a merge surface, so its :456-476 citation is not held
input-validation#1.42 · yes · add-to · F7: mix-blend-mode and isolation from page CSS now paint, validated by Stylo's parse, which this parse-validation row must carry (resolved_style.rs is a merge surface)
input-validation#1.43 · yes · stale · element.rs is a merge surface, so its :807-823 and :873-877 citations are not held
input-validation#1.44 · no · cssom.rs is not reached, and the selectorText rejection holds
input-validation#1.45 · no · query_selector.rs is not reached, and the parse-error return holds
input-validation#1.46 · yes · stale · document.rs is a merge surface and :1685-1690 sits past the moved document.rs citations
input-validation#1.47 · yes · stale · stylo.rs is a merge surface and :906-927 and :1210-1217 sit past the moved :449
input-validation#1.48 · yes · stale · stylo.rs is a merge surface and :929-958 and :1068-1074 sit past the moved :449
input-validation#1.49 · yes · stale · stylo.rs is a merge surface and :960-990 sits past the moved :449
input-validation#1.50 · yes · stale · stylo.rs is a merge surface and :1157 and :1191 sit past the moved :449
