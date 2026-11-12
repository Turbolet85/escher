fct-bla7ziiz@1 · The snapshot reads the flight booker's mode select at boot as ComboBox "Flight type" with value one-way and two ListBoxOptions, each at the select's client rect.
fct-zk3a7kfe@1 · While a select list is open each row reads its own bounds in the snapshot and the diff, and after a close the rows read back at the select's client rect.
fct-3yebjmlo@1 · Every document starts with the fixed skeleton html, head, body and main id="main", and the app mounts into main.
