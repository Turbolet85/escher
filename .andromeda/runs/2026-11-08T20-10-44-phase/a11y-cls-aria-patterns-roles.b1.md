@answer n=1 entity=aria-patterns-roles#1.1 v=no
why: landmark role mapping in the tree builder, the HTML-to-AccessKit role mapping the task excludes; the chunk reads roles and adds no mapping
@answer n=2 entity=aria-patterns-roles#1.2 v=no
why: header and section role mapping in the tree builder, excluded as role mapping; the chunk adds no mapping
@answer n=3 entity=aria-patterns-roles#2.1 v=no
why: role_from_name maps ARIA role names in the tree builder, excluded as role derivation; the graph reads closed words and adds no mapping
@answer n=4 entity=aria-patterns-roles#2.2 v=no
why: aria-selected, aria-pressed, aria-invalid and aria-checked state derivation, which the task excludes by name; the replay compares snapshot states by equality, not by their derivation
@answer n=5 entity=aria-patterns-roles#2.3 v=no
why: button and submit role mapping in the tree builder, excluded as role mapping
@answer n=6 entity=aria-patterns-roles#2.4 v=no
why: link role mapping with href in the tree builder; clickability is read from the element and vdom walk, not from the role
@answer n=7 entity=aria-patterns-roles#2.5 v=no
why: select ListBox and ComboBox role mapping in the tree builder, excluded as role mapping; the chunk names no select
@answer n=8 entity=aria-patterns-roles#2.6 v=no
why: single select ComboBox and expanded derivation, aria-expanded derivation the task excludes; the replay compares states by equality
@answer n=9 entity=aria-patterns-roles#2.7 v=no
why: multiple select set_multiselectable in the tree builder, a state the scope does not name
@answer n=10 entity=aria-patterns-roles#2.8 v=no
why: optgroup Group role and label naming, naming and role mapping the task excludes; the chunk names nothing
@answer n=11 entity=aria-patterns-roles#2.9 v=no
why: select popup header row behaviour, which the chunk's actions verb and coverage command do not name
@answer n=12 entity=aria-patterns-roles#2.10 v=no
why: list and table role mapping in the tree builder, excluded as role mapping; the chunk adds no table read
@answer n=13 entity=aria-patterns-roles#2.11 v=no
why: input type to role mapping in the tree builder, excluded as role mapping; the coverage result carries no control value, so the redaction reads no password role
@answer n=14 entity=aria-patterns-roles#2.12 v=no
why: generic, heading and paragraph role mapping in the tree builder, excluded as role mapping
@answer n=15 entity=aria-patterns-roles#2.13 v=yes
why: the hidden and display-none exclusion decides which nodes are agent-visible, and the coverage counts listeners only on agent-visible nodes, so a listener on a hidden node must not be counted
@answer n=16 entity=aria-patterns-roles#3.1 v=no
why: clickable div controls in apps/browser, a fact about an app the chunk does not touch
@answer n=17 entity=aria-patterns-roles#3.2 v=no
why: browser image alt text in apps/browser, outside the chunk's scope
@answer n=18 entity=aria-patterns-roles#3.3 v=no
why: rdme stylesheet focus states, and the chunk paints nothing
@answer n=19 entity=aria-patterns-roles#4.1 v=no
why: GitHub fixture attributes and aria-labelledby naming, which the scope names no fixture for and the task excludes as naming
@answer n=20 entity=aria-patterns-roles#4.2 v=no
why: Google fixture contentinfo role, and the scope names no fixture
@answer n=21 entity=aria-patterns-roles#4.3 v=no
why: Google fixture combobox attributes, and the scope names no fixture
@answer n=22 entity=aria-patterns-roles#4.4 v=no
why: Google fixture listbox and menu roles, and the scope names no fixture
@answer n=23 entity=aria-patterns-roles#4.5 v=no
why: Google fixture landmark roles, and the scope names no fixture
@answer n=24 entity=aria-patterns-roles#4.6 v=no
why: servo-new hamburger aria-expanded flip, and the scope names no fixture; expanded derivation is excluded
@answer n=25 entity=aria-patterns-roles#4.7 v=no
why: servo-new aria-label naming of nav and sections, naming the task excludes
