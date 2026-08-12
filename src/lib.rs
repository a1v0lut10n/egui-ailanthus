//! A decorated tree view widget for egui.
//!
//! `egui_ailanthus` renders trees whose nodes carry meaningful decoration:
//! built-in painted icons (folders, crates, file types), badge overlays,
//! trailing widgets, and custom row painting — all drawn with egui's painter,
//! so nothing depends on font glyph coverage or bundled image assets.
//!
//! The API is immediate-mode: each frame you walk your own data inside the
//! build closure and emit nodes; the widget keeps only openness and selection,
//! keyed by your node ids, and reports everything that happened as a
//! [`Vec<Action>`] for you to apply to your model after `show` returns.
//!
//! ```no_run
//! # use egui_ailanthus::{TreeView, TreeViewState, Node, Icon};
//! # fn demo(ui: &mut egui::Ui, state: &mut TreeViewState<&'static str>) {
//! let (_response, actions) = TreeView::new(egui::Id::new("tree"))
//!     .show_state(ui, state, |tree| {
//!         tree.dir(Node::new("src").label("src").icon(Icon::folder()));
//!         tree.leaf(Node::new("src/main.rs").label("main.rs").icon(Icon::FileRust));
//!         tree.close_dir();
//!         tree.leaf(Node::new("report.pdf").label("report.pdf").icon(Icon::FilePdf));
//!     });
//! # }
//! ```

mod action;
mod builder;
mod icon;
mod node;
mod state;

pub use action::{Action, DirPosition, DragAndDrop, NodeInfo};
pub use builder::TreeBuilder;
pub use icon::{Icon, IconContext, IconPainter, IconSource, IconSpec};
pub use node::{Badge, Node, RowContext};
pub use state::TreeViewState;

use builder::Row;
use egui::{
    Align, EventFilter, Id, Key, Modifiers, Pos2, Rect, Response, Sense, Ui, Vec2, accesskit, pos2,
    vec2,
};
use state::{ContextMenuState, DragState};

/// Ids used as tree node keys.
///
/// Blanket-implemented; you never implement this yourself.
pub trait NodeId: Clone + PartialEq + Eq + std::hash::Hash + std::fmt::Debug {}
impl<T: Clone + PartialEq + Eq + std::hash::Hash + std::fmt::Debug> NodeId for T {}

/// Visual and behavioral settings for a [`TreeView`].
#[derive(Clone, Debug)]
pub struct TreeViewSettings {
    /// Horizontal indent per tree level. `None` uses `Spacing::indent` scaled down.
    pub indent: Option<f32>,
    /// Row height. `None` uses `Spacing::interact_size.y`.
    pub row_height: Option<f32>,
    /// Side length of the icon slot. `None` derives it from the row height.
    pub icon_size: Option<f32>,
    /// Paint alternating faint row backgrounds.
    pub striped: bool,
    /// Allow selecting more than one node.
    pub allow_multi_select: bool,
    /// Allow dragging nodes to reorder / re-parent them (reported as
    /// [`Action::Move`]; the application applies the move to its own data).
    pub allow_drag_and_drop: bool,
    /// Modifier for range selection (default shift).
    pub range_select_modifier: Modifiers,
    /// Modifier for toggling single nodes in and out of the selection
    /// (default command / ctrl).
    pub toggle_select_modifier: Modifiers,
}

impl Default for TreeViewSettings {
    fn default() -> Self {
        Self {
            indent: None,
            row_height: None,
            icon_size: None,
            striped: false,
            allow_multi_select: true,
            allow_drag_and_drop: true,
            range_select_modifier: Modifiers::SHIFT,
            toggle_select_modifier: Modifiers::COMMAND,
        }
    }
}

