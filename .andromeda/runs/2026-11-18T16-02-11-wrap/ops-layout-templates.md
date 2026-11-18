@row_set n=1 row=contract/cold-agent-cli/usage cell=promise seen=1 detector=none
value: cold-agent.sh <verb> [task], run from the repository root, with verbs run, status, cleanup, logs and the one task stand: seven apps in one client session under a fixed prompt.
basis: fct-uduwzs7i
because: The served-task set moved from six to seven with the order page, and the cold-agent pipe's header and prompt now read seven apps, so the usage row's six is stale.
@row_set n=2 row=decision/stand-lints-clean-at-boot cell=rule seen=1 detector=none
value: At boot the counter, the flight booker, the timer, CRUD, the load page and the order page read zero visual-lint findings, in both layout modes.
basis: fct-kthgvt6d
because: The new order page is checked to read no lint finding at boot in both modes, so the decision's list of pages clean at boot must name it.
@row_add n=1 row=contract/escher-session-served-tasks/order area=surface-cli fact=F1 detector=none
boundary: escher-session served tasks
item: order
promise: The order page; a task start, the host role and the MCP start tool boot, listed after load in the usage line's apps and the start tool's enum.
basis: fct-wnmdhla7
because: escher-session's served tasks gain order as the seventh, started by that name and named in the usage line and the MCP start enum, so the set owes its row.
@row_add n=2 row=contract/cold-agent-verdict/caused-by-kind area=surface-cli fact=F2 detector=none
boundary: cold-agent verdict
item: caused_by_kind
promise: An object of the closed words panic, error and log to counts, each counted once per accepted acting answer whose caused list holds an item of it; any count reading 0 makes capability-unused fire.
basis: fct-vkogrj6o
because: The cold-agent verdict gains the caused_by_kind field and capability-unused reads it, a new promise of the pipe's output.
@row_add n=3 row=decision/order-page-built-to-cause area=surface-desktop-native fact=F3 detector=D-layout-surface
name: Order page controls built to cause
rule: The order page's controls are built to cause: Submit returns an error for a quantity not a whole number from 10 to 20, a good Submit logs one info record order placed, and Export panics with export is not built yet.
why: So a driver and a cold agent meet caused items of every kind: panic, error and log.
guard: stand_order_served
basis: fct-wnmdhla7
because: The chunk adds the order page as a new served stand surface whose controls are made to panic, return an error and log, and the surface must be described in the master.
@row_add n=4 row=decision/order-page-own-error-boundary area=surface-desktop-native fact=F4 detector=D-layout-surface
name: Order page error boundary of its own
rule: The order page holds an error boundary of its own around Submit, which shows Order failed and a Retry button that clears the error, so Retry replaces Submit in the tab order once Submit fails.
guard: stand_order_served
basis: fct-wnmdhla7
because: The new order page adds a failure region of its own, Order failed and Retry in Submit's place, which the master must describe as part of the surface.
@row_add n=5 row=contract/stand-element-ids/order-quantity area=surface-desktop-native fact=F6 detector=D-layout-surface
boundary: Stand stable element ids
item: order-quantity
promise: Stable author element id of the order page's quantity input.
basis: fct-wnmdhla7
because: The order page adds the element id order-quantity, an occupied stand id the set must hold.
@row_add n=6 row=contract/stand-element-ids/order-submit area=surface-desktop-native fact=F7 detector=D-layout-surface
boundary: Stand stable element ids
item: order-submit
promise: Stable author element id of the order page's Submit button.
basis: fct-wnmdhla7
because: The order page adds the element id order-submit, an occupied stand id the set must hold.
@row_add n=7 row=contract/stand-element-ids/order-retry area=surface-desktop-native fact=F8 detector=D-layout-surface
boundary: Stand stable element ids
item: order-retry
promise: Stable author element id of the order page's Retry button, present only once Submit fails.
basis: fct-wnmdhla7
because: The order page adds the element id order-retry, an occupied stand id the set must hold.
@row_add n=8 row=contract/stand-element-ids/order-failed area=surface-desktop-native fact=F9 detector=D-layout-surface
boundary: Stand stable element ids
item: order-failed
promise: Stable author element id of the order page's Order failed message.
basis: fct-wnmdhla7
because: The order page adds the element id order-failed, an occupied stand id the set must hold.
@row_add n=9 row=contract/stand-element-ids/order-export area=surface-desktop-native fact=F10 detector=D-layout-surface
boundary: Stand stable element ids
item: order-export
promise: Stable author element id of the order page's Export button, which panics.
basis: fct-wnmdhla7
because: The order page adds the element id order-export, an occupied stand id the set must hold.
@row_add n=10 row=contract/stand-element-ids/order-status area=surface-desktop-native fact=F11 detector=D-layout-surface
boundary: Stand stable element ids
item: order-status
promise: Stable author element id of the order page's status, reading Order placed after a good Submit.
basis: fct-wnmdhla7
because: The order page adds the element id order-status, an occupied stand id the set must hold.
@row_none n=1 fact=F5 type=4
relied: stand_order_served pins that a caused item from the order page carries no owner, the UI graph reading none for its controls.
@row_none n=2 fact=F12 type=4
relied: stand_keyboard walks the order page's stated tab order, with and without its failure shown; the test table holds the order.
