# Classified — a11y · screen-reader-support.b1

## Answers
screen-reader-support#1.1 · yes · option names come from text, which the text-node TextRun rule supplies; the open list's options read their names through the tree
screen-reader-support#1.2 · no · aria-label naming of elements; no chunk unit names aria-label, an optgroup's name is its `label` attribute, and select#flight-type's aria-label name is an existing reading the chunk does not change
screen-reader-support#1.3 · no · `<label>` association to labelable elements; no chunk unit touches label binding, and fx-pick's name by its label stays as built
screen-reader-support#1.4 · yes · options as ListBoxOption with `selected`, name from `label` else text, value through select_value, a closed select's options at its client rect; the open list changes these and the flight type's two options are read
screen-reader-support#1.5 · yes · a multiple select's rows read their own client rects; the closed-versus-open bounds split must leave multiple as built
screen-reader-support#1.6 · yes · the scope names a multiple select's `value` reading select_value and its keys and row clicks staying as built, so the Input/Change values rule over multiple_select_values binds
screen-reader-support#2.1 · yes · the list's options are element nodes in the tree, so each carries its element_id as author_id and the tree equals the harness doc by id; the driver acts on an option by that id
screen-reader-support#3.1 · yes · the snapshot takes role, name and focused from the tree and enabled from the presence of `disabled`; the open list's option reading and the disabled refusal the chunk restates rest on these
screen-reader-support#3.2 · no · the native checkbox or radio `checked` reading; the chunk adds no checkable role or native checked control
screen-reader-support#4.1 · yes · an option that becomes actionable through the driver must carry an author key, and unkeyed_actionable is the check over select#flight-type's options
screen-reader-support#4.2 · no · cells-grid's aria_label and the Cells count; the chunk changes no Cells unit or count
screen-reader-support#4.3 · no · the load page's keys, order and contrast rows; the chunk edits no load page element, and its keyboard edits are a select's keys with a list open
screen-reader-support#5.1 · yes · the flight booker's select#flight-type options state the choice by `selected`, and the unread aria-activedescendant pin bounds any tree focus an option is given under SC 2.4.3
screen-reader-support#6.1 · yes · the tree's tag name on element nodes is the `option` tag that the carve-out for an option under a select and the actionable report read for the list's options
screen-reader-support#6.2 · yes · tree focus is written only where the focused node is built, which bounds the focus agreement at each step and the focus fixup when a node comes under `inert`
screen-reader-support#6.3 · no · a test comment on the role-mapping goal; no unit the chunk changes
screen-reader-support#7.1 · no · accesskit_xplat routing of initial-tree, action and deactivation requests; the task's Not needed excludes the adapters
screen-reader-support#8.1 · no · adapter creation from the window handle; the task's Not needed excludes adapters and the chunk adds none
screen-reader-support#8.2 · no · the accessibility feature gating of blitz-shell and the Dioxus crates; the chunk adds no adapter or feature, and the list is an engine overlay
screen-reader-support#9.1 · no · `lang` and `xml:lang` element language; language is Not needed and the list adds no document language
screen-reader-support#9.2 · no · the default.css `dir` attribute mapping for bidi isolation; direction is Not needed and the list adds no dir rule
screen-reader-support#9.3 · no · html lang on fixtures; the chunk adds no document language and edits no fixture markup
screen-reader-support#9.4 · no · `lang="en"` on pages; language is Not needed and the chunk edits no page
screen-reader-support#10.1 · no · the absence of an html lang attribute in s02 fixtures; the chunk edits no fixture markup
screen-reader-support#11.1 · no · the sr-only tooltip in the GitHub fixture; text alternatives are Not needed and the chunk edits no fixture
