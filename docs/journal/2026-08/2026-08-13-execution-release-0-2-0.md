---
date: 2026-08-13
type: journal
components: [decoration]
aspects: [egui-version-tracking]
meta-impact:
  note: "egui_ailanthus 0.2.0 (egui 0.36) supersedes 0.1.0 on crates.io — downstream migrations (aicogito, aiquill-workspace) should adopt 0.2 directly and skip 0.1; Icon is #[non_exhaustive] from 0.2.0 on."
---

# egui_ailanthus 0.2.0 released

- **When:** 2026-08-13

## Context

The crates.io-inspired icons and the `#[non_exhaustive]` `Icon` enum (see the
same-day icons journal entry) are API changes; the enum attribute is
breaking, so the release is 0.2.0 rather than 0.1.1.

## Details

- Version bump + changelog stamp (`ca44014`), `cargo publish` clean, tag
  `v0.2.0` pushed, GitHub release created, CI green on the release commit,
  crates.io registry confirmed serving 0.2.0 as max version.
- No egui bump — still 0.36; this was the checklist's patch-release path
  (steps 3, 5, 6, 7) with a minor bump for the breaking attribute.

## Links

- Changes: `docs/journal/2026-08/2026-08-13-execution-cratesio-icons-and-wash.md`
- Release: <https://github.com/a1v0lut10n/egui-ailanthus/releases/tag/v0.2.0>
- Crate: <https://crates.io/crates/egui_ailanthus>
