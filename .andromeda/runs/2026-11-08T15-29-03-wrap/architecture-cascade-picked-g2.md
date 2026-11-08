@account n=1 entity=standard-contracts#2.3 v=not_touched
why: blitz-dom is unchanged (the report's diff-quiet gate on packages/blitz-dom is green) and actions.rs makes no close_select_list or focus_select call, so the callers it lists still hold
@account n=2 entity=standard-contracts#2.6 v=proposed
why: execute.rs is reached and the report says locate now ends in the new target_of that Session::actions also calls, so is_select read by locate alone may no longer hold as written
@account n=3 entity=standard-contracts#5.1 v=not_touched
why: blitz-dom scrolling is unchanged and actions.rs makes no scroll_into_view call, so the caller list of scroll_into_view still holds
@account n=4 entity=standard-contracts#6.2 v=not_touched
why: the report states in_view and crop are unchanged, and they remain the only driver readers of visible_region, now reached through check_target
@account n=5 entity=standard-contracts#6.4 v=not_touched
why: the report states covered is unchanged and blitz-dom's hit walk is untouched
@account n=6 entity=standard-contracts#15.1 v=not_touched
why: the harness crate is unchanged, and the report says the harness still checks can_paint_text itself; check_target's earlier read is a driver fact, not a harness one
