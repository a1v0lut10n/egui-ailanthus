---
date: 2026-08-11
type: journal
components: [tree-view, decoration, examples]
aspects: [egui-version-tracking, font-independence, wasm-compatibility]
tasks:
  - docs/tasks/2026-08/2026-08-11-core-tree-and-decoration-api.md
design:
  - docs/design/2026-08-11-fresh-crate-tracking-egui-0-36.md
  - docs/design/2026-08-11-decoration-first-api.md
---

# Core tree widget and decoration API implemented on egui 0.36

- **When:** 2026-08-11 18:55:00 local

## Context

Same-day follow-through on the captured requirements: the repo was connected
to <https://github.com/a1v0lut10n/egui-ailanthus>, the documentation workflow
bootstrapped, and the core of the crate built against egui 0.36.1.

## Details

Built and verified:

- Crate scaffold: `egui_ailanthus`, edition 2024, `egui = "0.36"` as the only
  mandatory dependency, dual MIT/Apache-2.0, README crediting egui_ltreeview
  (MIT) as prior art.
- `tree-view`: immediate-mode `TreeBuilder` (`dir`/`leaf`/`close_dir`),
  caller-owned or memory-backed `TreeViewState<Id>`, whole-widget interaction
  resolved against recorded row geometry, clip-rect culling, animated closer,
  stripes/hover/selection painting, keyboard navigation (arrows, Enter) with
  focus-lock filter, multi-selection with toggle/range modifiers.
- Actions with kind-carrying payloads (`NodeInfo { id, is_dir }`):
  `SetSelected`, `Activate`, `DirOpened`, `DirClosed` — the last two make
  lazy loading event-driven (no borrow-checker dance).
- Working programmatic state: `expand`, `collapse`, `expand_parents_of`,
  `reveal` (ancestor chain discovered by the next build pass), `scroll_to`.
- `decoration`: painted `Icon` set (folder/crate open+closed, file, Rust,
  PDF, HTML, Markdown, image), `IconSource::{Painted, Image, Custom}` with
  `IconContext` metrics, badge overlays, trailing slots, row-paint hook.
- Examples: `simple`, `decorated_file_tree`, `lazy_loading`.

Verification: `cargo test` (4 headless smoke tests via `egui::__run_test_ui`
covering render, openness, reveal, expand-parents + 1 doctest) passes;
`cargo clippy --all-targets` clean; lib compiles for
`wasm32-unknown-unknown`; `aivolution lint` clean (8 files). Caveat: this
machine is headless — the examples compile but have not yet been run on a
display, so visual polish of the painted icons is unreviewed.

Noteworthy: eframe 0.36 replaced `App::update(ctx, …)` with
`App::ui(&mut Ui, …)` and egui 0.36 folded `SidePanel`/`TopBottomPanel` into
`egui::Panel` — relevant for the aicogito/ailoci 0.36 migrations, not just
for this crate's examples.

## Links

- Requirements: `docs/journal/2026-08/2026-08-11-requirements-decorated-tree-widget.md`
- Follow-up tasks: `docs/tasks/2026-08/2026-08-11-parity-dnd-context-menus-persistence.md`,
  `docs/tasks/2026-08/2026-08-11-ci-wasm-demo.md`
- Component docs: `docs/architecture/components/tree-view.md`,
  `docs/architecture/components/decoration.md`
