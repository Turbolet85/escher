@row_add n=1 row=contract/driver-fixture-state/start-in-a-state area=surface fact=F1 detector=none
boundary: driver session fixture states
item: start in a state
promise: A driver session can start in a named fixture state of its app, in process only; the name is compared whole against the app's closed table, and the first screen reads as that state reached by hand from boot, in both layout modes.
basis: fct-urtem5yt, fct-bv2xw5ml
because: The chunk adds an in-process start in a named fixture state, a promise of the driver session no row of the surface states yet.
@row_add n=2 row=absence/fixture-state-on-carriers area=surface fact=F2 detector=none
capability: A fixture state on the command line, MCP, the session socket or a kept recording
basis: fct-urtem5yt, fct-ggyakcis, fct-kczcj7hv
because: The founder ratified that no fixture state crosses any driver carrier, so start, its schema and the session errors carry no state, a whole capability the project does not have.
@row_add n=3 row=contract/driver-fixture-state/unknown-state area=surface fact=F3 detector=none
boundary: driver session fixture states
item: UnknownState
promise: A fixture state name outside the app's table is refused UnknownState before anything boots, so nothing is started, and the refusal's text is fixed and holds nothing the caller supplied.
basis: fct-urtem5yt, fct-k57px24b
because: The chunk adds the refusal of an unknown state name, checked before the boot, an outward promise no row states.
@row_add n=4 row=contract/driver-fixture-state/reach-stop area=surface fact=F4 detector=none
boundary: driver session fixture states
item: a reach's stop
promise: A reach runs its steps in order through the validated run path and stops at the first refused or unsettled step, naming the step's index and its closed cause or busy state; the steps before it stand and are not rolled back.
basis: fct-urtem5yt
because: The chunk adds the reach of a state and its stop at the first refused or unsettled step, an outward promise no row states.
@row_add n=5 row=contract/driver-fixture-state/recording area=surface fact=F5 detector=none
boundary: driver session fixture states
item: a recorded session started in a state
promise: A recorded session started in a state replays from boot; the recording keeps the reach's steps as ordinary calls and its check form is unchanged.
basis: fct-bv2xw5ml, fct-urtem5yt
because: The chunk promises that a session started in a state records and replays as ordinary calls from boot, which no row states.
@row_add n=6 row=decision/std-result-under-native-prelude area=surface fact=F6 detector=none
name: Standard Result spelled in full under the native prelude
rule: In a file that globs the dioxus-native prelude, the standard two-parameter Result is spelled in full.
why: On Android the prelude brings a one-parameter Result alias that fails only the Android build.
basis: fct-ez7afaim, fct-n6mllq3p
because: An Android-only compile failure from the prelude's Result alias set a convention every future file globbing the prelude must follow.
