.raw-a11y-plan-draft-2.md:105: | Form controls | label→Label, select→ComboBox, select multiple→ListBox, textarea→MultilineTextInput, progress→ProgressIndicator, meter→Meter, radio→RadioButton, range→Slider, email→EmailInput, password→PasswordInput, submit→Button | (tests/blitz-tests/tests/accessibility_roles.rs:128-155) |
.raw-a11y-plan-draft-2.md:259: > NOT YET MEASURED — a project-wide UI colour token set: the examples slice (s01) and the DOM-node slice (s08) recorded the project's own colour tokens as out of slice, and the default user-agent stylesheet's full contents were out of slice for s05.
.raw-a11y-plan-draft.md:109: | Form controls | label→Label, select→ComboBox, select multiple→ListBox, textarea→MultilineTextInput, progress→ProgressIndicator, meter→Meter, radio→RadioButton, range→Slider, email→EmailInput, password→PasswordInput, submit→Button | (tests/blitz-tests/tests/accessibility_roles.rs:128-155) |
.raw-facts-s02.md:75: ## security-plan §Authentication & Authorization
.raw-facts-s02.md:76: - out of slice — no authentication or authorization code of the project is in the listed files
.raw-facts-s02.md:106: ## security-plan §Secret Management
.raw-facts-s02.md:107: - observed absent — any `password`, `token` or `secret` field or value · searched: `password`, `token`, `secret` over the 21 listed examples/assets/*.html files
.raw-facts-s04.md:169: ## security-plan §Authentication & Authorization
.raw-facts-s04.md:171: - observed absent — user authentication or authorization logic · searched: `password|secret|token|authorization|x-api-key` over the 86 listed files (only matches are the cells formula tokenizer in examples/seven_guis/src/tasks/cells.rs)
.raw-facts-s04.md:202: ## security-plan §Secret Management
.raw-facts-s04.md:203: - observed absent — environment-variable reads or secrets · searched: `env::var` and `password|secret|token|authorization|x-api-key` over the 86 listed files
.raw-facts-s04.md:241: - rdme body font stack -apple-system … sans-serif at 16px, line-height 1.5; monospace stack token `--fontStack-monospace` (apps/readme/assets/github-markdown.css:10; apps/readme/assets/github-markdown.css:132-134)
.raw-facts-s06.md:149: ## security-plan §Authentication & Authorization
.raw-facts-s06.md:150: - observed absent — authentication or authorization code · searched: `auth|login|session|permission` over the 17 listed files (only `author:` stylesheet-guard fields match, packages/blitz-dom/src/stylo.rs:68)
.raw-facts-s06.md:176: ## security-plan §Secret Management
.raw-facts-s06.md:177: - observed absent — secrets or credentials read or set · searched: `secret|token|api_key|x-api-key|cookie` over the 17 listed files (the only `token` hits are a Stylo traversal token, packages/blitz-dom/src/stylo.rs:143)
.raw-facts-s06.md:178: - A `password` input type is listed among the fields that block implicit submission; no value is read (packages/blitz-dom/src/events/keyboard.rs:149-164)
.raw-facts-s09.md:106: ## security-plan §Authentication & Authorization
.raw-facts-s09.md:107: - observed absent — authentication or authorization code · searched: `password|secret|token|authorization|cookie|x-api-key` (case-insensitive) over all 32 s09 files (one hit, the winit `ActivationTokenDone` window event at packages/blitz-shell/src/window.rs:593)
.raw-facts-s09.md:129: ## security-plan §Secret Management
.raw-facts-s09.md:130: - observed absent — secret, key or credential handling · searched: `password|secret|token|authorization|cookie|x-api-key` and `std::env|env!\(|env::var` over all 32 s09 files
.raw-facts-s10.md:135: ## security-plan §Authentication & Authorization
.raw-facts-s10.md:136: - a `password` URL-component accessor is defined on `<a>`/`<area>` elements (packages/blitz-vibey-script/src/dom/hyperlink.rs:32)
.raw-facts-s10.md:137: - observed absent — authentication or authorization code · searched: `password|token|secret|cookie|authorization|x-api-key|Authorization` over the 32 s10 files (only the hyperlink URL component matched)
.raw-facts-s10.md:166: ## security-plan §Secret Management
.raw-facts-s10.md:167: - observed absent — secrets or environment-read credentials · searched: `password|token|secret|cookie|authorization|x-api-key|Authorization` and `env::var|env!\(` over the 32 s10 files
a11y-plan-draft.md:109: - **Form input** — `<input>` roles are mapped by `type`, defaulting to TextInput (packages/blitz-dom/src/accessibility.rs:234-252); label→Label, textarea→MultilineTextInput, progress→ProgressIndicator, meter→Meter, radio→RadioButton, range→Slider, email→EmailInput, password→PasswordInput (tests/blitz-tests/tests/accessibility_roles.rs:128-155); text→TextInput, number→NumberInput, checkbox→CheckBox (tests/blitz-tests/tests/accessibility_roles.rs:157-180)
a11y-plan-facts.md:255: - Form controls: label→Label, select→ComboBox, select multiple→ListBox, textarea→MultilineTextInput, progress→ProgressIndicator, meter→Meter, radio→RadioButton, range→Slider, email→EmailInput, password→PasswordInput, submit→Button (tests/blitz-tests/tests/accessibility_roles.rs:128-155)
architecture-facts.md:483: - Text inputs (`textarea`, and `input` with no type or text/password/email/number/search/tel/url) get a Parley text editor; checkbox/radio inputs get checkbox data (packages/blitz-dom/src/layout/construct.rs:441-458)
design-system-draft.md:82: **Token sets:**
design-system-draft.md:103: > NOT YET MEASURED — a project-wide UI colour token set: the examples slice (s01) and the DOM-node slice (s08) recorded the project's own colour tokens as out of slice, and the default user-agent stylesheet's full contents were out of slice for s05.
design-system-draft.md:168: | Token | Value | Usage |
design-system-draft.md:237: | Token | Value | Usage |
facts-s01.md:109: ## security-plan §Authentication & Authorization
facts-s01.md:114: - out of slice — application authentication/authorization
facts-s01.md:125: - GitHub API calls go through the `gh api` CLI with `GH_TOKEN` from the workflow `token` secret (.github/scripts/wpt_diff_to_pr.py:175-182; .github/workflows/wpt-post-results.yml:29-49)
facts-s01.md:135: ## security-plan §Secret Management
facts-s01.md:136: - A macOS signing key is written from a secret to the path in `vars.APPLE_API_KEY_PATH` and removed in an `always()` step (.github/workflows/publish-browser.yml:105-107; .github/workflows/publish-browser.yml:167-169)
facts-s01.md:137: - An Android keystore is base64-decoded from a secret to `apps/browser/keystore.jks`, `password` fields (`jks_password`, `key_password`) are appended to Dioxus.toml from a secret, and the keystore is removed in an `always()` step (.github/workflows/publish-browser.yml:109-119; .github/workflows/publish-browser.yml:171-173)
facts-s01.md:138: - Apple certificate and certificate `password` are passed as env from secrets; API issuer, key id and key path come from `vars` (.github/workflows/publish-browser.yml:155-160)
facts-s01.md:139: - A `token` field is read from secret `WPT_GITHUB_TOKEN` for the cross-repo dispatch (.github/workflows/wpt.yml:113-116)
facts-s01.md:140: - The workflow `token` secret `GITHUB_TOKEN` is passed as `github-token` and `GH_TOKEN` (.github/workflows/wpt-post-results.yml:28; .github/workflows/wpt-post-results.yml:32; .github/workflows/wpt-post-results.yml:46)
facts-s02.md:69: ## security-plan §Authentication & Authorization
facts-s02.md:89: ## security-plan §Secret Management
facts-s02.md:90: - observed absent — password, token, secret, cookie, authorization or x-api-key fields · searched: `password|token|secret|cookie|authorization|x-api-key` over the 21 s02 files
facts-s03.md:97: ## security-plan §Authentication & Authorization
facts-s03.md:98: - observed absent — credential or auth handling · searched: `password|secret|token|authorization|cookie|x-api-key` over the 32 slice files; the only hits are a commented-out local variable named `token` in a style traversal sketch (tests/stylo_usage.rs:152; tests/stylo_usage.rs:157)
facts-s03.md:122: ## security-plan §Secret Management
facts-s03.md:124: - observed absent — secrets read from environment or files · searched: `password|secret|token|authorization|cookie|x-api-key` and `std::env::var|env!\(` over the 32 slice files
facts-s04.md:158: ## security-plan §Authentication & Authorization
facts-s04.md:159: - observed absent — authentication or authorization code · searched: `auth|login|session` (case-insensitive, excluding "Authors") over the 86 slice files
facts-s04.md:188: ## security-plan §Secret Management
facts-s04.md:189: - observed absent — secret-bearing fields or credentials · searched: `password|secret|api_key|x-api-key|authorization|token` (case-insensitive) over the 86 slice files; only the formula `Token` enum in cells.rs matched
facts-s05.md:138: ## security-plan §Authentication & Authorization
facts-s05.md:139: - observed absent — authentication or authorization logic · searched: `login|credential|permission|authenticat|authorization` over the 15 s05 files
facts-s05.md:170: ## security-plan §Secret Management
facts-s05.md:171: - observed absent — secrets read from environment or code · searched: `env::var|secret|token|authorization|cookie|x-api-key` over the 15 s05 files
facts-s05.md:172: - The only `password` occurrence maps the input type `password` to `Role::PasswordInput` (packages/blitz-dom/src/accessibility.rs:244)
facts-s06.md:139: ## security-plan §Authentication & Authorization
facts-s06.md:140: - observed absent — authentication or authorization logic · searched: `authenticat|authoriz|login|session|credential` over the 17 listed s06 files
facts-s06.md:166: ## security-plan §Secret Management
facts-s06.md:167: - observed absent — environment or secret reads · searched: `env::var|std::env|api_key` over the 17 listed s06 files
facts-s07.md:31: - Text inputs (`textarea`, and `input` with no type or text/password/email/number/search/tel/url) get a Parley text editor; checkbox/radio inputs get checkbox data (packages/blitz-dom/src/layout/construct.rs:441-458)
facts-s07.md:93: ## security-plan §Authentication & Authorization
facts-s07.md:94: - observed absent — authentication or authorization · searched: `auth|session|login|permission` over the 8 slice files
facts-s07.md:113: ## security-plan §Secret Management
facts-s07.md:114: - observed absent — secrets or credentials read · searched: `env::var|secret|token` over the 8 slice files
facts-s08.md:115: ## security-plan §Authentication & Authorization
facts-s08.md:116: - observed absent — authentication or authorization handling · searched: `password|secret|token|authorization|x-api-key` over the 16 listed files (only html5ever `TokenizerOpts` and a commented `DOMTokenList` matched)
facts-s08.md:117: - With feature `cookies`, the reqwest client enables a cookie store (packages/blitz-net/Cargo.toml:15; packages/blitz-net/src/lib.rs:88-89)
facts-s08.md:148: ## security-plan §Secret Management
facts-s08.md:149: - observed absent — secrets, API keys or credential loading · searched: `password|secret|token|authorization|x-api-key` over the 16 listed files (no credential match)
facts-s09.md:95: ## security-plan §Authentication & Authorization
facts-s09.md:96: - observed absent — credential or auth field names · searched: `token|password|secret|api_key|authorization|cookie` case-sensitive over the 32 listed s09 files
facts-s09.md:117: ## security-plan §Secret Management
facts-s09.md:119: - observed absent — secret-bearing field names · searched: `token|password|secret|api_key|authorization|cookie` case-sensitive over the 32 listed s09 files
facts-s10.md:144: ## security-plan §Authentication & Authorization
facts-s10.md:145: - a `password` accessor (the URL password component) is defined on the element prototype for `<a>`/`<area>`, read from and written into the resolved `href` (packages/blitz-vibey-script/src/dom/hyperlink.rs:32; packages/blitz-vibey-script/src/dom/hyperlink.rs:127; packages/blitz-vibey-script/src/dom/hyperlink.rs:170-172)
facts-s10.md:146: - observed absent — authentication or authorization code · searched: `auth|Auth|token|secret|password|cookie|authorization` over the 32 slice files (matches only the hyperlink URL component and an "author stylesheets" comment)
facts-s10.md:178: ## security-plan §Secret Management
facts-s11.md:130: ## security-plan §Authentication & Authorization
facts-s11.md:131: - observed absent — any authentication or authorization code or credential field · searched: `token|password|secret|authorization|cookie|x-api-key|auth` over the 21 listed s11 files (only `authors` matched)
facts-s11.md:153: ## security-plan §Secret Management
facts-s11.md:154: - observed absent — secrets or environment-variable reads · searched: `token|password|secret|authorization|cookie|x-api-key|auth` and `std::env|env::var|getenv` over the 21 listed s11 files
facts-s12.md:121: ## security-plan §Authentication & Authorization
facts-s12.md:122: - observed absent — authentication or authorization code · searched: `token|password|secret|api[_-]?key|authorization|cookie` (case-insensitive) over the 61 slice files
facts-s12.md:123: - The only `password` matches are an `<input type="password">` fixture mapped to `Role::PasswordInput` in an accessibility test (tests/blitz-tests/tests/accessibility_roles.rs:140; tests/blitz-tests/tests/accessibility_roles.rs:153)
facts-s12.md:143: ## security-plan §Secret Management
facts-s12.md:144: - observed absent — secret, token, API key or credential handling · searched: `token|password|secret|api[_-]?key|authorization|cookie` (case-insensitive) over the 61 slice files
facts-s12.md:296: - Form controls: label→Label, select→ComboBox, select multiple→ListBox, textarea→MultilineTextInput, progress→ProgressIndicator, meter→Meter, radio→RadioButton, range→Slider, email→EmailInput, password→PasswordInput, submit→Button (tests/blitz-tests/tests/accessibility_roles.rs:128-155)
facts-s13.md:126: ## security-plan §Authentication & Authorization
facts-s13.md:127: - observed absent — authentication or credential fields · searched: `authorization|cookie|x-api-key|password|secret|token` (case-insensitive) over the 12 listed files
facts-s13.md:150: ## security-plan §Secret Management
facts-s13.md:152: - observed absent — secret-bearing fields · searched: `authorization|cookie|x-api-key|password|secret|token` (case-insensitive) over the 12 listed files
inventory-no-marker.json:252: "Authentication & Authorization",
inventory-no-marker.json:258: "Secret Management",
security-plan-draft.md:26: ## Authentication & Authorization
security-plan-draft.md:28: **Auth approach:** none observed. Application authentication or authorization code is observed absent in every slice that searched:
security-plan-draft.md:29: - searched: `password|secret|token|authorization|cookie|x-api-key` over the 32 s03 files; the only hits are a commented-out local variable named `token` in a style traversal sketch (tests/stylo_usage.rs:152; tests/stylo_usage.rs:157)
security-plan-draft.md:31: - searched: `login|credential|permission|authenticat|authorization` over the 15 s05 files
security-plan-draft.md:34: - searched: `password|secret|token|authorization|x-api-key` over the 16 listed s08 files (only html5ever `TokenizerOpts` and a commented `DOMTokenList` matched)
security-plan-draft.md:35: - searched: `token|password|secret|api_key|authorization|cookie` case-sensitive over the 32 listed s09 files
security-plan-draft.md:36: - searched: `auth|Auth|token|secret|password|cookie|authorization` over the 32 s10 files (matches only the hyperlink URL component and an "author stylesheets" comment)
security-plan-draft.md:37: - searched: `token|password|secret|authorization|cookie|x-api-key|auth` over the 21 listed s11 files (only `authors` matched)
security-plan-draft.md:38: - searched: `token|password|secret|api[_-]?key|authorization|cookie` (case-insensitive) over the 61 s12 files
security-plan-draft.md:39: - searched: `authorization|cookie|x-api-key|password|secret|token` (case-insensitive) over the 12 listed s13 files
security-plan-draft.md:45: | Token / session storage | With feature `cookies`, the reqwest client enables a cookie store (packages/blitz-net/Cargo.toml:15; packages/blitz-net/src/lib.rs:88-89) | reqwest cookie store |
security-plan-draft.md:51: Other `password` occurrences are not credentials:
security-plan-draft.md:52: - A `password` accessor (the URL password component) is defined on the element prototype for `<a>`/`<area>`, read from and written into the resolved `href` (packages/blitz-vibey-script/src/dom/hyperlink.rs:32; packages/blitz-vibey-script/src/dom/hyperlink.rs:127; packages/blitz-vibey-script/src/dom/hyperlink.rs:170-172)
security-plan-draft.md:53: - An `<input type="password">` fixture is mapped to `Role::PasswordInput` in an accessibility test (tests/blitz-tests/tests/accessibility_roles.rs:140; tests/blitz-tests/tests/accessibility_roles.rs:153)
security-plan-draft.md:199: | CI API calls | GitHub API calls go through the `gh api` CLI with `GH_TOKEN` from the workflow `token` secret (.github/scripts/wpt_diff_to_pr.py:175-182; .github/workflows/wpt-post-results.yml:29-49) | `gh` CLI |
security-plan-draft.md:239: - **auth-scaffolding-baseline:** authentication or authorization code is recorded absent — see Authentication & Authorization.
security-plan-draft.md:246: ## Secret Management
security-plan-draft.md:250:   - A macOS signing key is written from a secret to the path in `vars.APPLE_API_KEY_PATH` and removed in an `always()` step (.github/workflows/publish-browser.yml:105-107; .github/workflows/publish-browser.yml:167-169)
security-plan-draft.md:251:   - An Android keystore is base64-decoded from a secret to `apps/browser/keystore.jks`, `password` fields (`jks_password`, `key_password`) are appended to Dioxus.toml from a secret, and the keystore is removed in an `always()` step (.github/workflows/publish-browser.yml:109-119; .github/workflows/publish-browser.yml:171-173)
security-plan-draft.md:252:   - Apple certificate and certificate `password` are passed as env from secrets; API issuer, key id and key path come from `vars` (.github/workflows/publish-browser.yml:155-160)
security-plan-draft.md:253:   - A `token` field is read from secret `WPT_GITHUB_TOKEN` for the cross-repo dispatch (.github/workflows/wpt.yml:113-116)
security-plan-draft.md:254:   - The workflow `token` secret `GITHUB_TOKEN` is passed as `github-token` and `GH_TOKEN` (.github/workflows/wpt-post-results.yml:28; .github/workflows/wpt-post-results.yml:32; .github/workflows/wpt-post-results.yml:46)
security-plan-draft.md:256: **Never in code:** secret reads in source are observed absent in every slice that searched:
security-plan-draft.md:257: - searched: `password|token|secret|cookie|authorization|x-api-key` over the 21 s02 files
security-plan-draft.md:258: - searched: `password|secret|token|authorization|cookie|x-api-key` and `std::env::var|env!\(` over the 32 s03 files
security-plan-draft.md:259: - searched: `password|secret|api_key|x-api-key|authorization|token` (case-insensitive) over the 86 s04 files; only the formula `Token` enum in cells.rs matched
security-plan-draft.md:260: - searched: `env::var|secret|token|authorization|cookie|x-api-key` over the 15 s05 files; the only `password` occurrence maps the input type `password` to `Role::PasswordInput` (packages/blitz-dom/src/accessibility.rs:244)
security-plan-draft.md:262: - searched: `env::var|secret|token` over the 8 s07 files
security-plan-draft.md:263: - searched: `password|secret|token|authorization|x-api-key` over the 16 listed s08 files (no credential match)
security-plan-draft.md:264: - searched: `std::env|env!\(|getenv` and `token|password|secret|api_key|authorization|cookie` case-sensitive over the 32 listed s09 files
security-plan-draft.md:266: - searched: `token|password|secret|authorization|cookie|x-api-key|auth` and `std::env|env::var|getenv` over the 21 listed s11 files
security-plan-draft.md:267: - searched: `token|password|secret|api[_-]?key|authorization|cookie` (case-insensitive) over the 61 s12 files
security-plan-draft.md:268: - searched: `authorization|cookie|x-api-key|password|secret|token` (case-insensitive) over the 12 listed s13 files
security-plan-draft.md:278: **What counts as secret (as observed):** CI signing material — the macOS signing key, the Android keystore and its passwords, the Apple certificate and its password — and the `WPT_GITHUB_TOKEN` and `GITHUB_TOKEN` tokens (see Storage above).
security-plan-draft.md:280: > NOT YET MEASURED — development secret storage, secret scanning in CI, rotation cadence and access auditing: no slice gathered them.
security-plan-draft.md:362: > NOT YET MEASURED — security-event logging (auth, authorization or validation failures), log retention, access controls and tamper evidence: no slice gathered them.
security-plan-facts.md:77: ## §Authentication & Authorization
security-plan-facts.md:85: - out of slice — application authentication/authorization
security-plan-facts.md:93: - observed absent — credential or auth handling · searched: `password|secret|token|authorization|cookie|x-api-key` over the 32 slice files; the only hits are a commented-out local variable named `token` in a style traversal sketch (tests/stylo_usage.rs:152; tests/stylo_usage.rs:157)
security-plan-facts.md:97: - observed absent — authentication or authorization code · searched: `auth|login|session` (case-insensitive, excluding "Authors") over the 86 slice files
security-plan-facts.md:101: - observed absent — authentication or authorization logic · searched: `login|credential|permission|authenticat|authorization` over the 15 s05 files
security-plan-facts.md:105: - observed absent — authentication or authorization logic · searched: `authenticat|authoriz|login|session|credential` over the 17 listed s06 files
security-plan-facts.md:109: - observed absent — authentication or authorization · searched: `auth|session|login|permission` over the 8 slice files
security-plan-facts.md:113: - observed absent — authentication or authorization handling · searched: `password|secret|token|authorization|x-api-key` over the 16 listed files (only html5ever `TokenizerOpts` and a commented `DOMTokenList` matched)
security-plan-facts.md:114: - With feature `cookies`, the reqwest client enables a cookie store (packages/blitz-net/Cargo.toml:15; packages/blitz-net/src/lib.rs:88-89)
security-plan-facts.md:118: - observed absent — credential or auth field names · searched: `token|password|secret|api_key|authorization|cookie` case-sensitive over the 32 listed s09 files
security-plan-facts.md:122: - a `password` accessor (the URL password component) is defined on the element prototype for `<a>`/`<area>`, read from and written into the resolved `href` (packages/blitz-vibey-script/src/dom/hyperlink.rs:32; packages/blitz-vibey-script/src/dom/hyperlink.rs:127; packages/blitz-vibey-script/src/dom/hyperlink.rs:170-172)
security-plan-facts.md:123: - observed absent — authentication or authorization code · searched: `auth|Auth|token|secret|password|cookie|authorization` over the 32 slice files (matches only the hyperlink URL component and an "author stylesheets" comment)
security-plan-facts.md:127: - observed absent — any authentication or authorization code or credential field · searched: `token|password|secret|authorization|cookie|x-api-key|auth` over the 21 listed s11 files (only `authors` matched)
security-plan-facts.md:131: - observed absent — authentication or authorization code · searched: `token|password|secret|api[_-]?key|authorization|cookie` (case-insensitive) over the 61 slice files
security-plan-facts.md:132: - The only `password` matches are an `<input type="password">` fixture mapped to `Role::PasswordInput` in an accessibility test (tests/blitz-tests/tests/accessibility_roles.rs:140; tests/blitz-tests/tests/accessibility_roles.rs:153)
security-plan-facts.md:136: - observed absent — authentication or credential fields · searched: `authorization|cookie|x-api-key|password|secret|token` (case-insensitive) over the 12 listed files
security-plan-facts.md:348: - GitHub API calls go through the `gh api` CLI with `GH_TOKEN` from the workflow `token` secret (.github/scripts/wpt_diff_to_pr.py:175-182; .github/workflows/wpt-post-results.yml:29-49)
security-plan-facts.md:494: ## §Secret Management
security-plan-facts.md:498: - A macOS signing key is written from a secret to the path in `vars.APPLE_API_KEY_PATH` and removed in an `always()` step (.github/workflows/publish-browser.yml:105-107; .github/workflows/publish-browser.yml:167-169)
security-plan-facts.md:499: - An Android keystore is base64-decoded from a secret to `apps/browser/keystore.jks`, `password` fields (`jks_password`, `key_password`) are appended to Dioxus.toml from a secret, and the keystore is removed in an `always()` step (.github/workflows/publish-browser.yml:109-119; .github/workflows/publish-browser.yml:171-173)
security-plan-facts.md:500: - Apple certificate and certificate `password` are passed as env from secrets; API issuer, key id and key path come from `vars` (.github/workflows/publish-browser.yml:155-160)
security-plan-facts.md:501: - A `token` field is read from secret `WPT_GITHUB_TOKEN` for the cross-repo dispatch (.github/workflows/wpt.yml:113-116)
security-plan-facts.md:502: - The workflow `token` secret `GITHUB_TOKEN` is passed as `github-token` and `GH_TOKEN` (.github/workflows/wpt-post-results.yml:28; .github/workflows/wpt-post-results.yml:32; .github/workflows/wpt-post-results.yml:46)
security-plan-facts.md:506: - observed absent — password, token, secret, cookie, authorization or x-api-key fields · searched: `password|token|secret|cookie|authorization|x-api-key` over the 21 s02 files
security-plan-facts.md:512: - observed absent — secrets read from environment or files · searched: `password|secret|token|authorization|cookie|x-api-key` and `std::env::var|env!\(` over the 32 slice files
security-plan-facts.md:516: - observed absent — secret-bearing fields or credentials · searched: `password|secret|api_key|x-api-key|authorization|token` (case-insensitive) over the 86 slice files; only the formula `Token` enum in cells.rs matched
security-plan-facts.md:520: - observed absent — secrets read from environment or code · searched: `env::var|secret|token|authorization|cookie|x-api-key` over the 15 s05 files
security-plan-facts.md:521: - The only `password` occurrence maps the input type `password` to `Role::PasswordInput` (packages/blitz-dom/src/accessibility.rs:244)
security-plan-facts.md:525: - observed absent — environment or secret reads · searched: `env::var|std::env|api_key` over the 17 listed s06 files
security-plan-facts.md:529: - observed absent — secrets or credentials read · searched: `env::var|secret|token` over the 8 slice files
security-plan-facts.md:533: - observed absent — secrets, API keys or credential loading · searched: `password|secret|token|authorization|x-api-key` over the 16 listed files (no credential match)
security-plan-facts.md:539: - observed absent — secret-bearing field names · searched: `token|password|secret|api_key|authorization|cookie` case-sensitive over the 32 listed s09 files
security-plan-facts.md:548: - observed absent — secrets or environment-variable reads · searched: `token|password|secret|authorization|cookie|x-api-key|auth` and `std::env|env::var|getenv` over the 21 listed s11 files
security-plan-facts.md:552: - observed absent — secret, token, API key or credential handling · searched: `token|password|secret|api[_-]?key|authorization|cookie` (case-insensitive) over the 61 slice files
security-plan-facts.md:558: - observed absent — secret-bearing fields · searched: `authorization|cookie|x-api-key|password|secret|token` (case-insensitive) over the 12 listed files
