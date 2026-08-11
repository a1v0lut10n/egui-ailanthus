---
date: 2026-08-11
type: task
status: planned
affects: []
components: [examples]
aspects: [wasm-compatibility, egui-version-tracking]
tags: [ci, wasm, release]
---

# CI matrix and wasm demo

## Objective

Continuous verification of the cross-platform requirement and a hosted wasm
demo of the decorated tree.

## Deliverables

1. GitHub Actions: build + test + clippy on ubuntu/macos/windows, plus a
   `wasm32-unknown-unknown` check of the crate and examples.
2. A trunk-based web demo (playground example) deployable to GitHub Pages.
3. Release checklist documenting the egui-minor tracking policy (bump egui,
   release, notify downstream repos).

## Explicitly out of scope

- crates.io publication automation (manual first release).

## Completion

`affects` is deliberately empty — CI and demo add infrastructure without
invalidating current-state docs. Write a linking journal entry on completion.
