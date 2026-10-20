- line: .andromeda/architecture.md:196
old: «`VERBS` lists eleven verbs in three levels»
new: «`VERBS` lists twelve verbs in three levels»
- line: .andromeda/architecture.md:196
old: «— first the seven asked of the held instance, in order:»
new: «— first the eight asked of the held instance, in order:»
- line: .andromeda/architecture.md:196
old: «(packages/escher-driver/src/schema.rs:276-308); then the three asked of the session itself»
new: «(packages/escher-driver/src/schema.rs:276-308), and `lint` (no argument, one named refused `malformed` with `Fault::Unnamed`; result `findings`, `FieldKind::Entries`, always present), whose help states the six defect words, each reason's keys and its limits — geometry only, no hit, right-to-left and vertical text read as left-to-right lines — a widening of the validated verb set ratified by the founder (2026-10-20) (packages/escher-driver/src/schema.rs:310-335; packages/escher-driver/src/schema.rs:458-461); then the three asked of the session itself»
- line: .andromeda/architecture.md:196
old: «11 verbs, 13 node keys, 12 key names, 3 busy classes, 9 causes, 4 exit statuses»
new: «12 verbs, 13 node keys, 12 key names, 3 busy classes, 8 causes, 4 exit statuses»
- line: .andromeda/architecture.md:196
old: «`Screenshot { id: Option<String> }` (the last since 2026-10-12-headless-screenshot), with `spec()»
new: «`Screenshot { id: Option<String> }` (since 2026-10-12-headless-screenshot) · `Lint` (validated from `("lint", [])`; packages/escher-driver/src/command.rs:175-176), with `spec()»
- line: .andromeda/architecture.md:196
old: «the last from `screenshot` alone among the verbs, checked before anything is painted (below), and from the in-process `Session::lint` before anything is read (below)»
new: «the last from `screenshot` and `lint` among the verbs — a shot checked before anything is painted (below), a lint, and `Session::lint` in process, before anything is read (below)»
- line: .andromeda/architecture.md:196
old: «and the in-process lint's `Bounds`, `Defect`, `Finding` and `Reason`»
new: «and the visual lint's `Bounds`, `Defect`, `Finding` and `Reason`»
- line: .andromeda/architecture.md:196
old: «— no base64 crate taken; a `start` answers»
new: «— no base64 crate taken; a `lint`'s outcome `{"findings":[…]}`, each finding an object keyed `id` · `defect` · `bounds` (`[x, y, width, height]`, as a node's bounds), then its reason's keys — `other` and `area` for an overlap, `visible` for a clip, `text` then `visible` (left out when no part can be seen) for a truncation, `ratio` (its shortest decimal, `null` when not finite) and `required` for contrast, none for off-screen and zero-size — holding stable ids, closed words and numbers only (packages/escher-driver/src/json.rs:255-309; packages/escher-driver/src/json.rs:360-364); a `start` answers»
- line: .andromeda/architecture.md:196
old: «The record is fed from the snapshot the in-process `Session::lint` reads, which takes no argument,»
new: «The record is fed from the snapshot `Session::lint` reads — in process or through the `lint` verb — which takes no argument,»
- line: .andromeda/architecture.md:196
old: «(the last since 2026-10-12-headless-screenshot, packages/escher-driver/src/execute.rs:66-74) (`Debug`, `Clone`, `PartialEq`), the diff being dioxus-native-dom's `SnapshotDiff` unchanged;»
new: «(since 2026-10-12-headless-screenshot, packages/escher-driver/src/execute.rs:66-74) · `Linted { findings: Vec<Finding> }` (packages/escher-driver/src/execute.rs:75-79) (`Debug`, `Clone`, `PartialEq`), the diff being dioxus-native-dom's `SnapshotDiff` unchanged, and the `lint` arm calling `Session::lint` (packages/escher-driver/src/execute.rs:261-263), so a lint dispatches nothing, runs no step and settles nothing;»
- line: .andromeda/architecture.md:196
old: «`snapshot` and, since 2026-10-12-headless-screenshot, an accepted `screenshot` record `verb` alone;»
new: «`snapshot`, an accepted `screenshot` (since 2026-10-12-headless-screenshot) and an accepted `lint` record `verb` alone;»
- line: .andromeda/architecture.md:196
old: «a shot's image, its size or its dimensions, the session's label,»
new: «a shot's image, its size or its dimensions, a lint's findings or their count, the session's label,»
- line: .andromeda/architecture.md:196
old: «`tools/list` answers the eleven verbs of `VERBS` in order (`screenshot` after `scroll` since 2026-10-12-headless-screenshot), `help` last,»
new: «`tools/list` answers the twelve verbs of `VERBS` in order (`screenshot` after `scroll` since 2026-10-12-headless-screenshot, `lint` after `screenshot`), `help` last,»
- line: .andromeda/architecture.md:196
old: «`help` among them since 2026-10-12-self-description and `screenshot` since 2026-10-12-headless-screenshot, no field added)»
new: «`help` among them since 2026-10-12-self-description, `screenshot` since 2026-10-12-headless-screenshot and `lint`, no field added)»
- line: .andromeda/architecture.md:200
old: «(a verb outside the seven instance verbs,»
new: «(a verb outside the eight instance verbs, a `lint` step kept like every other with its answer whole,»
- line: .andromeda/architecture.md:200
old: «closed keys, the seven instance verbs, argument names»
new: «closed keys, the eight instance verbs, argument names»
- line: .andromeda/architecture.md:202
old: «- The visual lint, in process only — its `lint` verb, on every carrier and recorded, owed to «Visual lint» (its gated part): `Session::lint(&mut self)»
new: «- The visual lint — `Session::lint` in process and the `lint` verb, the eighth instance verb, on every carrier (the socket's `call`, the command line, an MCP tool, `help`'s listing) and recorded, ratified by the founder (2026-10-20): `Session::lint(&mut self)»
- line: .andromeda/architecture.md:202
old: «No verb, `Command`, `Outcome` variant, written form, MCP tool, recorded step, span or field carries it, and `VERBS` stays eleven;»
new: «The `lint` verb carries it — `Command::Lint`, `Outcome::Linted`, the `{"findings":[…]}` written form, an MCP tool and a recorded step, its answer kept whole — its `command` span recording `verb` and, refused, `cause` alone, no field added, and `VERBS` holds twelve;»
- line: .andromeda/architecture.md:203
old: «and `snapshot` and `screenshot`, which run no step, wait on nothing»
new: «and `snapshot`, `screenshot` and `lint`, which run no step, wait on nothing»
- line: .andromeda/architecture.md:219
old: «the verbs are the ten of `VERBS` — `snapshot`, `click --id`, `type --id --text`, `press --key [--shift]`, `advance --ms`, `scroll --id`, `start <task> [--record <dir>]`,»
new: «the verbs are the twelve of `VERBS` — `snapshot`, `click --id`, `type --id --text`, `press --key [--shift]`, `advance --ms`, `scroll --id`, `screenshot [--id <id>]`, `lint`, `start <task> [--record <dir>]`,»
- line: .andromeda/architecture.md:222
old: «only the escher server's eleven tools»
new: «only the escher server's twelve tools»
- line: .andromeda/architecture.md:222
old: «isolated — the eleven `mcp__escher__{verb}` tools»
new: «isolated — the twelve `mcp__escher__{verb}` tools»
- line: .andromeda/architecture.md:354
old: «and lint (`Session::lint`, `Finding`, `Defect`, `Reason` and `Bounds`, the in-process visual lint)»
new: «and lint (`Session::lint`, `Finding`, `Defect`, `Reason` and `Bounds`, the visual lint, carried by the `lint` verb — execute holding `Outcome::Linted`, json the finding's written form)»
- line: .andromeda/architecture.md:118
old: «(2026-10-12, «В самом ответе», by question dialog in the overseer session, relayed verbatim by the overseer; the wrap's card confirmed, «Подтвердить все 16», the same channel) — as measured at»
new: «(2026-10-12, «В самом ответе», by question dialog in the overseer session, relayed verbatim by the overseer; the wrap's card confirmed, «Подтвердить все 16», the same channel); a lint's findings cross on those carriers and rest in a kept check as the `lint` verb's answer — stable ids, the closed defect words and numbers, no page text, colour, control value or `NodeId` — with no new carrier, exit, path argument, cause, session error, span field or dependency: a boundary widening ratified by the founder (2026-10-20, «Новая команда lint (Рекомендую)», by question dialog in the overseer session, relayed verbatim by the overseer; the wrap's card confirmed, «Подтвердить все (Рекомендую)», the same channel) — as measured at»
- line: .andromeda/architecture.md:227
old: «(as measured at escher-0.1.0/chunks/2026-10-12-headless-screenshot/report.md). As adopted, before that chunk»
new: «(as measured at escher-0.1.0/chunks/2026-10-12-headless-screenshot/report.md). Both carriers, and a kept check, also take a lint's findings out as the `lint` verb's answer — stable ids, the closed defect words and numbers, no `NodeId`, page text, colour or control value — a boundary widening ratified by the founder (2026-10-20, «Новая команда lint (Рекомендую)», relayed verbatim by the overseer); still the one listener and no port, the wire's bounds unchanged (as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/report-gated.md). As adopted, before that chunk»
