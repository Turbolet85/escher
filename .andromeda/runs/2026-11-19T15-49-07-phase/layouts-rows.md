# layout-templates · the rows that bind 2026-11-19-upstream-sync-ahead-of-the-debts — picked 79 of 129 (66 by every draw) · rule any · pin 2347

## surface-desktop-native · ## Surface: desktop-native
decision/crud-list-keyboard-region · CRUD list as one keyboard region — The CRUD list is one Tab stop whose ArrowDown, ArrowUp, Home and End move the selected row and fill the name fields as a click does; focus stays on the list, never a row, and no aria-activedescendant is read. · why: Ratified as a decision.
decision/flight-type-select-boot-reading · Flight type select reads its two options at boot — The snapshot reads the flight booker's mode select at boot as ComboBox "Flight type" with value one-way and two ListBoxOptions, each at the select's client rect. · why: the_flight_type_select_holds_its_two_options in stand_snapshot pins role, name, value, both ListBoxOptions and their select-rect bounds in both layout modes
decision/headless-stand-pinned-viewport · Headless stand pinned viewport — The headless stand skips Home and mounts one task in TaskShell at a pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans.
decision/held-instance-reads-as-fresh-boot · Held instance reads as a fresh boot — A session's held instance reads as a fresh boot, and the TaskShell mount and the pinned viewport hold after commands, in both layout modes. · guard: stand_session_fresh
decision/load-page-only-network · Load page alone has a network provider — Only the load page's boot has a network provider, an in-process stand-in answering after 200 ms from a thread of its own, so a driver click on load-fetch waits it out; every other boot keeps no network provider. · guard: stand_load_page
decision/open-list-rows-own-bounds · Open list rows read their own bounds — While a select list is open each row reads its own bounds in the snapshot and the diff, and after a close the rows read back at the select's client rect. · why: stand_act_popup checks a click opening the list names each row's bounds in the diff, and escape closing it puts the rows back at the select's box
decision/scroll-into-view-moves-every-scroller · Into-view scroll moves every holding scroller — An into-view scroll moves every scrolling box that holds its target, innermost first, and then the viewport, in every document. · why: Ratified for every document. · guard: stand_act_scroll · except: A row of an open select list is scrolled into view by writing the list's scroll offset at once, no scrolling box or viewport moving.
decision/session-roles-share-stand-boot · Session roles share the stand boot — Every escher-session role that boots a task (host, mcp, coverage) boots it through the same stand boot the command line uses, adding no markup, id, class, style, wrapper or order.
decision/stand-a11y-by-attributes-alone · Stand names, roles and states by attributes alone — Stand controls take accessible names, roles and states by attributes alone (for, aria-label, aria-invalid, role, aria-selected), adding no element, class, style, key, id or order, the classes that style invalid and selected states staying.
decision/stand-ids-as-attributes-alone · Stand ids as attributes alone — Author ids on stand elements are added as attributes alone, moving no element, class, style, wrapper or order, and no stand CSS selects by them.
decision/stand-lints-clean-at-boot · Stand lints clean at boot — At boot the counter, the flight booker, the timer, CRUD, the load page and the order page read zero visual-lint findings, in both layout modes. · guard: stand_lint
decision/visible-region-through-clipping-boxes · Visible region through clipping boxes — The part of the viewport an element can be seen through is the viewport narrowed to the padding box of every clipping box that holds it, in every document. · why: Ratified for every document. · except: A row or group of an open select list is seen through the list's scrollport alone, no clipping box answering.
contract/shell-provider-window-chrome/drag-window · ShellProvider window chrome controls · drag_window — Offered to a document drawing its own titlebar, to drag the window.
contract/shell-provider-window-chrome/is-window-maximized · ShellProvider window chrome controls · is_window_maximized — Offered to a document drawing its own titlebar, to read whether the window is maximized.
contract/shell-provider-window-chrome/request-window-close · ShellProvider window chrome controls · request_window_close — Offered to a document drawing its own titlebar, to request the window's close.
contract/shell-provider-window-chrome/set-window-decorations · ShellProvider window chrome controls · set_window_decorations — Offered to a document drawing its own titlebar, to set the window's decorations.
contract/shell-provider-window-chrome/set-window-maximized · ShellProvider window chrome controls · set_window_maximized — Offered to a document drawing its own titlebar, to set the window maximized.
contract/shell-provider-window-chrome/set-window-minimized · ShellProvider window chrome controls · set_window_minimized — Offered to a document drawing its own titlebar, to set the window minimized.
contract/stand-element-ids/cell-column-row · Stand stable element ids · cell-{A..Z}{0..25} — Stable author element id of each of the Cells task's 676 data cells, its column letter then its row number.
contract/stand-element-ids/cell-input · Stand stable element ids · cell-input — Stable author element id of the Cells task's one edit input.
contract/stand-element-ids/cells-grid · Stand stable element ids · cells-grid — Stable author element id of the Cells grid, one Tab stop and keyboard region named "Cells", with no role.
contract/stand-element-ids/circle-canvas · Stand stable element ids · circle-canvas — Stable author element id of the Circle Drawer's canvas.
contract/stand-element-ids/circle-close · Stand stable element ids · circle-close — Stable author element id of the Circle Drawer dialog's close control, present only while the dialog is open.
contract/stand-element-ids/circle-diameter · Stand stable element ids · circle-diameter — Stable author element id of the Circle Drawer dialog's range input, present only while the dialog is open.
contract/stand-element-ids/circle-redo · Stand stable element ids · circle-redo — Stable author element id of the Circle Drawer's redo control.
contract/stand-element-ids/circle-undo · Stand stable element ids · circle-undo — Stable author element id of the Circle Drawer's undo control.
contract/stand-element-ids/counter-increment · Stand stable element ids · counter-increment — Stable author element id of the Counter task's increment button.
contract/stand-element-ids/counter-value · Stand stable element ids · counter-value — Stable author element id of the Counter task's value display.
contract/stand-element-ids/crud-create · Stand stable element ids · crud-create — Stable author element id of the CRUD task's Create button.
contract/stand-element-ids/crud-delete · Stand stable element ids · crud-delete — Stable author element id of the CRUD task's Delete button.
contract/stand-element-ids/crud-filter · Stand stable element ids · crud-filter — Stable author element id of the CRUD task's filter input, named by its label's for.
contract/stand-element-ids/crud-list · Stand stable element ids · crud-list — Stable author element id of the CRUD task's people list, a listbox named "People".
contract/stand-element-ids/crud-name · Stand stable element ids · crud-name — Stable author element id of the CRUD task's name input, named by its label's for.
contract/stand-element-ids/crud-person-id · Stand stable element ids · crud-person-{person.id} — Each CRUD row's stable author element id, from its person's model-assigned id beside the same Dioxus key, following its person under filter, Create and Delete.
contract/stand-element-ids/crud-surname · Stand stable element ids · crud-surname — Stable author element id of the CRUD task's surname input, named by its label's for.
contract/stand-element-ids/crud-update · Stand stable element ids · crud-update — Stable author element id of the CRUD task's Update button.
contract/stand-element-ids/flight-book · Stand stable element ids · flight-book — Stable author element id of the flight booker's book button.
contract/stand-element-ids/flight-booked · Stand stable element ids · flight-booked — Stable author element id of the flight booker's booked display.
contract/stand-element-ids/flight-return-date · Stand stable element ids · flight-return-date — Stable author element id of the flight booker's return date input, named "Return date" by aria-label.
contract/stand-element-ids/flight-start · Stand stable element ids · flight-start — Stable author element id of the flight booker's departure date input, named "Departure date" by aria-label.
contract/stand-element-ids/flight-type · Stand stable element ids · flight-type — Stable author element id of the flight booker's mode select.
contract/stand-element-ids/flight-type-one-way · Stand stable element ids · flight-type-one-way — Stable author element id of the flight booker's one-way mode option.
contract/stand-element-ids/flight-type-return · Stand stable element ids · flight-type-return — Stable author element id of the flight booker's return mode option.
contract/stand-element-ids/load-done · Stand stable element ids · load-done — Stable author element id of the load page's "Loaded" paragraph, hidden until the stylesheet the button's click links lands.
contract/stand-element-ids/load-fetch · Stand stable element ids · load-fetch — Stable author element id of the load page's Load button, taking Enter and Space.
contract/stand-element-ids/order-export · Stand stable element ids · order-export — Stable author element id of the order page's Export button, which panics.
contract/stand-element-ids/order-failed · Stand stable element ids · order-failed — Stable author element id of the order page's Order failed message.
contract/stand-element-ids/order-quantity · Stand stable element ids · order-quantity — Stable author element id of the order page's quantity input.
contract/stand-element-ids/order-retry · Stand stable element ids · order-retry — Stable author element id of the order page's Retry button, present only once Submit fails.
contract/stand-element-ids/order-status · Stand stable element ids · order-status — Stable author element id of the order page's status, reading Order placed after a good Submit.
contract/stand-element-ids/order-submit · Stand stable element ids · order-submit — Stable author element id of the order page's Submit button.
contract/stand-element-ids/task-card-cells · Stand stable element ids · task-card-cells — Stable author element id of Home's Cells task-card button.
contract/stand-element-ids/task-card-circle-drawer · Stand stable element ids · task-card-circle-drawer — Stable author element id of Home's Circle Drawer task-card button.
contract/stand-element-ids/task-card-counter · Stand stable element ids · task-card-counter — Stable author element id of Home's Counter task-card button.
contract/stand-element-ids/task-card-crud · Stand stable element ids · task-card-crud — Stable author element id of Home's CRUD task-card button.
contract/stand-element-ids/task-card-flight-booker · Stand stable element ids · task-card-flight-booker — Stable author element id of Home's Flight Booker task-card button.
contract/stand-element-ids/task-card-temp-converter · Stand stable element ids · task-card-temp-converter — Stable author element id of Home's Temperature Converter task-card button.
contract/stand-element-ids/task-card-timer · Stand stable element ids · task-card-timer — Stable author element id of Home's Timer task-card button.
contract/stand-element-ids/temp-celsius · Stand stable element ids · temp-celsius — Stable author element id of the Temperature Converter's Celsius control.
contract/stand-element-ids/temp-fahrenheit · Stand stable element ids · temp-fahrenheit — Stable author element id of the Temperature Converter's Fahrenheit control.
contract/stand-element-ids/timer-duration · Stand stable element ids · timer-duration — Stable author element id of the Timer task's duration input, named by its label's for.
contract/stand-element-ids/timer-duration-value · Stand stable element ids · timer-duration-value — Stable author element id of the Timer task's duration value display.
contract/stand-element-ids/timer-elapsed · Stand stable element ids · timer-elapsed — Stable author element id of the Timer task's elapsed display.
contract/stand-element-ids/timer-progress · Stand stable element ids · timer-progress — Stable author element id of the Timer task's progress display.
contract/stand-element-ids/timer-reset · Stand stable element ids · timer-reset — Stable author element id of the Timer task's reset button.

