# egui-ailanthus

[![CI](https://github.com/a1v0lut10n/egui-ailanthus/actions/workflows/ci.yml/badge.svg)](https://github.com/a1v0lut10n/egui-ailanthus/actions/workflows/ci.yml)

A decorated tree view widget for [egui](https://github.com/emilk/egui), named
after *Ailanthus altissima* — the tree of heaven.

**Playground demo** (`demo/`) — every setting and decoration slot, with a
live action log. Natively: `cargo run -p egui_ailanthus_demo`; on the web:
`trunk serve` in `demo/` (CI builds the wasm bundle as an artifact; the
GitHub Pages deploy at <https://a1v0lut10n.github.io/egui-ailanthus/> switches
on once the repo is public — set the `DEPLOY_PAGES` repo variable to `true`).

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

## Testing and inspection

Interaction is covered by headless [`egui_kittest`](https://crates.io/crates/egui_kittest)
tests (`tests/interaction.rs`): every visible row registers an AccessKit node
(role `TreeItem`, label, bounds, expanded/selected), so tests — and assistive
technology — can find and click rows by label.

The same AccessKit nodes make the tree drivable through the
[egui MCP server](https://github.com/rerun-io/kittest_inspector). With
`egui-mcp` installed (`cargo install --git
https://github.com/rerun-io/kittest_inspector egui_mcp`) and this repo's
`.mcp.json`:

```sh
EGUI_INSPECTION=1 cargo run --example drag_drop
```

then `attach` from the MCP client and use `query_tree` / `click` / `drag` /
`screenshot`. Note: the inspection server needs a visible window (it is not
headless), and an `egui-mcp` binary built against an older egui may mismatch
on the wire format — reinstall it when in doubt.

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
