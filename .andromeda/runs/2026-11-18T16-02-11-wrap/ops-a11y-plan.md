@row_set n=1 row=decision/stated-tab-order-walked cell=rule seen=1 fact=F1 detector=D-a11y-surface
value: Each lean task, the load page, Cells and the order page, with its failure shown too, state their Tab order, walked forward and back in both layout modes, engine, snapshot and tree focus agreeing at every step, and every lean task's control lying in one of the two lean orders.
basis: fct-rtciuizy
because: The chunk adds the order page, whose stated Tab order and failure-shown order (Retry in Submit's place) are walked forward and back, so the row naming which pages state and walk their order must name it.
@row_set n=2 row=decision/stand-contrast-asserted cell=rule seen=1 fact=F3 detector=D-a11y-surface
value: SC 1.4.3 is asserted on every measured non-exempt stand row, in both layout modes, on the four lean tasks, the load page, Cells and the order page, at boot and with its failure, Retry and placed status shown, and the check runs in the workspace test leg, per package and in the a11y leg.
basis: fct-nwfaca3u
because: The stand contrast check now asserts the order page's ratios at boot and with its failure, Retry and placed status shown, so the row listing where SC 1.4.3 is asserted must include it.
@row_set n=3 row=decision/stand-shared-tab-order-table cell=except seen=1 fact=F2 detector=none
value: A Tab order reached only after a state change, such as the order page with its failure shown, is stated as a local table of the keyboard check, not in the shared table.
basis: fct-rvcgucdu
because: The failure-shown order of the order page is stated as a local constant of the keyboard check rather than in the shared Tab order table, an exception the row stating focus order once must carry.
@row_none n=1 fact=F4 type=4
relied: a caused item's owner is the UI graph's owner reading, so a control keyed by HTML ids under no keyed element yields caused items with no owner; the code and stand_order_served hold it.
