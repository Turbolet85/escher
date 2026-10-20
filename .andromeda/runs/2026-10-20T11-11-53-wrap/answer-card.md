# Answer card — 2026-10-20-large-screen-in-parts (wrap P2 Validate)

## His answers, by record
- F1 of the chunk's P4 (how an agent reads a screen over the 10,000-byte snapshot budget in parts; playbook «Boundary
  widening») — the option "`part` arg, default paging (Recommended)": his chosen label «Аргумент part, части по умолчанию
  (Рекомендую)», the founder, 2026-10-20, by question dialog in the overseer session, relayed verbatim by the overseer
  (inputs#I2). As asked: `snapshot` takes an optional `part` (a whole number, a new argument kind); over the budget with
  no argument it answers part 1 with the count fields `part` and `parts`; over MCP a paged answer is its JSON line; inside
  the budget nothing changes.
- F2 (what a call for a part past the last answers) — the option "Empty part (Recommended)": his chosen label «Пустая часть
  (Рекомендую)», the same dialog and relay (inputs#I2). As asked: accepted, with an empty `text` and `part` / `parts`
  saying where the walk stands; no new cause.

## Escalated proposals read as lying wholly inside them (numbered as fanout-results.md numbers them)
Each records the widening F1 chose — `snapshot`'s optional `part` coming in on every carrier and the paged answer
`{"text","part","parts"}` going out, a screen inside the budget answering as before — or F2's empty part. No other
widening, carrier, verb, cause, exit, span field or session error rides any of them.

- architecture 2 · §Standard Contracts → Driver session · `snapshot` takes an optional `part`; result `text` · `part` · `parts`
- architecture 3 · §Standard Contracts → Driver session · `ArgKind::Part`, the seventh kind, 1 to `MAX_PART` 1000
- architecture 4 · §Standard Contracts → Driver session · `Command::Snapshot { part }`, validated `1..=MAX_PART`, else `malformed`
- architecture 5 · §Standard Contracts → Driver session · `Outcome::Part { text, part, parts }` in the exhaustive list
- architecture 6 · §Standard Contracts → Driver session · default paging over the budget, the empty part past the last (F2)
- architecture 7 · §Standard Contracts → Driver session · a part read records the whole screen's ids, as `snapshot` does
- architecture 8 · §Standard Contracts → Driver session · `to_json` writes `{"text":…,"part":n,"parts":m}`
- architecture 9 · §Standard Contracts → Driver session · the command line's `--part`, usage `snapshot [--part <part>]`
- architecture 10 · §Standard Contracts → Driver session · MCP: a paged result its JSON line; `part` an integer 1–1000
- architecture 14 · §Standard Contracts → Dioxus DOM bridge · the snapshot text's exits gain the paged line
- architecture 17 · §Established Decisions → [Driver session] · the paged crossing recorded on F1/F2
- security-plan 1 · §Input Validation → Driver command schema · the seventh argument kind on `snapshot`, the paged answer
- security-plan 2 · §Input Validation → Markup attributes (states row) · a paged MCP `snapshot` result is its JSON line
- security-plan 3 · §Input Validation → Markup attributes (id row) · the same, among the id's exits
- design-system 1 · §Surface: cli §Component Patterns · a paged `snapshot` answers one line `{"text","part","parts"}`
- layout-templates 1 · §Surface: cli → Primary screens · `--part` and the paged answer line, a whole screen as before
- layout-templates 2 · §Surface: cli → Primary screens · the usage line's `snapshot [--part <part>]`
- test-plan 4 · §3 → Session lifecycle (`session-binary`) · the usage line's `snapshot [--part <part>]`
- test-plan 5 · §3 → Session lifecycle (`session-binary`) · `--part` and its paged answer line
- a11y-plan 1 · §7 Screen Reader Support · the parts, and the driver's `snapshot` reading a screen in parts
- a11y-plan 2 · §7 Screen Reader Support · the MCP `snapshot` result two-formed

21 proposals. Every other proposal of the pass (35) is routine (fanout-results.md) and rides no part of this card.

## The confirm
- Asked of the operator by question dialog at this wrap's P2 Validate; he carried the card to the founder.
- Answer: «Confirm all (Recommended)», with the operator's note, verbatim: «The founder's own confirm, by dialog in the
  overseer session at the answer card, relayed by the overseer: «Подтвердить все (Рекомендую)».»
- The 21 proposals above stand confirmed. The ratification stays his answers to F1 and F2 (inputs#I2). The confirm is his
  own word, relayed verbatim by the overseer, so it is not PROVISIONAL (inputs#I5).
