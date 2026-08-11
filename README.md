# egui-ailanthus

A decorated tree view widget for [egui](https://github.com/emilk/egui), named
after *Ailanthus altissima* — the tree of heaven.

`egui_ailanthus` renders trees whose nodes carry meaningful decoration: a PDF
icon for a PDF file, a Rust icon for Rust source, open/closed folder and crate
icons, badge overlays, trailing widgets, and custom row painting — all drawn
with egui's painter (no font-glyph or image-asset dependencies), so icons are
crisp at any scale, theme-aware, and identical across macOS, Windows, Linux,
and WebAssembly.

## Goals

- **Track the latest stable egui.** The crate builds against egui 0.36.x and
  releases in lockstep with egui minors, so downstream apps are never
  version-locked by their tree widget.
- **Decoration-first API.** Controlling node presentation is the point:
  built-in painted icons, `ImageSource` and custom-painter escape hatches,
  badges, trailing slots, and row-paint hooks.
- **Feature parity with `egui_ltreeview`**: multi-selection, keyboard
  navigation, activation, drag & drop, context menus, persistence, clip-rect
  culling for large trees.

```rust
let (_response, actions) = TreeView::new(egui::Id::new("tree"))
    .show_state(ui, &mut state, |tree| {
        tree.dir(Node::new("src").label("src").icon(Icon::folder()));
        tree.leaf(Node::new("src/main.rs").label("main.rs").icon(Icon::FileRust));
        tree.close_dir();
        tree.leaf(Node::new("report.pdf").label("report.pdf").icon(Icon::FilePdf));
    });
```

## Status

Early development. The core widget (rendering, selection, keyboard
navigation, decoration) is being built first; drag & drop, context menus, and
persistence follow. See `docs/tasks/` for the plan and `docs/design/` for the
decision records.

## Prior art

The architecture is informed by
[`egui_ltreeview`](https://github.com/LennysLounge/egui_ltreeview) by Leonard
Schüngel (MIT) — its immediate-mode builder, id-keyed state, and deferred
action model are proven patterns this crate adopts. egui-ailanthus is a fresh
implementation, not a fork.

## License

Dual-licensed under either of [Apache License 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.
