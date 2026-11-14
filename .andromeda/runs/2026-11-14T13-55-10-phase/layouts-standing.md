# layouts · standing conventions

- S1 · Each new test drives the stand only through its stable element ids, which are carried as attributes, and every control it reaches is keyed. · surface-desktop-native
- S2 · Each new fixture or stand control states its name, role and states by attributes alone, and lints clean at boot. · surface-desktop-native
  decision/stand-a11y-by-attributes-alone · Stand names, roles and states by attributes alone — Stand controls take accessible names, roles and states by attributes alone (for, aria-label, aria-invalid, role, aria-selected), adding no element, class, style, key, id or order, the classes that style invalid and selected states staying.
- S3 · Each kept check a new test writes is a replayable session recording in an owner-only private recording directory, and it holds no screenshot. · surface-cli
  contract/cold-agent-cli/record-dir · cold-agent.sh run pipe · target/cold-agent/record — The live run's server records its kept checks into this owner-only directory, removed by cleanup with the state, the checks never printed; replays run without recording.
- S4 · Each new log line or span holds fixed words and counts only, and never echoes caller input. · surface-cli
