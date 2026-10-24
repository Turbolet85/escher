# security extract

## Relevance
partial — a test-and-CI re-read of a11y behaviour that the plan records as remote-content input boundaries, so the chunk crosses no new boundary. What applies is that those gates stay as ratified, the widened `a11y` leg keeps lockfile verification, and nothing new enters (dependency, env read, carrier).

## Constraints
- security-plan §Input Validation (Markup attributes — `disabled`) requires `disabled` to be read by its presence (bare, `"false"` or `"true"`) on a form control alone, through `ElementData::is_disabled`. That one reading gates focusability, the Tab walk, the pointer and click gates and the Enter and Space activation arms. A re-asserted or new `stand_keyboard` row for a disabled state must expect no focus and no activation under it. Whether any 0.2.0 stand state has a disabled control that no row reaches is a question for research.
- security-plan §Input Validation (Markup attributes — keyboard activation) requires the activation gates to be a closed element-name and type match plus the `is_disabled` presence check. Enter applies to `button` · `summary` · `input` `submit`/`reset`/`button` · `a[href]`. Space applies on key-up to `button` · `summary` · `input` `submit`/`reset`/`button`/`checkbox`/`radio`. A `disabled` attribute on a `summary` or an `a` gates neither key. These sets stay as stated, and re-read rows assert against them, not wider ones.
- security-plan §Input Validation (Markup attributes — `select`) requires a single-choice select's keys to be a closed match: `ArrowDown` · `ArrowUp` · `Home` · `End`, plus typeahead on a `Key::Character` with no Ctrl, Alt or Meta. They act only when the select is focused and not `is_disabled`. Enter and Space open nothing. The typeahead text stays in the private `select_typeahead` buffer, is compared, is never kept past 1 s of the document's clock and is never carried out. A new stand row for the flight booker's select asserts its effects through the choice (`select_value` or the snapshot's `value`), never through the buffer.
- security-plan §Input Validation (Markup attributes — `inert`) requires `inert` to be read by its presence on the element or an ancestor: an inert node is not focusable, takes no hit and is out of the tree. HTML's focus fixup is a stated limit owned by a residuals line. A Tab-order re-read leaves that limit unabsorbed.
- security-plan §Input Validation (Markup attributes — control states; closed values) requires the ARIA reads to be closed parses carrying no attribute text. This covers `aria-selected` · `aria-pressed` · `aria-checked` (on the five checkable roles) · `aria-invalid` · `aria-expanded` · `aria-disabled` · `aria-level`. Witnesses moved into the `a11y` leg (`accessibility_states`, `accessibility_merged_states`) assert closed enums and booleans.
- security-plan §Dependency Security › Supply chain integrity (Lockfile verification) requires every ci.yml cargo leg, the a11y leg among them, to pass `--locked`. The widened a11y leg command keeps it.
- security-plan §Secret Management (Environment values read) holds a closed list of env reads, none secret-bearing. Test targets added to the leg or touched by the re-read add no env read outside it.

## Patterns to follow
- One gate shared by every path: focusability, the pointer and click gates and the activation keys all read the same `ElementData::is_disabled` presence check (per security-plan §Input Validation (Markup attributes — `disabled`)). Keyboard rows assert the outcome at that gate, not a per-path reading.
- Engine behaviour on markup is recorded as "an engine change for every document and a boundary widening, ratified by the founder", with remote pages included (per security-plan §Input Validation (Markup attributes — keyboard activation; `select`)). A re-read defect that needs such a change goes to the founder as a fork, as the scope's Boundaries state.
- The widening rule: a new field of a closed value set inside an already ratified carrier is not a widening, while authored text, an open value set, a new carrier or a new exit still is (per security-plan §Input Validation (Driver session socket — the widening rule)). The re-read adds no field or carrier, so the rule is not invoked.

## Anti-patterns to avoid
- Folding a change to engine behaviour for every document into the re-read without the founder's ratification, for example widening an activation set, the `disabled` reading or the select's key match. These rows govern remote pages too (per security-plan §Threat Model Summary (remote web content) and §Input Validation (Markup attributes)).
- Adding a print, log site, `unwrap`, `expect`, panic path or env read to the engine lines the rows describe while re-reading them. The keyboard-activation, `inert` and `select` rows each require none (per security-plan §Input Validation (Markup attributes — keyboard activation; `inert`; `select`)).

## Contract bindings
- a11y-plan §9 (the `a11y` leg) ↔ security-plan §Dependency Security › Supply chain integrity (Lockfile verification). The leg's widened command in `.github/scripts/ci-leg.sh` and its pin in `.github/scripts/test_ci_workflows.py` must keep `--locked`.
- tests §CI Integration (ci.yml's "Accessibility (a11y) tests" job, its tier unchanged) ↔ security-plan §Dependency Security (CI integration). The audit job's tier and gate are untouched by the a11y widening.

## Acceptance criteria contributions
- (security) After the widening, every cargo command of the `a11y` leg in `.github/scripts/ci-leg.sh` carries `--locked` (per security-plan §Dependency Security › Supply chain integrity).
- (security) `Cargo.toml`, `Cargo.lock` and `deny.toml` are byte-identical before and after the chunk, so no package enters (per security-plan §Dependency Security).
- (security) Files the chunk changes add no `std::env::var`, `env!` or `option_env!` read outside the closed list (per security-plan §Secret Management).
- (security) The diff under `packages/` is empty. Failing that, any engine line it holds is a founder-ratified fork and adds no print, log site, `unwrap`, `expect` or panic (per security-plan §Input Validation (Markup attributes — keyboard activation)).