/// The tree view widget.
///
/// Construct one per frame with [`TreeView::new`], configure it, then call
/// [`show`](TreeView::show) (widget-owned state in egui memory) or
/// [`show_state`](TreeView::show_state) (caller-owned [`TreeViewState`]).
pub struct TreeView<'a> {
    id: Id,
    settings: TreeViewSettings,
    fallback_context_menu: Option<node::ContextMenuFn<'a>>,
}

impl<'a> TreeView<'a> {
    pub fn new(id: Id) -> Self {
        Self {
            id,
            settings: TreeViewSettings::default(),
            fallback_context_menu: None,
        }
    }

    pub fn with_settings(mut self, settings: TreeViewSettings) -> Self {
        self.settings = settings;
        self
    }

    pub fn indent(mut self, indent: f32) -> Self {
        self.settings.indent = Some(indent);
        self
    }

    pub fn row_height(mut self, height: f32) -> Self {
        self.settings.row_height = Some(height);
        self
    }

    pub fn icon_size(mut self, size: f32) -> Self {
        self.settings.icon_size = Some(size);
        self
    }

    pub fn striped(mut self, striped: bool) -> Self {
        self.settings.striped = striped;
        self
    }

    pub fn allow_multi_select(mut self, allow: bool) -> Self {
        self.settings.allow_multi_select = allow;
        self
    }

    pub fn allow_drag_and_drop(mut self, allow: bool) -> Self {
        self.settings.allow_drag_and_drop = allow;
        self
    }

    /// A context menu shown when right-clicking a node without its own menu,
    /// or empty space below the tree. Read the selection from your
    /// [`TreeViewState`] if the menu should act on it.
    pub fn fallback_context_menu(mut self, add: impl FnMut(&mut Ui) + 'a) -> Self {
        self.fallback_context_menu = Some(Box::new(add));
        self
    }

    /// Show the tree with widget-owned state stored in egui memory.
    ///
    /// With the `persistence` cargo feature the state is persisted across
    /// sessions (which is why node ids then need serde bounds); without it,
    /// state lives for the session only.
    #[cfg(not(feature = "persistence"))]
    pub fn show<Id_: NodeId + Send + Sync + 'static>(
        self,
        ui: &mut Ui,
        build: impl FnOnce(&mut TreeBuilder<'_, '_, '_, Id_>),
    ) -> (Response, Vec<Action<Id_>>) {
        let state_id = self.id.with("ailanthus_state");
        let mut state: TreeViewState<Id_> =
            ui.data_mut(|d| d.get_temp(state_id)).unwrap_or_default();
        let result = self.show_state(ui, &mut state, build);
        ui.data_mut(|d| d.insert_temp(state_id, state));
        result
    }

