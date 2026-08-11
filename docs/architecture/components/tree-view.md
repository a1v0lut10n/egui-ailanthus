---
type: architecture-component
last-verified: 2026-08-12
decisions:
  - docs/design/2026-08-11-fresh-crate-tracking-egui-0-36.md
  - docs/design/2026-08-11-decoration-first-api.md
aspects: [egui-version-tracking, wasm-compatibility, accessibility]
---

# tree-view

The core widget: `TreeView` in `src/lib.rs`, the per-frame `TreeBuilder` in
`src/builder.rs`, the between-frames `TreeViewState<Id>` in `src/state.rs`,
and the `Action`/`NodeInfo` payloads in `src/action.rs`.

## Model

Immediate mode: each frame the caller constructs `TreeView::new(id)` and calls
`show_state(ui, &mut state, build_fn)` (or `show`, which keeps the state in
egui memory). The build closure walks the caller's own data and emits nodes
via `TreeBuilder::{dir, leaf, close_dir}`; the widget stores no node data —
only openness, selection, pivot, and keyboard cursor, keyed by caller ids
(`NodeId` is blanket-implemented for `Clone + Eq + Hash + Debug`).

## Build pass

`TreeBuilder` allocates one row per structurally visible node
(`ui.allocate_space`, zero vertical item spacing), records row geometry
(`Row`: rect, depth, closer rect, openness) for the input pass, and paints
immediately — stripe/hover/selection background, closer triangle (animated
via `ctx.animate_bool`), icon slot (always reserved so labels align), label
galley, trailing child `Ui`, and the caller's row-paint hook. Rows outside
`ui.clip_rect()` record geometry but skip painting (clip-rect culling).
Collapsed branches are skipped cheaply inside the builder; callers emit the
full tree each frame.

## Input pass

After the build closure, one whole-widget `ui.interact`
(`Sense::click_and_drag`) covers the union of row rects. Clicks resolve to a
row by position: closer clicks toggle openness only; row clicks select
(single, or toggle/range with the configured modifiers); clicking a dir row
with no modifier also toggles it; double-click activates leaves. Keyboard
(when focused, with a focus-lock filter): arrows navigate/collapse/expand
(left jumps to parent), Enter activates. All state mutations happen here,
after the build pass, and are reported as `Action`s: `SetSelected`,
`Activate`, `DirOpened`, `DirClosed`, `Drag`, `Move`, `MoveExternal` — node
payloads are `NodeInfo { id, is_dir }`, so callers never re-derive
dir-vs-leaf.

## Drag & drop

The drag source row is resolved at the pointer's *press origin* (by the time
`drag_started` fires the pointer has moved). Dragging a selected row drags
the whole selection, simplified (descendants of dragged dirs removed); an
unselected row drags alone. Past a 6 px threshold the drag activates: dragged
rows fade in place and ghosts follow the pointer on the tooltip layer. The
drop position is quarter-based (`Before`/`After` on edges, `First`/`Last`
into dirs honoring `Node::drop_allowed`), with self-and-descendant drops
rejected by an ancestor climb. Each frame a marker shape (line or dir
outline) is reserved via `ShapeIdx`; `Action::Drag` exposes it so the app can
veto via `DragAndDrop::remove_marker`. Release emits `Action::Move` (or
`MoveExternal` outside the tree); the application applies the move to its own
model (`examples/drag_drop.rs`).

## Context menus

`Node::context_menu` closures are retained by the build pass (builder
lifetime `'nodes`) so the input pass can render them same-frame: right-click
selects the row (if unselected), records `ContextMenuState`, and opens an
`egui::Popup` (kind `Menu`, anchored at the click position, close-on-click).
Rows without a menu fall back to `TreeView::fallback_context_menu`, which
also serves right-clicks on empty space.

## Persistence

The `persistence` cargo feature derives serde on `TreeViewState` (openness +
selection; transient drag/reveal state skipped) and switches
`TreeView::show`'s memory backing from `get_temp` to `get_persisted`, which
adds serde bounds to node ids (`examples/persistence.rs`).

## Accessibility

See the `accessibility` aspect: the build pass registers an AccessKit node
per visible row and marks the container `Role::Tree`; interaction tests in
`tests/interaction.rs` drive the widget through those nodes with
`egui_kittest`.
