# facts-s02 · HEAD 0f60502ea724ef703b34220b3847bb3483f70b5f · 21 files

## architecture §Design Philosophy
- out of slice — the 21 files are HTML example documents; none states a design philosophy for this repository

## architecture §Stack and Technologies
- every s02 file is a standalone HTML document under examples/assets; no build manifest, source module or config is in the slice (examples/assets/google.html:1-2; examples/assets/hr.html:1-2; examples/assets/inline-flex-transform.html:1)
- the gosub fixture links Font Awesome 6.4.2 from cdnjs.cloudflare.com (examples/assets/gosub.html:86)
- the pseudo fixture links Font Awesome v6.6.0 CSS and declares an @font-face for 'Font Awesome 6 Brands' from use.fontawesome.com v6.6.0 woff2/ttf (examples/assets/pseudo.html:4; examples/assets/pseudo.html:8-14)
- the servo fixtures link the remote stylesheet https://servo.org/css/style.css (examples/assets/newservo.html:4; examples/assets/servo-new-reduced.html:7; examples/assets/servo-new.html:18)
- the servo-new fixture links prismjs@1.20.0 prism-okaidia.css from unpkg.com (examples/assets/servo-new.html:20)
- the servo-new fixture carries two inline scripts that toggle the navbar menu, and loads Cloudflare's email-decode.min.js (examples/assets/servo-new.html:228-242; examples/assets/servo-new.html:383-392)
- the graphite fixture carries one inline async script that fetches the GitHub stars count from api.github.com (examples/assets/graphite.html:1859-1874)
- the google fixture sets base href https://www.google.com/ and is a script-free snapshot whose body is styled by inline style blocks (examples/assets/google.html:6; examples/assets/google.html:3024-3025)
- observed absent — script elements outside graphite.html and servo-new.html · searched: `<script` over the 21 s02 files

## architecture §Established Decisions
- out of slice — no decision record or rationale is stated in the 21 HTML documents

## architecture §Conventions
- reduced variants sit beside full pages: google_reduced.html keeps only the Google title and one div (examples/assets/google_reduced.html:1-9)
- gosub_reduced.html restates gosub.html's body colors and flex column layout as inline styles with the link list as plain divs (examples/assets/gosub_reduced.html:3-9; examples/assets/gosub.html:7-19)
- servo-new-reduced.html keeps the two "features" rows of servo-new.html's hero section (examples/assets/servo-new-reduced.html:12-49; examples/assets/servo-new.html:264-297)
- servo-new-reduced-1.html reduces one feature card to nested divs with inline styles and a 1px black border on div and p (examples/assets/servo-new-reduced-1.html:16-30)
- graphite_blog_section.html keeps graphite.html's style sheet with only the #recent-news summary paragraph as body (examples/assets/graphite_blog_section.html:1635-1645; examples/assets/graphite.html:2074-2097)
- graphite_software_overview.html keeps graphite.html's style sheet with only the sizzle-video diptych as body, the video replaced by an img (examples/assets/graphite_software_overview.html:1633-1645; examples/assets/graphite.html:1931-1942)
- observed absent — @font-face rules in the two graphite subset files, which graphite.html declares at examples/assets/graphite.html:1635-1822 · searched: `@font-face` over examples/assets/graphite_blog_section.html and examples/assets/graphite_software_overview.html
- feature fixtures group numbered cases under h2/h3 headings with a CSS comment per case (examples/assets/hr.html:31-188; examples/assets/hr.html:192-225; examples/assets/object_fit.html:12-49; examples/assets/inline-backgrounds.html:34-53)
- sibling assets are referenced with ./-relative URLs (examples/assets/iframe_navigation.html:37; examples/assets/iframe_page_a.html:23-28; examples/assets/iframe_page_b.html:18; examples/assets/object_fit.html:13-15; examples/assets/noscript.html:6)
- graphite.html is minified HTML: unquoted attribute values and omitted closing p tags (examples/assets/graphite.html:2; examples/assets/graphite.html:1846; examples/assets/graphite.html:1918)
- observed absent — a closing head tag in graphite.html and its two subsets, which go from the style element straight to body · searched: `</head>` over examples/assets/graphite.html, examples/assets/graphite_blog_section.html and examples/assets/graphite_software_overview.html
- graphite.html and graphite_software_overview.html end on a closing div with no closing body or html tag (examples/assets/graphite.html:2104; examples/assets/graphite_software_overview.html:1645)
- inline-flex-transform.html has no doctype and opens with a bare html tag (examples/assets/inline-flex-transform.html:1)
- gosub.html begins with an empty line before its doctype (examples/assets/gosub.html:1-2)

