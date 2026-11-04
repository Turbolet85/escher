@answer n=1 entity=aria-patterns-roles#1.1 v=no
why: landmark role inventory (nav, main, aside, footer and so on); the probe names no role, not_needed item 4
@answer n=2 entity=aria-patterns-roles#1.2 v=no
why: header and section landmark mapping; the landmark roles inventory is not_needed item 4
@answer n=3 entity=aria-patterns-roles#2.1 v=yes
why: the CRUD listbox's role attribute (listbox, with option rows) is the value the probe passes and reads off the tree, so the role-attribute name mapping that yields it binds
@answer n=4 entity=aria-patterns-roles#2.2 v=yes
why: aria-selected on the CRUD option rows is the selection state the probe reads and a row's click or the list's arrow keys write (needs item 8)
@answer n=5 entity=aria-patterns-roles#2.3 v=yes
why: Enter and Space dispatch a Click to the focused button (needs item 7 names button), so the button role the probe reads off the tree binds
@answer n=6 entity=aria-patterns-roles#2.4 v=no
why: a link is an anchor with href; the scope names no link or anchor
@answer n=7 entity=aria-patterns-roles#2.5 v=no
why: select multiple to ListBox; the probe reads no select list (not_needed item 2)
@answer n=8 entity=aria-patterns-roles#2.6 v=no
why: single select ComboBox with expanded written from its list's open state; the probe reads no select list (not_needed item 2)
@answer n=9 entity=aria-patterns-roles#2.7 v=no
why: multiple select writes set_multiselectable; the CRUD list is a role=listbox element, not a multiple select (not_needed item 2)
@answer n=10 entity=aria-patterns-roles#2.8 v=no
why: optgroup to Group and its label; select list only (not_needed item 2)
@answer n=11 entity=aria-patterns-roles#2.9 v=no
why: option group header row in an open list; select popup only (not_needed item 2)
@answer n=12 entity=aria-patterns-roles#2.10 v=no
why: list, table and row element mapping; the scope names a listbox of option rows, no list or table element
@answer n=13 entity=aria-patterns-roles#2.11 v=no
why: input type role mapping; the scope names no input, and the CRUD reads go through the list
@answer n=14 entity=aria-patterns-roles#2.12 v=no
why: div, h2 and p generic and text roles; the scope names none of them
@answer n=15 entity=aria-patterns-roles#2.13 v=no
why: hidden-content exclusion from the tree (aria-hidden, hidden, display none, visibility hidden); the probe reads no hidden content (not_needed item 7)
@answer n=16 entity=aria-patterns-roles#3.1 v=no
why: browser app's clickable divs; the chunk's stand is seven_guis, not the browser app
@answer n=17 entity=aria-patterns-roles#3.2 v=no
why: browser app image alt text; outside the chunk's scope
@answer n=18 entity=aria-patterns-roles#3.3 v=no
why: rdme stylesheet focus states for role=button and role=tabpanel; apps/readme is outside the chunk's scope
@answer n=19 entity=aria-patterns-roles#4.1 v=no
why: captured GitHub fixtures' aria-hidden, tooltip, img and labelledby; the probe reads no fixture, only the seven_guis stand
@answer n=20 entity=aria-patterns-roles#4.2 v=no
why: Google fixture contentinfo role; fixture not read by the chunk
@answer n=21 entity=aria-patterns-roles#4.3 v=no
why: Google fixture query combobox attributes; fixture not read by the chunk
@answer n=22 entity=aria-patterns-roles#4.4 v=no
why: Google fixture listbox, option and menu roles; fixture not read by the chunk
@answer n=23 entity=aria-patterns-roles#4.5 v=no
why: Google fixture landmark regions; fixture not read by the chunk
@answer n=24 entity=aria-patterns-roles#4.6 v=no
why: servo-new fixture hamburger aria-expanded flipped on click; fixture not read by the chunk
@answer n=25 entity=aria-patterns-roles#4.7 v=no
why: servo-new fixture nav and section aria-label; fixture not read by the chunk
