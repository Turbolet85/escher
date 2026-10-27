# Classified — architecture · cross-cutting-patterns.b1

## Answers
cross-cutting-patterns#1.1 · yes · stale · apps/browser/src/main.rs, a merge surface, is reached whole, so its cited lines 23-24, 48 and 79-89 may have moved
cross-cutting-patterns#2.1 · yes · stale · blitz-vibey-script runtime.rs, split into modules and now 151 lines, no longer holds its cited lines 1104-1105, and the merge surfaces apps/browser/src/main.rs and blitz-vibey-script document.rs are reached whole
cross-cutting-patterns#3.1 · yes · stale · redraw requests in the windowed shell now travel through its event loop, and blitz-shell window.rs gained lines at 193-194, moving its cited lines 419-440 and 601-606
cross-cutting-patterns#4.1 · yes · stale · blitz-vibey-script runtime.rs, now 151 lines, no longer holds its cited lines 1639 and 1690-1692, and the merge surface dom/element.rs (code moved into its dom/element/ modules) and the conflict surface dioxus-native-dom dioxus_document.rs are reached whole
cross-cutting-patterns#5.1 · yes · stale · blitz-dom stylo.rs gained lines at 510, moving its cited lines 1346-1350, blitz-shell window.rs gained lines at 193-194, moving its cited lines 269-273, 373-391 and 633-637, and the conflict surface blitz-dom mutator.rs gained the open-state flush in set_attribute and remove_attribute