## architecture §Standard Contracts
- the google fixture's search form submits GET to /search with autocomplete off and role=search (examples/assets/google.html:3130-3135)
- the google fixture's query field is a textarea named q, maxlength 2048, type search (examples/assets/google.html:3176-3201)
- the google fixture's submit inputs are named btnK and btnI (examples/assets/google.html:3493-3510; examples/assets/google.html:3544-3563)
- the google fixture's form carries hidden inputs named sca_esv, source, ei and iflsig (examples/assets/google.html:3569-3574)
- the graphite fixture's newsletter form POSTs to https://graphite.art/newsletter-signup with fields name, phone and email (examples/assets/graphite.html:2050-2057)

## architecture §Occupied Resources
- out of slice — no port, path, database or process resource is stated in the 21 HTML documents

## architecture §Infrastructure Patterns
- out of slice — no deployment, container or hosting configuration is in the slice

## architecture §Cross-cutting Patterns
- out of slice — the slice shows no shared code path; each document is self-contained

## architecture §Project Intent
- out of slice — no file states this repository's purpose; the servo-new fixture's prose describes Servo, a third-party engine (examples/assets/servo-new.html:261-263)

## architecture §Existing Scopes
- hr rendering: 21 hr cases across default UA styling, color and background, border styles, thickness, decorative and alignment (examples/assets/hr.html:192-225)
- object-fit: fill, contain and cover on square, wide and tall images at 50px, 100px and 200px (examples/assets/object_fit.html:12-49)
- inline element backgrounds: solid, wrapping, semi-transparent, nested and mixed-font-size spans (examples/assets/inline-backgrounds.html:57-108)
- iframe navigation: src navigation, srcdoc navigation, nested frames and top-document navigation (examples/assets/iframe_navigation.html:29-72; examples/assets/iframe_page_a.html:21-28; examples/assets/iframe_page_b.html:16-19)
- form input and focusable divs with tabindex 0 and -1 inside a center element (examples/assets/input.html:4-11)
- noscript content: an img inside a noscript element (examples/assets/noscript.html:5-7)
- pseudo-element content and icon fonts: ::before content and Font Awesome glyphs (examples/assets/pseudo.html:16-19; examples/assets/pseudo.html:55-65; examples/assets/pseudo.html:71-80)
- inline-flex link with transform transitions and hover scale (examples/assets/inline-flex-transform.html:20-42; examples/assets/inline-flex-transform.html:49-54)
- real-page snapshots: Google homepage, Gosub landing page, Graphite homepage and Servo homepage (examples/assets/google.html:9; examples/assets/gosub.html:93; examples/assets/graphite.html:5; examples/assets/servo-new.html:7)

## security-plan §Threat Model Summary
- out of slice — no threat model is stated in the 21 HTML documents

## security-plan §Authentication & Authorization
- out of slice — the slice holds no auth code; the google fixture only shows a "Sign in" link to accounts.google.com (examples/assets/google.html:3100-3107)

## security-plan §Input Validation
- the graphite fixture's name and email inputs are required and the email input is type=email (examples/assets/graphite.html:2052; examples/assets/graphite.html:2054)
- the graphite fixture's phone input has autocomplete=off and tabindex=-1, and its column is display:none (examples/assets/graphite.html:2053; examples/assets/graphite.html:293-295)
- the google fixture's query textarea is limited to maxlength 2048 (examples/assets/google.html:3181)

## security-plan §Data Protection
- the google fixture sets a referrer meta of "origin" (examples/assets/google.html:5)
- the servo-new fixture shows a Cloudflare email-protection link in place of a plain address (examples/assets/servo-new.html:358)

## security-plan §API Security
- the graphite fixture's script calls api.github.com with fetch and no request headers (examples/assets/graphite.html:1862)

