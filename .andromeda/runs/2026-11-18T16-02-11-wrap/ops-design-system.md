@row_set n=1 row=contract/escher-session-tasks/cells cell=promise seen=1 detector=none
value: cells is one of the seven tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
basis: fct-uduwzs7i
because: The served-task set moved from six to seven with order, so the row's count of served tasks must follow.
@row_set n=2 row=contract/escher-session-tasks/counter cell=promise seen=1 detector=none
value: counter is one of the seven tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
basis: fct-uduwzs7i
because: The served-task set moved from six to seven with order, so the row's count of served tasks must follow.
@row_set n=3 row=contract/escher-session-tasks/crud cell=promise seen=1 detector=none
value: crud is one of the seven tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
basis: fct-uduwzs7i
because: The served-task set moved from six to seven with order, so the row's count of served tasks must follow.
@row_set n=4 row=contract/escher-session-tasks/flight-booker cell=promise seen=1 detector=none
value: flight-booker is one of the seven tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
basis: fct-uduwzs7i
because: The served-task set moved from six to seven with order, so the row's count of served tasks must follow.
@row_set n=5 row=contract/escher-session-tasks/load cell=promise seen=1 detector=none
value: load is one of the seven tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
basis: fct-uduwzs7i
because: The served-task set moved from six to seven with order, so the row's count of served tasks must follow.
@row_set n=6 row=contract/escher-session-tasks/timer cell=promise seen=1 detector=none
value: timer is one of the seven tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
basis: fct-uduwzs7i
because: The served-task set moved from six to seven with order, so the row's count of served tasks must follow.
@row_add n=1 row=contract/escher-session-tasks/order area=surface fact=F1 detector=none
boundary: escher-session tasks
item: order
promise: order is one of the seven tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
basis: fct-wnmdhla7
because: escher-session's served-task table gains order as its seventh task, a new item of the closed served-task list this boundary promises.
@row_add n=2 row=name/other/order-quantity area=surface fact=F3 detector=none
class: other
name: order-quantity
basis: fct-wnmdhla7
because: The order page occupies order-quantity as a stand element id.
@row_add n=3 row=name/other/order-submit area=surface fact=F3 detector=none
class: other
name: order-submit
basis: fct-wnmdhla7
because: The order page occupies order-submit as a stand element id.
@row_add n=4 row=name/other/order-retry area=surface fact=F3 detector=none
class: other
name: order-retry
basis: fct-wnmdhla7
because: The order page occupies order-retry as a stand element id.
@row_add n=5 row=name/other/order-failed area=surface fact=F3 detector=none
class: other
name: order-failed
basis: fct-wnmdhla7
because: The order page occupies order-failed as a stand element id.
@row_add n=6 row=name/other/order-export area=surface fact=F3 detector=none
class: other
name: order-export
basis: fct-wnmdhla7
because: The order page occupies order-export as a stand element id.
@row_add n=7 row=name/other/order-status area=surface fact=F3 detector=none
class: other
name: order-status
basis: fct-wnmdhla7
because: The order page occupies order-status as a stand element id.
@row_none n=1 fact=F2 type=4
relied: a caused item's owner is the owner the UI graph reads, so a control keyed by an HTML id under no keyed element carries no owner on its caused items.
@row_none n=2 fact=F4 type=5
