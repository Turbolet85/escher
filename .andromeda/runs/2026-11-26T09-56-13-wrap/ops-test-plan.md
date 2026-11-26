@row_set n=1 row=contract/refusal-cause/covered cell=promise seen=2 detector=none
value: A target a hit at its centre does not reach, or a virtual node with an element over its region's centre or a later node of its canvas holding that centre, is refused `covered`; the fixed meaning also says an option of a closed select lies under it, to click the select first and then the option.
basis: fct-wndwrhax, fct-yen3arbg, fct-arwtq2r5
because: The `covered` meaning now also reads a virtual node covered by a later-declared node of its canvas whose region holds its centre, so the contract's promise must name that clause.
@row_set n=2 row=decision/circles-apart-written-through-attribute cell=rule seen=1 detector=none
value: A Circle Drawer check that reads circles apart at widget level writes `data-circles` through the mutator; a check whose click or key acts on circles apart adds them by harness clicks off the centre, since a `data-circles` write sets no app state.
basis: fct-lpo4g6lz, fct-gezhfiwk, fct-pobztq6c
because: A centre click now selects and a `data-circles` write never reaches the app's circles, so only widget-level readings still write the attribute and the rule must say how click and key cases get circles apart.
@row_add n=1 row=decision/widget-tab-pass-through-engine-checked area=integration-test-strategy fact=F1 detector=D-tests-coverage
name: Tab past a focused widget checked at engine level
rule: A focused custom-widget element passes Tab and Shift+Tab on to the focus order after the widget hears the key, held by an engine-level check on a fixture canvas in both layout modes with its red control.
why: Differs from upstream, whose widget arm returns before every default action so a focused widget element traps Tab; flagged upstreamable.
guard: stand_canvas_click
state: provisional
basis: fct-o2pekkuo, fct-hssrbcfb, fct-yen3arbg
because: The chunk changes engine behaviour for every custom-widget document and the plan holds no row saying how that fork difference is tested.
@row_add n=2 row=decision/canvas-keyboard-focus-readings-agree area=integration-test-strategy fact=F2 detector=none
name: Canvas keyboard checks read three focus readings
rule: A canvas keyboard check reads the engine's, the snapshot's and the accessibility tree's focus and requires the three to agree, with no virtual node ever the tree's focus.
guard: stand_keyboard, stand_circle_nodes
basis: fct-da5yvz32, fct-yen3arbg
because: The canvas keyboard path brings a check convention over three focus readings that no row of the plan states.
@row_none n=1 fact=F3 type=4
relied: Circle Drawer's checks rely on `circle-close` lying below the 600 px view, scrolling it into view or reaching it by tab and enter, and re-clicking the canvas after the dialog closes; the layout and the app code hold it, the refocus limit is a CARRY on the route.
@row_none n=2 fact=F4 type=4
relied: Canvas click checks rely on the engine focusing a custom-widget element on pointer down; pinning a fact by behaviour rather than a guarded call stands already in decision/inert-guard-dropped and decision/focus-test-pins-where-focus-lands.