## security-plan §Dependency Security
- the gosub fixture's Font Awesome link carries an integrity hash with crossorigin=anonymous and referrerpolicy=no-referrer (examples/assets/gosub.html:86)
- other remote stylesheets are linked without integrity attributes (examples/assets/pseudo.html:4; examples/assets/newservo.html:4; examples/assets/servo-new-reduced.html:7; examples/assets/servo-new.html:18; examples/assets/servo-new.html:20)
- observed absent — integrity attributes outside gosub.html · searched: `integrity=` over the 21 s02 files

## security-plan §Secret Management
- observed absent — password, token, secret, cookie, authorization or x-api-key fields · searched: `password|token|secret|cookie|authorization|x-api-key` over the 21 s02 files
- the google fixture carries an http-equiv origin-trial meta with an encoded value (examples/assets/google.html:7)

## security-plan §Error Handling
- the graphite fixture's stars script wraps the fetch in try/catch and removes the target element on any failure (examples/assets/graphite.html:1861-1872)
- the iframe page A fixture links to a missing page labelled "Broken link (should stay usable)" (examples/assets/iframe_page_a.html:25)

## security-plan §Logging & Monitoring
- observed absent — console logging calls · searched: `console\.` over the 21 s02 files

## design-system §Color Palette
- the graphite fixture defines 16 color custom properties on :root, among them --color-navy #16323f, --color-crimson #803847, --color-fog #eee and --color-seaside-rgb 176, 214, 203 (examples/assets/graphite.html:890-906)
- the graphite subsets repeat the same :root color properties (examples/assets/graphite_blog_section.html:890-906; examples/assets/graphite_software_overview.html:888-904)
- the graphite fixture's page text is var(--color-navy) on #fff and links are var(--color-crimson) (examples/assets/graphite.html:918-922; examples/assets/graphite.html:1405-1407)
- the google fixture sets body/input/button text #202124 on #fff, links #1a0dab and visited links #681da8 (examples/assets/google.html:19-36)
- observed absent — CSS custom properties in the google fixture · searched: `var\(--|--[a-z]+:` over examples/assets/google.html
- the gosub fixture is dark: #ffffff text on #121212, cyan links with a 2px solid #6e40c9 bottom border, and an h1 gradient from rgba(96,243,236,1) to rgba(30,84,231,1) (examples/assets/gosub.html:7-9; examples/assets/gosub.html:45-50; examples/assets/gosub.html:74-80)
- the hr fixture assigns a distinct hex color per case, e.g. #e63946, #2a9d8f, #457b9d, #8338ec (examples/assets/hr.html:43; examples/assets/hr.html:50; examples/assets/hr.html:72; examples/assets/hr.html:78)
- the inline-backgrounds fixture uses pastel span backgrounds and one rgba(214, 51, 108, 0.35) translucent background (examples/assets/inline-backgrounds.html:35-53)
- observed absent — a prefers-color-scheme rule · searched: `prefers-color-scheme` over the 21 s02 files
- the google fixture shows a "Dark theme: Off" settings menu item (examples/assets/google.html:3788)

## design-system §Typography
- the graphite fixture sets html/body to Inter Variable, 18px, weight 500, line-height 1.5, dropping to 16px at width<=780px (examples/assets/graphite.html:918-930; examples/assets/graphite.html:940-943)
- the graphite fixture sets h1 to Bona Nova, Palatino, serif at 2.66667rem weight 700, h2 1.75rem, h3 1.25rem, h4-h6 1rem in Inter Variable weight 800 (examples/assets/graphite.html:1322-1330; examples/assets/graphite.html:1341-1366)
- the graphite fixture defines --font-size-link as calc(1rem*4/3) (examples/assets/graphite.html:915)
- the graphite fixture declares Inter Variable @font-face rules (weight 100 900, woff2-variations, font-display swap) per unicode range, normal and italic, and Bona Nova weight 700 rules (examples/assets/graphite.html:1635-1758; examples/assets/graphite.html:1761-1822)
- the graphite nav font size steps down through --nav-font-size from 28px to 8px across width breakpoints (examples/assets/graphite.html:986-995; examples/assets/graphite.html:1149-1156)
- the google fixture sets body/input/button to 14px arial,sans-serif and the bar to 13px/27px Roboto (examples/assets/google.html:22-26; examples/assets/google.html:45-48)
- the google fixture uses Google Sans,Roboto,Helvetica,Arial,sans-serif stacks (examples/assets/google.html:563; examples/assets/google.html:1050)
- the gosub fixture uses "Arial", sans-serif with h1 at 6em, then overridden to 72px (examples/assets/gosub.html:10; examples/assets/gosub.html:20-21; examples/assets/gosub.html:74-75)
- the servo-new-reduced-1 fixture sets body to 'Space Grotesk', sans-serif (examples/assets/servo-new-reduced-1.html:13)
- the pseudo fixture's .qqq class renders Font Awesome 6 Brands glyphs at 32px (examples/assets/pseudo.html:21-25; examples/assets/pseudo.html:80)

