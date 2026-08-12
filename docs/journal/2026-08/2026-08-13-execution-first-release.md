---
date: 2026-08-13
type: journal
components: []
aspects: [egui-version-tracking, wasm-compatibility]
meta-impact:
  note: "egui_ailanthus 0.1.0 (egui 0.36) is on crates.io — aicogito and aiquill-workspace can plan their egui_ltreeview migrations; repos.yaml relationship already recorded."
---

# Repo public, Pages demo live, egui_ailanthus 0.1.0 on crates.io

- **When:** 2026-08-13

## Context

With all three planned tasks done and CI green, the user decided to make the
repository public and, once the Pages deploy was confirmed, publish the first
release to crates.io.

## Details

- **Repo made public** (`gh repo edit --visibility public`), GitHub Pages
  enabled with Source: GitHub Actions (`build_type=workflow`), repo variable
  `DEPLOY_PAGES=true` set — this flips on the deploy job that was gated while
  the repo was private.
- **Pages deploy verified:** re-ran the last main CI run; all jobs green
  including `deploy web demo to GitHub Pages`. The playground is live at
  <https://a1v0lut10n.github.io/egui-ailanthus/> (HTTP 200, trunk bundle with
  correct `/egui-ailanthus/` asset paths).
- **Pre-release docs:** README freshened (live demo link, status section now
  reflects finished parity work) and `CHANGELOG.md` added with the 0.1.0
  entry (commit `3d25de5`).
- **Published:** `cargo publish --dry-run` clean (15 files, ~51 KiB
  compressed; examples/tests excluded via `include`), then `cargo publish` —
  `egui_ailanthus 0.1.0` is live on crates.io, built against egui 0.36.
  Tagged `v0.1.0` and pushed; GitHub release created from the changelog.

## Links

- Release checklist followed: `docs/reference/release-checklist.md`
- Crate: <https://crates.io/crates/egui_ailanthus>
- Demo: <https://a1v0lut10n.github.io/egui-ailanthus/>
- Prior milestone: `docs/journal/2026-08/2026-08-12-execution-ci-wasm-demo.md`
