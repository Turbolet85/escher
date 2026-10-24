# Classified — security-plan · input-validation.b5

## Answers
input-validation#1.90 · no · blitz-vibey-script is untouched, so the registerProperty checks stand
input-validation#1.91 · no · blitz-vibey-script is untouched, and the class-list change does not reach the inline style that setProperty and cssText write
input-validation#1.92 · no · blitz-vibey-script is untouched, and the session adds no argument check on the Node or replaceChild paths
input-validation#1.93 · no · geometry.js is untouched, so the conversion checks stand
input-validation#1.94 · no · selection.rs and its test are untouched, and the session's hit-test and layout edits do not reach the Selection argument checks
input-validation#1.95 · no · apps/browser/persistence is untouched; the session edits only apps/browser/Cargo.toml and two browser sources, and no SQL statement changes
input-validation#1.96 · no · persistence/src/lib.rs is untouched, so the stored-URL re-parse at lines 184-188 stands
input-validation#1.97 · no · node/serialize.rs is untouched and the session changes no escaping rule; its node.rs and mutator edits are in other files
input-validation#1.98 · no · inline_svg_serialize.rs is untouched and the session changes no XML serialization rule; the aspect-ratio edit moves layout, not markup
input-validation#1.99 · no · scoped_query_selector.rs and blitz-test-harness inspect.rs are untouched; the class-list change alters class matching, not selector parsing or its panic
input-validation#1.100 · no · wpt/runner/src/test_runners/mod.rs is untouched; the session edits only main.rs, Cargo.toml and attr_test.rs in the WPT runner
input-validation#1.101 · no · fuzzy.rs is untouched by the session
input-validation#1.102 · yes · stale · attr_test.rs gained reads at added lines 3, 97, 135-136, 141-147 and 164-165, so the cites at :79-81 and :241-246 moved
input-validation#1.103 · no · js_wrapper.rs is untouched by the session
input-validation#1.104 · no · wpt/runner/src/test_runners/mod.rs is untouched, so the script-type check stands
input-validation#1.105 · no · net_provider.rs is untouched by the session, and the WPT allocator change does not reach its data: URL path
input-validation#1.106 · no · harness_test.rs is untouched by the session
input-validation#1.107 · no · no .github workflow or script changed, so wpt-post-results.yml stands
input-validation#1.108 · no · no .github script changed, so wpt_diff_to_pr.py stands
input-validation#1.109 · no · the 7GUIs task sources are untouched, and the windowed launch's Parallel style threading changes no input check
input-validation#1.110 · no · custom_widget.rs and the TodoMVC page are untouched, and the new multi_doc example is neither a widget nor TodoMVC
input-validation#1.111 · no · the graphite and google fixtures are untouched by the session
