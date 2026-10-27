# a11y · Cognitive Accessibility — category read

## Nothing in scope
Nearest: (.andromeda/a11y-plan-detail/cognitive-accessibility.md:9) This line covers the flight booker's invalid-date cue: the `invalid` class, the `aria-invalid` state and the Book button's `disabled` attribute, which the headless stand asserts without colour. It also covers the lean tasks' Tab-order tables and the `focused` reading. The chunk refactors `lint.rs::findings`, one MCP listing test and one requirement-sweep test so each reads at or under the complexity ceiling. It changes no stand task, markup, focus order or check of these cues, and no status cue on the load page (line 19), so none of the category's error-recovery or orientation cues is reached.
