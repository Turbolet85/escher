### input-validation#1.69 · values · input · ## Input Validation · lines 249–251 · stale
- **Boundary:** Layout values
  - **What to validate:** Text extent (`BaseDocument::text_extent`)
  - **How (stack-specific):** Since 2026-10-19-visual-lint a read-only reader on every document, remote pages included: an id that does not resolve, a node that is no inline root or one with no inline layout data reads `None` through `get_node`, never an index; it mutates nothing and has no log site, print, env read or panic path; it returns geometry only, and its one non-test caller is `Session::lint`, in process and through the `lint` verb's arm (packages/blitz-dom/src/text_extent.rs:18-54)
