# layouts · standing conventions

- S1 · every new stand control or surface carries a stable keyed id set as an attribute, names its role and state by attributes alone, and lints clean at boot · surface-desktop-native
  decision/every-stand-control-keyed · Every stand control keyed — Every actionable element of all seven stand tasks carries an author id, so no actionable element reads unkeyed, an open dialog included.
  decision/stand-a11y-by-attributes-alone · Stand names, roles and states by attributes alone — Stand controls take accessible names, roles and states by attributes alone (for, aria-label, aria-invalid, role, aria-selected), adding no element, class, style, key, id or order, the classes that style invalid and selected states staying.
  decision/stand-ids-as-attributes-alone · Stand ids as attributes alone — Author ids on stand elements are added as attributes alone, moving no element, class, style, wrapper or order, and no stand CSS selects by them.
  decision/stand-lints-clean-at-boot · Stand lints clean at boot — At boot the counter, the flight booker, the timer, CRUD and the load page read zero visual-lint findings, in both layout modes. · guard: stand_lint
- S2 · every new verb argument is listed in the verb table, help's self-description and the MCP tools listing alike · surface-cli
- S3 · every new log line or answer carries no caller-supplied value back out · surface-cli
