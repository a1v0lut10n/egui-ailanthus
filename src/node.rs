use egui::{Color32, Painter, Rect, Ui, WidgetText};

use crate::icon::IconSpec;

/// Boxed trailing-widgets closure.
pub type TrailingFn<'a> = Box<dyn FnMut(&mut Ui) + 'a>;
/// Boxed row-paint hook.
pub type RowPaintFn<'a> = Box<dyn FnMut(&Painter, &RowContext) + 'a>;

/// Everything the row painter knows about a row, handed to the
/// [`row_paint`](Node::row_paint) hook so custom decoration can react to
/// state without guessing geometry.
#[derive(Clone, Copy, Debug)]
pub struct RowContext {
    /// The full row rectangle (indent included).
    pub rect: Rect,
    /// Tree depth, root nodes are 0.
    pub depth: usize,
    pub is_dir: bool,
    pub is_open: bool,
    pub is_selected: bool,
    pub is_hovered: bool,
}

/// A badge overlaid on a node's icon — a status dot in the icon's corner.
#[derive(Clone, Copy, Debug)]
pub struct Badge {
    pub color: Color32,
}

impl Badge {
    /// A colored status dot.
    pub fn dot(color: Color32) -> Self {
        Self { color }
    }
}

/// Per-node configuration, built with a fluent API and passed to
/// [`TreeBuilder::dir`](crate::TreeBuilder::dir) or
/// [`TreeBuilder::leaf`](crate::TreeBuilder::leaf).
pub struct Node<'a, Id> {
    pub(crate) id: Id,
    pub(crate) label: WidgetText,
    pub(crate) icon: Option<IconSpec<'a>>,
    pub(crate) badge: Option<Badge>,
    pub(crate) trailing: Option<TrailingFn<'a>>,
    pub(crate) row_paint: Option<RowPaintFn<'a>>,
    pub(crate) default_open: bool,
}

impl<'a, Id> Node<'a, Id> {
    pub fn new(id: Id) -> Self {
        Self {
            id,
            label: WidgetText::default(),
            icon: None,
            badge: None,
            trailing: None,
            row_paint: None,
            default_open: true,
        }
    }

    pub fn label(mut self, label: impl Into<WidgetText>) -> Self {
        self.label = label.into();
        self
    }

    /// The leading icon. Accepts an [`Icon`](crate::Icon), an
    /// [`IconSpec`](crate::IconSpec) with open/closed variants, or an
    /// `egui::ImageSource`.
    pub fn icon(mut self, icon: impl Into<IconSpec<'a>>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Overlay a badge on the icon's lower-right corner.
    pub fn badge(mut self, badge: Badge) -> Self {
        self.badge = Some(badge);
        self
    }

    /// Widgets rendered right-aligned in the row (sizes, counts, buttons).
    pub fn trailing(mut self, add: impl FnMut(&mut Ui) + 'a) -> Self {
        self.trailing = Some(Box::new(add));
        self
    }

    /// Paint hook running before the row content, over the row background —
    /// the escape hatch for arbitrary direct-rendered decoration.
    pub fn row_paint(mut self, paint: impl FnMut(&Painter, &RowContext) + 'a) -> Self {
        self.row_paint = Some(Box::new(paint));
        self
    }

    /// Whether a directory starts open the first time the tree sees it
    /// (default `true`).
    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }
}
