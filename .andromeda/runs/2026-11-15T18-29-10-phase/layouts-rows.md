# layout-templates · the rows that bind 2026-11-15-taffy-from-upstream-git — picked 18 of 120 (6 by every draw) · rule any · pin 1162

## surface-desktop-native · ## Surface: desktop-native
decision/headless-stand-pinned-viewport · Headless stand pinned viewport — The headless stand skips Home and mounts one task in TaskShell at a pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans.
decision/held-instance-reads-as-fresh-boot · Held instance reads as a fresh boot — A session's held instance reads as a fresh boot, and the TaskShell mount and the pinned viewport hold after commands, in both layout modes. · guard: stand_session_fresh
decision/open-list-rows-own-bounds · Open list rows read their own bounds — While a select list is open each row reads its own bounds in the snapshot and the diff, and after a close the rows read back at the select's client rect. · why: stand_act_popup checks a click opening the list names each row's bounds in the diff, and escape closing it puts the rows back at the select's box
decision/scroll-into-view-moves-every-scroller · Into-view scroll moves every holding scroller — An into-view scroll moves every scrolling box that holds its target, innermost first, and then the viewport, in every document. · why: Ratified for every document. · guard: stand_act_scroll · except: A row of an open select list is scrolled into view by writing the list's scroll offset at once, no scrolling box or viewport moving.
decision/stand-lints-clean-at-boot · Stand lints clean at boot — At boot the counter, the flight booker, the timer, CRUD and the load page read zero visual-lint findings, in both layout modes. · guard: stand_lint
decision/visible-region-through-clipping-boxes · Visible region through clipping boxes — The part of the viewport an element can be seen through is the viewport narrowed to the padding box of every clipping box that holds it, in every document. · why: Ratified for every document. · except: A row or group of an open select list is seen through the list's scrollport alone, no clipping box answering.
decision/flight-type-select-boot-reading · Flight type select reads its two options at boot — The snapshot reads the flight booker's mode select at boot as ComboBox "Flight type" with value one-way and two ListBoxOptions, each at the select's client rect. · why: the_flight_type_select_holds_its_two_options in stand_snapshot pins role, name, value, both ListBoxOptions and their select-rect bounds in both layout modes

## surface-cli · ## Surface: cli
decision/session-recording-replayable-check · Session recording as a replayable check — --record <dir> keeps a session as one owner-only replayable check, per call its verb, arguments, acceptance, answer digest and masked answer, read back and replayed by the check replay. · why: Ratified.
contract/escher-session-coverage/answer · escher-session coverage role · answer — Each check replayed on a fresh boot; one uncoloured line {"unexercised":[{id,owner,event}],"checks","held"}, owner left out when absent; a failed boot is the session error, exit 3.
contract/escher-session-coverage/usage · escher-session coverage role · usage — escher-session coverage <app> <check>..., no --session or --record; any other shape is usage, exit 2.
contract/escher-session-refusals/off-screen · escher-session refusals · off-screen — An element screenshot that crops to no pixel is refused off-screen, exit 1.
contract/escher-session-screenshot/answer · escher-session screenshot · answer — One uncoloured line {"width","height","png"} with base64 PNG; with --id the named element cropped to its visible bounds, without it the whole screen.
contract/escher-session-served-tasks/cells · escher-session served tasks · cells — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/escher-session-served-tasks/counter · escher-session served tasks · counter — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/escher-session-served-tasks/crud · escher-session served tasks · crud — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/escher-session-served-tasks/flight-booker · escher-session served tasks · flight-booker — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/escher-session-served-tasks/load · escher-session served tasks · load — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.
contract/escher-session-served-tasks/timer · escher-session served tasks · timer — A task start, the host role and the MCP start tool boot, listed in the usage line's apps and the start tool's enum.

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
