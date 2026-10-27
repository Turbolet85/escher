# Classified — a11y-plan · keyboard-navigation.b1

## Answers
keyboard-navigation#1.1 · no · its focusability rule lives in blitz-dom's element.rs and node.rs and in stand_keyboard.rs, which the session did not reach, so the claim holds
keyboard-navigation#1.2 · no · the Tab and Shift+Tab walk lives in blitz-dom's keyboard.rs and document.rs, which the session did not reach, so the claim holds
keyboard-navigation#1.3 · no · roving-tabindex recomputation lives in blitz-dom's mutator.rs and in focusability_updates.rs, which the session did not reach, so the claim holds
keyboard-navigation#1.4 · no · a button's disabled focusability lives in blitz-dom's element.rs and node.rs and in focusability_updates.rs, which the session did not reach, so the claim holds
keyboard-navigation#1.5 · no · the disabled-presence rule lives in blitz-dom's element.rs and pointer.rs and in stand_keyboard.rs, which the session did not reach, so the claim holds
keyboard-navigation#1.6 · no · the file input's inner tabindex="-1" lives in blitz-dom's mutator.rs, which the session did not reach, so the claim holds
keyboard-navigation#1.7 · no · the autofocus rule lives in blitz-dom's mutator.rs and in autofocus_attribute.rs, which the session did not reach, so the claim holds
keyboard-navigation#1.8 · no · the focus and focus-ring element states live in blitz-dom's document.rs and node.rs, which the session did not reach, so the claim holds
keyboard-navigation#1.9 · no · the :focus and :focus-within matching lives in blitz-dom's stylo.rs, which the session did not reach, so the claim holds
keyboard-navigation#1.10 · no · focus and focusin bubbling live in blitz-traits' events.rs, which the session did not reach, so the claim holds
keyboard-navigation#2.1 · no · focus reset on removal lives in blitz-dom's document.rs and in interaction_state_teardown.rs, which the session did not reach, so the claim holds
keyboard-navigation#2.2 · no · clearing focus on a non-interactive click lives in blitz-dom's pointer.rs, which the session did not reach, so the claim holds
keyboard-navigation#2.3 · no · a button's click focus and submit live in blitz-dom's pointer.rs and in the stand checks, which the session did not reach, so the claim holds
keyboard-navigation#2.4 · no · the double-click window lives in blitz-dom's pointer.rs and in stand_virtual_clock.rs, which the session did not reach, so the claim holds
keyboard-navigation#2.5 · no · the summary toggle lives in blitz-dom's pointer.rs, which the session did not reach, so the claim holds
keyboard-navigation#2.6 · no · checkbox and radio click toggles live in blitz-dom's pointer.rs and in harness_smoke.rs, which the session did not reach, so the claim holds
keyboard-navigation#2.7 · no · a range input's click focus lives in blitz-dom's pointer.rs, which the session did not reach, so the claim holds
keyboard-navigation#3.1 · no · text-input key handling lives in blitz-dom's text.rs, which the session did not reach, so the claim holds
keyboard-navigation#3.2 · no · the range input's arrow and Home/End keys live in blitz-dom's keyboard.rs, which the session did not reach, so the claim holds
keyboard-navigation#3.3 · no · the select key model lives in blitz-dom's keyboard.rs and select.rs, which the session did not reach, so the claim holds
keyboard-navigation#3.4 · no · Enter and Space activation live in blitz-dom's keyboard.rs and events/mod.rs and in the stand checks, which the session did not reach, so the claim holds
keyboard-navigation#3.5 · no · the macOS Backspace bindings live in blitz-dom's text.rs, which the session did not reach, so the claim holds
keyboard-navigation#3.6 · no · copy with the action modifier lives in blitz-dom's keyboard.rs, which the session did not reach, so the claim holds
keyboard-navigation#3.7 · no · implicit form submission lives in blitz-dom's keyboard.rs, which the session did not reach, so the claim holds
keyboard-navigation#3.8 · no · a label's default click lives in blitz-dom's pointer.rs, which the session did not reach, so the claim holds
