# arch · standing conventions

- S1 · each new module or test file opens with a module doc stating what it holds · conventions
  decision/module-and-item-docs · Module and item documentation — Modules, files and examples open with `//!` docs, examples describing what they demonstrate, and public items carry `///`. · except: The driver crate opens every file with `//!`; upstream engine modules and examples largely open without it, and some upstream public items carry no `///`.
  decision/module-docs · Module and file docs — Modules and files open with `//!` docs. · except: The driver crate opens every file with `//!`; upstream engine modules largely open without it.
- S2 · each test keeps a descriptive name stating the behaviour it checks · conventions
  decision/descriptive-test-names · Descriptive test names — Tests carry descriptive snake_case names.
- S3 · section delimiter comments mark the sections of a split file · conventions
  decision/section-delimiter-comments · Section delimiter comments — Code sections are delimited with `// === … ===` comments.
  decision/section-delimiters · Section delimiters — Sections are delimited with `// === … ===`.
- S4 · Rust code passes the formatting and clippy gate before it lands · conventions · infrastructure-patterns
  decision/blitz-dom-feature-subset-clippy · Upstream blitz-dom feature-subset clippy line — When the workspace clippy line is green, the clippy leg also runs upstream's clippy over blitz-dom with no default features and its named feature subset, warnings denied.
