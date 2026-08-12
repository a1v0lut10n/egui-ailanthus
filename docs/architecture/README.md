# Architecture — egui-ailanthus

Index and **controlled vocabulary** for egui-ailanthus. The names listed here
are canonical: any `components:` / `aspects:` field in this repo's documents
must use them verbatim. Component and aspect docs live under
[`components/`](components/) and [`aspects/`](aspects/) and are written on
demand — a name can exist in the vocabulary before its doc does.

The shared documentation workflow (taxonomy, schemas, mutation patterns) is
owned by aivolution-meta's `docs/README.md`; this repo's docs follow it.

## Components

| Name              | What it is |
|-------------------|------------|
| `tree-view`       | The core `TreeView` widget: builder API, id-keyed state, row layout, input handling, actions. |
| `decoration`      | The decoration system: built-in painted `Icon` set, badge overlays, trailing slots, row-paint hooks, custom icon sources. |
| `examples`        | The `examples/` binaries demonstrating each feature (also manual test surfaces and the egui-MCP inspection targets), plus the `demo/` playground crate — the trunk-built wasm demo deployed to GitHub Pages by CI. |

## Aspects

| Name                    | Invariant (summary) |
|-------------------------|---------------------|
| `egui-version-tracking` | The crate builds against the latest stable egui minor; a new egui minor triggers an egui-ailanthus release. `egui` is the only mandatory dependency. |
| `font-independence`     | No built-in visual relies on font glyph coverage (no emoji/tofu risk); built-in icons are painted with epaint primitives. |
| `wasm-compatibility`    | Everything in the crate (and every example) compiles and runs on wasm32 as well as macOS/Windows/Linux. |
| `accessibility`         | Every visible row registers an AccessKit node (role, label, bounds, state), so assistive tech, egui_kittest, and the egui MCP inspection server can see and target rows. |

## Litmus test

If you can name the module/crate that implements it, it's a **component**; if
it's a rule that survives replacing any single component, it's an **aspect**.
