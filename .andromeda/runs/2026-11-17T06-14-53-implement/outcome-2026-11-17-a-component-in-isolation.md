# Outcome — 2026-11-17-a-component-in-isolation (run 2026-11-17T06-14-53-implement)

**Outcome: green.** Smoke ✓ (entry `bash scripts/agent-run.sh run session`, green at P2, not re-run).
Next → the OPERATOR PASS: the plan's `operator` entries this skill never fires — the hygiene entry
(`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`), the push entry
(`git diff --quiet && git diff --cached --quiet && git push`) and the CI read (`… ci.py conclusion --sha HEAD --wait 2100`),
gate entries 10, 11 and 12 — then `/andromeda-wrap-session`.

## Files changed
- New: `tests/blitz-tests/tests/stand_component_alone.rs`
- Modified: `packages/blitz-test-harness/src/harness.rs` · `examples/seven_guis/src/stand.rs` ·
  `packages/dioxus-native-dom/src/boundary.rs` · `scripts/cold-agent.sh` · `.github/scripts/test_cold_agent.py` ·
  `docs/capabilities.md`
- Out of list (scope record, both companion, self): `packages/blitz-test-harness/src/lib.rs` (re-exports
  `vdom_alone`, serves harness.rs) · `packages/blitz-test-harness/Cargo.toml` (dev-dependency `dioxus-native-dom` with
  feature `accessibility`, so the constructor's own test reads the UI graph; serves step 1; `Cargo.lock` unchanged,
  `--locked` green)
- `gate.py scope`: `scope: clean — changed 9 · listed 7 · recorded 2 (companion 2 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 188`

## Deviations from plan, with justifications
1. **Props as a maker, not a value** (steps 1, 2). `Harness::from_component_alone(component, props, options)` and
   `stand::boot_alone` take `props: impl Fn() -> P + 'static`, run once in the anonymous root's scope at its first
   render (`use_hook`). Reason: a prop holding an `EventHandler` — `TaskShell`'s `on_back` — can only be made inside a
   Dioxus runtime (dioxus-core 0.7.10 `Callback::new` calls `Runtime::current()`), so a props value built before the
   vdom exists cannot carry one, and the plan's own TaskShell-props case needs it.
2. **A public `blitz_test_harness::vdom_alone`** beside the constructor (step 2). The stand provides `TimerTicks`
   as a root context before the first build (`vdom_alone(..).with_root_context(..)` then `Harness::from_vdom`); the
   constructor builds at once, so a context added after it would reach no render. `from_component_alone` is
   `from_vdom(vdom_alone(..), options)`; the stand's boot goes through the same mount.
3. **Boot conditions read the viewport** (step 3). The plan's «the isolated document's root reads 800 × 600» read red
   on the first run: alone, without the shell's `html, body, #main { height: 100% }`, the root `html` box sizes to its
   content. The case reads the document's viewport `(800, 600)` at scale 1 instead — the matrix acceptance's «the
   isolated document reads 800 × 600», satisfied as written.
4. **The Timer-alone case reads the root's ticks** (step 3, control (b)). The plan's case («`timer-elapsed` reading
   `Elapsed: 0.0s`») cannot read red under control (b): `Timer` takes `try_use_context::<TimerTicks>()` and falls back
   to a 100 ms wall-clock delay, so it reads 0.0s at boot either way. The case reads the `TimerTicks` held at
   `ScopeId::ROOT`, delivers 3 ticks and reads `Elapsed: 0.3s`; control (b) reads it red.
5. **Control (a)'s edit text**: `vdom_alone`'s named-child mount replaced by
   `let _ = alone_root; let props = Rc::new(props); let root = move |_: ()| component.rebuild((*props)()); VirtualDom::new_with_props(root, ())`
   (evidence `control-1.txt`).
6. **docs/capabilities.md** took a three-line subsection `### A component alone` with a `Witnesses:` line, after
   «Exploring an app», where the plan said one sentence: the page's convention names a check behind every claim.
7. The cold-agent contract test asserts, beside the plan's passing run, that a `layout` call is no act: two clicks
   plus a `layout` leave the counter unmet (`task-unmet`), which would read passed if the fake counted it.

## Gates (run `gate.py run`, one firing, 1 iteration)
- `cargo test -p dioxus-native-dom --locked --lib boundary` — green · exit 0
- `cargo test -p blitz-test-harness --locked` — green · exit 0
- `cargo test -p blitz-tests --locked --test stand_component_alone` — green · exit 0
- `python3 -m unittest discover -s .github/scripts -p test_cold_agent.py` — green · exit 0 (38 tests)
- `git diff --quiet c7c09a7bf… -- packages/escher-driver examples/seven_guis/src/session_host.rs` — green · exit 0
- `bash scripts/agent-run.sh boot` — green · exit 0
- `bash scripts/agent-run.sh run session` — green · exit 0
- `bash .github/scripts/ci-leg.sh fast` — green · exit 0 (359.98 s)
- `bash .github/scripts/ci-leg.sh doc` — green · exit 0
- the three `operator` entries — not run — leg operator (the operator pass's)
- No deferral. No guard entry (`gate.py guard`) in the block.
- Before the gate run, the first hand run of `stand_component_alone` read red on the root's 800 × 600 (deviation 3).

## Controls (one-shot, `gate.py control`)
- `control-1.toml`: a-named-child · entry 3 · holds; b-timer-ticks · entry 3 · holds — `control: 2 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`
- `control-2.toml`: c-root-fallback · entry 1 · holds — `control: 1 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`
- Evidence: `.andromeda/cache/chunks/2026-11-17-a-component-in-isolation/evidence/control-{1,2}.txt`.
  CARRY cry-c4emakc4's mechanism (P3's reading of boundary.rs) is confirmed: one `Root` mark before, red with the
  fallback insert removed.

## Matrix
- v030-12 → implemented, ref `tests/blitz-tests/tests/stand_component_alone.rs::the_shell_given_the_stands_props_reads_the_stands_counter_node_for_node · ::the_counter_alone_reads_its_in_app_subtree_without_the_shell · ::a_click_alone_moves_the_counter_as_on_the_stand · ::the_isolated_document_keeps_the_stands_viewport_and_font`

## Surfaced / stuck
- none.

## Process hygiene
| process | started by | final state |
|---|---|---|
| escher-session, cargo, seven_guis_nati, `stand_component_alone` | this run (gate entries, controls) | terminated — `census: none running` (`gate.py census`, this run dir) |

## Decisions & corrections
- Recurrence of a standing learning: none met. The testing rule (2026-11-14) on a control's `case` naming the test
  function alone was followed; each control named one case, and each read `holds`.
- Sweep hazard: a `#[component]`'s props containing an `EventHandler` cannot be built outside a Dioxus runtime —
  `Callback::new` reads `Runtime::current()` (dioxus-core 0.7.10 events.rs:487-491); measured when designing the
  TaskShell-props case.
- Sweep hazard: `seven_guis::tasks::timer::Timer` reads `TimerTicks` through `try_use_context`, falling back to wall
  time, so a check that the context is provided cannot rest on a boot reading of the elapsed text.
- Guard refusal: the Bash guard refused a `cd` into the skill's references directory (session cwd move); re-issued
  once as a `cat` by absolute path.
