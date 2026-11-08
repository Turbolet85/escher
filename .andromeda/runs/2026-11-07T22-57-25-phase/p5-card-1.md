# P5 review — 2026-11-07-ui-graph · «The UI graph»

**The plan:** open `.andromeda/cache/docs/2026-11-07-ui-graph/plan.md` in an editor (rev 6). No plan stands in a
chunk's folder. Writing in that file changes no record until a `put`. `python -X utf8
~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-07-ui-graph` diffs the chunk's records.

## Goal
One read of the screen's UI graph is added, as the thirteenth driver verb `graph` on every carrier. For each
agent-visible node it gives the component that rendered it, its current handlers, the registered states each handler
wrote, and the nodes that read each state. The snapshot gains a closed `clickable` flag. The stand's seven tasks hand
their signals to an escher-side registry. The id grammar reserves `{owner}///{key}` for 0.4.0's virtual nodes.

There is no blitz-dom change, no Dioxus fork, and no change to any manifest or the lock.

## The founder's two forks (P4, by dialog in the overseer session, relayed by you through the P4 dialog: inputs#I4)
- Fork 1: a new `graph` verb (Recommended). Component names and registered state names cross (authored text);
  handler event names cross (Dioxus's closed set). No value and no `NodeId` crosses. This matches the playbook
  pattern «Boundary widening» (`rul-izni57qv`), and the plan's Expected amendments carry it for the wrap's escalation.
- Fork 2: the form `{owner}///{key}` (Recommended). It is reserved through a constant and `virtual_node_id`, with
  nothing producing it.

## Leans to confirm (decided from the artifacts, shown here for your word)
- The flag is named `clickable`. It reads true exactly on an element whose own current `click` listener (from the
  vdom walk) is set and which is not a disabled form control. Native activation with no app handler reads false.
  The text token appears only when true; the JSON flag is always written, like `invalid` and `focused`.
- Link 3 names the writes a handler made in this session (the founder's fork 1 of 2026-11-04: «recorded at the
  wrapper's write»). A handler that has not run names none.
- Link 4's reader is a component: every node that component rendered reads its state.
- Owner: a select's `option` and `optgroup` read their select (`BaseDocument::select_of`). Every other node is its own
  owner. This was corrected at P4 from the design cascade.
- A graph read feeds the session's record of ids, as `lint` does.
- The cold-agent verdict's `capability-unused` set is unchanged: a cold agent is not required to call `graph`.
- The stand reaches the registry through dioxus-native's existing `pub use dioxus_native_dom::*`, with no manifest
  change.
- The bridge's dispatch hook in `DioxusDocument`'s event path changes no document's behaviour where no signal is
  registered. I leaned this is not a fork, because the founder's rule asks for a behaviour change. It does touch
  `dioxus_document.rs`, which is part of the buffered render's merge surface (`res-3v42legm`), though outside the
  render lines.
- The registry's mechanism (a subscribed `ReactiveContext::new_with_callback`) was read from dioxus-core 0.7.10's
  code and not run. Implement's first registry test is its control; a red there stops implement.

## Top acceptance criteria
1. matrix#v030-05, on the stand, both layout modes, through `Session::run`, the command line, MCP and in process
   (entry 4, `stand_graph`).
2. No id holds `///`, the reserved form collides with nothing, and every `element_id` test reads as before (entry 1).
3. blitz-dom, the manifests, `Cargo.lock` and `deny.toml` are unchanged; `mutation_writer.rs` is byte-identical to
   upstream; the probe stays at 10 green (entries 6, 13, 14).
4. 0 needles at `trace` under both builds; one `tool_call` span parents one `command` span; 2 `info_span!` sites;
   the new files are silent (entries 10–12, 17).
5. `VERBS` 13, `NODE_FIELDS` 14, describe's row `[13, 14, 12, 3, 8, 4, 15]`, the committed checks re-recorded, the
   capability-statement pin green (entries 3, 8–10, 17).

## Touchpoints
3 new files (`graph.rs`, `signal_registry.rs`, `stand_graph.rs`) and 49 modified across dioxus-native-dom (7),
escher-driver (14), the stand's tasks (7), blitz-tests (12, the two committed checks among them), seven_guis tests
(5), CI scripts (2), `docs/capabilities.md` and `README.md`.

## Test commands (22 entries; baselines at `be4180b8`)
Unit tests read 83 and 100 at the base. `stand_graph` is red (its target is absent). `signal_registry` is red (no
test yet). `run stand` reads 237 · 0 · 7. Audit reads ok. Three probes are each controlled both ways in a scratch
clone (`.andromeda/runs/2026-11-07T22-57-25-phase/controls.md`). Entry 8's re-recording at the base wrote the
committed checks byte-identical. The operator pass runs: hygiene, then push, then `ci.py conclusion --wait 1800`.

## Concretization preview — v030-05 (the never-weaken checkpoint)
- **Current:** «For the current screen of a stand task, one read names each node's component, the handlers it
  carries, the state those handlers write and the nodes that read that state, and says which nodes react to a click;
  the probe of what the runtime exposes reached the founder before it.»
- **Proposed:** «For the current screen of each lean stand task and Cells, in both layout modes, one read — the
  `graph` verb through `Session::run`, the command line and MCP, and `DioxusDocument::graph` in process — names each
  snapshot node's component, the handlers it carries (its current listeners), for each handler the registered states
  its runs in the session wrote, and for each registered state the nodes of the components that read it; after
  `click counter-increment` it names `count` written by `counter-increment`'s `click` and read by `counter-value`;
  the snapshot's `clickable` reads true exactly on an element with a current `click` listener that is no disabled
  form control (`counter-increment` true, `counter-value` false, Cells' 676 data cells true, `cells-grid` false); the
  probe reached the founder before it (2026-11-04-dioxus-runtime-probe). Witness: `stand_graph`.»
- **Where it is narrower than the current text, for your judgement:** «the state those handlers write» becomes «the
  registered states its runs in the session wrote». This follows the founder's fork 1 (link 3 recorded at the
  write), and it covers registered signals only. «The nodes that read that state» becomes the nodes of the reading
  components, which is Dioxus's subscription grain. Both limits go into `docs/capabilities.md` as stated limits.

## Notes
- `sampler: off`.
- No gated criterion: the entry carries no `BLOCKED-ON`.
- inputs#I2 reads `drifted`. The overseer ticked the checklist row («☑ snapped 2026-11-07 …; his fork 2 reserved
  `{owner}///{key}`»); the direction text is byte-identical, so its citations stand.
- The verb fork rides playbook `rul-izni57qv`. The wrap halts on it once, with the founder's relayed answer as the
  card.

## Beyond the plan
none
