# Classified — architecture · design-philosophy.b1

## Answers
design-philosophy#1.1 · no · the session moves no crate boundary (no crate added or retired, blitz-vibey-script's split stays inside its crate) and none of the files this entity cites is reached
design-philosophy#2.1 · yes · stale · blitz-vibey-script's document.rs (gained lines at 16, 163-169, 210-211 and 222) moves the document.rs:251-261 citation, runtime.rs is split into modules, lib.rs is a merge surface, and dioxus_document.rs (a conflict surface) is reached whole for its :41 and :67-68 citations
design-philosophy#3.1 · yes · stale · stylo.rs (a merge surface that gained a line at 510) moves the :921-927 citation, and element.rs (a merge surface) is reached whole for the :832-854 citation
design-philosophy#4.1 · yes · stale · construct.rs (a merge surface that gained lines at 313-317, 860 and 867-872) moves the :600-605 and :939-941 citations, and node/element.rs (a merge surface that gained ElementData::flush_open_state) is reached whole for its citations
design-philosophy#5.1 · yes · stale · apps/browser/src/main.rs and apps/browser/src/tab.rs (both merge surfaces, tab.rs changed so tab lenses survive a props replacement) are reached whole for the :5 and :126-154 citations
