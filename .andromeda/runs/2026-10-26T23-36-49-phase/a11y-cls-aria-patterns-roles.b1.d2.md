# Classified — a11y · aria-patterns-roles.b1

## Answers
aria-patterns-roles#1.1 · yes · names accessibility_roles.rs:74-93, an a11y-leg check the chunk re-runs green; the lint reads each node's role from the Snapshot
aria-patterns-roles#1.2 · yes · names accessibility_roles.rs:157-180, an a11y-leg check the chunk re-runs green; the lint reads each node's role from the Snapshot
aria-patterns-roles#2.1 · yes · the ARIA role-attribute mapping behind the Snapshot's role the lint reads; the helpers read it and add no second role mapping
aria-patterns-roles#2.2 · yes · names accessibility_states.rs:134-316, a stand-repeat file the chunk re-runs three rounds; the lint reads no state, but the re-run still asserts these reads
aria-patterns-roles#2.3 · yes · names accessibility_roles.rs:128-180, an a11y-leg check the chunk re-runs green; the button role the lint reads through the Snapshot
aria-patterns-roles#2.4 · yes · names accessibility_roles.rs:117-126, an a11y-leg check the chunk re-runs green
aria-patterns-roles#2.5 · yes · names accessibility_roles.rs:128-155, an a11y-leg check the chunk re-runs green; the flight booker's select role the lint reads
aria-patterns-roles#2.6 · yes · names accessibility_roles.rs:95-115, an a11y-leg check the chunk re-runs green
aria-patterns-roles#2.7 · yes · names accessibility_roles.rs:128-180, an a11y-leg check the chunk re-runs green; the range and password roles the lint reads
aria-patterns-roles#2.8 · yes · names accessibility_roles.rs:157-180, an a11y-leg check the chunk re-runs green; the heading and paragraph roles the lint reads
aria-patterns-roles#2.9 · yes · display none, visibility hidden and inert stay out of the lint's walk, while a select's options stay in and read their select's client rect as bounds
aria-patterns-roles#3.1 · no · clickable divs in apps/browser; the chunk reaches no browser file and names no re-run check
aria-patterns-roles#3.2 · no · alt text of apps/browser images; the text-alternatives section is out of scope and no chunk file reaches the browser
aria-patterns-roles#3.3 · no · focus-state CSS in apps/readme; the chunk touches no readme file or focus style
aria-patterns-roles#4.1 · no · a markup fact of the examples/assets GitHub fixtures; no chunk file, Needs item or re-run check reads them
aria-patterns-roles#4.2 · no · a markup fact of the examples/assets Google fixture; no chunk file, Needs item or re-run check reads it
aria-patterns-roles#4.3 · no · a markup fact of the examples/assets Google fixture's combobox; no chunk file, Needs item or re-run check reads it
aria-patterns-roles#4.4 · no · a markup fact of the examples/assets Google fixture's listbox and menu; no chunk file, Needs item or re-run check reads it
aria-patterns-roles#4.5 · no · a markup fact of the examples/assets Google fixture's landmarks; no chunk file, Needs item or re-run check reads it
aria-patterns-roles#4.6 · no · a markup fact of the examples/assets servo-new fixture; no chunk file, Needs item or re-run check reads it
aria-patterns-roles#4.7 · no · a markup fact of the examples/assets servo-new fixture's labels; no chunk file, Needs item or re-run check reads it
aria-patterns-roles#4.8 · no · a markup fact of the examples/assets graphite fixture; no chunk file, Needs item or re-run check reads it
aria-patterns-roles#4.9 · no · a markup fact of the examples/assets servo.html fixture; no chunk file, Needs item or re-run check reads it
aria-patterns-roles#4.10 · no · a markup fact of the examples/assets servo.html download links; no chunk file, Needs item or re-run check reads it
aria-patterns-roles#4.11 · no · a markup fact of the examples/assets svg.html fixture; no chunk file, Needs item or re-run check reads it
