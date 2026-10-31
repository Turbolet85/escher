# Classified — design · spacing.b1

## Answers
spacing#1.1 · no · rdme GitHub markdown spacing tokens in apps/readme, an app's own stylesheet with no engine default (Not needed)
spacing#2.1 · no · browser urlbar, tab and menu padding is app chrome the list does not touch (Not needed)
spacing#2.2 · no · about:history page padding and list item spacing, an app page (Not needed)
spacing#2.3 · no · rdme markdown body max-width and padding, an app stylesheet (Not needed)
spacing#2.4 · no · seven_guis card and Home padding held in layout-templates; the list reads the select's client rect, not the card's padding
spacing#2.5 · no · body margin 8px is another unit of default.css the change does not edit, a re-check every change owes; the list is not the body
spacing#2.6 · yes · `multiple` select row padding 0 2px is named by the scope: the list's values sit beside these rows and must not move what they paint
spacing#3.1 · no · hspace and vspace on embed, img, object and marquee in stylo.rs, which is not reached and not a select
spacing#3.2 · no · body marginwidth and leftmargin mapping in stylo.rs, not reached and not a select
spacing#3.3 · yes · overlay scrollbar thumb geometry: the long list is a new nested scroll box painted with it, so the list's padding and border must keep the thumb inside
spacing#3.4 · no · single-line text input vertical centring applies to text inputs only; the closed select and the list rows are not text inputs
spacing#3.5 · no · outside list-marker padding applies to list-item boxes; the open list's options paint no marker
spacing#3.6 · no · double text decoration spacing, and no text decoration is named on the list's rows
spacing#4.1 · no · example fixture gaps and body padding, a demonstration the new engine default does not read (Not needed)
spacing#4.2 · no · graphite example fixture values, a demonstration the new engine default does not read (Not needed)
spacing#4.3 · no · hr example fixture margin and body padding, a demonstration the new engine default does not read (Not needed)
spacing#4.4 · no · servo.css example fixture column gap and button padding, a demonstration the new engine default does not read (Not needed)
spacing#4.5 · no · Preact TodoMVC example max-width and list padding, a demonstration the new engine default does not read (Not needed)
