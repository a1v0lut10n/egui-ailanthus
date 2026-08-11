use egui::{
    Align, Id, Layout, Rect, TextStyle, TextWrapMode, UiBuilder, pos2, vec2,
};

use crate::{
    Node, NodeId, NodeInfo, RowContext, TreeViewSettings, TreeViewState,
    icon::{self, IconContext, IconSource},
    row_metrics, RowMetrics,
    state::PendingReveal,
};

/// Geometry and identity of one structurally visible row, recorded during the
/// build pass and consumed by input handling.
pub(crate) struct Row<Id_> {
    pub info: NodeInfo<Id_>,
    pub depth: usize,
    pub rect: Rect,
    pub closer_rect: Option<Rect>,
    pub open: bool,
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
pub struct TreeBuilder<'ui, 'state, Id_: NodeId> {
    ui: &'ui mut egui::Ui,
    state: &'state mut TreeViewState<Id_>,
    settings: &'state TreeViewSettings,
    interact_id: Id,
    metrics: RowMetrics,
    stack: Vec<DirFrame<Id_>>,
    rows: Vec<Row<Id_>>,
    reveal: Option<RevealMatch<Id_>>,
    bounds: Option<Rect>,
}

impl<'ui, 'state, Id_: NodeId> TreeBuilder<'ui, 'state, Id_> {
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
        }
    }

    /// Emit a directory node. Returns whether it is open. Children follow
    /// until the matching [`close_dir`](Self::close_dir); emit them
    /// unconditionally — collapsed branches are skipped cheaply inside the
    /// builder (cull caller-side only for very large trees).
    pub fn dir(&mut self, node: Node<'_, Id_>) -> bool {
        let parents_open = self.parents_open();
        let open = self
            .state
            .is_open(&node.id)
            .unwrap_or(node.default_open);
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
    pub fn leaf(&mut self, node: Node<'_, Id_>) {
        self.check_reveal(&node.id, false);
        if self.parents_open() {
            self.render_row(node, false, false);
        }
    }

    /// The id of the directory currently being filled, if any.
    pub fn parent_id(&self) -> Option<&Id_> {
        self.stack.last().map(|f| &f.id)
    }

    pub(crate) fn finish(self) -> (Vec<Row<Id_>>, Option<RevealMatch<Id_>>, Option<Rect>) {
        if self.reveal.is_some() {
            self.state.pending_reveal = None;
        }
        (self.rows, self.reveal, self.bounds)
    }

    fn parents_open(&self) -> bool {
        self.stack.last().is_none_or(|f| f.cumulative_open)
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

    fn render_row(&mut self, mut node: Node<'_, Id_>, is_dir: bool, open: bool) {
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
            rect,
            closer_rect: is_dir.then_some(closer_slot),
            open,
        });
        self.bounds = Some(match self.bounds {
            Some(b) => b.union(rect),
            None => rect,
        });

        if !self.ui.is_rect_visible(rect) {
            return; // Outside the clip rect: geometry recorded, painting skipped.
        }

        let is_selected = self.state.is_selected(&node.id);
        let is_hovered = self
            .ui
            .ctx()
            .pointer_hover_pos()
            .is_some_and(|p| rect.contains(p));

        // Row background: stripe, then hover, then selection.
        let painter = self.ui.painter();
        if self.settings.striped && self.rows.len() % 2 == 0 {
            painter.rect_filled(rect, 0.0, visuals.faint_bg_color);
        }
        if is_hovered && !is_selected {
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
    }
}
