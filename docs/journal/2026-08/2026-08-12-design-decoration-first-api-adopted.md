---
date: 2026-08-12
type: journal
components: [tree-view, decoration]
aspects: [font-independence]
design:
  - docs/design/2026-08-11-decoration-first-api.md
---

# Decoration-first API design record adopted

- **When:** 2026-08-12 00:39:49 local

## Context

The decoration-first API record was written `proposed` on 2026-08-11 alongside
the core implementation, pending review of the concrete API shape.

## Details

Reviewed and adopted as-is; the record's status moved to `accepted`. The
implementation already realizes the decided shape (painted built-in icons,
slot-based decoration with `ImageSource`/custom-painter escape hatches,
kind-carrying action payloads, openness-change actions for lazy loading,
working programmatic expand/reveal), so no code change follows from the
adoption — the record now binds future API evolution: parity work (drag &
drop, context menus, persistence) extends this shape rather than revisiting
it.

## Links

- Design record: `docs/design/2026-08-11-decoration-first-api.md`
- Implementation milestone:
  `docs/journal/2026-08/2026-08-11-execution-core-tree-and-decoration-api.md`
