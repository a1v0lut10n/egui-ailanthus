# Release checklist

egui-ailanthus tracks the **latest stable egui minor** and releases in
lockstep (see `docs/design/2026-08-11-fresh-crate-tracking-egui-0-36.md` and
the `egui-version-tracking` aspect). A new egui minor is a scheduled
maintenance event for this repo.

## On a new egui minor (0.3X)

1. Bump `egui` in `Cargo.toml`, plus dev-deps `eframe` and `egui_kittest`
   (kittest is version-locked to egui) and the demo crate's `egui`/`eframe`.
2. Read the egui/eframe changelogs for API breaks (0.36 precedent: eframe
   `App::update` → `App::ui`, `SidePanel`/`TopBottomPanel` → `Panel`).
3. `cargo fmt --all --check`, `cargo clippy --workspace --all-targets
   --all-features`, `cargo test --workspace --all-features`, wasm checks for
   the crate (both feature sets) and the demo — CI runs the same set.
4. Reinstall the egui MCP bridge so inspection keeps working (wire protocol
   has no version negotiation):
   `cargo install --git https://github.com/rerun-io/kittest_inspector egui_mcp --force`,
   then verify `attach` against `EGUI_INSPECTION=1 cargo run --example drag_drop`.
5. Bump the crate minor (`0.Y.0` — the crate minor tracks the egui minor
   cadence), update README if the egui version is mentioned, changelog entry.
6. `cargo publish --dry-run`, then `cargo publish`.
7. Tag `vX.Y.Z`, push, confirm CI green and the Pages demo deployed.
8. Notify downstream: aicogito and aiquill-workspace (journal a `meta-impact`
   note if the API changed shape).

## On a patch release (no egui bump)

Steps 3, 5 (patch bump), 6, 7.
