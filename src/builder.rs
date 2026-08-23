use egui::{
    Align, Id, LayerId, Layout, Order, Rect, TextStyle, TextWrapMode, UiBuilder, pos2, vec2,
};

use crate::{
    Node, NodeId, NodeInfo, RowContext, RowMetrics, TreeViewSettings, TreeViewState,
    icon::{self, IconContext, IconSource},
    node::ContextMenuFn,
    row_metrics,
    state::PendingReveal,
};

/// Geometry and identity of one structurally visible row, recorded during the
/// build pass and consumed by input handling.
pub(crate) struct Row<Id_> {
    pub info: NodeInfo<Id_>,
    pub depth: usize,
    pub parent: Option<Id_>,
    pub rect: Rect,
    pub closer_rect: Option<Rect>,
    pub open: bool,
    pub drop_allowed: bool,
}

/// Everything the build pass hands to the input pass.
pub(crate) struct BuildOutput<'nodes, Id_> {
    pub rows: Vec<Row<Id_>>,
    pub reveal: Option<RevealMatch<Id_>>,
    pub bounds: Option<Rect>,
    pub context_menus: Vec<(Id_, ContextMenuFn<'nodes>)>,
}

/// A matched pending reveal: the target node plus the ancestor chain the
/// build pass discovered for it.
pub(crate) struct RevealMatch<Id_> {
    pub id: Id_,
    pub is_dir: bool,
    pub ancestors: Vec<Id_>,
    pub select: bool,
}

struct DirFrame<Id_> {
    id: Id_,
    /// This dir and all its ancestors are open — children are rendered.
    cumulative_open: bool,
}

/// Emits the tree's nodes for one frame. Handed to the build closure by
/// [`TreeView::show`](crate::TreeView::show) /
/// [`show_state`](crate::TreeView::show_state).
pub struct TreeBuilder<'ui, 'state, 'nodes, Id_: NodeId> {
    ui: &'ui mut egui::Ui,
    state: &'state mut TreeViewState<Id_>,
    settings: &'state TreeViewSettings,
    interact_id: Id,
    metrics: RowMetrics,
    stack: Vec<DirFrame<Id_>>,
    rows: Vec<Row<Id_>>,
    reveal: Option<RevealMatch<Id_>>,
    bounds: Option<Rect>,
    context_menus: Vec<(Id_, ContextMenuFn<'nodes>)>,
}

impl<'ui, 'state, 'nodes, Id_: NodeId> TreeBuilder<'ui, 'state, 'nodes, Id_> {
    pub(crate) fn new(
        ui: &'ui mut egui::Ui,
        state: &'state mut TreeViewState<Id_>,
        settings: &'state TreeViewSettings,
        interact_id: Id,
    ) -> Self {
        let metrics = row_metrics(ui, settings);
        Self {
            ui,
            state,
            settings,
            interact_id,
            metrics,
            stack: Vec::new(),
            rows: Vec::new(),
            reveal: None,
            bounds: None,
            context_menus: Vec::new(),
        }
    }

    /// Emit a directory node. Returns whether it is open. Children follow
    /// until the matching [`close_dir`](Self::close_dir); emit them
    /// unconditionally — collapsed branches are skipped cheaply inside the
    /// builder (cull caller-side only for very large trees).
    pub fn dir(&mut self, node: Node<'nodes, Id_>) -> bool {
        let parents_open = self.parents_open();
        let open = self.state.is_open(&node.id).unwrap_or(node.default_open);
        let id = node.id.clone();
        self.check_reveal(&id, true);
        if parents_open {
            self.render_row(node, true, open);
        }
        self.stack.push(DirFrame {
            id,
            cumulative_open: parents_open && open,
        });
        open
    }

    /// Close the most recently opened directory. Every [`dir`](Self::dir)
    /// call must be balanced by one `close_dir`.
    pub fn close_dir(&mut self) {
        self.stack.pop();
    }

    /// Emit a leaf node.
    pub fn leaf(&mut self, node: Node<'nodes, Id_>) {
        self.check_reveal(&node.id, false);
        if self.parents_open() {
            self.render_row(node, false, false);
        }
    }

    /// The id of the directory currently being filled, if any.
    pub fn parent_id(&self) -> Option<&Id_> {
        self.stack.last().map(|f| &f.id)
    }

