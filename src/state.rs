use std::collections::HashMap;

use crate::NodeId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum PendingReveal<Id> {
    /// Open every ancestor of the node.
    ExpandParents(Id),
    /// Open ancestors, select the node, and scroll to it.
    Reveal(Id),
}

/// The state a tree view keeps between frames: openness and selection, keyed
/// by node id. Own one per tree (or let [`TreeView::show`](crate::TreeView::show)
/// keep it in egui memory for you).
///
/// With the `persistence` cargo feature the durable parts (openness,
/// selection) serialize; transient interaction state (pending reveals,
/// scroll requests, drags) is skipped.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "persistence",
    serde(bound(
        serialize = "Id: serde::Serialize",
        deserialize = "Id: serde::de::DeserializeOwned + Eq + std::hash::Hash"
    ))
)]
pub struct TreeViewState<Id> {
    openness: HashMap<Id, bool>,
    selected: Vec<Id>,
    pivot: Option<Id>,
    cursor: Option<Id>,
    #[cfg_attr(feature = "persistence", serde(skip))]
    pub(crate) pending_reveal: Option<PendingReveal<Id>>,
    #[cfg_attr(feature = "persistence", serde(skip))]
    scroll_to: Option<Id>,
    #[cfg_attr(feature = "persistence", serde(skip))]
    pub(crate) drag: Option<DragState<Id>>,
    #[cfg_attr(feature = "persistence", serde(skip))]
    pub(crate) context_menu: Option<ContextMenuState<Id>>,
}

/// An in-progress drag: the (simplified) source set and where it started.
#[derive(Clone, Debug)]
pub(crate) struct DragState<Id> {
    /// Dragged nodes with descendants of dragged dirs removed.
    pub sources: Vec<Id>,
    pub start_pos: egui::Pos2,
    /// True once the pointer moved past the drag threshold.
    pub active: bool,
}

/// An open context menu: which node (None = fallback over empty space /
/// selection) and where it was summoned.
#[derive(Clone, Debug)]
pub(crate) struct ContextMenuState<Id> {
    pub node: Option<Id>,
    pub pos: egui::Pos2,
}

impl<Id> Default for TreeViewState<Id> {
    fn default() -> Self {
        Self {
            openness: HashMap::new(),
            selected: Vec::new(),
            pivot: None,
            cursor: None,
            pending_reveal: None,
            scroll_to: None,
            drag: None,
            context_menu: None,
        }
    }
}

impl<Id: NodeId> TreeViewState<Id> {
    pub fn new() -> Self {
        Self::default()
    }

    /// The currently selected node ids, in selection order.
    pub fn selected(&self) -> &[Id] {
        &self.selected
    }

    pub fn set_selected(&mut self, ids: Vec<Id>) {
        self.selected = ids;
    }

    pub(crate) fn toggle_selected(&mut self, id: Id) {
        if let Some(pos) = self.selected.iter().position(|s| *s == id) {
            self.selected.remove(pos);
        } else {
            self.selected.push(id);
        }
    }

    pub fn is_selected(&self, id: &Id) -> bool {
        self.selected.contains(id)
    }

    /// Whether a directory is open. `None` if the tree has never seen the id
    /// (it will use the node's `default_open` on first encounter).
    pub fn is_open(&self, id: &Id) -> Option<bool> {
        self.openness.get(id).copied()
    }

    pub fn set_openness(&mut self, id: Id, open: bool) {
        self.openness.insert(id, open);
    }

    /// Open a directory programmatically.
    pub fn expand(&mut self, id: Id) {
        self.set_openness(id, true);
    }

    /// Close a directory programmatically.
    pub fn collapse(&mut self, id: Id) {
        self.set_openness(id, false);
    }

    /// Open every ancestor of `id` on the next frame, so the node becomes
    /// visible. The parent chain is discovered from the next build pass —
    /// callers don't supply it.
    pub fn expand_parents_of(&mut self, id: Id) {
        self.pending_reveal = Some(PendingReveal::ExpandParents(id));
    }

    /// Open ancestors, select `id`, and scroll it into view on the next frame
    /// — "reveal in tree" as one call.
    pub fn reveal(&mut self, id: Id) {
        self.pending_reveal = Some(PendingReveal::Reveal(id));
    }

    /// Scroll the node into view on the next frame (without changing openness
    /// or selection).
    pub fn scroll_to(&mut self, id: Id) {
        self.scroll_to = Some(id);
    }

    pub(crate) fn request_scroll_to(&mut self, id: Id) {
        self.scroll_to = Some(id);
    }

    pub(crate) fn take_scroll_to(&mut self) -> Option<Id> {
        self.scroll_to.take()
    }

    pub(crate) fn pivot(&self) -> Option<&Id> {
        self.pivot.as_ref()
    }

    pub(crate) fn set_pivot(&mut self, id: Option<Id>) {
        self.pivot = id;
    }

    /// The keyboard cursor node, if any.
    pub fn cursor(&self) -> Option<&Id> {
        self.cursor.as_ref()
    }

    pub(crate) fn set_cursor(&mut self, id: Option<Id>) {
        self.cursor = id;
    }
}
