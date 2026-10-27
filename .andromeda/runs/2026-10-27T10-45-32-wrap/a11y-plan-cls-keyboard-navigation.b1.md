# Classified — a11y-plan · keyboard-navigation.b1

## Answers
keyboard-navigation#1.1 · yes · stale · element.rs, a merge surface, shifts by the session's +2 at 488-489 and +8 at 543-550, moving the cited lines 516-520 to 518-522, 742-774 to 752-784 and 743 to 753
keyboard-navigation#1.2 · no · the Tab walk lives in events/keyboard.rs and document.rs, neither reached, and the focusability it reads is re-read unchanged
keyboard-navigation#1.3 · yes · stale · mutator.rs, a conflict surface, shifts by the session's +4 at 368-371, moving the cited lines 507-514 to 511-518 while 346-355 stays put
keyboard-navigation#1.4 · yes · stale · element.rs, a merge surface, shifts by the session's +2 at 488-489, moving the cited lines 512-514 to 514-516 while the disabled states at 479-486 stay put
keyboard-navigation#1.5 · yes · stale · element.rs, a merge surface, shifts by the session's +2 at 488-489 and +8 at 543-550, moving the cited lines 516-520 to 518-522 and 743 to 753 while the presence reading is re-read unchanged
keyboard-navigation#1.6 · yes · stale · mutator.rs, a conflict surface, shifts by the session's +4 at 368-371 and +4 at 520-523, moving the cited lines 1446-1460 to 1454-1468
keyboard-navigation#1.7 · yes · stale · mutator.rs, a conflict surface, shifts by the session's +4 at 368-371 and +4 at 520-523, moving the cited lines 1122-1134 to 1130-1142 and 1037-1042 to 1045-1050
keyboard-navigation#1.8 · no · the FOCUS and FOCUSRING states are set in document.rs and node/node.rs, neither reached, and the session writes no focus state
keyboard-navigation#1.9 · yes · stale · stylo.rs, a merge surface reached whole, gains the session's NonTSPseudoClass::Open at 510, so the cited 488-492 reading is touched though its lines stay put
keyboard-navigation#1.10 · no · the focus and blur bubbling flags live in blitz-traits events.rs, not reached
keyboard-navigation#2.1 · yes · add-to · a text input already focused when its editor is built now enables IME at create_text_editor (layout/construct.rs:313-317), which this line must carry beside its disable on removal
keyboard-navigation#2.2 · no · the clear-on-click walk lives in events/pointer.rs, not reached, and the focusability it reads is re-read unchanged
keyboard-navigation#2.3 · no · the button and select click focus (click_button, events/pointer.rs) is unchanged and the is_disabled gate it relies on is re-read unmoved in element.rs
keyboard-navigation#2.4 · no · the double-click window and the document's clock are unchanged with no time read added, and events/pointer.rs is not reached
keyboard-navigation#2.5 · yes · add-to · F1: the details this summary's activation toggles now holds ElementState::OPEN, so :open matches it after the toggle
keyboard-navigation#2.6 · no · checkbox and radio click focus and toggling live in events/pointer.rs, not reached, and are unchanged
keyboard-navigation#2.7 · no · the range click focus and pointerdown value live in events/pointer.rs and node/range.rs, neither reached, and are re-read unchanged
keyboard-navigation#3.1 · no · text-input keys live in node/text.rs, not reached, and the session changes no key arm
keyboard-navigation#3.2 · no · the range keyboard model lives in events/keyboard.rs and node/range.rs, neither reached, and is re-read unchanged
keyboard-navigation#3.3 · no · the select keys and typeahead live in events/keyboard.rs and node/select.rs, neither reached, and the engine's redraw ask stays while only the shell's routing moved
keyboard-navigation#3.4 · yes · add-to · F1: the Enter and Space activation of a summary stands unchanged, but the details it toggles now holds ElementState::OPEN, so :open matches it
keyboard-navigation#3.5 · no · the macOS Backspace bindings live in node/text.rs, not reached
keyboard-navigation#3.6 · no · the action-modifier copy arm lives in events/keyboard.rs, not reached
keyboard-navigation#3.7 · no · implicit form submission lives in events/keyboard.rs, not reached
keyboard-navigation#3.8 · no · the label's default click lives in events/pointer.rs, not reached
