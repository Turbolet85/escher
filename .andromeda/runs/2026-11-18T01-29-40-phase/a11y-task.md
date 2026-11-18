@task_prose part=chunk
Re-asserts three WCAG 2.2 success criteria on the 7GUIs stand and the pages the session binary serves, at HEAD, after the 0.3.0 engine changes: SC 2.1.1 Keyboard, SC 2.4.3 Focus Order, SC 1.4.3 Contrast (Minimum).
Re-reads the existing stand keyboard and stand contrast checks and the a11y fixtures, and extends them where a 0.3.0 surface (the engine-painted select list, the multiple select, optgroup, checkbox and radio change, inert, disabled by presence, bundled DejaVu Sans fonts) is unasserted.
Every re-assertion runs in the a11y CI leg, and a stand-facing one also in the stand-repeat rounds; the plan rows stating the SC assertions and their guards are brought to what is asserted.
Pins, not removes, the known SC 2.4.3 gap: a focused control that stops showing by hidden, display:none or CSS alone keeps focus and takes keys.
Adds no engine behaviour, no driver verb, carrier, field or log field, and no crate dependency.
@task_file n=1 path=tests/blitz-tests/tests/stand_keyboard.rs mark=edits
what: the stand keyboard check — SC 2.1.1 keyboard reach and SC 2.4.3 stated Tab order walked on each stand screen — edits: its assertions, extended where a 0.3.0 surface is unasserted
@task_file n=2 path=tests/blitz-tests/tests/stand_contrast.rs mark=edits
what: the stand contrast check — SC 1.4.3 asserted on the stand, contrast measured from computed styles — edits: its assertions, extended where a 0.3.0 surface is unasserted
@task_file n=3 path=.github/scripts/ci-leg.sh mark=edits
what: the CI leg script holding the accessibility test leg in the slow tier and the stand checks repeated three rounds — edits: the a11y leg's --test list; the stand-repeat named list if a new non-stand_ witness joins it
@task_file n=4 path=.github/scripts/test_ci_workflows.py mark=edits
what: the pins of the CI legs, the a11y leg's test list among them — edits: the legs' pins
@task_file n=5 path=.github/scripts/test_blitz_tests_targets.py mark=edits
what: the pins of the blitz-tests test targets each leg runs — edits: the legs' pins
@task_item n=1 part=needs
SC 2.1.1 Keyboard re-asserted: every control on the stand and its served pages is reached and operated by keyboard alone — Enter and Space activating as a click, the single select's closed-list and open-list keys, the multiple select's keys, range input keys, the CRUD list and Cells grid keys.
@task_item n=2 part=needs
SC 2.4.3 Focus Order re-asserted: Tab and Shift+Tab walk each screen's stated order against the shared tab-order table and wrap, with focus events.
@task_item n=3 part=needs
Focus after the 0.3.0 interactions: Tab chooses the highlighted row and closes the select list before focus moves; focus stays on the select while its list is open; Escape and close return to the select; a control turning disabled, inert or removed moves or drops focus as ratified.
@task_item n=4 part=needs
The known SC 2.4.3 gap pinned as it stands: the focus fixup reads inertness alone, so a focused control hidden by the hidden attribute, display:none or CSS keeps focus and takes keys; the check names it a known gap and does not change it.
@task_item n=5 part=needs
SC 1.4.3 Contrast re-asserted: every text/background pair the stand paints meets 4.5:1, 3:1 for large text, by the WCAG contrast ratio helper and the required-ratio rule, read with the bundled DejaVu Sans fonts whose glyph metrics may move the large-text threshold and the sampled text.
@task_item n=6 part=needs
The 0.3.0 contrast surfaces: the open select list's border, highlight and disabled-text colours are engine constants; the multiple select's chosen-row colour pair; optgroup header rows; contrast unread past non-colour paint and on a highlighted list row stays as ratified.
@task_item n=7 part=needs
Disabled controls stay exempt from contrast; disabled-ness is read by attribute presence alone on a form control, and a disabled control stops the click walk and is no Tab stop.
@task_item n=8 part=needs
Inert subtrees are excluded: no focus, no hit, out of the accessibility tree; hidden content is kept out of the tree.
@task_item n=9 part=needs
The engine debts reached by the re-assertion: checkbox and radio change on click and key, the multiple select's keys and row clicks, size on a multiple select, optgroup options in a single select.
@task_item n=10 part=needs
The served pages: the six tasks the session binary serves (Counter, Flight Booker, Timer, CRUD, Cells, the load page) are in reach of the re-assertion; which ones the existing checks leave out is closed at P3.
@task_item n=11 part=needs
Every re-assertion is a check in the a11y CI leg's --test list, its pins updated to match; a stand-facing check also runs in the stand-repeat three rounds.
@task_item n=12 part=needs
A failing re-assertion at HEAD is fixed only inside an already ratified engine behaviour; a fix that changes engine behaviour for every document with no ruling is the founder's fork, never taken.
@task_item n=13 part=needs
No driver verb, carrier, argument, field, log field or span is added, and no crate dependency.
@task_item n=14 part=needs
The a11y plan rows that state the SC 2.1.1, 2.4.3 and 1.4.3 assertions on the stand and their guards are updated to what the re-assertion reads.
@task_item n=15 part=needs
The focus, hit and layout paths the existing checks read after the upstream syncs and Taffy moves: a synthetic click at the node centre, hit-testing, removed focused node dropping focus with blur steps.
@task_standing n=1
convention: accessibility assertions are Rust tests over the AccessKit tree, against WCAG thresholds
home: 3-a11y-assertion-harness-contract
@task_standing n=2
convention: every a11y check runs in the accessibility test leg of the slow tier, which gates CI
home: 9-ci-integration · bootstrap-phases-derive-for-route-setup-project
@task_standing n=3
convention: every stand-facing check repeats three rounds in stand-repeat
home: ci-integration
@task_standing n=4
convention: stand expectations — roles, names and the Tab order — are stated once in shared tables and asserted against them
home: screen-reader-support · cognitive-accessibility
@task_item n=1 part=not_needed
ARIA name and property reads, role mapping by HTML-AAM and the ARIA state attribute rules: the chunk adds no role, name or state reading.
@task_item n=2 part=not_needed
The snapshot, actions answer, snapshot text form, snapshot diff, MCP result and served node id contracts: the chunk changes no carrier or its fields.
@task_item n=3 part=not_needed
The UI graph state registration and the Dioxus reached-handler record: unrelated to keyboard, focus order or contrast.
@task_item n=4 part=not_needed
The platform adapter choice and creation order, the accessibility feature defaults and the windowed assistive-technology witness: residual, out of this chunk.
@task_item n=5 part=not_needed
Password masking, the password caret and IME enabling: no SC this chunk re-asserts reads them.
@task_item n=6 part=not_needed
Scrollbar drag, hidpi or zoom reshaping, scheme change as a device change and the :open pseudo-class on details or dialog: no stand surface this chunk re-asserts reaches them.
@task_item n=7 part=not_needed
Sections 10 to 12 (SLO budgets, anti-patterns, decisions log): no recorded intent.
