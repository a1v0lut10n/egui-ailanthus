---
date: 2026-08-11
type: task
status: done
affects:
  - docs/architecture/components/tree-view.md
components: [tree-view, examples]
aspects: [wasm-compatibility]
tags: [drag-drop, context-menu, persistence]
design:
  - docs/design/2026-08-11-decoration-first-api.md
---

# Feature parity: drag & drop, context menus, persistence

## Objective

Close the remaining feature gap with egui_ltreeview 0.8: internal drag & drop
with drop-marker veto, external drag/drop actions, per-node and fallback
context menus, and `serde`-feature state persistence.

## Context

The core widget (previous task) ships without these; the aicogito/ailoci
migrations don't need them on day one (survey 2026-08-11: all three call
sites leave drag & drop and context menus unused), but parity is a stated
requirement before egui_ltreeview can be considered fully replaced.

## Deliverables

1. Drag & drop: multi-node drag with simplified source set, drag overlay
   layer, quarter-based drop position resolution (`Before`/`After`/`First`/
   `Last`), `Action::Move` / `Action::Drag` with veto via drop-marker
   removal, external-drop actions.
2. Context menus: per-node closure + tree-wide fallback (selection /
   single-node / empty-space variants).
3. `persistence` cargo feature persisting `TreeViewState` (openness +
   selection), consistent with programmatic expansion and
   `DirOpened`/`DirClosed` actions.
4. Examples covering each feature. (2026-08-12 scope change, agreed: the
   playground example moves to the CI/wasm-demo task; in its place this task
   gained per-row AccessKit nodes, headless `egui_kittest` interaction
   tests, and egui-MCP inspection wiring for the examples.)

## Explicitly out of scope

- Virtualized rendering; multi-column trees.

## Completion

Before setting `status: done`, update `docs/architecture/components/tree-view.md`
to describe the implemented interaction model, then write a linking journal
entry.
