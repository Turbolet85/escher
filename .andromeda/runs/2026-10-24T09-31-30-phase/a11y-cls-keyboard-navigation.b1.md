# Classified — a11y · keyboard-navigation.b1

## Answers
keyboard-navigation#1.1 · yes · the focusability rule in node/element.rs and document.rs that focusability_updates and the actionable-key check read
keyboard-navigation#1.2 · yes · the Tab and Shift+Tab walk that stand_keyboard's tab_order and ENABLED_TAB_ORDER read
keyboard-navigation#1.3 · yes · the focusability cache recomputed in mutator.rs, the chunk's predicted merge-conflict file, which focusability_updates reads
keyboard-navigation#1.4 · yes · the disabled rule on button, input, select and textarea, which focusability_updates reads
keyboard-navigation#1.5 · yes · the presence-only disabled rule on the four form controls, which the Dioxus bridge's falsy-attribute removal must agree with
keyboard-navigation#1.6 · yes · tabindex -1 keeps a file input's generated button out of the Tab order, the focusability rule in mutator.rs
keyboard-navigation#1.7 · yes · autofocus skips disabled and inert candidates by the focusability rule, in mutator.rs
keyboard-navigation#1.8 · yes · the FOCUS and FOCUSRING states set and cleared in document.rs and node.rs on each focus move the Tab walk and click focus make
keyboard-navigation#1.9 · no · the :focus and :focus-within match in stylo.rs styles focus indicators the chunk does not need, and no standing check cites it
keyboard-navigation#1.10 · no · focus and blur not bubbling is blitz-traits' event table, outside the chunk's named surfaces, and no named need or check reads it
keyboard-navigation#2.1 · yes · focus reset to body and its blur on node removal in document.rs, on the removal path of the accessibility lifecycle the chunk re-reads
keyboard-navigation#2.2 · yes · an unmatched click clearing focus in pointer.rs, the other arm of the button and select click focus the chunk names
keyboard-navigation#2.3 · yes · the button and select click focus in pointer.rs, which stand_keyboard's stand-button clicks rest on
keyboard-navigation#2.4 · yes · the double-click window read on the document's clock through config.rs and pointer.rs, which the chunk names
keyboard-navigation#2.5 · yes · summary toggling in pointer.rs, the default click that Enter and Space on a focused summary reach
keyboard-navigation#2.6 · yes · checkbox and radio toggling in pointer.rs, the click a released Space on either dispatches
keyboard-navigation#2.7 · yes · the range input's click focus and pointer value in pointer.rs, which the chunk names
keyboard-navigation#3.1 · no · text editing keys in node/text.rs are not among the chunk's named handlers and no standing check reads them
keyboard-navigation#3.2 · yes · the range input's arrow, Home and End keys in events/keyboard.rs, which the chunk names
keyboard-navigation#3.3 · yes · the single-choice select's keys and typeahead on the document's clock in node/select.rs and events/keyboard.rs, which the chunk names
keyboard-navigation#3.4 · yes · Enter and Space dispatching a Click on focused buttons, summaries and submit or checkbox inputs, the activation the chunk names and stand_keyboard reads
keyboard-navigation#3.5 · no · macOS Backspace bindings are the text editor's Apple keymap in node/text.rs, outside the chunk's named handlers
keyboard-navigation#3.6 · no · copy with no text input focused is clipboard editing, not one of the chunk's named handlers, and no standing check reads it
keyboard-navigation#3.7 · no · implicit submission from a text field is the text-input arm, not the named button and summary activation, and no check reads it
keyboard-navigation#3.8 · yes · a label's click reaching a select runs the select click focus the chunk names, in pointer.rs
