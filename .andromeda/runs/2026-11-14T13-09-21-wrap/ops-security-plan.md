@row_add n=1 row=decision/fixture-name-closed-table area=input-validation fact=F1 detector=D-security-input
name: Fixture state name checked against a closed table
rule: A fixture state name is compared whole against the app's closed table of named states before any session starts; any other name is refused as an unknown state and nothing is booted.
basis: fct-urtem5yt
because: The chunk adds a start in a named fixture state whose name is a caller-supplied argument, and its validation as a whole-name lookup in a closed table before the boot is the input rule future work must keep.
@row_add n=2 row=decision/fixture-reach-through-session-run area=input-validation fact=F2 detector=D-security-input
name: Fixture state reached through the session's run path
rule: A session reaches a named fixture state only in process, each step's call run in order through the session's one validated run path, stopping at the first refused or unsettled step with the steps before it standing, not rolled back.
basis: fct-urtem5yt
because: The chunk adds a reach of a fixture state whose steps become calls, and their passing through the one validated run path, never around it, is the boundary's contract.
@row_add n=3 row=absence/fixture-state-on-carriers area=input-validation fact=F3 detector=D-security-input
capability: A fixture state on the command line, MCP, the session socket or a kept check
basis: fct-ggyakcis
because: The founder chose the in-process-only option, so starting in a state crosses no carrier and the whole carrier capability is a ratified absence.
@row_add n=4 row=decision/reach-error-closed-set area=error-handling fact=F4 detector=none
name: Reach errors as a closed set with fixed texts
rule: A reach error is one of a closed set: unknown state, refused at a step with the step's index and its closed refusal cause, or not settled at a step with the busy reading; each reads a fixed text holding nothing a caller supplied.
guard: a_reach_error_reads_fixed_text_naming_no_name_tried
basis: fct-urtem5yt
because: The chunk adds the reach's error type, and its closed set of fixed texts keeps a refused name or step from echoing anything supplied, as the other driver errors do.
