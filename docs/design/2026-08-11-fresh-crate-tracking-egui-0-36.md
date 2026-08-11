---
date: 2026-08-11
type: design
status: accepted
components: []
aspects: []
tags: [egui, versioning, licensing]
---

# egui-ailanthus is a fresh crate that tracks the latest stable egui

## Context

aicogito and ailoci (aiquill-workspace) render their trees with
`egui_ltreeview 0.8`, which pins egui 0.35. Upstream egui is at 0.36.1 and
egui_ltreeview has had no commits since 2026-07-02; aicogito's egui 0.36
upgrade is formally blocked on it. We control neither upstream's release
cadence nor its API direction, and we want decoration features upstream does
not have. The options:

1. **Wait for upstream** to release against egui 0.36 — already rejected in
   aicogito's blocker doc after a month of no movement; leaves the decoration
   requirements unmet regardless.
2. **Fork egui_ltreeview** and patch the egui dependency — fastest unblock,
   but inherits an API we would then bend heavily for decorations, and every
   upstream divergence becomes merge debt in code we did not write.
3. **Fresh crate** informed by egui_ltreeview's architecture (MIT-licensed, so
   studying and borrowing patterns with attribution is clean), designed around
   decoration and API ergonomics from the start.

## Decision

Build `egui_ailanthus` as a fresh implementation in the
`a1v0lut10n/egui-ailanthus` repo.

- **egui version policy:** the crate depends on the latest stable egui minor
  (0.36.x now). Each egui minor release gets an egui-ailanthus release; the
  crate's own minor version bumps in lockstep. `egui` is the only mandatory
  dependency; everything else (serde, image loaders) is feature-gated.
- **Prior art, not a fork:** egui_ltreeview's proven mechanics — immediate-mode
  builder walking caller-owned data, id-keyed open/selection state, deferred
  action list, whole-widget interact + per-row painter drawing, clip-rect
  culling — are adopted as architecture. Its code is not copied wholesale; the
  README credits it as prior art.
- **Rust edition 2024**, matching aiquill-workspace.
- **Dual license MIT OR Apache-2.0**, the egui-ecosystem norm, since the crate
  is intended for crates.io publication.

## Consequences

- aicogito and ailoci can upgrade to egui 0.36 with egui-ailanthus as the tree
  widget; the class of "tree widget pins egui" blockers ends with us, because
  we own the release cadence.
- We take on maintenance of a non-trivial widget (input handling, drag & drop,
  keyboard navigation) instead of consuming it for free — the price of the
  decoration features and version control.
- Migration in the two apps is a small port, not a drop-in swap: the API is
  deliberately different where egui_ltreeview's shape caused the documented
  pain points (kind-less action payloads, stubbed expand APIs, lazy-loading
  borrow dances).
- Each egui minor becomes a scheduled maintenance event for this repo.
