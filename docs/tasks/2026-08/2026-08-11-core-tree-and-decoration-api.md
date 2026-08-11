---
date: 2026-08-11
type: task
status: done
affects:
  - docs/architecture/README.md
  - docs/architecture/components/tree-view.md
  - docs/architecture/components/decoration.md
components: [tree-view, decoration, examples]
aspects: [egui-version-tracking, font-independence, wasm-compatibility]
tags: [scaffold, core, api]
design:
  - docs/design/2026-08-11-fresh-crate-tracking-egui-0-36.md
  - docs/design/2026-08-11-decoration-first-api.md
---

# Implement the core tree widget and decoration API

## Objective

A compiling, usable `egui_ailanthus` crate on egui 0.36 that renders a
decorated tree with open/close, selection, keyboard navigation, activation,
and the decoration slots (built-in painted icons, badges, trailing widgets,
row-paint hook), demonstrated by runnable examples.

## Context

Requirements and API shape are captured in the linked design records and the
2026-08-11 requirements journal entry. This task covers the core; drag & drop,
context menus, and persistence are follow-up tasks.

## Deliverables

1. Crate scaffold: `Cargo.toml` (egui 0.36, edition 2024, dual MIT/Apache),
   README crediting egui_ltreeview as prior art, CI-ready layout.
2. `tree-view` core: `TreeView` + `TreeViewState<Id>` + build closure
   (`dir`/`leaf`/`node`, `close_dir`), id-keyed openness/selection, row
   layout with indent + closer, clip-rect culling, whole-widget interaction.
3. Actions: `SetSelected`, `Activate`, `DirOpened`/`DirClosed`, payloads
   carrying `NodeInfo { id, is_dir }`.
4. Working programmatic state: `expand`, `collapse`, `expand_parents_of`,
   `reveal` (expand parents + select + scroll-to).
5. `decoration`: `Icon` enum with painted folder open/closed, crate
   open/closed, generic file, Rust, PDF, HTML, Markdown; `IconSource`
   escape hatches (`ImageSource`, custom painter with `IconContext`);
   badge overlay; trailing slot; row-paint hook.
6. Keyboard navigation (arrows, Enter, Space) and multi-selection with
   shift/command modifiers.
7. Examples: `simple`, `decorated_file_tree` (file-type icons + badges),
   `lazy_loading` (via `DirOpened` actions).
8. Architecture component docs for `tree-view` and `decoration` reflecting
   the implemented state.

## Explicitly out of scope

- Drag & drop (internal and external), context menus, serde persistence —
  follow-up task.
- CI workflows, wasm demo hosting — follow-up task.
- Migration of aicogito / ailoci call sites (happens in those repos).

## Completion

Before setting `status: done`, verify every path in `affects` reflects the
post-task state (architecture README vocabulary matches reality; both
component docs describe the implemented modules), then write a linking
journal entry.
