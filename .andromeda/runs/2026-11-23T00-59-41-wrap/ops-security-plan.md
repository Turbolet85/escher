@row_add n=1 row=decision/dom-token-list-replace area=input-validation fact=F1 detector=D-security-input
name: DOMTokenList replace for page scripts
rule: Page scripts reach DOMTokenList replace, classList.replace among them, through the script bootstrap's DOM interfaces, adding no input surface beyond page script.
why: Upstream's behaviour, ratified as an engine change for every script document.
basis: fct-yxmdkbm5, fct-rmpinexc
because: The merge brings DOMTokenList replace to every page script, taken as written on the ruling, and no row of security-plan states this page-script API or that it opens no new input surface.
@row_add n=2 row=decision/ime-composition-underline area=input-validation fact=F2 detector=D-security-input
name: IME composition underline painted
rule: In every document with an IME preedit the engine paints an underline under the composition text until the text is committed, an engine paint alone with no new input channel and no log.
why: Upstream's behaviour, ratified as an engine change for every document.
basis: fct-yxmdkbm5, fct-rmpinexc
because: The merge makes the engine paint the composition underline in every document with a preedit, taken as written on the ruling, and no row of security-plan states it.
@row_none n=1 fact=F3 type=5
what: The one advisory ignore's reason naming its residual row is held by deny.toml and the residual row itself; the rule that every ignore is by ID with a written reason naming its owning residual row, re-read on every push, already stands in decision/advisory-ignores-by-id.
