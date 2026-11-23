# layouts · standing conventions

- S1 · every new stand control is keyed with a stable element id carried as an attribute alone · surface-desktop-native
  decision/every-stand-control-keyed · Every stand control keyed — Every actionable element of all seven stand tasks carries an author id, so no actionable element reads unkeyed, an open dialog included.
  decision/stand-ids-as-attributes-alone · Stand ids as attributes alone — Author ids on stand elements are added as attributes alone, moving no element, class, style, wrapper or order, and no stand CSS selects by them.
- S2 · a new stand control's name, role and state come from attributes alone, and the stand lints clean at boot · surface-desktop-native
  decision/stand-a11y-by-attributes-alone · Stand names, roles and states by attributes alone — Stand controls take accessible names, roles and states by attributes alone (for, aria-label, aria-invalid, role, aria-selected), adding no element, class, style, key, id or order, the classes that style invalid and selected states staying.
- S3 · a new kept check is a replayable session recording read in both layout modes · surface-cli
