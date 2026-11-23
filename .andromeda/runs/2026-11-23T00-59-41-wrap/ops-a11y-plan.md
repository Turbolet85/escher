@row_add n=1 row=decision/ime-composition-underlined area=visual-design-verification fact=F1 detector=none
name: IME composition drawn underlined
rule: Text being composed through an IME is drawn with an underline in the engine's own paint until it is committed, in every document, so a composition in progress is shown by more than its caret.
why: Upstream's behaviour, taken and ratified.
guard: composing_text_is_underlined_until_committed
basis: fct-yxmdkbm5 (E5, the composition underline in blitz-paint's text and text decorations); fct-rmpinexc (IME composition underline, every document with a preedit, tested by ime_preedit_underline)
because: The merge brings upstream's composition underline into the engine's paint for every document, a visual indication of composition state no row of the visual design section states yet.