    /// Show the tree with widget-owned state persisted in egui memory across
    /// sessions (`persistence` feature).
    #[cfg(feature = "persistence")]
    pub fn show<Id_>(
        self,
        ui: &mut Ui,
        build: impl FnOnce(&mut TreeBuilder<'_, '_, '_, Id_>),
    ) -> (Response, Vec<Action<Id_>>)
    where
        Id_: NodeId + Send + Sync + 'static + serde::Serialize + serde::de::DeserializeOwned,
    {
        let state_id = self.id.with("ailanthus_state");
        let mut state: TreeViewState<Id_> = ui
            .data_mut(|d| d.get_persisted(state_id))
            .unwrap_or_default();
        let result = self.show_state(ui, &mut state, build);
        ui.data_mut(|d| d.insert_persisted(state_id, state));
        result
    }

    /// Show the tree with caller-owned state.
    pub fn show_state<'nodes, Id_: NodeId>(
        mut self,
        ui: &mut Ui,
        state: &mut TreeViewState<Id_>,
        build: impl FnOnce(&mut TreeBuilder<'_, '_, 'nodes, Id_>),
    ) -> (Response, Vec<Action<Id_>>) {
        let interact_id = self.id.with("interact");

        // Mark the tree container for assistive tech / inspection (no-op
        // unless the integration enabled AccessKit).
        ui.ctx().accesskit_node_builder(interact_id, |node| {
            node.set_role(accesskit::Role::Tree);
        });

        // Build phase: walk caller data, paint rows, collect row geometry.
        let builder::BuildOutput {
            rows,
            reveal,
            bounds,
            mut context_menus,
        } = ui
            .scope(|ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                let mut tree = TreeBuilder::new(ui, state, &self.settings, interact_id);
                build(&mut tree);
                tree.finish()
            })
            .inner;

        let full_rect = bounds.unwrap_or_else(|| {
            // Empty tree: claim a minimal strip so the widget still exists.
            let (_, rect) = ui.allocate_space(vec2(ui.available_width(), 0.0));
            rect
        });

        let response = ui.interact(full_rect, interact_id, Sense::click_and_drag());
        let mut actions = Vec::new();

        self.handle_pointer(ui, state, &rows, &response, &mut actions);
        self.handle_drag(ui, state, &rows, full_rect, &response, &mut actions);
        self.handle_context_menu(
            ui,
            state,
            &rows,
            &response,
            &mut context_menus,
            &mut actions,
        );
        self.handle_keys(ui, state, &rows, &response, &mut actions);
        self.apply_reveal(ui, state, reveal, &mut actions);

        // Serve a pending programmatic scroll once the target row exists.
        if let Some(target) = state.take_scroll_to()
            && let Some(row) = rows.iter().find(|r| r.info.id == target)
        {
            ui.scroll_to_rect(row.rect, Some(Align::Center));
        }

        (response, actions)
    }

    fn handle_pointer<Id_: NodeId>(
        &self,
        ui: &mut Ui,
        state: &mut TreeViewState<Id_>,
        rows: &[Row<Id_>],
        response: &Response,
        actions: &mut Vec<Action<Id_>>,
    ) {
        if !(response.clicked() || response.double_clicked()) {
            return;
        }
        let Some(pos) = response.interact_pointer_pos() else {
            return;
        };
        let Some(row) = rows.iter().find(|r| r.rect.contains(pos)) else {
            return;
        };
        response.request_focus();

        let modifiers = ui.input(|i| i.modifiers);
        let on_closer = row.closer_rect.is_some_and(|c| c.contains(pos));

        if on_closer {
            // Closer toggles openness without touching the selection.
            self.toggle_dir(ui, state, row, actions);
            return;
        }

        if response.double_clicked() {
            if !row.info.is_dir {
                actions.push(Action::Activate {
                    nodes: selected_infos(state, rows),
                    modifiers,
                });
            }
            return;
        }

        self.click_select(state, rows, row, modifiers, actions);
        if row.info.is_dir && modifiers.is_none() {
            self.toggle_dir(ui, state, row, actions);
        }
    }

    fn handle_drag<Id_: NodeId>(
        &self,
        ui: &mut Ui,
        state: &mut TreeViewState<Id_>,
        rows: &[Row<Id_>],
        full_rect: Rect,
        response: &Response,
        actions: &mut Vec<Action<Id_>>,
    ) {
        if !self.settings.allow_drag_and_drop {
            return;
        }
        let pointer = response.interact_pointer_pos();

        // The source row is the one under the *press origin* — by the time
        // drag_started fires the pointer may already be rows away.
        if response.drag_started()
            && let Some(pos) = ui.input(|i| i.pointer.press_origin()).or(pointer)
            && let Some(row) = rows.iter().find(|r| r.rect.contains(pos))
        {
            let sources = if state.is_selected(&row.info.id) {
                simplify_sources(rows, state)
            } else {
                vec![row.info.id.clone()]
            };
            state.drag = Some(DragState {
                sources,
                start_pos: pos,
                active: false,
            });
        }

        if response.dragged()
            && let Some(pos) = pointer
        {
            let activated = if let Some(drag) = &mut state.drag {
                if !drag.active && (pos - drag.start_pos).length() > 6.0 {
                    drag.active = true;
                }
                drag.active
            } else {
                false
            };
            if activated {
                ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
                ui.ctx().request_repaint();
                if let Some((target_dir, position, marker_shape)) =
                    self.drop_position(ui, state, rows, pos)
                {
                    let marker = ui.painter().add(marker_shape);
                    let sources = source_infos(state, rows);
                    actions.push(Action::Drag(DragAndDrop::new(
                        sources, target_dir, position, marker,
                    )));
                }
            }
        }

        if response.drag_stopped()
            && let Some(drag) = state.drag.take()
            && drag.active
        {
            let sources: Vec<NodeInfo<Id_>> =
                drag.sources.iter().map(|id| info_for(rows, id)).collect();
            state.drag = Some(drag); // keep for drop_position's source checks
            if let Some(pos) = pointer {
                if let Some((target_dir, position, _)) = self.drop_position(ui, state, rows, pos) {
                    let marker = ui.painter().add(egui::Shape::Noop);
                    actions.push(Action::Move(DragAndDrop::new(
                        sources, target_dir, position, marker,
                    )));
                } else if !full_rect.contains(pos) {
                    actions.push(Action::MoveExternal { sources, pos });
                }
            }
            state.drag = None;
            ui.ctx().request_repaint();
        }
    }

