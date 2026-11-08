@answer n=1 entity=input-validation#1.51 v=no
why: the session's files are the escher-driver crate and the dioxus-native-dom actionable and lib modules, and the `<style>` text decode in blitz-dom document.rs is not reached
@answer n=2 entity=input-validation#1.52 v=no
why: the bgcolor parse lives in blitz-dom stylo.rs, which the session does not reach
@answer n=3 entity=input-validation#1.53 v=no
why: the dimension-value parse lives in blitz-dom stylo.rs, which the session does not reach
@answer n=4 entity=input-validation#1.54 v=no
why: the SVG width and height check lives in blitz-dom stylo.rs, which the session does not reach
@answer n=5 entity=input-validation#1.55 v=no
why: the border and body margin parse lives in blitz-dom stylo.rs, which the session does not reach
@answer n=6 entity=input-validation#1.56 v=no
why: the markup numeric parses live in blitz-dom layout modules the session does not reach
@answer n=7 entity=input-validation#1.57 v=no
why: the rowspan clamp lives in blitz-dom layout/table.rs, which the session does not reach
@answer n=8 entity=input-validation#1.58 v=no
why: the multiple-select size read lives in blitz-dom select_list.rs, stylo.rs and default.css, none of which the session reaches
@answer n=9 entity=input-validation#1.59 v=no
why: the ol start parse lives in blitz-dom layout/construct.rs, which the session does not reach
@answer n=10 entity=input-validation#1.60 v=no
why: attr_parsed lives in blitz-dom element.rs, which the session does not reach
@answer n=11 entity=input-validation#1.61 v=no
why: the disabled presence reading stays in blitz-dom, and the session's changed actionable.rs keeps focusability as a leg of the shared rule, so the claim that unkeyed_actionable reads disabled through focusability still holds on the changed file
@answer n=12 entity=input-validation#1.62 v=no
why: the open-state reading lives in blitz-dom element.rs, mutator.rs and stylo.rs, which the session does not reach
@answer n=13 entity=input-validation#1.63 v=no
why: autofocus is read in blitz-dom mutator.rs, which the session does not reach
@answer n=14 entity=input-validation#1.64 v=no
why: dir matching lives in blitz-dom and is not reached by the session
@answer n=15 entity=input-validation#1.65 v=no
why: the canvas src parse lives in blitz-dom mutator.rs, which the session does not reach
@answer n=16 entity=input-validation#1.66 v=no
why: custom widget attribute replay lives in blitz-dom element.rs and mutator.rs, which the session does not reach
@answer n=17 entity=input-validation#1.67 v=yes mode=add-to
why: the session's actions read takes the ids out through element_ids (F1) and its answer carries ids and owner ids on the driver answers, and the shared Readings joins the id readers through actionable_nodes beside unkeyed_actionable (F4)
@answer n=18 entity=input-validation#1.68 v=no
why: the `///` virtual id form is built in element_id.rs, which the session does not reach, and no walk the session adds calls virtual_node_id
@answer n=19 entity=input-validation#1.69 v=no
why: examples/seven_guis/tests/mcp_log.rs gained actions needles beside its name and id needles and still reads 0 id and 0 name needles at trace under both builds, so the claim holds on the changed file, and the actions answer carries no name
@answer n=20 entity=input-validation#1.70 v=no
why: aria-describedby and aria-description are read in blitz-dom accessibility.rs, which the session does not reach, and the actions answer carries no description
@answer n=21 entity=input-validation#1.71 v=no
why: aria-expanded, aria-disabled and aria-level are read in blitz-dom accessibility.rs, which the session does not reach
@answer n=22 entity=input-validation#1.72 v=no
why: the select open and aria-expanded reading lives in blitz-dom accessibility.rs and select_popup tests, which the session does not reach
@answer n=23 entity=input-validation#1.73 v=no
why: the control-state readings live in blitz-dom accessibility.rs and the snapshot, neither reached, and the actions answer carries verb and cause words only, no states
@answer n=24 entity=input-validation#1.74 v=no
why: the native checked state is read in blitz-dom accessibility.rs, which the session does not reach
@answer n=25 entity=input-validation#1.75 v=no
why: inert is read in blitz-dom node.rs and accessibility.rs, which the session does not reach, and unkeyed_actionable's skip rests on focusability and the accessibility tree, neither of which the session moved; only the listener source moved
