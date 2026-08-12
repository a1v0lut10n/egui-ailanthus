---
date: 2026-08-12
type: journal
components: [examples]
aspects: [wasm-compatibility, egui-version-tracking]
tasks:
  - docs/tasks/2026-08/2026-08-11-ci-wasm-demo.md
meta-impact:
  note: "Downstream docs/reference/ files have no generic schema (only meta-only reference-repo), so they lint as legacy; consider adding a generic reference schema to meta."
---

# CI matrix, wasm playground demo, and release checklist landed

- **When:** 2026-08-12 09:47:22 local

## Context

Third and last of the initially planned tasks: continuous verification of the
cross-platform requirement, plus the playground demo that moved here from the
parity task.

## Details

- **CI** (`.github/workflows/ci.yml`): fmt + clippy (`-D warnings` via
  RUSTFLAGS) + `cargo test --workspace --all-features` on
  ubuntu/macos/windows; a wasm job checking the crate (both feature sets) and
  the demo on `wasm32-unknown-unknown` and trunk-building the web bundle
  (uploaded as a Pages artifact). rustfmt was applied across the codebase in
  the same change; CI enforces it from now on. First run: green on all three
  OSes + wasm.
- **Playground demo** (`demo/`, workspace member, `publish = false`): every
  setting live (striped, multi-select, drag & drop, indent / row height /
  icon size sliders), reveal / expand-all / collapse-all buttons, all icon
  types incl. the crate icon, badges, trailing widgets, a row-paint
  decoration, per-node + fallback context menus, a read-only drop-veto dir,
  and an action log. One `eframe::App`, cfg-split `main` for native
  (`cargo run -p egui_ailanthus_demo`) and web (`trunk serve`); trunk release
  build verified locally.
- **Pages caveat:** the repo is currently private and this plan has no Pages
  for private repos — the deploy job is gated behind a `DEPLOY_PAGES` repo
  variable (documented in the workflow and README). Once the repo goes
  public: enable Pages (Source: GitHub Actions), set the variable, and
  <https://a1v0lut10n.github.io/egui-ailanthus/> goes live.
- **Release checklist** (`docs/reference/release-checklist.md`): the
  egui-minor tracking procedure — bump deps (kittest is version-locked to
  egui), changelog review, full CI set, egui-mcp reinstall (wire protocol has
  no version negotiation), version bump, publish, downstream notification.
  The file lints as `legacy` because meta has no generic reference schema —
  flagged via meta-impact.

## Links

- Task: `docs/tasks/2026-08/2026-08-11-ci-wasm-demo.md`
- Green run: <https://github.com/a1v0lut10n/egui-ailanthus/actions/runs/31574662346>
- Prior milestone: `docs/journal/2026-08/2026-08-12-execution-parity-accesskit-kittest.md`