    /// Resolve the drop position under the pointer, or `None` if dropping
    /// there is not allowed. Also returns the marker shape to show.
    #[allow(clippy::type_complexity)]
    fn drop_position<Id_: NodeId>(
        &self,
        ui: &Ui,
        state: &TreeViewState<Id_>,
        rows: &[Row<Id_>],
        pos: Pos2,
    ) -> Option<(Option<NodeInfo<Id_>>, DirPosition<Id_>, egui::Shape)> {
        let drag = state.drag.as_ref()?;
        let stroke = ui.visuals().selection.stroke;

        let row = rows.iter().find(|r| r.rect.contains(pos));
        let Some(row) = row else {
            // Below the last row but still inside the tree: root level, last.
            let last = rows.last()?;
            if pos.y < last.rect.bottom() {
                return None;
            }
            let y = last.rect.bottom();
            let shape = egui::Shape::line_segment(
                [pos2(last.rect.left(), y), pos2(last.rect.right(), y)],
                stroke,
            );
            return Some((None, DirPosition::Last, shape));
        };

        if drag.sources.contains(&row.info.id) {
            return None; // Dropping a node onto itself.
        }

        let q = (pos.y - row.rect.top()) / row.rect.height();
        let (target, position) = if row.info.is_dir && row.drop_allowed {
            if q < 0.25 {
                (row.parent.clone(), DirPosition::Before(row.info.id.clone()))
            } else if q > 0.75 && !row.open {
                (row.parent.clone(), DirPosition::After(row.info.id.clone()))
            } else if q > 0.75 {
                (Some(row.info.id.clone()), DirPosition::First)
            } else {
                (Some(row.info.id.clone()), DirPosition::Last)
            }
        } else if q < 0.5 {
            (row.parent.clone(), DirPosition::Before(row.info.id.clone()))
        } else {
            (row.parent.clone(), DirPosition::After(row.info.id.clone()))
        };

        // A directory cannot be dropped into itself or its own descendants:
        // climb from the target to the root and reject on any source hit.
        let mut current = target.clone();
        while let Some(id) = current {
            if drag.sources.contains(&id) {
                return None;
            }
            current = rows
                .iter()
                .find(|r| r.info.id == id)
                .and_then(|r| r.parent.clone());
        }

        let shape = match &position {
            DirPosition::First | DirPosition::Last if target.is_some() => {
                let target_row = rows.iter().find(|r| Some(&r.info.id) == target.as_ref())?;
                egui::Shape::Rect(egui::epaint::RectShape::stroke(
                    target_row.rect,
                    2.0,
                    stroke,
                    egui::StrokeKind::Inside,
                ))
            }
            DirPosition::Before(_) => {
                egui::Shape::line_segment([row.rect.left_top(), row.rect.right_top()], stroke)
            }
            _ => {
                egui::Shape::line_segment([row.rect.left_bottom(), row.rect.right_bottom()], stroke)
            }
        };

        let target_info = target.map(|id| NodeInfo { id, is_dir: true });
        Some((target_info, position, shape))
    }

