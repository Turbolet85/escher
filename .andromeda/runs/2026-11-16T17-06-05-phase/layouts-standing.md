# layouts · standing conventions

- S1 · each new acceptance test of a session answer runs in both layout modes and on each carrier it names: in process, the command line and MCP · surface-cli
- S2 · each new fixture control a test drives is keyed and carries its id as an attribute alone, and the fixture lints clean at boot · surface-desktop-native
  decision/every-stand-control-keyed · Every stand control keyed — Every actionable element of all seven stand tasks carries an author id, so no actionable element reads unkeyed, an open dialog included.
  decision/stand-ids-as-attributes-alone · Stand ids as attributes alone — Author ids on stand elements are added as attributes alone, moving no element, class, style, wrapper or order, and no stand CSS selects by them.
  decision/stand-lints-clean-at-boot · Stand lints clean at boot — At boot the counter, the flight booker, the timer, CRUD and the load page read zero visual-lint findings, in both layout modes. · guard: stand_lint
