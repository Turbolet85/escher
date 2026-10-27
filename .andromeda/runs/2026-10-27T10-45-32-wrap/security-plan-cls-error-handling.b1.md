# Classified — security-plan · error-handling.b1

## Answers
error-handling#1.1 · no · escher-driver is unchanged, none of its cited files is reached, and the task's Untouched list keeps its refusal causes, session errors and spans
error-handling#2.1 · yes · stale · blitz-dom stylo.rs, which holds the cited todo!, unimplemented! and panic! sites, is a merge surface of the upstream sync, so those lines can have moved
error-handling#3.1 · yes · stale · the cited files are merge surfaces (blitz-shell lib.rs and window.rs, blitz-vibey-script runtime.rs and document.rs), and the window.rs and document.rs insertions above the cited lines (275-279, 353-356) moved them
error-handling#4.1 · yes · stale · blitz-dom stylo.rs, which holds the cited PARALLEL_TRAVERSAL_LOCK at lines 29-35 and 157-163, is a merge surface of the upstream sync, so those lines can have moved