    pub(crate) fn finish(self) -> BuildOutput<'nodes, Id_> {
        if self.reveal.is_some() {
            self.state.pending_reveal = None;
        }
        BuildOutput {
            rows: self.rows,
            reveal: self.reveal,
            bounds: self.bounds,
            context_menus: self.context_menus,
        }
    }

    fn parents_open(&self) -> bool {
        self.stack.last().is_none_or(|f| f.cumulative_open)
    }

    /// Register an AccessKit node for a visible row (no-op unless an
    /// integration enabled AccessKit), so assistive tech, `egui_kittest`
    /// queries, and the egui MCP inspection server can see and target rows.
    #[allow(clippy::too_many_arguments)]
    fn accesskit_row(
        &self,
        id: &Id_,
        label: &str,
        is_dir: bool,
        open: bool,
        is_selected: bool,
        depth: usize,
        rect: Rect,
    ) {
        use egui::accesskit;
        let ak_id = self.interact_id.with(("row", id));
        let label = label.to_owned();
        self.ui.ctx().accesskit_node_builder(ak_id, |node| {
            node.set_role(accesskit::Role::TreeItem);
            node.set_label(label);
            node.set_level(depth + 1);
            node.set_bounds(accesskit::Rect {
                x0: rect.min.x.into(),
                y0: rect.min.y.into(),
                x1: rect.max.x.into(),
                y1: rect.max.y.into(),
            });
            if is_dir {
                node.set_expanded(open);
            }
            if is_selected {
                node.set_selected(true);
            } else {
                node.clear_selected();
            }
            node.add_action(accesskit::Action::Click);
        });
    }

    /// Paint a dragged row's ghost at the pointer, on the tooltip layer.
    fn paint_drag_ghost(
        &self,
        node: &Node<'nodes, Id_>,
        galley: &std::sync::Arc<egui::Galley>,
        is_dir: bool,
        open: bool,
    ) {
        let Some(drag) = self.state.drag.as_ref() else {
            return;
        };
        let Some(pointer) = self.ui.ctx().pointer_hover_pos() else {
            return;
        };
        let Some(index) = drag.sources.iter().position(|s| s == &node.id) else {
            return;
        };
        let m = self.metrics;
        let visuals = self.ui.visuals();
        let painter = self.ui.ctx().layer_painter(LayerId::new(
            Order::Tooltip,
            self.interact_id.with("drag_overlay"),
        ));
        let origin = pointer
            + vec2(
                12.0,
                index as f32 * (m.row_height * 0.9) - m.row_height * 0.5,
            );
        let ghost_rect = Rect::from_min_size(
            origin,
            vec2(m.icon_size + m.gap + galley.size().x + 12.0, m.row_height),
        );
        painter.rect_filled(ghost_rect, 3.0, visuals.panel_fill.gamma_multiply(0.9));
        if let Some(spec) = &node.icon {
            let icon_slot = Rect::from_min_size(
                pos2(ghost_rect.left() + 4.0, ghost_rect.top()),
                vec2(m.icon_size, ghost_rect.height()),
            );
            let source = match (&spec.open, is_dir && open) {
                (Some(open_source), true) => open_source,
                _ => &spec.closed,
            };
            if let IconSource::Painted(icon) = source {
                icon::paint_icon(*icon, &painter, m.icon_rect_in(icon_slot), visuals);
            }
        }
        painter.galley(
            pos2(
                ghost_rect.left() + 4.0 + m.icon_size + m.gap,
                ghost_rect.center().y - galley.size().y * 0.5,
            ),
            galley.clone(),
            visuals.strong_text_color(),
        );
    }

    fn check_reveal(&mut self, id: &Id_, is_dir: bool) {
        let Some(pending) = &self.state.pending_reveal else {
            return;
        };
        let (target, select) = match pending {
            PendingReveal::ExpandParents(t) => (t, false),
            PendingReveal::Reveal(t) => (t, true),
        };
        if target == id {
            self.reveal = Some(RevealMatch {
                id: id.clone(),
                is_dir,
                ancestors: self.stack.iter().map(|f| f.id.clone()).collect(),
                select,
            });
        }
    }

    fn render_row(&mut self, mut node: Node<'nodes, Id_>, is_dir: bool, open: bool) {
        let m = self.metrics;
        let depth = self.stack.len();
        let visuals = self.ui.visuals().clone();

        let galley = std::mem::take(&mut node.label).into_galley(
            self.ui,
            Some(TextWrapMode::Extend),
            f32::INFINITY,
            TextStyle::Body,
        );

        let indent = depth as f32 * m.indent;
        let content_width =
            indent + m.closer_width + m.gap + m.icon_size + m.gap + galley.size().x + 8.0;
        let width = content_width.max(self.ui.available_width());
        let (_, rect) = self.ui.allocate_space(vec2(width, m.row_height));

        let closer_slot = Rect::from_min_size(
            pos2(rect.left() + indent, rect.top()),
            vec2(m.closer_width, rect.height()),
        );

        self.rows.push(Row {
            info: NodeInfo {
                id: node.id.clone(),
                is_dir,
            },
            depth,
            parent: self.stack.last().map(|f| f.id.clone()),
            rect,
            closer_rect: is_dir.then_some(closer_slot),
            open,
            drop_allowed: node.drop_allowed.unwrap_or(is_dir),
        });
        self.bounds = Some(match self.bounds {
            Some(b) => b.union(rect),
            None => rect,
        });

        if let Some(menu) = node.context_menu.take() {
            self.context_menus.push((node.id.clone(), menu));
        }

        if !self.ui.is_rect_visible(rect) {
            return; // Outside the clip rect: geometry recorded, painting skipped.
        }

        let is_selected = self.state.is_selected(&node.id);
        self.accesskit_row(
            &node.id,
            galley.text(),
            is_dir,
            open,
            is_selected,
            depth,
            rect,
        );

        // While a drag is active, paint dragged rows as a ghost at the
        // pointer and dim them in place.
        let dragged = self
            .state
            .drag
            .as_ref()
            .is_some_and(|d| d.active && d.sources.contains(&node.id));
        if dragged {
            self.paint_drag_ghost(&node, &galley, is_dir, open);
        }

        // Layer-aware: a popup or menu above the tree must block row
        // hover (raw `pointer_hover_pos` ignores occlusion — field-caught
        // in aicogito: a combo's items highlighted tree rows through the
        // popup).
        let is_hovered = self.ui.rect_contains_pointer(rect);

        // Row background: stripe, then hover, then selection.
        let painter = self.ui.painter();
        if self.settings.striped && self.rows.len() % 2 == 0 {
            painter.rect_filled(rect, 0.0, visuals.faint_bg_color);
        }
        if is_hovered && !is_selected && self.state.drag.is_none() {
            painter.rect_filled(rect, 2.0, visuals.widgets.hovered.weak_bg_fill);
        }
        if is_selected {
            painter.rect_filled(rect, 2.0, visuals.selection.bg_fill);
        }

        if let Some(paint) = &mut node.row_paint {
            let ctx = RowContext {
                rect,
                depth,
                is_dir,
                is_open: open,
                is_selected,
                is_hovered,
            };
            paint(self.ui.painter(), &ctx);
        }

        // Closer triangle (directories only).
        if is_dir {
            let openness = self
                .ui
                .ctx()
                .animate_bool(self.interact_id.with(("closer", &node.id)), open);
            let color = visuals.widgets.noninteractive.fg_stroke.color;
            icon::paint_closer(self.ui.painter(), closer_slot, openness, color);
        }

        // Icon slot (always reserved, so labels align whether or not a node
        // has an icon).
        let icon_slot = Rect::from_min_size(
            pos2(closer_slot.right() + m.gap, rect.top()),
            vec2(m.icon_size, rect.height()),
        );
        if let Some(spec) = &mut node.icon {
            let source = match (&mut spec.open, is_dir && open) {
                (Some(open_source), true) => open_source,
                _ => &mut spec.closed,
            };
            let icon_rect = m.icon_rect_in(icon_slot);
            match source {
                IconSource::Painted(icon) => {
                    icon::paint_icon(*icon, self.ui.painter(), icon_rect, &visuals);
                }
                IconSource::Image(src) => {
                    egui::Image::new(src.clone()).paint_at(self.ui, icon_rect);
                }
                IconSource::Custom(paint) => {
                    let ctx = IconContext {
                        rect: icon_rect,
                        is_dir,
                        is_open: open,
                        is_selected,
                        is_hovered,
                    };
                    paint(self.ui, ctx);
                }
            }
            if let Some(badge) = node.badge {
                let icon_rect = m.icon_rect_in(icon_slot);
                let radius = m.icon_size * 0.18;
                let center = icon_rect.right_bottom();
                self.ui.painter().circle(
                    center,
                    radius,
                    badge.color,
                    egui::Stroke::new(1.0, visuals.panel_fill),
                );
            }
        }

        // Label.
        let text_color = if is_selected {
            visuals.strong_text_color()
        } else {
            visuals.text_color()
        };
        let label_pos = pos2(
            icon_slot.right() + m.gap,
            rect.center().y - galley.size().y * 0.5,
        );
        let label_right = label_pos.x + galley.size().x;
        self.ui.painter().galley(label_pos, galley, text_color);

        // Trailing widgets, right-aligned in the remaining row space.
        if let Some(add) = &mut node.trailing {
            let trailing_rect = Rect::from_min_max(
                pos2(label_right + m.gap, rect.top()),
                pos2(rect.right() - 4.0, rect.bottom()),
            );
            if trailing_rect.width() > 0.0 {
                let mut child = self.ui.new_child(
                    UiBuilder::new()
                        .max_rect(trailing_rect)
                        .layout(Layout::right_to_left(Align::Center)),
                );
                add(&mut child);
            }
        }

        if dragged {
            // Fade the in-place row while its ghost follows the pointer.
            let fade = visuals.panel_fill.gamma_multiply(0.6);
            self.ui.painter().rect_filled(rect, 0.0, fade);
        }
    }
}
