use egui::Modifiers;

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
}