## surface-cli · ## Surface: cli
contract/escher-session-screenshot/answer · escher-session screenshot · answer — One uncoloured line {"width","height","png"} with base64 PNG; with --id the named element cropped to its visible bounds, without it the whole screen.
decision/mcp-server-local-stdio · MCP server local over stdio — The escher-session mcp role is an MCP server over stdio, local to the invoking user, with no listener or auth surface.
decision/no-caller-input-echoed · Caller input never written back — No escher-session command's ending writes back anything the caller typed, a recording path included. · except: The layout answer's id field names the element the call's id named.
contract/escher-session-layout/answer · escher-session layout · answer — One required --id; one uncoloured line of id, bounds and laid_out (x, y, width, height), placed (by, container, canvas, at, relative), shifts (by, holder, dx, dy), width and height (px, by, size, min, max, flex with basis, grow, shrink); ids, closed words and numbers only.
contract/escher-session-refusals/fonts-unavailable · escher-session refusals · fonts-unavailable — screenshot or lint on an app with no font it can paint text with is refused fonts-unavailable, exit 1.
contract/escher-session-served-tasks/cells · escher-session served tasks · cells — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/escher-session-served-tasks/counter · escher-session served tasks · counter — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/escher-session-served-tasks/crud · escher-session served tasks · crud — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/escher-session-served-tasks/flight-booker · escher-session served tasks · flight-booker — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/escher-session-served-tasks/load · escher-session served tasks · load — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/escher-session-served-tasks/order · escher-session served tasks · order — The order page; a task start, the host role and the MCP start tool boot, listed after load in the usage line's apps and the start tool's enum.
contract/escher-session-served-tasks/timer · escher-session served tasks · timer — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/paint-bench-cli/arguments · paint_bench command line · arguments — paint_bench takes <url> [width] [height] [scale] [iters] [backend], positionally.
contract/paint-bench-cli/backend · paint_bench command line · backend — One of vello (the default), cpu or gpu, vello_gpu also accepted for gpu.

# layout-templates · its absences, names and overviews, whole

## surface-desktop-native · ## Surface: desktop-native
> NOT YET MEASURED — no gathered fact reaches this surface's tooling context, expression level, signature placement or hero/signature section; slice s01's reading did not reach the browser app's window/page layout code (slice s04's facts below cover it).
absence/arbitrary-index-html-templates — Arbitrary index.html templates for a document

## surface-mobile-native · ## Surface: mobile-native
> NOT YET MEASURED — no gathered fact reaches this surface's tooling context, expression level, signature placement, primary screens beyond the browser frame, or hero, content and footer components; slice s01's reading did not reach mobile layout code.

## surface-cli · ## Surface: cli
> NOT YET MEASURED — no gathered fact reaches this surface's tooling context, expression level or signature placement; slice s03's reading did not reach any further CLI help or argument parser beyond the `paint_bench` usage line.

## surface-web-spa · ## Surface: web-spa
> NOT YET MEASURED — no gathered fact reaches this surface's tooling context, expression level, signature placement, navigation, header, footer or IA.

## decisions-log · ## Decisions Log
> NO RECORDED INTENT
