# Classified — a11y-plan · screen-reader-support.b1

## Answers
screen-reader-support#1.1 · no · the TextRun mapping is the builder's untouched HTML-to-AccessKit read, no engine crate moved and no reached file names it
screen-reader-support#1.2 · no · the aria-label and aria-labelledby naming is an untouched merged engine read, no engine crate moved, and the new select's aria-label is named by this same rule, which still holds
screen-reader-support#1.3 · no · the label-to-control association is an untouched merged engine read, no engine crate moved, and the new select is named by aria-label, not by a label element
screen-reader-support#1.4 · yes · add-to · F2: the flight booker's select now holds two options, named one-way flight and return flight, with exactly the chosen one reading selected, as this entity's select options do
screen-reader-support#2.1 · yes · stale · the lean stand's controls read 14, not 15, since the two mode buttons are retired and the flight-type select joins; F6 joins this group too: the load page's Button load-fetch named Load and its Paragraph load-done, out of the tree until its sheet lands
screen-reader-support#3.1 · yes · contradict · the flight booker's two mode buttons no longer read pressed, the stand states no aria-pressed anywhere, and its snapshot reads the flight type as a ComboBox valued one-way with two ListBoxOptions
screen-reader-support#4.1 · yes · add-to · the host now serves six tasks (the lean four, Cells and the load page) and unkeyed_actionable reads empty on all six, a reading this entity must carry for the two added pages
screen-reader-support#5.1 · yes · contradict · the flight booker's two mode buttons and their aria-pressed are retired and the select holds the mode, so the stand states no aria-pressed and the attribute-stated kinds drop from three to two
screen-reader-support#6.1 · no · the tag name on element nodes is the builder's untouched HTML-to-AccessKit mapping, and no engine crate moved
screen-reader-support#6.2 · no · the Window root and the tree focus are the builder's untouched reads, no engine crate moved, and the session names no change to how focus is reported
screen-reader-support#6.3 · no · the role-mapping test's goal line sits in accessibility_roles.rs, which no reached file names, and the role mapping is untouched
screen-reader-support#7.1 · no · accesskit_xplat's event routing is the untouched platform adapter, and no stand binary builds an adapter
screen-reader-support#8.1 · no · the shell's adapter creation is the untouched platform adapter, and no stand binary builds one
screen-reader-support#8.2 · no · the accessibility feature gating is an untouched crate claim, and the reached files name no manifest or engine crate
screen-reader-support#9.1 · no · language and direction are untouched and no session fact reaches them
screen-reader-support#9.2 · no · the dir attribute's bidi mapping is an untouched default stylesheet, and no session fact reaches language and direction
screen-reader-support#9.3 · no · the eleven fixtures' html lang attributes sit in examples/assets, which no reached file names, and no session fact reaches language
screen-reader-support#9.4 · no · the pages' lang="en" sit in examples/assets and examples/preact, neither reached, and no session fact reaches language
screen-reader-support#10.1 · no · the absence is searched over the s02 fixture files, which no reached file names, and no session fact reaches language
screen-reader-support#11.1 · no · the sr-only tooltip sits in the examples/assets GitHub fixture, not reached, and no session fact reaches text alternatives
screen-reader-support#11.2 · no · the google fixture's descriptive aria-labels sit in examples/assets, not reached, and no session fact reaches text alternatives
screen-reader-support#11.3 · no · the google fixture's aria-hidden and aria-atomic sit in examples/assets, not reached, and no session fact reaches text alternatives
screen-reader-support#11.4 · no · the servo-new burger's aria-hidden bars sit in examples/assets, not reached, and no session fact reaches text alternatives
screen-reader-support#11.5 · no · the servo.html image alt texts sit in examples/assets, not reached, and no session fact reaches text alternatives
screen-reader-support#11.6 · no · the servo_header_reduced image with no alt sits in examples/assets, not reached, and no session fact reaches text alternatives
