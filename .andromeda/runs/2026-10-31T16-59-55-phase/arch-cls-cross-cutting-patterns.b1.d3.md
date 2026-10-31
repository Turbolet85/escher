# Classified — arch · cross-cutting-patterns.b1

## Answers
cross-cutting-patterns#1.1 · yes · DocumentConfig and its clock and ua_stylesheets fields, which the chunk's user-agent colour sheet via DocumentConfig::default() and its reduced-motion clock go through
cross-cutting-patterns#2.1 · yes · escher-driver's command span in execute.rs, a unit the chunk edits, and the harness screenshot's timing against the rule that screenshot never moves time
cross-cutting-patterns#3.1 · yes · redraw requested through ShellProvider::request_redraw after the list's open and close mutations and its focus moves
cross-cutting-patterns#4.1 · no · text editor construction and IME enabling for a focused text input; the chunk adds no text input and reaches no create_text_editor path
cross-cutting-patterns#5.1 · yes · default-action queue order and handler cancellation, which Enter and Space's queued Click and the list's default actions rely on
cross-cutting-patterns#6.1 · yes · removed-node interaction state clearing, damage propagation and the changed set written at apply_select_choice, which the open list's state and redraw rely on
