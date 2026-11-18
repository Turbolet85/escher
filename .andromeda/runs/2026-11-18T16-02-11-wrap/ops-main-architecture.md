@row_set n=1 row=contract/driver-forms/is-select cell=promise seen=1 detector=none
value: is_select is read by the driver's crate-private target_of, which both locate and Session::actions call.
basis: fct-rvcgucdu
because: The row says locate alone reads it, while target_of, which reads it, is also called from Session::actions — measured at the cold-agent-bar wrap and owed at this entry's wrap by CARRY cry-72hc55ey.
