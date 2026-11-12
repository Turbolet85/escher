@row_add n=1 row=decision/advance-moves-clock-by-exactly-ms area=mocking-stubbing-discipline rec=1
name: Advance moves the virtual clock by exactly ms
rule: The driver's advance moves the session's clock, the harness's VirtualClock that every harness-built document reads, by exactly ms inside one settled step, and advanced_ms reports exactly ms, not what the Timer's counter gained.
why: ratified
guard: stand_act_timer
