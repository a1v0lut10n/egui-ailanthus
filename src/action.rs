use egui::{Modifiers, Pos2, Shape, Ui, layers::ShapeIdx};

/// A node reference in an action payload: the id plus whether the node was
/// declared a directory this frame — so click handlers never need to re-derive
/// dir-vs-leaf from the id.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeInfo<Id> {
    pub id: Id,
    pub is_dir: bool,
}

/// Everything the tree wants the application to know about this frame,
/// returned from [`TreeView::show`](crate::TreeView::show) /
/// [`show_state`](crate::TreeView::show_state).
#[derive(Clone, Debug)]
pub enum Action<Id> {
    /// The selection changed (click or keyboard).
    SetSelected(Vec<NodeInfo<Id>>),
    /// A node was activated (Enter or double-click on a leaf).
    Activate {
        nodes: Vec<NodeInfo<Id>>,
        modifiers: Modifiers,
    },
    /// A directory was opened by the user. Lazy loaders react to this instead
    /// of threading mutable state through the build closure.
    DirOpened(NodeInfo<Id>),
    /// A directory was closed by the user.
    DirClosed(NodeInfo<Id>),
    /// An in-progress drag hovering a valid drop position. Emitted every
    /// frame while dragging; call [`DragAndDrop::remove_marker`] to veto the
    /// shown drop marker.
    Drag(DragAndDrop<Id>),
    /// A completed drop: the application should move `sources` to
    /// `position` under `target_dir`. Ignore it to reject the move.
    Move(DragAndDrop<Id>),
    /// Nodes were dragged out of the tree and released elsewhere.
    MoveExternal {
        sources: Vec<NodeInfo<Id>>,
        pos: Pos2,
    },
}

/// Where dragged nodes would be inserted relative to their new parent.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DirPosition<Id> {
    /// As the first child.
    First,
    /// As the last child.
    Last,
    /// Immediately before this sibling.
    Before(Id),
    /// Immediately after this sibling.
    After(Id),
}

/// Payload of [`Action::Drag`] and [`Action::Move`].
#[derive(Clone, Debug)]
pub struct DragAndDrop<Id> {
    /// The dragged nodes, simplified: descendants of dragged directories are
    /// removed (moving the directory moves them implicitly).
    pub sources: Vec<NodeInfo<Id>>,
    /// The directory receiving the nodes; `None` means the root level.
    pub target_dir: Option<NodeInfo<Id>>,
    pub position: DirPosition<Id>,
    marker: ShapeIdx,
}

impl<Id> DragAndDrop<Id> {
    pub(crate) fn new(
        sources: Vec<NodeInfo<Id>>,
        target_dir: Option<NodeInfo<Id>>,
        position: DirPosition<Id>,
        marker: ShapeIdx,
    ) -> Self {
        Self {
            sources,
            target_dir,
            position,
            marker,
        }
    }

    /// Remove the drop-marker highlight for this frame — call while handling
    /// [`Action::Drag`] to signal that this drop position is not allowed.
    pub fn remove_marker(&self, ui: &Ui) {
        ui.painter().set(self.marker, Shape::Noop);
    }
}
