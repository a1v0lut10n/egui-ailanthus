---
date: 2026-08-11
type: design
status: accepted
components: []
aspects: []
tags: [api, decoration, icons]
---

# Decoration-first tree API

## Context

The primary design goal is an easy API for controlling node presentation. The
three existing call sites show what the API must make trivial and what it must
stop making painful:

- ailoci hand-paints folder/file vector icons into a guessed 11 px rect
  because emoji glyphs tofu with embedded fonts and egui_ltreeview offers no
  built-in icons — decorating a node should be one method call, not custom
  painter code.
- All three call sites re-derive dir-vs-leaf outside the widget (`is_file()`
  syscalls, trailing-slash id sentinels) because action payloads carry bare
  ids.
- ailoci defers lazy loading through a `pending: Vec<PathBuf>` drained after
  `show()` because the build closure borrows the model immutably.
- `expand_node` / `expand_parents_of` are stubs upstream, so "reveal file in
  tree" cannot be built.

## Decision

Immediate-mode builder API (walking caller-owned data each frame, id-keyed
widget state, deferred `Vec<Action>`), with decoration as a first-class node
attribute:

```rust
let (response, actions) = TreeView::new(id)
    .show_state(ui, &mut state, |tree| {
        tree.dir(Node::new("src").label("src").icon(Icon::FolderClosed.open(Icon::FolderOpen)));
        tree.leaf(Node::new("src/main.rs").label("main.rs").icon(Icon::FileRust));
        tree.leaf(Node::new("report.pdf").label("report.pdf")
            .icon(Icon::FilePdf)
            .badge(Badge::dot(Color32::ORANGE))       // overlay decoration
            .trailing(|ui| { ui.weak("2 MB"); }));    // right-aligned slot
        tree.close_dir();
    });
```

The load-bearing choices:

1. **Built-in icons are painted, not glyphs and not bundled images.** `Icon`
   is an enum (folder open/closed, crate open/closed, generic file, Rust
   source, PDF, HTML, Markdown, image, …) rendered with epaint primitives —
   theme-aware, crisp at any scale and DPI, immune to font coverage, zero
   extra dependencies, wasm-safe. Custom icons plug into the same slot as
   `egui::ImageSource` (user installs image loaders) or an arbitrary
   `FnMut(&mut Ui, IconContext)` painter; `IconContext` carries the slot rect,
   openness, selection, and hover so custom painters need not guess metrics.
2. **Decoration slots, not one icon closure**: leading icon (with automatic
   open/closed variant switching), badge overlays on the icon, trailing
   widgets right-aligned in the row, and an optional row-paint hook
   (background/underline behind the whole row) for the "more imaginative than
   IntelliJ" cases.
3. **Actions carry node kind.** Every action payload uses
   `NodeInfo { id, is_dir }` (captured at build time) instead of bare ids —
   no more `is_file()` round-trips in click handlers.
4. **Openness changes are actions.** `Action::DirOpened(id)` /
   `Action::DirClosed(id)` are emitted alongside `SetSelected` / `Activate` /
   `Move` / `Drag`, so lazy loaders react after `show()` returns — the
   supported pattern replaces the borrow-checker dance. A dir with unloaded
   children can render a spinner/placeholder via `Node::pending_children()`.
5. **Programmatic state works**: `TreeViewState::{expand, collapse,
   expand_parents_of, reveal, set_selected, scroll_to}` are implemented, not
   stubs — "reveal file in tree" is a supported one-liner.
6. **Parity features carried over** from egui_ltreeview: multi-selection with
   modifiers, keyboard navigation, activation, per-node + fallback context
   menus, internal & external drag/drop with drop-marker veto, `serde`
   persistence behind a feature, clip-rect culling for large trees.

## Consequences

- Migrating aicogito's two trees is mechanical (`builder.dir/leaf` →
  `tree.dir/leaf` plus dropping the dir-vs-file workarounds); ailoci
  additionally deletes its hand-painted icon code and pending-vec plumbing.
- The painted icon set is our art: each new file type is Rust painting code,
  not a dropped-in SVG. The `ImageSource` escape hatch covers long-tail types
  cheaply.
- Slots (icon, badge, trailing, row-paint) constrain layout enough to keep
  rows aligned and culling cheap, while the row-paint hook and custom painter
  keep the door open for arbitrary direct-rendering decoration.
- Emitting open/close actions makes openness observable app-side, which the
  persistence story must keep consistent with programmatic expansion.
