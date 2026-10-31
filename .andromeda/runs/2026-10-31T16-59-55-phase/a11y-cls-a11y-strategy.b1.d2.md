# Classified — a11y · a11y-strategy.b1

## Answers
a11y-strategy#1.1 · yes · the shell's poll refresh rebuilds the tree only on a non-empty changed set, so an open or close the tree must report has to reach that set
a11y-strategy#1.2 · yes · the engine's changed-set marking: a select's changed choice marks it while hover, active, scroll and layout do not, and apply_select_choice marks nothing for the chosen option; the drain-first check clause is S6's own
a11y-strategy#1.3 · no · AccessibilityDeactivated and ActionRequested are shell-side TODOs, and the chunk adds no platform adapter or AT action path
a11y-strategy#1.4 · no · window focus and window bounds forwarded to the adapter are shell window events, which the chunk does not reach
a11y-strategy#2.1 · yes · role mapping follows HTML-AAM, which the optgroup's Role::Group and the option roles the chunk reads must follow
a11y-strategy#2.2 · yes · the optgroup and option roles the chunk reads must not map to Role::Unknown, only html and body being exempt
a11y-strategy#2.3 · yes · an option reads its chosen state as selected with no aria-selected read, an optgroup's non-empty label is its name, and each attribute is read once, which the chunk's option, group and expanded reads rely on
a11y-strategy#2.4 · no · the rule is the toggled state of a native checkbox or radio, and the chunk adds no checkable control
a11y-strategy#2.5 · yes · aria-expanded reads as a boolean, the state the open-list read rests on; the snapshot's enabled reads the disabled attribute, not aria-disabled, which the disabled gate needs
a11y-strategy#3.1 · no · the platform adapter module selection; the chunk builds no adapter
a11y-strategy#3.2 · no · an accesskit_xplat warning about its own approach; the chunk changes no adapter
a11y-strategy#3.3 · no · Android and Windows focus and macOS bounds no-ops in the platform adapters, which the chunk does not touch
a11y-strategy#3.4 · no · the Android adapter's Rect import, a platform-adapter detail the chunk does not reach
a11y-strategy#3.5 · no · the Android adapter's surface-view field, a platform-adapter detail the chunk does not reach
a11y-strategy#4.1 · yes · the accessibility feature gates the snapshot model, Snapshot::diff and unkeyed_actionable, which the options' bounds and the actionable-key rule reach, and dioxus-native forwards that feature
