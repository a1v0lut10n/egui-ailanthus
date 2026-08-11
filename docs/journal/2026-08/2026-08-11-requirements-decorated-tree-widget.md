---
date: 2026-08-11
type: journal
components: []
aspects: []
meta-impact:
  note: "New repo egui-ailanthus created; needs a repos.yaml entry (related_to: aicogito, aiquill-workspace)."
---

# Requirements captured for the egui-ailanthus decorated tree widget

- **When:** 2026-08-11 18:24:35 local

## Context

aicogito and aiquill-workspace (the `ailoci` crate) both render tree navigation
with `egui_ltreeview = "0.8"`, which pins egui 0.35. aicogito's egui 0.36
upgrade is blocked on exactly this pin
(`aicogito/docs/implementation/backlog/2026-08-07-sum-0042-egui-0-36-upgrade.md`
lists egui_ltreeview as the ❌ blocker; upstream has had no activity since
2026-07-02). Beyond the version lock, both apps want richer node presentation
than egui_ltreeview offers out of the box: ailoci hand-paints folder/file
vector icons because emoji glyphs render as tofu boxes with the embedded IBM
Plex fonts, and aicogito's trees have no icons at all.

egui-ailanthus (repo: <https://github.com/a1v0lut10n/egui-ailanthus>, named
after *Ailanthus altissima*, the tree of heaven) is a new crate that replaces
egui_ltreeview in both apps.

## Details

In scope:

- **egui version control.** The crate builds against egui 0.36.x from day one,
  and its release cadence tracks egui minors so downstream apps are never
  version-locked by the tree widget again.
- **Decorated nodes.** Leaf nodes can carry an image/icon of what they
  represent (PDF icon for a PDF file, HTML icon for an HTML file, Rust icon
  for Rust source, crate icon for a Rust crate). Directory-like nodes get
  open/closed variants (open folder, closed folder, open crate, closed
  crate). Decoration uses egui's direct (painter-level) rendering so trees can
  be decorated more imaginatively than IntelliJ / VS Code tree views — badges,
  overlays, status marks, custom row painting.
- **Feature parity with egui_ltreeview**: drag & drop (internal moves and
  external drops), multi-selection with keyboard modifiers, keyboard
  navigation, activation (Enter / double-click), per-node and fallback context
  menus, state persistence, clip-rect culling for large trees.
- **Cross-platform**: macOS, Windows, Linux, and WebAssembly.
- **API ergonomics as the primary design goal.** Controlling node
  presentation must be easy; lessons from the current call sites (see below)
  feed the design directly.

Derived requirements from surveying the three existing call sites
(aicogito `views/projects.rs` + `views/knowledge.rs`, ailoci `src/lib.rs`):

- Action payloads should say whether a node is a directory or leaf — today all
  three call sites re-derive this with `is_file()` syscalls or trailing-slash
  id sentinels.
- Programmatic expand/reveal must actually work (`expand_node` /
  `expand_parents_of` are unimplemented stubs in egui_ltreeview 0.8).
- Lazy child loading should be a supported pattern, not a borrow-checker dance
  (ailoci defers loads through a `pending: Vec<PathBuf>` drained after
  `show()`).
- Icons must not depend on font glyph coverage (the tofu-box problem); the
  built-in icon set is painted or image-based.
- The icon slot must be queryable (ailoci guesses `ICON_SIZE = 11.0` against
  the reserved rect).

Out of scope for now: virtualized (index-based) rendering beyond clip-rect
culling; filesystem watching (stays app-side); tables/multi-column trees.

## Links

- Prior art: `egui_ltreeview` (MIT, Leonard Schüngel) — sibling checkout
  `../egui_ltreeview`, currently 0.8.1-dev on egui 0.35.
- Blocker record: `aicogito/docs/implementation/backlog/2026-08-07-sum-0042-egui-0-36-upgrade.md`.
- ailoci tree adoption journal:
  `aiquill-workspace/docs/journal/2026-07-18-23-44-03-execution-ltreeview-workspace-navigation.md`.
- Consumer call sites: `aicogito/crates/aicogito-app/src/views/projects.rs`,
  `.../views/knowledge.rs`, `aiquill-workspace/ailoci/src/lib.rs`.