## design-system §Spacing
- the graphite fixture's :root sets --max-width 1200px, --max-extended-width 1600px, --max-width-reading-material 800px, --variable-px Min(1px, .15vw), --page-edge-padding 40px, --border-thickness 2px and --feature-box-padding 80 (examples/assets/graphite.html:907-914)
- the graphite fixture lowers --page-edge-padding to 28px and --feature-box-padding to 40 at width<=780px, and --page-edge-padding to 20px for print or width<=500px (examples/assets/graphite.html:932-938; examples/assets/graphite.html:946-951)
- the graphite fixture spaces sibling main sections by calc(120*var(--variable-px)) (examples/assets/graphite.html:1203-1205)
- the hr fixture gives hr a 24px vertical margin and the body 16px 40px 40px padding (examples/assets/hr.html:8; examples/assets/hr.html:25-29)

## design-system §Depth Strategy
- the google fixture's search box takes box-shadow 0 1px 6px rgba(32,33,36,.28) in its active state (examples/assets/google.html:3151)
- the google fixture uses layered shadows such as 0 1px 3px 1px rgba(66,64,67,.15),0 1px 2px 0 rgba(60,64,67,.3) (examples/assets/google.html:364-365)
- the google fixture's bar sits at z-index 986 and the graphite header at z-index 1000 (examples/assets/google.html:47; examples/assets/graphite.html:961)
- the hr fixture exercises an inset shadow and an outset glow (examples/assets/hr.html:137; examples/assets/hr.html:146)

## design-system §Border Radius
- the google fixture's search box has border-radius 24px and its buttons 4px (examples/assets/google.html:3151; examples/assets/google.html:3491)
- the google fixture's style sheet also uses 2px, 8px and 50% radii (examples/assets/google.html:100; examples/assets/google.html:923; examples/assets/google.html:459)
- the graphite fixture's newsletter inputs have border-radius 0 and its carousel dots 50% (examples/assets/graphite.html:341; examples/assets/graphite.html:577)
- the hr fixture's pill cases use border-radius 999px (examples/assets/hr.html:128)
- the inline-flex-transform fixture's .button uses border-radius 1.5rem 0 (examples/assets/inline-flex-transform.html:22)
- the input fixture's third div uses border-radius 100px (examples/assets/input.html:9)

## design-system §Motion
- the google fixture declares keyframes gb__a, g-bubble-show, g-bubble-hide, g-snackbar-show/hide and qli spinner animations (examples/assets/google.html:49-56; examples/assets/google.html:1589; examples/assets/google.html:1597; examples/assets/google.html:1700; examples/assets/google.html:1709; examples/assets/google.html:2757)
- the google fixture uses transitions such as box-shadow 250ms and transform/opacity/visibility .3s ease-in-out (examples/assets/google.html:597; examples/assets/google.html:1457)
- the graphite fixture transitions carousel images with transform .5s and rotates the open details marker 90deg (examples/assets/graphite.html:498-500; examples/assets/graphite.html:1243-1245)
- the gosub fixture transitions link border-bottom over 0.3s ease-in-out (examples/assets/gosub.html:49)
- the inline-flex-transform fixture transitions scale, translate, rotate and transform over 0.15s and scales to 1.2 on hover (examples/assets/inline-flex-transform.html:27-28; examples/assets/inline-flex-transform.html:37-38)
- observed absent — a prefers-reduced-motion rule · searched: `prefers-reduced-motion` over the 21 s02 files

