---
date: 2026-08-13
type: journal
components: [decoration, examples]
aspects: [font-independence]
---

# crates.io-inspired icons; row-paint showcase switched to a wash

- **When:** 2026-08-13

## Context

User feedback on the live demo: the archive row's `row_paint` underline read
as a stray divider above the row below it. And crates.io's source view
(seen while browsing the freshly published crate) has distinct icons for
Cargo.toml, Cargo.lock, README, and JSON — worth adopting.

## Details

- **Four new painted icons** in the existing page-plus-overlay style:
  `Icon::FileCargo` (miniature open crate, reusing the crate painter's
  tones), `Icon::FileLock` (padlock: stroked-circle shackle whose lower half
  the body covers), `Icon::FileReadme` (open book: two convex panels meeting
  at a spine), `Icon::FileJson` (`{}` glyphs — ASCII, within the
  font-independence rule). Verified visually via the egui MCP inspection
  loop, zoomed screenshots at the smallest icon-size setting; the book and
  mini-crate needed one sizing iteration to stay inside the page.
- **`Icon` is now `#[non_exhaustive]`** so the set can keep growing without
  semver breaks; this change itself is breaking → next release is 0.2.0.
- **Row-paint showcases** (demo archive dir, `decorated_file_tree` example)
  switched from a bottom-edge underline to a translucent background wash —
  an underline at the row boundary reads as a divider belonging to the
  neighbor row.
- The demo crate now enables eframe's `inspection` feature on native, so the
  playground itself is MCP-drivable (`EGUI_INSPECTION=1`).

## Links

- Feedback origin: live demo <https://a1v0lut10n.github.io/egui-ailanthus/>
- Prior milestone: `docs/journal/2026-08/2026-08-13-execution-first-release.md`
