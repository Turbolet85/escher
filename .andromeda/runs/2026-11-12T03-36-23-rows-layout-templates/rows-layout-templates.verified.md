@row_add n=1 row=decision/headless-stand-pinned-viewport area=surface-desktop-native rec=fct-nxmh4fah@2
name: Headless stand pinned viewport
rule: The headless stand skips Home and mounts one task in TaskShell at a pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans.
@row_check n=1 row=decision/headless-stand-pinned-viewport sha=5cd0d9ff5bb7 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/stand.rs:50
quote: pub const VIEWPORT_WIDTH: u32 = 800;
at: examples/seven_guis/src/stand.rs:51-52
quote: /// Pinned viewport height, in CSS pixels. pub const VIEWPORT_HEIGHT: u32 = 600;
at: examples/seven_guis/src/stand.rs:56
quote: pub const COLOR_SCHEME: ColorScheme = ColorScheme::Light;
at: examples/seven_guis/src/stand.rs:128
quote: task_in_shell(props.task.task(), EventHandler::new(|_| {}))
@row_add n=2 row=decision/session-roles-share-stand-boot area=surface-desktop-native rec=fct-nxmh4fah@2
name: Session roles share the stand boot
rule: Every escher-session role that boots a task (host, mcp, coverage) boots it through the same stand boot the command line uses, adding no markup, id, class, style, wrapper or order.
@row_check n=2 row=decision/session-roles-share-stand-boot sha=3c2c1a42b2d4 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/session_host.rs:50
quote: Served::Lean(task) => stand::boot(task, stand::options(true)),
at: packages/escher-driver/src/cli.rs:457
quote: |app| boot(app)?.recording_into(&dir),
at: packages/escher-driver/src/cli.rs:469
quote: handler_coverage(&checks, || boot(&app)).map(|coverage| coverage.to_json()),
@row_add n=3 row=decision/held-instance-reads-as-fresh-boot area=surface-desktop-native rec=fct-nxmh4fah@2
name: Held instance reads as a fresh boot
rule: A session's held instance reads as a fresh boot, and the TaskShell mount and the pinned viewport hold after commands, in both layout modes.
guard: stand_session_fresh
@row_check n=3 row=decision/held-instance-reads-as-fresh-boot sha=748090b9d988 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/session_host.rs:44
quote: let session = Session::start(slug, || match task {
guard_at: tests/blitz-tests/tests/stand_session_fresh.rs:40
@row_add n=4 row=decision/stand-ids-as-attributes-alone area=surface-desktop-native rec=fct-nxmh4fah@2
name: Stand ids as attributes alone
rule: Author ids on stand elements are added as attributes alone, moving no element, class, style, wrapper or order, and no stand CSS selects by them.
@row_check n=4 row=decision/stand-ids-as-attributes-alone sha=2b91e0ba359a v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/crud.rs:108
quote: div { id: "crud-list", class: "list",
searched: #crud- · in examples/seven_guis/src/tasks/crud.rs CSS
searched: #(counter|flight|timer|crud|temp|circle|cell)- · in examples/seven_guis/src/tasks/*.rs
searched: #task-card · in examples/seven_guis/src/app.rs
@row_add n=5 row=decision/stand-a11y-by-attributes-alone area=surface-desktop-native rec=fct-nxmh4fah@2
name: Stand names, roles and states by attributes alone
rule: Stand controls take accessible names, roles and states by attributes alone (for, aria-label, aria-invalid, role, aria-selected), adding no element, class, style, key, id or order, the classes that style invalid and selected states staying.
@row_check n=5 row=decision/stand-a11y-by-attributes-alone sha=10549dad3791 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/timer.rs:89
quote: label { class: "timer-label", r#for: "timer-duration", "Duration: " }
at: examples/seven_guis/src/tasks/flight_booker.rs:110
quote: aria_label: "Departure date",
at: examples/seven_guis/src/tasks/flight_booker.rs:111
quote: aria_invalid: if start_valid { "false" } else { "true" },
at: examples/seven_guis/src/tasks/crud.rs:109
quote: role: "listbox",
at: examples/seven_guis/src/tasks/crud.rs:148
quote: aria_selected: if selected() == Some(i) { "true" } else { "false" },
at: examples/seven_guis/src/tasks/crud.rs:149
quote: class: if selected() == Some(i) { "list-item selected" } else { "list-item" },
@row_add n=6 row=decision/every-stand-control-keyed area=surface-desktop-native rec=fct-nxmh4fah@2
name: Every stand control keyed
rule: Every actionable element of all seven stand tasks carries an author id, so no actionable element reads unkeyed, an open dialog included.
@row_check n=6 row=decision/every-stand-control-keyed sha=11aca5dfb742 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/counter.rs:19
quote: id: "counter-increment",
at: examples/seven_guis/src/tasks/temp_converter.rs:21
quote: id: "temp-celsius",
at: examples/seven_guis/src/tasks/flight_booker.rs:126
quote: id: "flight-book",
at: examples/seven_guis/src/tasks/timer.rs:107
quote: id: "timer-reset",
at: examples/seven_guis/src/tasks/crud.rs:182
quote: id: "crud-create",
at: examples/seven_guis/src/tasks/circle_drawer.rs:156
quote: id: "circle-close",
at: examples/seven_guis/src/tasks/cells.rs:462
quote: id: "cell-{col_label(c)}{r}",
@row_add n=7 row=decision/crud-list-keyboard-region area=surface-desktop-native rec=fct-nxmh4fah@2
name: CRUD list as one keyboard region
rule: The CRUD list is one Tab stop whose ArrowDown, ArrowUp, Home and End move the selected row and fill the name fields as a click does; focus stays on the list, never a row, and no aria-activedescendant is read.
why: Ratified as a decision.
@row_check n=7 row=decision/crud-list-keyboard-region sha=f21aec4e1f85 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/crud.rs:111-112
quote: tabindex: "0", onkeydown: move |e| {
at: examples/seven_guis/src/tasks/crud.rs:122-129
quote: let to = match (e.key(), at) { (Key::ArrowDown, None) => 0, (Key::ArrowDown, Some(k)) => k + 1, (Key::ArrowUp, Some(k)) if k > 0 => k - 1, (Key::Home, _) => 0, (Key::End, _) if !visible.is_empty() => visible.len() - 1, _ => return, };
at: examples/seven_guis/src/tasks/crud.rs:135-136
quote: first_field.set(p.first); last_field.set(p.last);
searched: activedescendant · in examples/seven_guis
@row_add n=8 row=contract/stand-element-ids/task-card-counter area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: task-card-counter
promise: Stable author element id of Home's Counter task-card button.
@row_check n=8 row=contract/stand-element-ids/task-card-counter sha=cef07b593bf4 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/app.rs:144
quote: id: "task-card-{meta.slug}",
at: examples/seven_guis/src/app.rs:28
quote: slug: "counter",
@row_add n=9 row=contract/stand-element-ids/task-card-temp-converter area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: task-card-temp-converter
promise: Stable author element id of Home's Temperature Converter task-card button.
@row_check n=9 row=contract/stand-element-ids/task-card-temp-converter sha=63548745626b v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/app.rs:144
quote: id: "task-card-{meta.slug}",
at: examples/seven_guis/src/app.rs:35
quote: slug: "temp-converter",
@row_add n=10 row=contract/stand-element-ids/task-card-flight-booker area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: task-card-flight-booker
promise: Stable author element id of Home's Flight Booker task-card button.
@row_check n=10 row=contract/stand-element-ids/task-card-flight-booker sha=dffb57ad119f v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/app.rs:144
quote: id: "task-card-{meta.slug}",
at: examples/seven_guis/src/app.rs:42
quote: slug: "flight-booker",
@row_add n=11 row=contract/stand-element-ids/task-card-timer area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: task-card-timer
promise: Stable author element id of Home's Timer task-card button.
@row_check n=11 row=contract/stand-element-ids/task-card-timer sha=2249b453cbea v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/app.rs:144
quote: id: "task-card-{meta.slug}",
at: examples/seven_guis/src/app.rs:49
quote: slug: "timer",
@row_add n=12 row=contract/stand-element-ids/task-card-crud area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: task-card-crud
promise: Stable author element id of Home's CRUD task-card button.
@row_check n=12 row=contract/stand-element-ids/task-card-crud sha=8471134fa891 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/app.rs:144
quote: id: "task-card-{meta.slug}",
at: examples/seven_guis/src/app.rs:56
quote: slug: "crud",
@row_add n=13 row=contract/stand-element-ids/task-card-circle-drawer area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: task-card-circle-drawer
promise: Stable author element id of Home's Circle Drawer task-card button.
@row_check n=13 row=contract/stand-element-ids/task-card-circle-drawer sha=4ee4932f0f50 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/app.rs:144
quote: id: "task-card-{meta.slug}",
at: examples/seven_guis/src/app.rs:63
quote: slug: "circle-drawer",
@row_add n=14 row=contract/stand-element-ids/task-card-cells area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: task-card-cells
promise: Stable author element id of Home's Cells task-card button.
@row_check n=14 row=contract/stand-element-ids/task-card-cells sha=906ced9a37ab v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/app.rs:144
quote: id: "task-card-{meta.slug}",
at: examples/seven_guis/src/app.rs:70
quote: slug: "cells",
@row_add n=15 row=contract/stand-element-ids/counter-value area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: counter-value
promise: Stable author element id of the Counter task's value display.
@row_check n=15 row=contract/stand-element-ids/counter-value sha=9be7890df666 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/counter.rs:17
quote: p { id: "counter-value", class: "counter-display", "{count}" }
@row_add n=16 row=contract/stand-element-ids/counter-increment area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: counter-increment
promise: Stable author element id of the Counter task's increment button.
@row_check n=16 row=contract/stand-element-ids/counter-increment sha=6299d11f47da v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/counter.rs:19
quote: id: "counter-increment",
@row_add n=17 row=contract/stand-element-ids/flight-type area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: flight-type
promise: Stable author element id of the flight booker's mode select.
@row_check n=17 row=contract/stand-element-ids/flight-type sha=daf01749a35e v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/flight_booker.rs:85
quote: id: "flight-type",
@row_add n=18 row=contract/stand-element-ids/flight-type-one-way area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: flight-type-one-way
promise: Stable author element id of the flight booker's one-way mode option.
@row_check n=18 row=contract/stand-element-ids/flight-type-one-way sha=e02581dd227f v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/flight_booker.rs:96
quote: id: "flight-type-one-way",
@row_add n=19 row=contract/stand-element-ids/flight-type-return area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: flight-type-return
promise: Stable author element id of the flight booker's return mode option.
@row_check n=19 row=contract/stand-element-ids/flight-type-return sha=525ad2f15529 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/flight_booker.rs:102
quote: id: "flight-type-return",
@row_add n=20 row=contract/stand-element-ids/flight-start area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: flight-start
promise: Stable author element id of the flight booker's departure date input, named "Departure date" by aria-label.
@row_check n=20 row=contract/stand-element-ids/flight-start sha=460981f0b3c3 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/flight_booker.rs:109-110
quote: id: "flight-start", aria_label: "Departure date",
@row_add n=21 row=contract/stand-element-ids/flight-return-date area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: flight-return-date
promise: Stable author element id of the flight booker's return date input, named "Return date" by aria-label.
@row_check n=21 row=contract/stand-element-ids/flight-return-date sha=a4283cdf71da v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/flight_booker.rs:117-118
quote: id: "flight-return-date", aria_label: "Return date",
@row_add n=22 row=contract/stand-element-ids/flight-book area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: flight-book
promise: Stable author element id of the flight booker's book button.
@row_check n=22 row=contract/stand-element-ids/flight-book sha=0fb03fad53e8 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/flight_booker.rs:126
quote: id: "flight-book",
@row_add n=23 row=contract/stand-element-ids/flight-booked area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: flight-booked
promise: Stable author element id of the flight booker's booked display.
@row_check n=23 row=contract/stand-element-ids/flight-booked sha=62d5b17e4fee v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/flight_booker.rs:140
quote: p { id: "flight-booked", class: "booked-msg", "{msg}" }
@row_add n=24 row=contract/stand-element-ids/timer-progress area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: timer-progress
promise: Stable author element id of the Timer task's progress display.
@row_check n=24 row=contract/stand-element-ids/timer-progress sha=c34543829581 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/timer.rs:85
quote: div { id: "timer-progress", class: "progress-fill", style: "width: {pct:.1}%;" }
@row_add n=25 row=contract/stand-element-ids/timer-elapsed area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: timer-elapsed
promise: Stable author element id of the Timer task's elapsed display.
@row_check n=25 row=contract/stand-element-ids/timer-elapsed sha=3aca8f0c47ba v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/timer.rs:87
quote: p { id: "timer-elapsed", class: "timer-elapsed", "Elapsed: {elapsed():.1}s" }
@row_add n=26 row=contract/stand-element-ids/timer-duration area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: timer-duration
promise: Stable author element id of the Timer task's duration input, named by its label's for.
@row_check n=26 row=contract/stand-element-ids/timer-duration sha=7c0bd475ddc7 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/timer.rs:91
quote: id: "timer-duration",
at: examples/seven_guis/src/tasks/timer.rs:89
quote: label { class: "timer-label", r#for: "timer-duration", "Duration: " }
@row_add n=27 row=contract/stand-element-ids/timer-duration-value area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: timer-duration-value
promise: Stable author element id of the Timer task's duration value display.
@row_check n=27 row=contract/stand-element-ids/timer-duration-value sha=c5ee6bbacc3a v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/timer.rs:104
quote: span { id: "timer-duration-value", class: "timer-duration-label", "{duration():.1}s" }
@row_add n=28 row=contract/stand-element-ids/timer-reset area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: timer-reset
promise: Stable author element id of the Timer task's reset button.
@row_check n=28 row=contract/stand-element-ids/timer-reset sha=bab5d278c4eb v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/timer.rs:107
quote: id: "timer-reset",
@row_add n=29 row=contract/stand-element-ids/crud-filter area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: crud-filter
promise: Stable author element id of the CRUD task's filter input, named by its label's for.
@row_check n=29 row=contract/stand-element-ids/crud-filter sha=4f12802c1c3e v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/crud.rs:97
quote: id: "crud-filter",
at: examples/seven_guis/src/tasks/crud.rs:95
quote: label { r#for: "crud-filter", "Filter prefix: " }
@row_add n=30 row=contract/stand-element-ids/crud-list area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: crud-list
promise: Stable author element id of the CRUD task's people list, a listbox named "People".
@row_check n=30 row=contract/stand-element-ids/crud-list sha=d88bfd1ee637 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/crud.rs:108-110
quote: div { id: "crud-list", class: "list", role: "listbox", aria_label: "People",
@row_add n=31 row=contract/stand-element-ids/crud-name area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: crud-name
promise: Stable author element id of the CRUD task's name input, named by its label's for.
@row_check n=31 row=contract/stand-element-ids/crud-name sha=fc2ca0d94484 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/crud.rs:167
quote: id: "crud-name",
at: examples/seven_guis/src/tasks/crud.rs:165
quote: label { r#for: "crud-name", "Name: " }
@row_add n=32 row=contract/stand-element-ids/crud-surname area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: crud-surname
promise: Stable author element id of the CRUD task's surname input, named by its label's for.
@row_check n=32 row=contract/stand-element-ids/crud-surname sha=9b60a0770aca v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/crud.rs:173
quote: id: "crud-surname",
at: examples/seven_guis/src/tasks/crud.rs:171
quote: label { r#for: "crud-surname", "Surname: " }
@row_add n=33 row=contract/stand-element-ids/crud-create area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: crud-create
promise: Stable author element id of the CRUD task's Create button.
@row_check n=33 row=contract/stand-element-ids/crud-create sha=80653246c184 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/crud.rs:182
quote: id: "crud-create",
@row_add n=34 row=contract/stand-element-ids/crud-update area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: crud-update
promise: Stable author element id of the CRUD task's Update button.
@row_check n=34 row=contract/stand-element-ids/crud-update sha=ec1b9c321342 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/crud.rs:190
quote: id: "crud-update",
@row_add n=35 row=contract/stand-element-ids/crud-delete area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: crud-delete
promise: Stable author element id of the CRUD task's Delete button.
@row_check n=35 row=contract/stand-element-ids/crud-delete sha=0a7a271968bd v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/crud.rs:205
quote: id: "crud-delete",
@row_add n=36 row=contract/stand-element-ids/crud-person-id area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: crud-person-{person.id}
promise: Each CRUD row's stable author element id, from its person's model-assigned id beside the same Dioxus key, following its person under filter, Create and Delete.
@row_check n=36 row=contract/stand-element-ids/crud-person-id sha=5ae6a7d6e9ff v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/crud.rs:145-146
quote: key: "{person.id}", id: "crud-person-{person.id}",
@row_add n=37 row=contract/stand-element-ids/temp-celsius area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: temp-celsius
promise: Stable author element id of the Temperature Converter's Celsius control.
@row_check n=37 row=contract/stand-element-ids/temp-celsius sha=98cf40eae94a v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/temp_converter.rs:21
quote: id: "temp-celsius",
@row_add n=38 row=contract/stand-element-ids/temp-fahrenheit area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: temp-fahrenheit
promise: Stable author element id of the Temperature Converter's Fahrenheit control.
@row_check n=38 row=contract/stand-element-ids/temp-fahrenheit sha=60f304a42192 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/temp_converter.rs:34
quote: id: "temp-fahrenheit",
@row_add n=39 row=contract/stand-element-ids/circle-undo area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: circle-undo
promise: Stable author element id of the Circle Drawer's undo control.
@row_check n=39 row=contract/stand-element-ids/circle-undo sha=f752ea9cc69e v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/circle_drawer.rs:59
quote: id: "circle-undo",
@row_add n=40 row=contract/stand-element-ids/circle-redo area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: circle-redo
promise: Stable author element id of the Circle Drawer's redo control.
@row_check n=40 row=contract/stand-element-ids/circle-redo sha=aaab022c73c9 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/circle_drawer.rs:70
quote: id: "circle-redo",
@row_add n=41 row=contract/stand-element-ids/circle-canvas area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: circle-canvas
promise: Stable author element id of the Circle Drawer's canvas.
@row_check n=41 row=contract/stand-element-ids/circle-canvas sha=8df5a8b7c030 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/circle_drawer.rs:83
quote: id: "circle-canvas",
@row_add n=42 row=contract/stand-element-ids/circle-diameter area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: circle-diameter
promise: Stable author element id of the Circle Drawer dialog's range input, present only while the dialog is open.
@row_check n=42 row=contract/stand-element-ids/circle-diameter sha=45ac8a2bca68 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/circle_drawer.rs:132
quote: if let Some(idx) = dialog_idx() {
at: examples/seven_guis/src/tasks/circle_drawer.rs:136
quote: id: "circle-diameter",
@row_add n=43 row=contract/stand-element-ids/circle-close area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: circle-close
promise: Stable author element id of the Circle Drawer dialog's close control, present only while the dialog is open.
@row_check n=43 row=contract/stand-element-ids/circle-close sha=d3f33f5574cb v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/circle_drawer.rs:132
quote: if let Some(idx) = dialog_idx() {
at: examples/seven_guis/src/tasks/circle_drawer.rs:156
quote: id: "circle-close",
@row_add n=44 row=contract/stand-element-ids/cell-column-row area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: cell-{A..Z}{0..25}
promise: Stable author element id of each of the Cells task's 676 data cells, its column letter then its row number.
@row_check n=44 row=contract/stand-element-ids/cell-column-row sha=8178e358097a v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/cells.rs:462
quote: id: "cell-{col_label(c)}{r}",
at: examples/seven_guis/src/tasks/cells.rs:343-344
quote: const COLS: usize = 26; const ROWS: usize = 26;
@row_add n=45 row=contract/stand-element-ids/cell-input area=surface-desktop-native rec=fct-nxmh4fah@2
boundary: Stand stable element ids
item: cell-input
promise: Stable author element id of the Cells task's one edit input.
@row_check n=45 row=contract/stand-element-ids/cell-input sha=4b6c3c9afc6b v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/cells.rs:491
quote: id: "cell-input",
@row_add n=46 row=decision/load-page-only-network area=surface-desktop-native rec=fct-5a7qhqx6@1
name: Load page alone has a network provider
rule: Only the load page's boot has a network provider, an in-process stand-in answering after 200 ms from a thread of its own, so a driver click on load-fetch waits it out; every other boot keeps no network provider.
guard: stand_load_page
@row_check n=46 row=decision/load-page-only-network sha=b2e7202b5c56 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/stand.rs:148
quote: net_provider: Some(Arc::new(LoadNet)),
at: examples/seven_guis/src/stand.rs:171
quote: thread::sleep(LOAD_ANSWER_DELAY);
at: examples/seven_guis/src/stand.rs:72
quote: net_provider: None,
guard_at: tests/blitz-tests/tests/stand_load_page.rs:1
@row_add n=47 row=contract/stand-element-ids/load-fetch area=surface-desktop-native rec=fct-5a7qhqx6@1
boundary: Stand stable element ids
item: load-fetch
promise: Stable author element id of the load page's Load button, taking Enter and Space.
@row_check n=47 row=contract/stand-element-ids/load-fetch sha=0a100e3b0569 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/stand.rs:192
quote: button { id: "load-fetch", class: "load-btn", onclick: move |_| linked.set(true), "Load" }
at: packages/blitz-dom/src/events/keyboard.rs:336
quote: local_name!("button") | local_name!("summary") => true,
at: packages/blitz-dom/src/events/keyboard.rs:353
quote: local_name!("button") | local_name!("summary") => true,
@row_add n=48 row=contract/stand-element-ids/load-done area=surface-desktop-native rec=fct-5a7qhqx6@1
boundary: Stand stable element ids
item: load-done
promise: Stable author element id of the load page's "Loaded" paragraph, hidden until the stylesheet the button's click links lands.
@row_check n=48 row=contract/stand-element-ids/load-done sha=a8eaa2475a7b v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/stand.rs:196
quote: p { id: "load-done", class: "load-done", "Loaded" }
at: examples/seven_guis/src/stand.rs:240
quote: display: none;
at: examples/seven_guis/src/stand.rs:161
quote: const LOADED_CSS: &str = ".load-body .load-done { display: block; }";
@row_add n=49 row=contract/stand-element-ids/cells-grid area=surface-desktop-native rec=fct-xlkkhy76@1
boundary: Stand stable element ids
item: cells-grid
promise: Stable author element id of the Cells grid, one Tab stop and keyboard region named "Cells", with no role.
@row_check n=49 row=contract/stand-element-ids/cells-grid sha=347741ac6cb9 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/cells.rs:412-415
quote: id: "cells-grid", class: "cells-grid", tabindex: "0", aria_label: "Cells",
@row_add n=50 row=decision/stand-accent-colour area=surface-desktop-native rec=fct-xlzsjdlc@1
name: Stand accent colour
rule: The stand accent is #3a5ce5 (hover #2a4cd3, active #1f3fc1) in every task CSS that declares it, on the page #f5f5f5 with text #1a1a1a, with white button text on accent controls.
why: White text on #3a5ce5 reads 5.47 against SC 1.4.3's 4.5, a pass; the lighter #4a6cf7 reads 4.39, a fail.
guard: stand_contrast
@row_check n=50 row=decision/stand-accent-colour sha=808fb718ca8c v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/counter.rs:65
quote: background-color: #3a5ce5;
at: examples/seven_guis/src/tasks/counter.rs:73
quote: background-color: #2a4cd3;
at: examples/seven_guis/src/tasks/counter.rs:77
quote: background-color: #1f3fc1;
at: examples/seven_guis/src/tasks/counter.rs:36
quote: background-color: #f5f5f5;
at: examples/seven_guis/src/tasks/counter.rs:55
quote: color: #1a1a1a;
guard_at: tests/blitz-tests/tests/stand_contrast.rs:1
@row_add n=51 row=contract/shell-provider-window-chrome/request-window-close area=surface-desktop-native rec=fct-ttkpderd@1
boundary: ShellProvider window chrome controls
item: request_window_close
promise: Offered to a document drawing its own titlebar, to request the window's close.
@row_check n=51 row=contract/shell-provider-window-chrome/request-window-close sha=87d9fa690330 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-traits/src/shell.rs:47
quote: fn request_window_close(&self) {}
@row_add n=52 row=contract/shell-provider-window-chrome/set-window-minimized area=surface-desktop-native rec=fct-ttkpderd@1
boundary: ShellProvider window chrome controls
item: set_window_minimized
promise: Offered to a document drawing its own titlebar, to set the window minimized.
@row_check n=52 row=contract/shell-provider-window-chrome/set-window-minimized sha=8eec884bec82 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-traits/src/shell.rs:48
quote: fn set_window_minimized(&self, minimized: bool) {
@row_add n=53 row=contract/shell-provider-window-chrome/set-window-maximized area=surface-desktop-native rec=fct-ttkpderd@1
boundary: ShellProvider window chrome controls
item: set_window_maximized
promise: Offered to a document drawing its own titlebar, to set the window maximized.
@row_check n=53 row=contract/shell-provider-window-chrome/set-window-maximized sha=099dd46d83bf v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-traits/src/shell.rs:51
quote: fn set_window_maximized(&self, maximized: bool) {
@row_add n=54 row=contract/shell-provider-window-chrome/is-window-maximized area=surface-desktop-native rec=fct-ttkpderd@1
boundary: ShellProvider window chrome controls
item: is_window_maximized
promise: Offered to a document drawing its own titlebar, to read whether the window is maximized.
@row_check n=54 row=contract/shell-provider-window-chrome/is-window-maximized sha=81ef8189e24d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-traits/src/shell.rs:54
quote: fn is_window_maximized(&self) -> bool {
@row_add n=55 row=contract/shell-provider-window-chrome/set-window-decorations area=surface-desktop-native rec=fct-ttkpderd@1
boundary: ShellProvider window chrome controls
item: set_window_decorations
promise: Offered to a document drawing its own titlebar, to set the window's decorations.
@row_check n=55 row=contract/shell-provider-window-chrome/set-window-decorations sha=f4c5b9104a89 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-traits/src/shell.rs:57
quote: fn set_window_decorations(&self, decorations: bool) {
@row_add n=56 row=contract/shell-provider-window-chrome/drag-window area=surface-desktop-native rec=fct-ttkpderd@1
boundary: ShellProvider window chrome controls
item: drag_window
promise: Offered to a document drawing its own titlebar, to drag the window.
@row_check n=56 row=contract/shell-provider-window-chrome/drag-window sha=662f0e01d501 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-traits/src/shell.rs:62
quote: fn drag_window(&self) {}
@row_add n=57 row=absence/arbitrary-index-html-templates area=surface-desktop-native rec=fct-wu6baq7k@1
capability: Arbitrary index.html templates for a document
@row_check n=57 row=absence/arbitrary-index-html-templates sha=12c68be71f5a v=holds judge=sonnet head=edd0cfaea7f0
searched: index.html · in examples/seven_guis, packages/escher-driver, packages/blitz-test-harness (only a grid-template-columns hit unrelated)
searched: index_html|template_html|with_template · in packages/*/src
searched: TODO: Support arbitrary "index.html" templates · in packages/dioxus-native-dom/src/dioxus_document.rs:128 (a TODO comment only; the document is built with the fixed html/head/body/div#main)
@row_add n=58 row=decision/scroll-into-view-moves-every-scroller area=surface-desktop-native rec=fct-kontkuux@1
name: Into-view scroll moves every holding scroller
rule: An into-view scroll moves every scrolling box that holds its target, innermost first, and then the viewport, in every document.
why: Ratified for every document.
guard: stand_act_scroll
except: A row of an open select list is scrolled into view by writing the list's scroll offset at once, no scrolling box or viewport moving.
@row_check n=58 row=decision/scroll-into-view-moves-every-scroller sha=eb57ced62f76 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/scrolling.rs:713
quote: self.scroll_to(box_id, x, y, ScrollBehavior::Instant);
at: packages/blitz-dom/src/scrolling.rs:737
quote: self.scroll_to(root_id, x, y, behavior);
at: packages/blitz-dom/src/scrolling.rs:670
quote: if self.scroll_select_list_to(node_id) {
guard_at: tests/blitz-tests/tests/stand_act_scroll.rs:1
@row_add n=59 row=decision/visible-region-through-clipping-boxes area=surface-desktop-native rec=fct-kontkuux@1
name: Visible region through clipping boxes
rule: The part of the viewport an element can be seen through is the viewport narrowed to the padding box of every clipping box that holds it, in every document.
why: Ratified for every document.
except: A row or group of an open select list is seen through the list's scrollport alone, no clipping box answering.
@row_check n=59 row=decision/visible-region-through-clipping-boxes sha=72c72c680a14 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/scrolling.rs:779
quote: let shown = self.padding_box_in_view(box_id)?;
at: packages/blitz-dom/src/scrolling.rs:781-782
quote: left = left.max(snap(shown.x0)); right = right.min(snap(shown.x1));
at: packages/blitz-dom/src/scrolling.rs:754
quote: if let Some(region) = self.select_list_region(node_id) {
@row_add n=60 row=decision/stand-lints-clean-at-boot area=surface-desktop-native rec=fct-g6ui4uo3@1
name: Stand lints clean at boot
rule: At boot the counter, the flight booker, the timer, CRUD and the load page read zero visual-lint findings, in both layout modes.
guard: stand_lint
@row_check n=60 row=decision/stand-lints-clean-at-boot sha=f7a8b45cb37f v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/stand_lint.rs:338
quote: assert_eq!(LeanTask::ALL.map(|task| at_boot(task).len()), [0, 0, 0, 0]);
at: tests/blitz-tests/tests/stand_load_page.rs:123
quote: linted(&mut session, &lint()).is_some_and(|findings| findings.is_empty()),
guard_at: tests/blitz-tests/tests/stand_lint.rs:1
@row_add n=61 row=contract/paint-bench-cli/arguments area=surface-cli rec=fct-h7hmdmzm@1
boundary: paint_bench command line
item: arguments
promise: paint_bench takes <url> [width] [height] [scale] [iters] [backend], positionally.
@row_check n=61 row=contract/paint-bench-cli/arguments sha=e0dba11f3759 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/paint_bench.rs:6
quote: //! Usage: paint_bench <url> [width] [height] [scale] [iters] [backend]
@row_add n=62 row=contract/paint-bench-cli/backend area=surface-cli rec=fct-h7hmdmzm@1
boundary: paint_bench command line
item: backend
promise: One of vello (the default), cpu or hybrid.
@row_check n=62 row=contract/paint-bench-cli/backend sha=1ca29cf740ba v=holds judge=sonnet head=edd0cfaea7f0
at: examples/paint_bench.rs:64
quote: let backend: String = args.next().unwrap_or_else(|| "vello".into());
at: examples/paint_bench.rs:182
quote: "cpu" | "vello_cpu" => {
at: examples/paint_bench.rs:209
quote: "hybrid" | "vello_hybrid" => {
@row_add n=63 row=contract/multi-doc-cli/arguments area=surface-cli rec=fct-qxr4jcph@1
boundary: multi_doc command line
item: arguments
promise: multi_doc takes a mode and an iteration count as positional arguments; an unknown mode ends in a panic.
@row_check n=63 row=contract/multi-doc-cli/arguments sha=569e4c8c89cd v=holds judge=sonnet head=edd0cfaea7f0
at: examples/multi_doc.rs:51-54
quote: let iters: usize = std::env::args() .nth(2) .and_then(|s| s.parse().ok()) .unwrap_or(200);
at: examples/multi_doc.rs:108
quote: other => panic!("unknown mode {other}"),
@row_add n=64 row=contract/multi-doc-cli/multi-doc-n area=surface-cli rec=fct-qxr4jcph@1
boundary: multi_doc command line
item: MULTI_DOC_N
promise: Sets the document count, a default applying when absent or unparsed.
@row_check n=64 row=contract/multi-doc-cli/multi-doc-n sha=d096f8700e47 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/multi_doc.rs:55-58
quote: let n: usize = std::env::var("MULTI_DOC_N") .ok() .and_then(|s| s.parse().ok()) .unwrap_or(2000);
@row_add n=65 row=contract/multi-doc-cli/multi-doc-flat area=surface-cli rec=fct-qxr4jcph@1
boundary: multi_doc command line
item: MULTI_DOC_FLAT
promise: A flag that makes multi_doc's top-level elements leaves instead of <div><p/><span/></div>.
@row_check n=65 row=contract/multi-doc-cli/multi-doc-flat sha=54622cef1d01 v=stale judge=sonnet head=edd0cfaea7f0
at: examples/multi_doc.rs:22
quote: let flat = env_flag("MULTI_DOC_FLAT");
shows: MULTI_DOC_FLAT only makes the generated top-level elements leaves; it does not touch style threading (that is MULTI_DOC_SEQ).
cell: promise
value: A flag that makes multi_doc's top-level elements leaves instead of <div><p/><span/></div>.
@row_add n=66 row=contract/multi-doc-cli/multi-doc-seq area=surface-cli rec=fct-qxr4jcph@1
boundary: multi_doc command line
item: MULTI_DOC_SEQ
promise: A flag that sets multi_doc's style threading to Sequential instead of Parallel.
@row_check n=66 row=contract/multi-doc-cli/multi-doc-seq sha=f32d53e5dd99 v=stale judge=sonnet head=edd0cfaea7f0
at: examples/multi_doc.rs:36-37
quote: let style_threading = if env_flag("MULTI_DOC_SEQ") { StyleThreading::Sequential
shows: MULTI_DOC_SEQ only selects StyleThreading::Sequential instead of Parallel; it does not affect the layout of the generated document (that is MULTI_DOC_FLAT).
cell: promise
value: A flag that sets multi_doc's style threading to Sequential instead of Parallel.
@row_add n=67 row=contract/multi-doc-cli/stdout area=surface-cli rec=fct-qxr4jcph@1
boundary: multi_doc command line
item: stdout
promise: multi_doc's timing is printed as one line on stdout.
@row_check n=67 row=contract/multi-doc-cli/stdout sha=68e7361e0769 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/multi_doc.rs:111-115
quote: println!( "{mode}: ok in {:?} (resolve: {:?})", start.elapsed(), resolve_time );
@row_add n=68 row=contract/agent-run-cli/usage area=surface-cli rec=fct-am364u2w@1
boundary: agent-run.sh test contract
item: usage
promise: agent-run.sh <verb> [selection], run from the repository root, with verbs boot, run, status, cleanup, logs and run selections stand, all or a blitz-tests file name.
@row_check n=68 row=contract/agent-run-cli/usage sha=7370e15a44af v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/agent-run.sh:13-15
quote: echo "usage: $0 <verb> [selection]" >&2 echo "verbs: ${VERBS[*]}" >&2 echo "run selections: stand · all · <blitz-tests file name>" >&2
at: scripts/agent-run.sh:8
quote: VERBS=(boot run status cleanup logs)
@row_add n=69 row=contract/agent-run-cli/exit-2 area=surface-cli rec=fct-am364u2w@1
boundary: agent-run.sh test contract
item: exit 2
promise: A usage error prints the usage to stderr and exits 2.
@row_check n=69 row=contract/agent-run-cli/exit-2 sha=92b13d8e42de v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/agent-run.sh:16
quote: exit 2
at: scripts/agent-run.sh:13
quote: echo "usage: $0 <verb> [selection]" >&2
@row_add n=70 row=contract/agent-run-cli/stdout area=surface-cli rec=fct-am364u2w@1
boundary: agent-run.sh test contract
item: stdout
promise: Every verb's stdout is JSON lines only, one object per event.
@row_check n=70 row=contract/agent-run-cli/stdout sha=3fd1afb8ab51 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/agent-run.sh:44-46
quote: def emit(event, append=True): line = json.dumps(event, ensure_ascii=False) print(line, flush=True)
@row_add n=71 row=contract/agent-run-cli/run-log area=surface-cli rec=fct-am364u2w@1
boundary: agent-run.sh test contract
item: target/agent-run/run.log
promise: The raw cargo output stays in this file, never on stdout.
@row_check n=71 row=contract/agent-run-cli/run-log sha=9a5b76fcf918 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/agent-run.sh:204
quote: cargo test -p blitz-tests --locked ${test_args[@]+"${test_args[@]}"} >"$STATE_DIR/run.log" 2>&1 ||
@row_add n=72 row=contract/agent-run-cli/windows-pass-through area=surface-cli rec=fct-am364u2w@1
boundary: agent-run.sh test contract
item: agent-run.ps1
promise: The Windows pass-through to agent-run.sh.
@row_check n=72 row=contract/agent-run-cli/windows-pass-through sha=393927e18648 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/agent-run.ps1:3
quote: & bash "$PSScriptRoot/agent-run.sh" @args
@row_add n=73 row=contract/cold-agent-cli/usage area=surface-cli rec=fct-b7sh5og5@1
boundary: cold-agent.sh run pipe
item: usage
promise: cold-agent.sh <verb> [task], run from the repository root, with verbs run, status, cleanup, logs and the one task stand: six apps in one client session under a fixed prompt.
@row_check n=73 row=contract/cold-agent-cli/usage sha=53991a75251d v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:15
quote: VERBS=(run status cleanup logs)
at: scripts/cold-agent.sh:16
quote: TASKS=(stand)
at: scripts/cold-agent.sh:34
quote: echo "usage: $0 <verb> [task]" >&2
at: scripts/cold-agent.sh:19-20
quote: read -r -d '' STAND_PROMPT <<'PROMPT' || true Use only your tools. The stand has six apps: counter, flight booker, timer, CRUD, cells and the load page. Work on one app at a time: start it, reach its goal, take a snapshot of its final screen, then stop it before you start the next app.
@row_add n=74 row=contract/cold-agent-cli/exit-2 area=surface-cli rec=fct-b7sh5og5@1
boundary: cold-agent.sh run pipe
item: exit 2
promise: A usage error prints to stderr with empty stdout and exits 2, checked before any precondition.
@row_check n=74 row=contract/cold-agent-cli/exit-2 sha=7374bd58be8b v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:34-37
quote: echo "usage: $0 <verb> [task]" >&2 echo "verbs: ${VERBS[*]}" >&2 echo "run tasks: ${TASKS[*]}" >&2 exit 2
at: scripts/cold-agent.sh:551
quote: [[ $# -ge 1 ]] && contains "$1" "${VERBS[@]}" || usage
at: scripts/cold-agent.sh:556
quote: [[ $# -eq 2 ]] && contains "$2" "${TASKS[@]}" || usage
@row_add n=75 row=contract/cold-agent-cli/exit-3 area=surface-cli rec=fct-b7sh5og5@1
boundary: cold-agent.sh run pipe
item: exit 3
promise: run without claude, python3 or a built, executable target/debug/escher-session exits 3 with one stderr line naming the cargo build command to run.
@row_check n=75 row=contract/cold-agent-cli/exit-3 sha=f7e6af774e3e v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:558-561
quote: for tool in claude python3; do if ! command -v "$tool" >/dev/null 2>&1; then echo "cold-agent: $tool is not on PATH" >&2 exit 3
at: scripts/cold-agent.sh:564-566
quote: if [[ ! -f "$BINARY" || ! -x "$BINARY" ]]; then echo "cold-agent: $BINARY is not built — run \`cargo build --locked -p seven_guis --bin escher-session\` first" >&2 exit 3
@row_add n=76 row=contract/cold-agent-cli/stdout area=surface-cli rec=fct-b7sh5og5@1
boundary: cold-agent.sh run pipe
item: stdout
promise: Every verb's stdout is JSON lines only, one object per event; the session transcript and the agent's check stay in files under target/cold-agent, never printed.
@row_check n=76 row=contract/cold-agent-cli/stdout sha=74122a3668bb v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:85
quote: print(line, flush=True)
at: scripts/cold-agent.sh:585
quote: ) </dev/null >"$state/transcript.jsonl" 2>"$state/client.log" || client_exit=$?
at: scripts/cold-agent.sh:50
quote: CHECK = os.path.join(STATE, "check.json")
@row_add n=77 row=contract/cold-agent-cli/record-dir area=surface-cli rec=fct-b7sh5og5@1
boundary: cold-agent.sh run pipe
item: target/cold-agent/record
promise: The live run's server records its kept checks into this owner-only directory, removed by cleanup with the state, the checks never printed; replays run without recording.
@row_check n=77 row=contract/cold-agent-cli/record-dir sha=ff607e22b38d v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:571
quote: mkdir -m 700 "$STATE_DIR/record"
at: scripts/cold-agent.sh:504
quote: server = 'exec env RUST_LOG=info "$1" mcp --record "$3" 2>"$2"'
at: scripts/cold-agent.sh:595
quote: rm -rf "$STATE_DIR"
@row_add n=78 row=contract/cold-agent-cli/windows-pass-through area=surface-cli rec=fct-b7sh5og5@1
boundary: cold-agent.sh run pipe
item: cold-agent.ps1
promise: The Windows pass-through to cold-agent.sh.
@row_check n=78 row=contract/cold-agent-cli/windows-pass-through sha=acb877560f7c v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.ps1:3
quote: & bash "$PSScriptRoot/cold-agent.sh" @args
@row_add n=79 row=decision/cold-agent-prompt-names-no-id area=surface-cli rec=fct-b7sh5og5@1
name: Cold-agent prompt names no id
rule: The cold-agent stand run uses one fixed prompt that names no element id or app slug.
@row_check n=79 row=decision/cold-agent-prompt-names-no-id sha=2cba506eace0 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:20
quote: Use only your tools. The stand has six apps: counter, flight booker, timer, CRUD, cells and the load page. Work on one app at a time: start it, reach its goal, take a snapshot of its final screen, then stop it before you start the next app.
@row_add n=80 row=decision/cold-agent-pipe-builds-nothing area=surface-cli rec=fct-b7sh5og5@1
name: Cold-agent pipe builds nothing
rule: The cold-agent pipe builds nothing; a missing escher-session binary is a failed precondition.
@row_check n=80 row=decision/cold-agent-pipe-builds-nothing sha=51fe48d443b6 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:564-566
quote: if [[ ! -f "$BINARY" || ! -x "$BINARY" ]]; then echo "cold-agent: $BINARY is not built — run \`cargo build --locked -p seven_guis --bin escher-session\` first" >&2 exit 3
@row_add n=81 row=decision/cold-agent-uses-driver-mcp area=surface-cli rec=fct-b7sh5og5@1
name: Cold agent drives through the driver's MCP server
rule: The cold-agent client is given the driver's own MCP server, escher-session mcp, spawned through a per-run MCP configuration.
@row_check n=81 row=decision/cold-agent-uses-driver-mcp sha=90d751556cab v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:504-506
quote: server = 'exec env RUST_LOG=info "$1" mcp --record "$3" 2>"$2"' config = {"mcpServers": {"escher": {"type": "stdio", "command": "sh", "args": ["-c", server, "sh", binary, server_log, record]}}}
@row_add n=82 row=contract/escher-session-command/form area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session command line
item: command form
promise: Every command is escher-session <verb> [arguments] --session <dir> over the fourteen verbs of the driver's verb table, --session exactly once on every command but help.
@row_check n=82 row=contract/escher-session-command/form sha=69b65f287dff v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:577-579
quote: pub const VERBS: &[VerbSpec] = &[ SNAPSHOT, CLICK, TYPE, PRESS, ADVANCE, SCROLL, SCREENSHOT, LINT, GRAPH, ACTIONS, START, STATUS, STOP, HELP,
at: packages/escher-driver/src/cli.rs:218-220
quote: let [Some(state_dir)] = state_dirs.as_slice() else { return Line::Usage; };
at: packages/escher-driver/src/cli.rs:213-214
quote: return match (state_dirs.as_slice(), records.as_slice()) { ([] | [Some(_)], []) => Line::Help,
@row_add n=83 row=contract/escher-session-exit-codes/0 area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session exit codes
item: 0
promise: Accepted: the answer as one line of uncoloured JSON on stdout.
@row_check n=83 row=contract/escher-session-exit-codes/0 sha=8bb23ce4abd3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:340-343
quote: Ending::Accepted(answer) => Written { stdout: Some(answer.clone()), stderr: None, status: ACCEPTED,
@row_add n=84 row=contract/escher-session-exit-codes/1 area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session exit codes
item: 1
promise: Refused: the refusal's JSON on stdout.
@row_check n=84 row=contract/escher-session-exit-codes/1 sha=34f4de640c62 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:345-348
quote: Ending::Refused(refusal) => Written { stdout: Some(refusal.clone()), stderr: None, status: REFUSED,
@row_add n=85 row=contract/escher-session-exit-codes/2 area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session exit codes
item: 2
promise: Usage: one fixed line on stderr built from the verb table and the task names, stdout empty, nothing booted and no state created.
@row_check n=85 row=contract/escher-session-exit-codes/2 sha=74959234b60f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:350-353
quote: Ending::Usage => Written { stdout: None, stderr: Some(usage(apps)), status: USAGE,
@row_add n=86 row=contract/escher-session-exit-codes/3 area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session exit codes
item: 3
promise: Session error: the error object with its fixed remedy on stdout and its fixed message on stderr.
@row_check n=86 row=contract/escher-session-exit-codes/3 sha=cdb6e762e7e4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:355-358
quote: Ending::Failed(error) => Written { stdout: Some(error.to_json()), stderr: Some(error.to_string()), status: SESSION_ERROR,
at: packages/escher-driver/src/json.rs:544
quote: error.text("remedy", self.remedy());
@row_add n=87 row=contract/escher-session-refusals/unknown-verb area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session refusals
item: unknown-verb
promise: A first word the verb table does not hold is refused unknown-verb, exit 1.
@row_check n=87 row=contract/escher-session-refusals/unknown-verb sha=694f16ed8d3a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:200-201
quote: let Some(spec) = schema::verb(word) else {         return Line::Refused(Refusal::new(Cause::UnknownVerb));
@row_add n=88 row=contract/escher-session-refusals/fonts-unavailable area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session refusals
item: fonts-unavailable
promise: screenshot or lint on an app with no font it can paint text with is refused fonts-unavailable, exit 1.
@row_check n=88 row=contract/escher-session-refusals/fonts-unavailable sha=55dfab427c84 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:307
quote: ScreenshotError::FontsUnavailable => Refusal::new(Cause::FontsUnavailable),
at: packages/escher-driver/src/lint.rs:224-225
quote: if !self.harness.base().can_paint_text() {             return Err(Refusal::new(Cause::FontsUnavailable));
@row_add n=89 row=contract/escher-session-refusals/off-screen area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session refusals
item: off-screen
promise: An element screenshot that crops to no pixel is refused off-screen, exit 1.
@row_check n=89 row=contract/escher-session-refusals/off-screen sha=ffdcc23b5d8f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:518
quote: let rect = crop(harness, target).ok_or(Refusal::new(Cause::OffScreen))?;
@row_add n=90 row=contract/escher-session-snapshot/answer area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session snapshot
item: answer
promise: One uncoloured line: {"text"} for a whole screen, {"text","part","parts"} for a screen read in parts; a part past the last gives empty text with the same parts.
@row_check n=90 row=contract/escher-session-snapshot/answer sha=85306e0a658b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:458
quote: object.text("text", text);
at: packages/escher-driver/src/json.rs:463-465
quote: object.text("text", text);                 object.count("part", u64::from(*part));                 object.count("parts", u64::from(*parts));
at: packages/escher-driver/src/execute.rs:214
quote: let text = part
@row_add n=91 row=contract/escher-session-snapshot/part area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session snapshot
item: --part
promise: Optional whole number 1 to 1000, part 1 without it; outside the bound refused malformed, exit 1.
@row_check n=91 row=contract/escher-session-snapshot/part sha=814d8cf49536 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:372
quote: .filter(|part| (1..=MAX_PART).contains(part))
at: packages/escher-driver/src/schema.rs:18
quote: pub const MAX_PART: u32 = 1000;
at: packages/escher-driver/src/execute.rs:212
quote: let part = part.unwrap_or(1);
@row_add n=92 row=contract/escher-session-screenshot/answer area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session screenshot
item: answer
promise: One uncoloured line {"width","height","png"} with base64 PNG; with --id the named element cropped to its visible bounds, without it the whole screen.
@row_check n=92 row=contract/escher-session-screenshot/answer sha=838f82995f09 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:494-495
quote: let mut object = shot(*width, *height);                 object.text("png", &image::base64(png));
at: packages/escher-driver/src/execute.rs:296
quote: let shot = match id {
@row_add n=93 row=contract/escher-session-lint/answer area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session lint
item: answer
promise: No argument; one uncoloured line {"findings"}, each finding keyed id, defect, bounds then its reason's keys, carrying ids, closed words and numbers only.
@row_check n=93 row=contract/escher-session-lint/answer sha=750bbc8a27bc v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:390-392
quote: object.text("id", &finding.id);     object.text("defect", finding.defect.name());     object.json("bounds", &bounds(finding.bounds));
at: packages/escher-driver/src/json.rs:500
quote: object.json("findings", &list(findings.iter().map(finding)));
@row_add n=94 row=contract/escher-session-graph/answer area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session graph
item: answer
promise: No argument; one uncoloured line {"nodes","states"} carrying ids, component names, handler event names, state names, closed flag words and booleans only, never a value.
@row_check n=94 row=contract/escher-session-graph/answer sha=b99f70eae62b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:505-506
quote: object.json("nodes", &list(graph.nodes.iter().map(graph_node)));                 object.json("states", &list(graph.states.iter().map(graph_state)));
at: packages/escher-driver/src/json.rs:281
quote: object.text("component", component);
@row_add n=95 row=contract/escher-session-actions/answer area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session actions
item: answer
promise: No argument; one unpaged uncoloured line {"nodes"} of id, owner, keyed and verbs with verb and cause, owner and cause left out when absent, never a value, typed text or NodeId.
@row_check n=95 row=contract/escher-session-actions/answer sha=d49a91fbb7ad v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:255-259
quote: object.text("id", &node.id);     if let Some(owner) = &node.owner {         object.text("owner", owner);     }     object.flag("keyed", node.keyed);
at: packages/escher-driver/src/json.rs:264-266
quote: entry.text("verb", reading.verb);             if let Some(cause) = reading.cause {                 entry.text("cause", cause.name());
@row_add n=96 row=contract/escher-session-help/answer area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session help
item: answer
promise: One uncoloured JSON line with seven keys (verbs, node_keys, key_names, busy_classes, causes, exit_statuses, session_errors), fixed schema strings, reaching no session.
@row_check n=96 row=contract/escher-session-help/answer sha=ea61b8c22f4e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/describe.rs:54-55
quote: for (field, value) in HELP.fields.iter().zip(values) {         document.json(field.name, &value);
at: packages/escher-driver/src/schema.rs:527
quote: name: "verbs",
at: packages/escher-driver/src/schema.rs:567
quote: name: "session_errors",
@row_add n=97 row=contract/escher-session-served-tasks/counter area=surface-cli rec=fct-exqct2ck@4,fct-nxmh4fah@2
boundary: escher-session served tasks
item: counter
promise: A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
@row_check n=97 row=contract/escher-session-served-tasks/counter sha=94a9cf21f13e v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/session_host.rs:24
quote: ("counter", Served::Lean(LeanTask::Counter)),
at: packages/escher-driver/src/cli.rs:400
quote: apps.join(" | ")
at: packages/escher-driver/src/mcp/listing.rs:72
quote: property.json("enum", &strings(apps.iter().copied()));
@row_add n=98 row=contract/escher-session-served-tasks/flight-booker area=surface-cli rec=fct-exqct2ck@4,fct-nxmh4fah@2
boundary: escher-session served tasks
item: flight-booker
promise: A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
@row_check n=98 row=contract/escher-session-served-tasks/flight-booker sha=89fdaf0b1e39 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/session_host.rs:25
quote: ("flight-booker", Served::Lean(LeanTask::FlightBooker)),
at: packages/escher-driver/src/cli.rs:400
quote: apps.join(" | ")
at: packages/escher-driver/src/mcp/listing.rs:72
quote: property.json("enum", &strings(apps.iter().copied()));
@row_add n=99 row=contract/escher-session-served-tasks/timer area=surface-cli rec=fct-exqct2ck@4,fct-nxmh4fah@2
boundary: escher-session served tasks
item: timer
promise: A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
@row_check n=99 row=contract/escher-session-served-tasks/timer sha=74b7575621eb v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/session_host.rs:26
quote: ("timer", Served::Lean(LeanTask::Timer)),
at: packages/escher-driver/src/cli.rs:400
quote: apps.join(" | ")
at: packages/escher-driver/src/mcp/listing.rs:72
quote: property.json("enum", &strings(apps.iter().copied()));
@row_add n=100 row=contract/escher-session-served-tasks/crud area=surface-cli rec=fct-exqct2ck@4,fct-nxmh4fah@2
boundary: escher-session served tasks
item: crud
promise: A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
@row_check n=100 row=contract/escher-session-served-tasks/crud sha=397ada9ad14c v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/session_host.rs:27
quote: ("crud", Served::Lean(LeanTask::Crud)),
at: packages/escher-driver/src/cli.rs:400
quote: apps.join(" | ")
at: packages/escher-driver/src/mcp/listing.rs:72
quote: property.json("enum", &strings(apps.iter().copied()));
@row_add n=101 row=contract/escher-session-served-tasks/cells area=surface-cli rec=fct-exqct2ck@4,fct-nxmh4fah@2
boundary: escher-session served tasks
item: cells
promise: A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
@row_check n=101 row=contract/escher-session-served-tasks/cells sha=7409201a6297 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/session_host.rs:28
quote: ("cells", Served::Cells),
at: packages/escher-driver/src/cli.rs:400
quote: apps.join(" | ")
at: packages/escher-driver/src/mcp/listing.rs:72
quote: property.json("enum", &strings(apps.iter().copied()));
@row_add n=102 row=contract/escher-session-served-tasks/load area=surface-cli rec=fct-exqct2ck@4,fct-nxmh4fah@2
boundary: escher-session served tasks
item: load
promise: A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
@row_check n=102 row=contract/escher-session-served-tasks/load sha=4207c24ef506 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/session_host.rs:29
quote: ("load", Served::Load),
at: packages/escher-driver/src/cli.rs:400
quote: apps.join(" | ")
at: packages/escher-driver/src/mcp/listing.rs:72
quote: property.json("enum", &strings(apps.iter().copied()));
@row_add n=103 row=decision/no-caller-input-echoed area=surface-cli rec=fct-exqct2ck@4
name: Caller input never written back
rule: No escher-session command's ending writes back anything the caller typed, a recording path included.
@row_check n=103 row=decision/no-caller-input-echoed sha=ed605ae91bf9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:350-354
quote: Ending::Usage => Written { stdout: None, stderr: Some(usage(apps)), status: USAGE, },
at: packages/escher-driver/src/cli.rs:355-359
quote: Ending::Failed(error) => Written { stdout: Some(error.to_json()), stderr: Some(error.to_string()), status: SESSION_ERROR, },
searched: stderr: Some(usage(apps)) is the fixed line, no caller path or argument is formatted into any Written arm · in packages/escher-driver/src/cli.rs:338-372
@row_add n=104 row=decision/mcp-server-local-stdio area=surface-cli rec=fct-exqct2ck@4
name: MCP server local over stdio
rule: The escher-session mcp role is an MCP server over stdio, local to the invoking user, with no listener or auth surface.
@row_check n=104 row=decision/mcp-server-local-stdio sha=fd7398931332 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp.rs:84-90
quote: pub fn serve_mcp( mut input: impl BufRead, mut output: impl Write, apps: &[&str], boot: impl FnMut(&str) -> Result<Session, SessionError>, idle_expiry: Duration, ) -> Result<(), SessionError> {
searched: TcpListener · in packages/escher-driver/src
searched: TcpStream · in packages/escher-driver/src
searched: UnixListener (only host.rs and client.rs session socket, none in the mcp role) · in packages/escher-driver/src
@row_add n=105 row=contract/escher-session-mcp/stdio area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session MCP server
item: stdio
promise: Reads newline-delimited JSON-RPC 2.0 requests on stdin and writes protocol lines only on stdout, uncoloured with no ESC byte, echoing a request's admitted id.
@row_check n=105 row=contract/escher-session-mcp/stdio sha=0ede30ed9d61 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/framing.rs:197-200
quote: pub(super) fn write_line(output: &mut impl Write, line: &str) -> io::Result<()> { output.write_all(line.as_bytes())?; output.write_all(b"\n")?; output.flush()
at: packages/escher-driver/src/mcp/framing.rs:181
quote: object.json("id", &id.written());
@row_add n=106 row=contract/escher-session-mcp/protocol-revisions area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session MCP server
item: protocol revisions
promise: Speaks four MCP handshake revisions and answers its newest one to a client asking for any revision it does not speak.
@row_check n=106 row=contract/escher-session-mcp/protocol-revisions sha=be9f9621dbf6 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/listing.rs:14
quote: const PROTOCOL_VERSIONS: [&str; 4] = [LATEST_VERSION, "2025-06-18", "2025-03-26", "2024-11-05"];
at: packages/escher-driver/src/mcp/listing.rs:37
quote: .unwrap_or(LATEST_VERSION);
@row_add n=107 row=contract/escher-session-mcp/tools area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session MCP server
item: tools/list
promise: Lists the fourteen verbs of the verb table as tools in order, help last, each description the verb's help then ". Returns: " and its result fields.
@row_check n=107 row=contract/escher-session-mcp/tools sha=5544edacffad v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:578-579
quote: SNAPSHOT, CLICK, TYPE, PRESS, ADVANCE, SCROLL, SCREENSHOT, LINT, GRAPH, ACTIONS, START, STATUS, STOP, HELP,
at: packages/escher-driver/src/mcp/listing.rs:58
quote: format!("{}. Returns: {}", verb.help, fields.join("; "))
at: packages/escher-driver/src/mcp/listing.rs:120
quote: result.json("tools", &list(VERBS.iter().map(|verb| tool(verb, apps))));
@row_add n=108 row=contract/escher-session-mcp/screenshot-result area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session MCP server
item: screenshot result
promise: An accepted screenshot answers an image block (base64 PNG, image/png) then a text block without png, isError false; a refused shot one text block, isError true.
@row_check n=108 row=contract/escher-session-mcp/screenshot-result sha=4b5fa968b179 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/result.rs:90-92
quote: block.text("type", "image"); block.text("data", data); block.text("mimeType", "image/png");
at: packages/escher-driver/src/mcp/result.rs:101
quote: result.flag("isError", self.ending != Ending::Accepted);
at: packages/escher-driver/src/mcp.rs:231-232
quote: Ok(Outcome::Shot { width, height, png }) => { ToolResult::shot(width, height, &png)
@row_add n=109 row=contract/escher-session-mcp/result-bound area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session MCP server
item: result bound
promise: A result holding an image is bounded at 1,048,576 bytes by its whole written form, a text-only one by its text.
@row_check n=109 row=contract/escher-session-mcp/result-bound sha=ee28bfa5d528 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:31
quote: pub(crate) const MAX_ANSWER_BYTES: usize = 1_048_576;
at: packages/escher-driver/src/mcp/result.rs:75-78
quote: let size = match self.image { Some(_) => self.written().len(), None => self.text.len(), };
@row_add n=110 row=contract/escher-session-mcp/end area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session MCP server
item: end
promise: The server's end writes nothing on stdout: exit 0 on Ok, exit 3 with the error's fixed message on stderr on Err.
@row_check n=110 row=contract/escher-session-mcp/end sha=41e9c97fd7bf v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:360
quote: Ending::Served | Ending::Spoken(Ok(())) => Written {
at: packages/escher-driver/src/cli.rs:363
quote: status: ACCEPTED,
at: packages/escher-driver/src/cli.rs:366
quote: Ending::Spoken(Err(error)) => Written {
at: packages/escher-driver/src/cli.rs:369
quote: status: SESSION_ERROR,
@row_add n=111 row=contract/escher-session-coverage/usage area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session coverage role
item: usage
promise: escher-session coverage <app> <check>..., no --session or --record; any other shape is usage, exit 2.
@row_check n=111 row=contract/escher-session-coverage/usage sha=1071bc58fb69 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:185-190
quote: [app, checks @ ..] if apps.contains(&app.as_str()) && !checks.is_empty() && !checks .iter() .any(|check| check == SESSION_OPTION || check == RECORD_OPTION) =>
at: packages/escher-driver/src/cli.rs:197
quote: _ => Line::Usage,
@row_add n=112 row=contract/escher-session-coverage/check-bound area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session coverage role
item: check file
promise: Each check is read bounded at 1,048,576 + 1 bytes; unreadable, oversized, not a check, or for another app is refused malformed, exit 1, holding nothing of the path or file.
@row_check n=112 row=contract/escher-session-coverage/check-bound sha=8f6915ba3b61 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:498
quote: let bound = u64::try_from(MAX_CHECK_BYTES).ok()?.saturating_add(1);
at: packages/escher-driver/src/cli.rs:465
quote: .map(|path| read_check(path).filter(|check| check.app() == app))
at: packages/escher-driver/src/cli.rs:471
quote: None => Ending::Refused(Refusal::new(Cause::Malformed).to_json()),
@row_add n=113 row=contract/escher-session-coverage/answer area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session coverage role
item: answer
promise: Each check replayed on a fresh boot; one uncoloured line {"unexercised":[{id,owner,event}],"checks","held"}, owner left out when absent; a failed boot is the session error, exit 3.
@row_check n=113 row=contract/escher-session-coverage/answer sha=b3a36741922f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:469
quote: handler_coverage(&checks, || boot(&app)).map(|coverage| coverage.to_json()),
at: packages/escher-driver/src/json.rs:430-431
quote: if let Some(owner) = &handler.owner { entry.text("owner", owner);
at: packages/escher-driver/src/json.rs:437-438
quote: object.count("checks", u64::try_from(self.checks).unwrap_or(u64::MAX)); object.count("held", u64::try_from(self.held).unwrap_or(u64::MAX));
at: packages/escher-driver/src/cli.rs:355-358
quote: Ending::Failed(error) => Written { stdout: Some(error.to_json()), stderr: Some(error.to_string()), status: SESSION_ERROR,
@row_add n=114 row=contract/escher-session-host/log-service-name area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session host log
item: service.name
promise: The host's stderr log lines carry service.name=seven_guis.
@row_check n=114 row=contract/escher-session-host/log-service-name sha=6af67758a095 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/session_host.rs:32
quote: if let Err(error) = escher_telemetry::init(escher_telemetry::service_identity!()) {
at: packages/escher-telemetry/src/lib.rs:62
quote: name: env!("CARGO_PKG_NAME"),
at: packages/escher-telemetry/src/format.rs:152
quote: " {} {} service.name={} service.version={}",
@row_add n=115 row=decision/session-recording-replayable-check area=surface-cli rec=fct-exqct2ck@4
name: Session recording as a replayable check
rule: --record <dir> keeps a session as one owner-only replayable check, per call its verb, arguments, acceptance, answer digest and masked answer, read back and replayed by the check replay.
why: Ratified.
@row_check n=115 row=decision/session-recording-replayable-check sha=624dfb5d6486 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:129-136
quote: record.text("tool", verb.name); record.json("arguments", &arguments.end()); if masked { record.flag("masked", true); } record.flag("accepted", answer.is_ok()); record.text("digest", &hex(answer_digest(answer))); record.json("answer", &kept_line(answer));
at: packages/escher-driver/src/recording.rs:213
quote: match DirBuilder::new().mode(0o700).create(dir) {
at: packages/escher-driver/src/recording.rs:232
quote: .mode(0o600)
at: packages/escher-driver/src/replay.rs:46
quote: pub fn replay(&self, session: &mut Session) -> Replay {
@row_add n=116 row=contract/escher-session-record/check-file area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session recording
item: check-{n}.json
promise: Written 0600, never replacing a file, in a 0700 directory whose parent must exist, removed by escher only when its recording fails; a shot kept without its png.
@row_check n=116 row=contract/escher-session-record/check-file sha=7de9937fca59 v=stale judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:274
quote: let _ = fs::remove_file(file);
shows: The recording removes the check file itself (`remove`, and the session error text says "the check is removed") when a recording fails, so escher does remove checks.
cell: promise
value: Written 0600, never replacing a file, in a 0700 directory whose parent must exist, removed by escher only when its recording fails; a shot kept without its png.
@row_add n=117 row=contract/escher-session-record/record-dir-not-private area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session recording
item: record-dir-not-private
promise: A recording directory open to its group or others is this session error, exit 3, start spawning nothing.
@row_check n=117 row=contract/escher-session-record/record-dir-not-private sha=742bf124ea91 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:217-220
quote: if mode & 0o077 == 0 {                    Ok(())                } else {                    Err(SessionError::RecordDirNotPrivate)
@row_add n=118 row=contract/escher-session-record/recording-failed area=surface-cli rec=fct-exqct2ck@4
boundary: escher-session recording
item: recording-failed
promise: A recording that ends answers this session error, exit 3; over MCP isError.
@row_check n=118 row=contract/escher-session-record/recording-failed sha=0cedc493735b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp.rs:227
quote: return ToolResult::failed(SessionError::RecordingFailed);
@row_none n=1 rec=fct-vrnkoqnl@1 type=4
@row_none n=2 rec=fct-654t6lta@1 type=4
@row_none n=3 rec=fct-n7fg5vzn@1 type=4
@row_none n=4 rec=fct-2nmc7hj4@1 type=S
@row_none n=5 rec=fct-bla7ziiz@1 type=4
relied: The snapshot reads the flight booker's mode select at boot as ComboBox "Flight type" with value one-way and two ListBoxOptions, each at the select's client rect.
@row_none n=6 rec=fct-zk3a7kfe@1 type=4
relied: While a select list is open each row reads its own bounds in the snapshot and the diff, and after a close the rows read back at the select's client rect.
@row_none n=7 rec=fct-7z6s6h7n@1 type=4
@row_none n=8 rec=fct-dj37firg@1 type=4
@row_none n=9 rec=fct-tgcrnro7@1 type=4
@row_none n=10 rec=fct-yzd6nis7@1 type=4
@row_none n=11 rec=fct-4wzmflh5@1 type=4
@row_none n=12 rec=fct-7moghy5x@1 type=S
@row_none n=13 rec=fct-ypw4imkp@1 type=4
@row_none n=14 rec=fct-7tw7uyf2@1 type=4
@row_none n=15 rec=fct-wryrwkrm@1 type=4
@row_none n=16 rec=fct-nz5alqsh@1 type=4
@row_none n=17 rec=fct-p5cpvfgx@1 type=4
@row_none n=18 rec=fct-soimm2g7@1 type=4
@row_none n=19 rec=fct-infp5ncw@1 type=4
@row_none n=20 rec=fct-f4ald5en@1 type=4
@row_none n=21 rec=fct-sf4ltnp3@1 type=4
@row_none n=22 rec=fct-goa5h6cv@1 type=4
@row_none n=23 rec=fct-nasbtqcq@1 type=4
@row_none n=24 rec=fct-3yebjmlo@1 type=4
relied: Every document starts with the fixed skeleton html, head, body and main id="main", and the app mounts into main.
@row_none n=25 rec=fct-bvy3ajht@1 type=4
@row_none n=26 rec=fct-3kjbfxzm@1 type=4
@row_none n=27 rec=fct-aswgwjrz@1 type=4
@row_none n=28 rec=fct-nlsolafi@1 type=4
@row_none n=29 rec=fct-qqfmbabe@1 type=4
@row_none n=30 rec=fct-bb6g774i@1 type=4
@row_none n=31 rec=fct-ou5kiwlz@1 type=4
@row_none n=32 rec=fct-r4mtslde@1 type=4
@row_none n=33 rec=fct-hgjbw4ph@1 type=S
@row_none n=34 rec=fct-crjk6am2@1 type=4
@row_none n=35 rec=fct-fwzvlguk@1 type=4
@row_none n=36 rec=fct-sqslkbda@1 type=4
@row_none n=37 rec=fct-btkcmgpp@1 type=S
@row_none n=38 rec=fct-l463kzol@1 type=4
@row_none n=39 rec=fct-v4i42if5@1 type=4
@row_none n=40 rec=fct-gnuwsgqb@1 type=4
@row_none n=41 rec=fct-wduz3zi7@1 type=4
@row_none n=42 rec=fct-lhnpcvkp@1 type=4
@row_none n=43 rec=fct-lvky7wiz@1 type=4
@row_none n=44 rec=fct-2fi7fsuc@1 type=S
@row_none n=45 rec=fct-covlfp4w@1 type=4
@row_none n=46 rec=fct-rzhqzrl5@1 type=4
@row_none n=47 rec=fct-vorwvhmx@1 type=S
