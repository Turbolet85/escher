@history_item n=1 marker=amd-xwdpo3ht
**amd-xwdpo3ht** — 2026-10-27-readme-and-records: the capability statement's four-term secret search (`/home/|api[_-]?key|authorization:|bearer `, case-insensitive) was extended to read over `README.md` beside `docs/capabilities.md` and the pin, 0 hits; the e-mail-domain half stayed over the statement and pin only, not re-run. Matters here: this chunk edits `README.md` again (the 0.3.0 heading line). The README is now inside that search's reach, so the edit has to keep it at 0 hits. It is also the precedent for README edits under decision/live-evidence-holds-no-credentials: no key, auth header, login or e-mail enters a committed file.

@history_item n=2 marker=amd-p4afs3gh
**amd-p4afs3gh** — 2026-10-25-capability-statement: added Secret Management's search row (the four terms plus two e-mail domains over `docs/capabilities.md` and its pin, 0 hits; the pin reads no env var). Matters here: this is the search the README edit extends and must still pass. The scope's P3 also re-reads `docs/capabilities.md` and `.github/scripts/test_capability_statement.py` for any other statement of 0.3.0's state.

@history_item n=3 marker=amd-bmkpmlae
**amd-bmkpmlae** — 2026-10-31-engine-debts: RUSTSEC-2026-0414 fixed by a `Cargo.lock`-only pulldown-cmark update (0.13.4 → 0.13.5), reached only through `rdme`, which never sets `ENABLE_WIKILINKS`. Matters here: it is the rdme area's dependency history behind decision/rdme-raw-html-passthrough, the renderer a README heading is rendered through. The chunk touches no dependency.

@history_item n=4 marker=amd-7l3wwszj
**amd-7l3wwszj** — 2026-11-04-dioxus-runtime-probe: pulldown-cmark 0.13.6 fixes RUSTSEC-2026-0433 by a lock-only update, reached only through `rdme` (no wikilinks). Matters here: the latest rdme-path advisory record, read beside decision/rdme-raw-html-passthrough. A README heading in plain markdown adds no raw HTML for the unsafe-mode renderer to pass through.

@history_item n=5 marker=amd-sjg4jcvb
**amd-sjg4jcvb** — 2026-10-06-cold-agent-run-pipe: Secret Management recorded that the cold-agent live run uses the operator's own Claude Code login (`apiKeySource` none, no env read, no CI secret, evidence census 0). "What counts as secret" names that login, and the transcript stays raw, gitignored, with its one committed copy host-path-masked. Matters here: this is the origin of decision/live-evidence-holds-no-credentials. 0.3.0's "done and verified" heading points at the cold-agent-run evidence, and no login or credential may surface in the README.

@history_item n=6 marker=amd-sifaily4
**amd-sifaily4** — 2026-10-06-founder-rulings: the cold-agent credential path (the operator's own login, no API key, no env read, no CI secret) was ratified by the founder. Matters here: it is the standing ruling behind decision/live-evidence-holds-no-credentials, which the README statement of the verified version must respect.

@history_item n=7 marker=amd-fmb2pbwq
**amd-fmb2pbwq** — 2026-10-15-cold-agent-test: the cold-agent pipe on the escher server runs through the operator's own login, with no new secret source. The evidence's one credential-scan hit is `apiKeySource: none`, and `check.json` and `server.log` are never printed. Matters here: it is the precedent for reading live evidence clean of credentials (decision/live-evidence-holds-no-credentials) when a version is stated verified on that evidence.

@history_item n=8 marker=amd-j6mt7x3v
**amd-j6mt7x3v** — 2026-10-25-cold-agent-bar: the second live run went through the same login, and its committed evidence (`live-check.json`, a host-path-masked `live-transcript.jsonl`) was read clean by the operator pass's hygiene read, with no kept check committed. Matters here: this is the live evidence the README's 0.3.0-era statements rest on. Under decision/live-evidence-holds-no-credentials it stays credential-free, and the README names none of it.

@history_item n=9 marker=amd-npk6wxfv
**amd-npk6wxfv** — 2026-11-18-cold-agent-live-run: added decision/credential-probe-reads-valued-keys (the credential probe reads a key name carrying a value, after a field-name pattern hit `apiKeySource`). Matters here: it is the latest Secret Management decision on how live evidence is probed for credentials, beside decision/live-evidence-holds-no-credentials. It is the probe to re-run if the chunk's check reads any committed evidence or README line for credentials.

@history_item n=10 marker=amd-syjcwfkx
**amd-syjcwfkx** — 2026-11-12T03-36-23-rows: the security-plan master's text became rows (decision, contract, absence, overview), and this is where decision/rdme-raw-html-passthrough and decision/live-evidence-holds-no-credentials took their current row form. Matters here: no later amendment changes either picked row, so both stand as landed then.
