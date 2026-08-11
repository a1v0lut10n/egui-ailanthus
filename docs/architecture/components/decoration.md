---
type: architecture-component
last-verified: 2026-08-11
decisions: [docs/design/2026-08-11-fresh-crate-tracking-egui-0-36.md]
aspects: [font-independence, wasm-compatibility]
---

# decoration

The decoration system: `src/icon.rs` (icon sources and the painted icon set)
and the per-node decoration attributes on `Node` in `src/node.rs`.

## Slots

Each node row has four decoration points, set fluently on `Node`:

- **Icon** (`Node::icon`): an `IconSpec` — a closed/only source plus an
  optional open-directory variant (`Icon::folder()` and `Icon::rust_crate()`
  pair these automatically). The slot is always reserved so labels align.
- **Badge** (`Node::badge`): a status dot overlaid on the icon's lower-right
  corner, ringed with the panel fill so it reads on any background.
- **Trailing** (`Node::trailing`): arbitrary widgets in a right-to-left child
  `Ui` at the row's right edge (sizes, counts, buttons).
- **Row paint** (`Node::row_paint`): a painter hook over the row background,
  receiving a `RowContext` (rect, depth, dir/open/selected/hovered) — the
  escape hatch for arbitrary direct-rendered decoration.

## Icon sources

`IconSource` is one of:

- `Painted(Icon)` — the built-in set (folder open/closed, crate open/closed,
  generic file page, Rust, PDF, HTML, Markdown, image), drawn with epaint
  primitives in `paint_icon`. Colors are theme-toned (`dark_mode`-aware);
  page-style icons use single ASCII letters or painted glyph shapes, never
  emoji — see the `font-independence` aspect.
- `Image(egui::ImageSource)` — any egui image; the caller is responsible for
  installing image loaders.
- `Custom(IconPainter)` — a closure receiving `&mut Ui` and an `IconContext`
  with the exact slot rect and row state, so custom painters never guess
  metrics.

The directory closer triangle (`paint_closer`) is also painted, animated by
openness.
