@row_add n=1 row=decision/advance-moves-session-clock area=standard-contracts rec=1,2
name: Advance moves the session's clock
rule: An advance moves the session's clock by exactly the asked milliseconds on every session, and the app's own time through a caller's time step where one was handed; a session with no time step still takes advance, which moves its clock alone; advance reports the clock's move.
why: Ratified.
