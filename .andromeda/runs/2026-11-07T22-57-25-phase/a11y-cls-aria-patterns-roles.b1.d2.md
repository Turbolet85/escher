@answer n=1 entity=aria-patterns-roles#1.1 v=no
why: landmark tag-to-role mapping for nav, main and others; the chunk changes no landmark role and the scope names none
@answer n=2 entity=aria-patterns-roles#1.2 v=no
why: header and section tag-to-role mapping; the scope names no such role and the chunk leaves it to its own code
@answer n=3 entity=aria-patterns-roles#2.1 v=no
why: role attribute mapping for stated roles; the scope binds only the no-role GenericContainer reading and the role must stay untouched by the chunk's own code
@answer n=4 entity=aria-patterns-roles#2.2 v=no
why: ARIA state attribute mapping (aria-selected, aria-pressed, aria-invalid, aria-checked); the scope lists this mapping as not needed and the graph reads no ARIA state
@answer n=5 entity=aria-patterns-roles#2.3 v=no
why: button and submit map to Button; the element states its role and the scope names no button reading
@answer n=6 entity=aria-patterns-roles#2.4 v=yes
why: an a element without href reads GenericContainer; the scope's rule that a click-reacting element stating no role reads GenericContainer covers that case
@answer n=7 entity=aria-patterns-roles#2.5 v=no
why: select single and multiple role mapping; the select states an implicit role and the scope names no select role reading
@answer n=8 entity=aria-patterns-roles#2.6 v=no
why: single select ComboBox with engine-state expanded; the scope names no select popup reading and the chunk changes no select behaviour
@answer n=9 entity=aria-patterns-roles#2.7 v=no
why: multiple select ListBox multiselectable write; the scope names no listbox reading and the chunk changes no listbox behaviour
@answer n=10 entity=aria-patterns-roles#2.8 v=no
why: optgroup Group role and label naming; the scope names no option group reading and the chunk changes no list painting
@answer n=11 entity=aria-patterns-roles#2.9 v=no
why: open-list group header row behaviour; this is painting and choice behaviour the chunk does not change and the scope does not name
@answer n=12 entity=aria-patterns-roles#2.10 v=no
why: list and table tag-to-role mapping; the element states its implicit role and the scope names no table or list reading
@answer n=13 entity=aria-patterns-roles#2.11 v=yes
why: password input maps to PasswordInput, the role identity the graph's password value masking acts over; the scope names password input masking
@answer n=14 entity=aria-patterns-roles#2.12 v=yes
why: div maps to GenericContainer; the scope's rule that a click-reacting element stating no role reads GenericContainer names this default for a div
@answer n=15 entity=aria-patterns-roles#2.13 v=no
why: hidden-content exclusion rules; the scope lists hidden-content exclusion as not needed and the chunk changes no exclusion rule
@answer n=16 entity=aria-patterns-roles#3.1 v=no
why: describes browser app control markup; the chunk's graph and tests do not read the browser app
@answer n=17 entity=aria-patterns-roles#3.2 v=no
why: image alt text in the browser app; text alternatives in fixtures and apps are not needed by the chunk
@answer n=18 entity=aria-patterns-roles#3.3 v=no
why: rdme stylesheet focus states for role button and tabpanel; the chunk changes no focus rule and reads no stylesheet
@answer n=19 entity=aria-patterns-roles#4.1 v=no
why: captured GitHub fixture attributes; the chunk's graph and tests do not read this fixture and text alternatives in fixtures are not needed
@answer n=20 entity=aria-patterns-roles#4.2 v=no
why: Google fixture contentinfo role; the chunk's graph and tests do not read this fixture
@answer n=21 entity=aria-patterns-roles#4.3 v=no
why: Google fixture combobox textarea attributes; the chunk's graph and tests do not read this fixture and the scope names no combobox reading
@answer n=22 entity=aria-patterns-roles#4.4 v=no
why: Google fixture listbox, option and menu roles; the chunk's graph and tests do not read this fixture
@answer n=23 entity=aria-patterns-roles#4.5 v=no
why: Google fixture landmark roles; the chunk's graph and tests do not read this fixture
@answer n=24 entity=aria-patterns-roles#4.6 v=no
why: servo-new hamburger aria-expanded flipped on click; the scope lists upstream expanded reads as not needed and the chunk reads no such fixture
@answer n=25 entity=aria-patterns-roles#4.7 v=no
why: servo-new nav and section aria-label naming; the chunk's graph and tests do not read this fixture and text alternatives in fixtures are not needed
