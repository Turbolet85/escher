@compare side=phase doc=security-plan marker=2026-11-04-dioxus-runtime-probe e_items=46 held=13 lost_classify=3 lost_strong=3 beside=- cascade_only=12 listed=2 body=16 unplaced=0 held_ids=api-security#1.0,api-security#2.20,api-security#2.6,dependency-security#2.8,error-handling#1.1,error-handling#2.1,error-handling#6.1,input-validation#1.106,input-validation#1.64,input-validation#1.88,logging-monitoring#1.1,logging-monitoring#1.2,threat-model-summary#3.1 only_ids=authentication-authorization#2.6,dependency-security#2.6,dependency-security#2.7,input-validation#1.10,input-validation#1.127,input-validation#1.137,input-validation#1.138,input-validation#1.15,input-validation#1.16,input-validation#1.17,input-validation#1.47,secret-management#2.11
@compare_item n=1 part=lost how=strong entity=data-protection#1.1 label=security-plan/data-protection fact=fct-cl72c3yo
@carried n=1 label=security-plan/data-protection fact=fct-cl72c3yo
why: Any probe run on the stand that records a session must keep its check owner-only, masked and never logged, and the probe adds no field or carrier to what a kept check holds.
quote: the answer itself as `answer` — the line the driver wrote, masked as on every carrier (a password's or file input's value as `MASKED_VALUE`), a shot's as `{"width","height"}` with no `png` — holding call content (the target ids, `press` keys, `advance` ms and text typed into unmasked controls; a masked or unresolved `type`'s text as `MASKED_STAND_IN`) and the screens' readings at rest — ids, names, values, snapshot text, diffs and a lint's findings, never a pixel — never logged, and never removed by escher but for its own check when a recording ends
@compare_item n=2 part=lost how=strong entity=dependency-security#2.1 label=security-plan/dependency-security fact=fct-zrg6urnw
@carried n=5 label=security-plan/dependency-security fact=fct-zrg6urnw
why: If the probe's test or example code brings any package into `Cargo.lock`, that package must be reviewed by hand the way earlier entries were before the chunk closes.
quote: each from crates.io with a checksum and reviewed by hand for licence, repository, build script, proc-macro flag and direct dependent, its source beyond the manifest not read
@compare_item n=3 part=lost how=strong entity=input-validation#1.65 label=security-plan/input-validation fact=fct-r5yt6pat
@carried n=6 label=security-plan/input-validation fact=fct-r5yt6pat
why: Any accessible name the probe meets while reading link (4), state to the nodes that read it, must reach no log. The probe's code must log no name.
quote: escher's own code logs no name, and a host that installs escher's sink writes none to stderr at any `RUST_LOG` level
@compare_item n=4 part=lost how=classify entity=logging-monitoring#4.1 label=security-plan/logging-monitoring fact=fct-ksl3fslx
@carried n=3 label=security-plan/logging-monitoring fact=fct-ksl3fslx
why: the probe adds no span or span field, so these counts must still read the same at the chunk's end.
quote: `packages/escher-driver/src` holds two `info_span!`, `stand_act_spans` reads eight span fields
@compare_item n=5 part=lost how=classify entity=threat-model-summary#4.1 label=security-plan/threat-model-summary fact=fct-gphdtjgv
@carried n=5 label=security-plan/threat-model-summary fact=fct-gphdtjgv
why: The MCP stdio server must stay listener-free and its result set unchanged, because anything put in a result can reach the model provider through the client.
quote: it reads JSON-RPC 2.0 from stdin and writes its replies to stdout — no listener, port or socket — and holds the session it boots in its own process
@compare_item n=6 part=lost how=classify entity=threat-model-summary#4.1 label=security-plan/threat-model-summary fact=fct-gphdtjgv
@carried n=6 label=security-plan/threat-model-summary fact=fct-gphdtjgv
why: The probe's readings must not enter an MCP result, since what a client does with a result is out of escher's hands.
quote: A result carries ids, accessible names, control values (a password's or a file input's as the mask), the snapshot text and an acting call's diff to the client
@compare_item n=7 part=lost how=classify entity=threat-model-summary#5.1 label=security-plan/threat-model-summary fact=fct-r5teidpw
@carried n=7 label=security-plan/threat-model-summary fact=fct-r5teidpw
why: Since the probe adds no carrier and no carried item, the committed statement in `docs/capabilities.md` must remain true without an edit.
quote: a statement of the ratified crossing that adds no carrier, its pin reading none of it
@compare_item n=1 part=listed how=strong entity=data-protection#2.1 label=security-plan/data-protection fact=fct-6n6morf4
@carried n=2 label=security-plan/data-protection fact=fct-6n6morf4
why: The probe's test or example code under tests/blitz-tests/ must commit no new check, and must leave the two committed Counter checks byte-equal to what the binary records.
quote: read only through the strict `Check::from_bytes` (`stand_replay`) and compared byte for byte with a fresh recording (seven_guis `recording::the_committed_checks_are_what_the_binary_records`); a check a launcher records of any other session stays owner-only and is never committed — a boundary widening ratified by the founder (2026-10-19, F1 A, «Коммитить две проверки (Рекомендую)»)
@compare_item n=2 part=listed how=strong entity=dependency-security#1.0 label=security-plan/dependency-security fact=fct-a4qh6ioi
@carried n=1 label=security-plan/dependency-security fact=fct-a4qh6ioi
why: The probe's tree, including any probe check or example it keeps, must pass the audit leg over the whole workspace's graph, advisories and licences both.
quote: `cargo deny --locked --workspace --all-features check advisories licenses` against the RustSec advisory database and the licence table, over the whole workspace's graph
@compare_item n=3 part=listed how=strong entity=dependency-security#1.0 label=security-plan/dependency-security fact=fct-a4qh6ioi
@carried n=2 label=security-plan/dependency-security fact=fct-a4qh6ioi
why: The chunk must not add a blanket allow or an unreasoned ignore to `deny.toml`; if a new advisory turns the audit red, the only acceptable ignore is one per ID with a written reason.
quote: `[advisories]` per-ID ignores only, each with a written reason, no blanket allow, `unmaintained` and `unsound` at their defaults (deny.toml:1-29; .github/scripts/ci-leg.sh:40).