## design-system §Iconography
- Font Awesome icon classes are used in the gosub, pseudo and servo-new fixtures (examples/assets/gosub.html:107-109; examples/assets/pseudo.html:71-73; examples/assets/servo-new.html:206-218)
- the pseudo fixture defines .fa-github:before and .gh:before with content "\f09b" (examples/assets/pseudo.html:55-65)
- the google fixture draws its icons as inline SVG paths in 24-unit viewBoxes (examples/assets/google.html:3086-3095; examples/assets/google.html:3159-3161; examples/assets/google.html:3219-3221)
- the graphite fixture positions icons from one sprite atlas through an --atlas-index custom property (examples/assets/graphite.html:661-666; examples/assets/graphite.html:748-753; examples/assets/graphite.html:1948)
- the graphite fixture embeds SVG icons as data URIs in CSS (examples/assets/graphite.html:1229; examples/assets/graphite.html:1627)

## design-system §Surface: none observed
- observed absent — a UI surface of this repository's own product; every listed file is a standalone example HTML document · searched: `<html` over the 21 s02 files

## layout-templates §Surface: none observed
- observed absent — a layout template of this repository's own product; every listed file is a standalone example HTML document · searched: `<html` over the 21 s02 files

## test-plan §Test Scope Summary
- the slice is a set of HTML example documents, each built around one rendering feature or a real page (examples/assets/hr.html:192; examples/assets/inline-backgrounds.html:57; examples/assets/iframe_navigation.html:29; examples/assets/object_fit.html:12)

## test-plan §Test Strategy
- fixture prose states the expected rendering for a human viewer: frame navigation replaces only the sub-document (examples/assets/iframe_navigation.html:33-36; examples/assets/iframe_navigation.html:58-61)
- fixture comments state what a case checks: correct alpha with no double-paint, and per-run ascent/descent (examples/assets/inline-backgrounds.html:40; examples/assets/inline-backgrounds.html:47; examples/assets/inline-backgrounds.html:84-86)
- the hr fixture's first case is left unstyled to show the UA stylesheet only (examples/assets/hr.html:33-36; examples/assets/hr.html:195)
- full real-page snapshots are paired with reduced variants (examples/assets/google_reduced.html:1-9; examples/assets/gosub_reduced.html:1-12; examples/assets/servo-new-reduced.html:1-51; examples/assets/servo-new-reduced-1.html:1-31)

## test-plan §Test Harness Contract
- out of slice — no harness, runner or test code is in the slice

## test-plan §Unit Test Strategy
- out of slice — no unit test code is in the slice

## test-plan §Integration Test Strategy
- out of slice — no integration test code is in the slice

## test-plan §E2E Test Strategy
- the iframe fixtures carry click-through instructions and expected outcomes instead of assertions (examples/assets/iframe_navigation.html:33-36; examples/assets/iframe_navigation.html:42-45; examples/assets/iframe_navigation.html:67-70)
- observed absent — automated assertions · searched: `\bassert|expect\(` over the 21 s02 files

## test-plan §Test Data & Fixtures
- page snapshots name their origins: google.com, graphite.art and servo.org (examples/assets/google.html:6; examples/assets/graphite.html:7; examples/assets/servo-new.html:18)
- fixtures reference sibling images square.png, wide.png, tall.png and gosub-logo.svg, which are not s02 files (examples/assets/object_fit.html:13-15; examples/assets/noscript.html:6; examples/assets/iframe_page_b.html:18; examples/assets/gosub.html:101)
- several fixtures load images and styles from remote hosts at render time (examples/assets/newservo.html:4; examples/assets/newservo.html:8; examples/assets/servo-new-reduced.html:16; examples/assets/graphite_software_overview.html:1641)
- the iframe page A fixture links to does_not_exist.html as a deliberate broken link (examples/assets/iframe_page_a.html:25)

## test-plan §Mocking & Stubbing Discipline
- out of slice — no mock or stub is in the slice

## test-plan §CI Integration
- out of slice — no CI configuration is in the slice

## obs-plan §Obs Scope Summary
- out of slice — no observability code is in the slice

## obs-plan §Telemetry Strategy
- out of slice — no telemetry configuration is in the slice

## obs-plan §Observability Harness Contract
- out of slice — no observability harness is in the slice

## obs-plan §Span / Trace Coverage
- out of slice — no tracing code is in the slice

## obs-plan §Metric Coverage
- out of slice — no metric code is in the slice

## obs-plan §Log Coverage
- observed absent — console logging calls · searched: `console\.` over the 21 s02 files

