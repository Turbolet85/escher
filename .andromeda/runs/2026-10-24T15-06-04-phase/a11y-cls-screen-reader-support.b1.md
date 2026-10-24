# Classified — a11y · screen-reader-support.b1

## Answers
screen-reader-support#1.1 · yes · TextRun labelling is the name the snapshot reads for each text-labelled control through its labelled_by, and the diff's never-a-text-run-entry witness rests on it
screen-reader-support#1.2 · yes · the aria-label rule gives the stand's aria-label names that the INPUT_NAMES and REGION_NAMES tables assert; the aria-labelledby outranking is the Not-needed upstream read and binds nothing here
screen-reader-support#1.3 · yes · the label-for association names the stand's label-named inputs that INPUT_NAMES asserts; the aria-labelledby override is the Not-needed upstream read
screen-reader-support#1.4 · yes · the select's options kept in the tree under display none, each carrying its chosen state, are the Needs' select witnesses (v020-08) and the visibility exception the chunk names
screen-reader-support#2.1 · yes · the author_id on every element node, the stand's named controls and the tree the shell hands over are the Needs' stable-id witnesses, asserted through the common/mod.rs tables (a reached file)
screen-reader-support#3.1 · yes · the snapshot model's role, name, focus, selected, pressed and invalid from the tree, and enabled and value from the element, is the Needs' snapshot consumer, with its password, part-reading and fixtures::password_named witnesses
screen-reader-support#4.1 · yes · the actionable-key rule and unkeyed_actionable are a Needs item, reached through the lint findings' ids and the stderr id-free measurement
screen-reader-support#5.1 · yes · the CRUD list's listbox and rows, the flight booker's aria-pressed modes and aria-invalid dates, and the unread aria-activedescendant are the Needs' stand states the witnesses read
screen-reader-support#6.1 · no · no Needs item or witness reads the tree's per-node HTML tag; the roles and names the sweep asserts come from the tree's role and label
screen-reader-support#6.2 · yes · the Window root at u64::MAX and the focused DOM node as tree focus are what the snapshot's focused reading and the Tab and click focus witnesses read
screen-reader-support#6.3 · no · a purpose sentence in the role-mapping test binds no code or test, and the chunk adds no role and changes no mapping (Not needed: landmark and HTML-to-AccessKit role mapping)
screen-reader-support#7.1 · no · accesskit_xplat's request routing is the platform adapter, Not needed: no stand binary builds an adapter and the chunk works headless only
screen-reader-support#8.1 · no · the adapter's creation from the window handle is the platform adapter and its creation-before-window-shown rule, both Not needed, with no windowed run
screen-reader-support#8.2 · no · the accessibility feature gate on screen-reader integration is the platform-adapter item, Not needed; the headless witnesses run under the default-on feature and the chunk changes no feature set
screen-reader-support#9.1 · no · lang and xml:lang are the Language and direction item, Not needed: no requirement in reach and no check this chunk adds reads them
screen-reader-support#9.2 · no · the dir attribute's bidi-isolation mapping is the Language and direction item, Not needed; the RTL text_extent checks the chunk adds read text_extent's limit, not this mapping
screen-reader-support#9.3 · no · lang attributes on the examples/assets fixtures are the Language item, Not needed, and examples/assets is not a reached path
screen-reader-support#9.4 · no · pages declaring lang="en" are the Language item, Not needed, and the examples/assets and examples/preact pages are not reached
screen-reader-support#10.1 · no · an absence of html lang across the other s02 fixtures is the Language item, Not needed, and reaches no file the chunk touches
screen-reader-support#11.1 · no · the sr-only tooltip on a GitHub fixture is a text-alternatives fact, Not needed, and that fixture is not reached
screen-reader-support#11.2 · no · the google fixture's descriptive aria-labels are a text-alternatives fact, Not needed, and that fixture is not reached
screen-reader-support#11.3 · no · aria-hidden and aria-atomic on the google fixture are text-alternatives facts, Not needed, and no Needs item reads aria-hidden
screen-reader-support#11.4 · no · aria-hidden on the servo-new burger bars is a text-alternatives fixture fact, Not needed, and no Needs item reads aria-hidden
screen-reader-support#11.5 · no · the alt text on servo.html's images is the text-alternatives item, Not needed, and that fixture is not reached
screen-reader-support#11.6 · no · the servo_header_reduced image with no alt attribute is the text-alternatives item, Not needed, and that fixture is not reached