    fn handle_context_menu<Id_: NodeId>(
        &mut self,
        ui: &mut Ui,
        state: &mut TreeViewState<Id_>,
        rows: &[Row<Id_>],
        response: &Response,
        context_menus: &mut [(Id_, crate::node::ContextMenuFn<'_>)],
        actions: &mut Vec<Action<Id_>>,
    ) {
        let popup_id = self.id.with("context_menu");

        if response.secondary_clicked()
            && let Some(pos) = response.interact_pointer_pos()
        {
            let row = rows.iter().find(|r| r.rect.contains(pos));
            if let Some(row) = row
                && !state.is_selected(&row.info.id)
            {
                // Right-clicking an unselected row selects it first.
                state.set_selected(vec![row.info.id.clone()]);
                state.set_pivot(Some(row.info.id.clone()));
                state.set_cursor(Some(row.info.id.clone()));
                actions.push(Action::SetSelected(selected_infos(state, rows)));
            }
            state.context_menu = Some(ContextMenuState {
                node: row.map(|r| r.info.id.clone()),
                pos,
            });
            egui::Popup::open_id(ui.ctx(), popup_id);
        }

        let Some(menu_state) = state.context_menu.clone() else {
            return;
        };
        if !egui::Popup::is_id_open(ui.ctx(), popup_id) {
            state.context_menu = None;
            return;
        }

        let node_menu = menu_state.node.as_ref().and_then(|id| {
            context_menus
                .iter_mut()
                .find(|(menu_id, _)| menu_id == id)
                .map(|(_, menu)| menu)
        });

        let show = |add: &mut dyn FnMut(&mut Ui)| {
            egui::Popup::new(
                popup_id,
                ui.ctx().clone(),
                egui::PopupAnchor::Position(menu_state.pos),
                ui.layer_id(),
            )
            .kind(egui::PopupKind::Menu)
            .open_memory(None)
            .close_behavior(egui::PopupCloseBehavior::CloseOnClick)
            .style(egui::containers::menu::menu_style)
            .show(|ui| add(ui));
        };
        match node_menu {
            Some(menu) => show(menu),
            None => {
                if let Some(fallback) = self.fallback_context_menu.as_mut() {
                    show(fallback);
                } else {
                    egui::Popup::close_id(ui.ctx(), popup_id);
                    state.context_menu = None;
                }
            }
        }
    }

    fn toggle_dir<Id_: NodeId>(
        &self,
        ui: &Ui,
        state: &mut TreeViewState<Id_>,
        row: &Row<Id_>,
        actions: &mut Vec<Action<Id_>>,
    ) {
        let now_open = !row.open;
        state.set_openness(row.info.id.clone(), now_open);
        actions.push(if now_open {
            Action::DirOpened(row.info.clone())
        } else {
            Action::DirClosed(row.info.clone())
        });
        ui.ctx().request_repaint();
    }

    fn click_select<Id_: NodeId>(
        &self,
        state: &mut TreeViewState<Id_>,
        rows: &[Row<Id_>],
        row: &Row<Id_>,
        modifiers: Modifiers,
        actions: &mut Vec<Action<Id_>>,
    ) {
        let multi = self.settings.allow_multi_select;
        if multi && modifiers.matches_logically(self.settings.toggle_select_modifier) {
            state.toggle_selected(row.info.id.clone());
            state.set_pivot(Some(row.info.id.clone()));
        } else if multi
            && modifiers.matches_logically(self.settings.range_select_modifier)
            && state.pivot().is_some()
        {
            let pivot = state.pivot().cloned().unwrap();
            state.set_selected(range_ids(rows, &pivot, &row.info.id));
        } else {
            state.set_selected(vec![row.info.id.clone()]);
            state.set_pivot(Some(row.info.id.clone()));
        }
        state.set_cursor(Some(row.info.id.clone()));
        actions.push(Action::SetSelected(selected_infos(state, rows)));
    }

    fn handle_keys<Id_: NodeId>(
        &self,
        ui: &mut Ui,
        state: &mut TreeViewState<Id_>,
        rows: &[Row<Id_>],
        response: &Response,
        actions: &mut Vec<Action<Id_>>,
    ) {
        if !response.has_focus() || rows.is_empty() {
            return;
        }
        ui.memory_mut(|m| {
            m.set_focus_lock_filter(
                response.id,
                EventFilter {
                    horizontal_arrows: true,
                    vertical_arrows: true,
                    ..Default::default()
                },
            )
        });

        let cursor_index = state
            .cursor()
            .and_then(|c| rows.iter().position(|r| &r.info.id == c));

        let shift_held = self.settings.allow_multi_select && ui.input(|i| i.modifiers.shift_only());

        let select_index = |index: usize,
                            state: &mut TreeViewState<Id_>,
                            actions: &mut Vec<Action<Id_>>,
                            ui: &Ui| {
            let row = &rows[index];
            if shift_held && state.pivot().is_some() {
                let pivot = state.pivot().cloned().unwrap();
                state.set_selected(range_ids(rows, &pivot, &row.info.id));
            } else {
                state.set_selected(vec![row.info.id.clone()]);
                state.set_pivot(Some(row.info.id.clone()));
            }
            state.set_cursor(Some(row.info.id.clone()));
            actions.push(Action::SetSelected(selected_infos(state, rows)));
            ui.scroll_to_rect(row.rect, None);
            ui.ctx().request_repaint();
        };

        if consume_key(ui, Key::ArrowDown) {
            let next = cursor_index.map_or(0, |i| (i + 1).min(rows.len() - 1));
            select_index(next, state, actions, ui);
        }
        if consume_key(ui, Key::ArrowUp) {
            let next = cursor_index.map_or(0, |i| i.saturating_sub(1));
            select_index(next, state, actions, ui);
        }
        if consume_key(ui, Key::ArrowRight)
            && let Some(i) = cursor_index
        {
            let row = &rows[i];
            if row.info.is_dir && !row.open {
                self.toggle_dir(ui, state, row, actions);
            } else if i + 1 < rows.len() {
                select_index(i + 1, state, actions, ui);
            }
        }
        if consume_key(ui, Key::ArrowLeft)
            && let Some(i) = cursor_index
        {
            let row = &rows[i];
            if row.info.is_dir && row.open {
                self.toggle_dir(ui, state, row, actions);
            } else if row.depth > 0 {
                // Jump to the parent: nearest earlier row one level up.
                if let Some(parent) = rows[..i].iter().rposition(|r| r.depth == row.depth - 1) {
                    select_index(parent, state, actions, ui);
                }
            }
        }
        if ui.input_mut(|i| i.consume_key(Modifiers::NONE, Key::Enter)) {
            actions.push(Action::Activate {
                nodes: selected_infos(state, rows),
                modifiers: Modifiers::NONE,
            });
        }
    }

    fn apply_reveal<Id_: NodeId>(
        &self,
        ui: &Ui,
        state: &mut TreeViewState<Id_>,
        reveal: Option<builder::RevealMatch<Id_>>,
        actions: &mut Vec<Action<Id_>>,
    ) {
        let Some(reveal) = reveal else { return };
        for ancestor in reveal.ancestors {
            state.set_openness(ancestor, true);
        }
        if reveal.select {
            state.set_selected(vec![reveal.id.clone()]);
            state.set_pivot(Some(reveal.id.clone()));
            state.set_cursor(Some(reveal.id.clone()));
            actions.push(Action::SetSelected(vec![NodeInfo {
                id: reveal.id.clone(),
                is_dir: reveal.is_dir,
            }]));
        }
        state.request_scroll_to(reveal.id);
        ui.ctx().request_repaint();
    }
}

fn consume_key(ui: &Ui, key: Key) -> bool {
    ui.ctx()
        .input_mut(|i| i.consume_key(Modifiers::NONE, key) || i.consume_key(Modifiers::SHIFT, key))
}

fn selected_infos<Id_: NodeId>(
    state: &TreeViewState<Id_>,
    rows: &[Row<Id_>],
) -> Vec<NodeInfo<Id_>> {
    state
        .selected()
        .iter()
        .map(|id| {
            let is_dir = rows
                .iter()
                .find(|r| &r.info.id == id)
                .map(|r| r.info.is_dir)
                .unwrap_or(false);
            NodeInfo {
                id: id.clone(),
                is_dir,
            }
        })
        .collect()
}

fn info_for<Id_: NodeId>(rows: &[Row<Id_>], id: &Id_) -> NodeInfo<Id_> {
    NodeInfo {
        id: id.clone(),
        is_dir: rows
            .iter()
            .find(|r| &r.info.id == id)
            .map(|r| r.info.is_dir)
            .unwrap_or(false),
    }
}

fn source_infos<Id_: NodeId>(state: &TreeViewState<Id_>, rows: &[Row<Id_>]) -> Vec<NodeInfo<Id_>> {
    state
        .drag
        .as_ref()
        .map(|d| d.sources.iter().map(|id| info_for(rows, id)).collect())
        .unwrap_or_default()
}

/// The selected rows with descendants of selected directories removed —
/// moving the directory moves them implicitly.
fn simplify_sources<Id_: NodeId>(rows: &[Row<Id_>], state: &TreeViewState<Id_>) -> Vec<Id_> {
    let mut sources = Vec::new();
    let mut skip_deeper_than: Option<usize> = None;
    for row in rows {
        if let Some(depth) = skip_deeper_than {
            if row.depth > depth {
                continue; // Descendant of an already-dragged directory.
            }
            skip_deeper_than = None;
        }
        if state.is_selected(&row.info.id) {
            sources.push(row.info.id.clone());
            if row.info.is_dir {
                skip_deeper_than = Some(row.depth);
            }
        }
    }
    sources
}

fn range_ids<Id_: NodeId>(rows: &[Row<Id_>], a: &Id_, b: &Id_) -> Vec<Id_> {
    let ia = rows.iter().position(|r| &r.info.id == a);
    let ib = rows.iter().position(|r| &r.info.id == b);
    match (ia, ib) {
        (Some(ia), Some(ib)) => {
            let (lo, hi) = (ia.min(ib), ia.max(ib));
            rows[lo..=hi].iter().map(|r| r.info.id.clone()).collect()
        }
        _ => vec![b.clone()],
    }
}

pub(crate) fn row_metrics(ui: &Ui, settings: &TreeViewSettings) -> RowMetrics {
    let row_height = settings
        .row_height
        .unwrap_or_else(|| ui.spacing().interact_size.y);
    RowMetrics {
        row_height,
        indent: settings
            .indent
            .unwrap_or_else(|| ui.spacing().indent * 0.75),
        closer_width: ui.spacing().icon_width,
        icon_size: settings.icon_size.unwrap_or(row_height * 0.7),
        gap: ui.spacing().item_spacing.x.max(4.0),
    }
}

#[derive(Clone, Copy)]
pub(crate) struct RowMetrics {
    pub row_height: f32,
    pub indent: f32,
    pub closer_width: f32,
    pub icon_size: f32,
    pub gap: f32,
}

impl RowMetrics {
    pub fn icon_rect_in(&self, slot: Rect) -> Rect {
        Rect::from_center_size(slot.center(), Vec2::splat(self.icon_size))
    }
}