## obs-plan §Error Capture & Reporting
- observed absent — an error-reporting client · searched: `sentry|analytics|gtag` over the 21 s02 files

## obs-plan §PII Scrubbing & Compliance
- out of slice — no PII handling code is in the slice

## obs-plan §CI Integration
- out of slice — no CI configuration is in the slice

## a11y-plan §A11y Scope Summary
- the input fixture exercises focusable divs with tabindex 0 and tabindex -1 beside a text input (examples/assets/input.html:4-9)

## a11y-plan §A11y Strategy
- out of slice — no accessibility strategy is stated in the 21 HTML documents

## a11y-plan §A11y Assertion Harness Contract
- out of slice — no accessibility assertion code is in the slice

## a11y-plan §ARIA Patterns & Roles
- the google fixture's query textarea is a combobox with aria-autocomplete=both, aria-controls/aria-owns Alh6id and aria-haspopup=both (examples/assets/google.html:3184-3199)
- the google fixture uses role=listbox and role=option for suggestions, and role=menu, menuitem and separator for its settings menu (examples/assets/google.html:3294; examples/assets/google.html:3318; examples/assets/google.html:3667; examples/assets/google.html:3680; examples/assets/google.html:3753)
- the google fixture marks landmark regions with role=navigation, role=search and role=contentinfo (examples/assets/google.html:3027; examples/assets/google.html:3134; examples/assets/google.html:3583)
- the servo-new fixture's hamburger button carries aria-label and aria-expanded, which its script flips on click (examples/assets/servo-new.html:33; examples/assets/servo-new.html:232-239)
- the servo-new fixture labels nav and sections with aria-label (examples/assets/servo-new.html:25; examples/assets/servo-new.html:244; examples/assets/servo-new.html:300; examples/assets/servo-new.html:338; examples/assets/servo-new.html:348)
- the graphite fixture's carousel buttons carry aria-label text (examples/assets/graphite.html:1892; examples/assets/graphite.html:1898; examples/assets/graphite.html:1899)

## a11y-plan §Keyboard Navigation
- the google fixture gives non-button elements role=button with tabindex=0 (examples/assets/google.html:3083-3084; examples/assets/google.html:3211-3214; examples/assets/google.html:3237-3238)
- the google fixture binds keydown handlers through jsaction (examples/assets/google.html:3171; examples/assets/google.html:3653)
- the google fixture styles :focus-visible with a 1px solid outline (examples/assets/google.html:469-474)
- the graphite fixture removes the outline on newsletter inputs and marks focus by border-color var(--input-focus-color) (examples/assets/graphite.html:342; examples/assets/graphite.html:347-349; examples/assets/graphite.html:358-361)

## a11y-plan §Visual Design Verification
- the google fixture switches focus outlines to currentcolor under @media (forced-colors:active) (examples/assets/google.html:475-479)
- observed absent — prefers-reduced-motion and prefers-color-scheme rules · searched: `prefers-reduced-motion|prefers-color-scheme` over the 21 s02 files

## a11y-plan §Screen Reader Support
- the google fixture's links carry descriptive aria-labels such as "Gmail (opens a new tab)" (examples/assets/google.html:3052; examples/assets/google.html:3061)
- the google fixture hides decoration with aria-hidden and uses aria-atomic on suggestion options (examples/assets/google.html:3339; examples/assets/google.html:3318)
- the servo-new fixture hides the burger's bar spans with aria-hidden (examples/assets/servo-new.html:34-36)
- 11 fixtures set an html lang attribute (examples/assets/google.html:2; examples/assets/gosub.html:3; examples/assets/gosub_reduced.html:2; examples/assets/graphite.html:2; examples/assets/graphite_blog_section.html:2; examples/assets/graphite_software_overview.html:2; examples/assets/newservo.html:2; examples/assets/pseudo.html:2; examples/assets/servo-new-reduced-1.html:2; examples/assets/servo-new-reduced.html:2; examples/assets/servo-new.html:2)
- observed absent — an html lang attribute in the other 10 fixtures · searched: `lang=` over the 21 s02 files

## a11y-plan §Cognitive Accessibility
- out of slice — no cognitive accessibility provision is stated in the 21 HTML documents

## a11y-plan §CI Integration
- out of slice — no CI configuration is in the slice
