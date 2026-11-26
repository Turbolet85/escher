# Relay 1 — the operator's answer on the canvas Tab trap (2026-11-26, during /andromeda-implement)

Question asked: Circle Drawer's canvas, once focused, traps Tab: blitz-dom forwards every event on a custom-widget
element to the widget and returns before any default action (events/mod.rs:209-245), so Tab/Shift+Tab never move focus
off it. The plan's a11y acceptance (Tab walk forward and back) cannot read green without an engine change in a file
outside research's list, for every document behind the `custom-widget` feature. How should this chunk proceed?

Option chosen: «Engine: Tab passes through»

Notes, verbatim: A focusable widget that traps Tab is an engine defect (a keyboard trap), fixed where it is: the widget
still gets the keydown, then the Tab arm runs. The operator answers it as a technical fix; record it with the
PROVISIONAL mark. Add an engine-level check on a fixture canvas — Tab and Shift+Tab leave a focused custom-widget
element and the widget still received the key — with its red control, name the file in research lists, flag it
upstreamable in the report.
