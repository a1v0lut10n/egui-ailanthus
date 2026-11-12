---
date: 2026-11-12
type: journal
components: [tree-view, decoration, examples]
aspects: [egui-version-tracking, wasm-compatibility]
meta-impact:
  note: "egui_ailanthus 0.3.0 (egui 0.37) is on crates.io; no API change of its own. Downstream egui-0.37 migrations should take `egui_ailanthus = \"0.3\"` from the registry rather than a git rev — aicogito did in SUM-0236; aiquill-workspace's ailoci still pins rev 64a6930."
---

# egui_ailanthus 0.3.0 released — tracks egui 0.37

- **When:** 2026-11-12 00:48:54 local

## Context

egui 0.37.0 shipped on 2026-10-15. Under the `egui-version-tracking`
aspect a new egui minor triggers a release here, and aicogito's own
egui 0.37 upgrade (SUM-0236) was blocked on it: 0.2.1's `egui = "0.36"`
requirement does not admit 0.37.

## Details

- Followed the release checklist's egui-minor path: `egui` 0.36 → 0.37,
  plus dev-deps `eframe` and `egui_kittest` and the demo crate.
  The library compiled clean. The only fallout was egui 0.37's deprecation of
  `Id::new`, which is now an alias for `Id::unique`. Docs, examples, tests
  and the demo moved to `Id::unique` (ids unchanged). `LayerId::new` is
  untouched, and `TreeView::new` still takes an `egui::Id`, so there is no
  public API change. Minor bump to 0.3.0 per the checklist; changelog
  and README egui version updated.
- Verified locally with `RUSTFLAGS=-D warnings`: fmt, clippy
  (all targets/features), 14 tests, and wasm32 checks for the crate
  (both feature sets) and the demo. CI was green on Linux, macOS,
  Windows and the trunk build. `cargo publish --dry-run` was clean.
- Released: PR #3 merged (`6004f7c`), published to crates.io
  2026-11-11, tag `v0.3.0` on the merge commit, GitHub release created
  (v0.2.1 had none, so GitHub's "Latest" was stale until now).
- Aspects held: `egui-version-tracking` (this release is the invariant
  at work), `wasm-compatibility` (wasm checks pass), `font-independence`
  and `accessibility` untouched.
- Not done: checklist step 4 (reinstall the `egui_mcp` bridge and
  re-attach). The reinstall is pending a manual run.
- Gotcha: the `cargo` on PATH is Homebrew's and does not see rustup
  targets; the wasm checks need
  `~/.rustup/toolchains/stable-aarch64-apple-darwin/bin` first on PATH.

## Links

- PR: https://github.com/a1v0lut10n/egui-ailanthus/pull/3
- Release: https://github.com/a1v0lut10n/egui-ailanthus/releases/tag/v0.3.0
- Crate: https://crates.io/crates/egui_ailanthus/0.3.0
- Previous release: `docs/journal/2026-08/2026-08-13-execution-release-0-2-0.md`
- Checklist: `docs/reference/release-checklist.md`
- Downstream: aicogito SUM-0236 (Aivolution-GmbH/aicogito PR #433)
