use egui::{
    Align2, Color32, CornerRadius, FontId, ImageSource, Painter, Rect, Stroke, StrokeKind, Ui,
    Visuals, emath::Rot2, pos2, vec2,
};

/// The built-in painted icon set.
///
/// Every icon is drawn with epaint primitives — theme-aware, crisp at any
/// scale and DPI, no font-glyph or image-asset dependency, wasm-safe.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Icon {
    FolderClosed,
    FolderOpen,
    CrateClosed,
    CrateOpen,
    /// A generic file: page with a folded corner.
    File,
    FileRust,
    FilePdf,
    FileHtml,
    FileMarkdown,
    FileImage,
}

impl Icon {
    /// Folder with automatic open/closed variant switching.
    pub fn folder() -> IconSpec<'static> {
        IconSpec::new(Icon::FolderClosed).open(Icon::FolderOpen)
    }

    /// Rust crate with automatic open/closed variant switching.
    pub fn rust_crate() -> IconSpec<'static> {
        IconSpec::new(Icon::CrateClosed).open(Icon::CrateOpen)
    }
}

/// State handed to custom icon painters, so they never guess metrics.
#[derive(Clone, Copy, Debug)]
pub struct IconContext {
    /// The icon slot, already centered in the row.
    pub rect: Rect,
    pub is_dir: bool,
    pub is_open: bool,
    pub is_selected: bool,
    pub is_hovered: bool,
}

/// A boxed custom icon painter.
pub type IconPainter<'a> = Box<dyn FnMut(&mut Ui, IconContext) + 'a>;

/// Where an icon comes from: the built-in painted set, an image, or a custom
/// painter closure.
pub enum IconSource<'a> {
    Painted(Icon),
    Image(ImageSource<'a>),
    Custom(IconPainter<'a>),
}

impl<'a> IconSource<'a> {
    pub fn custom(paint: impl FnMut(&mut Ui, IconContext) + 'a) -> Self {
        IconSource::Custom(Box::new(paint))
    }
}

impl From<Icon> for IconSource<'_> {
    fn from(icon: Icon) -> Self {
        IconSource::Painted(icon)
    }
}

impl<'a> From<ImageSource<'a>> for IconSource<'a> {
    fn from(src: ImageSource<'a>) -> Self {
        IconSource::Image(src)
    }
}

/// A node's icon: the closed (or only) source, plus an optional variant shown
/// while the node is an open directory.
pub struct IconSpec<'a> {
    pub(crate) closed: IconSource<'a>,
    pub(crate) open: Option<IconSource<'a>>,
}

impl<'a> IconSpec<'a> {
    pub fn new(closed: impl Into<IconSource<'a>>) -> Self {
        Self {
            closed: closed.into(),
            open: None,
        }
    }

    /// The variant shown while the directory is open.
    pub fn open(mut self, open: impl Into<IconSource<'a>>) -> Self {
        self.open = Some(open.into());
        self
    }

    /// A purely custom-painted icon.
    pub fn painter(paint: impl FnMut(&mut Ui, IconContext) + 'a) -> Self {
        Self {
            closed: IconSource::custom(paint),
            open: None,
        }
    }
}

impl<'a, T: Into<IconSource<'a>>> From<T> for IconSpec<'a> {
    fn from(src: T) -> Self {
        IconSpec::new(src)
    }
}

/// The animated open/close triangle in front of directories.
pub(crate) fn paint_closer(painter: &Painter, rect: Rect, openness: f32, color: Color32) {
    let center = rect.center();
    let size = rect.width().min(rect.height()) * 0.32;
    let rot = Rot2::from_angle(openness * std::f32::consts::FRAC_PI_2);
    // Right-pointing triangle, rotated toward down-pointing as it opens.
    let points = [
        vec2(size, 0.0),
        vec2(-size * 0.5, size * 0.87),
        vec2(-size * 0.5, -size * 0.87),
    ]
    .iter()
    .map(|v| center + rot * *v)
    .collect();
    painter.add(egui::Shape::convex_polygon(points, color, Stroke::NONE));
}

fn tone(visuals: &Visuals, light: Color32, dark: Color32) -> Color32 {
    if visuals.dark_mode { dark } else { light }
}

pub(crate) fn paint_icon(icon: Icon, painter: &Painter, rect: Rect, visuals: &Visuals) {
    match icon {
        Icon::FolderClosed => folder(painter, rect, visuals, false),
        Icon::FolderOpen => folder(painter, rect, visuals, true),
        Icon::CrateClosed => krate(painter, rect, visuals, false),
        Icon::CrateOpen => krate(painter, rect, visuals, true),
        Icon::File => {
            page(painter, rect, visuals);
        }
        Icon::FileRust => {
            page(painter, rect, visuals);
            letter(
                painter,
                rect,
                "R",
                tone(
                    visuals,
                    Color32::from_rgb(0xB4, 0x54, 0x14),
                    Color32::from_rgb(0xF0, 0x8A, 0x3C),
                ),
            );
        }
        Icon::FilePdf => {
            page(painter, rect, visuals);
            letter(
                painter,
                rect,
                "P",
                tone(
                    visuals,
                    Color32::from_rgb(0xC0, 0x32, 0x32),
                    Color32::from_rgb(0xE8, 0x5A, 0x5A),
                ),
            );
        }
        Icon::FileHtml => {
            page(painter, rect, visuals);
            chevrons(
                painter,
                rect,
                tone(
                    visuals,
                    Color32::from_rgb(0xC8, 0x4A, 0x1E),
                    Color32::from_rgb(0xF0, 0x7A, 0x46),
                ),
            );
        }
        Icon::FileMarkdown => {
            page(painter, rect, visuals);
            letter(
                painter,
                rect,
                "M",
                tone(
                    visuals,
                    Color32::from_rgb(0x2E, 0x6E, 0xB4),
                    Color32::from_rgb(0x64, 0xA4, 0xE8),
                ),
            );
        }
        Icon::FileImage => {
            page(painter, rect, visuals);
            picture(painter, rect, visuals);
        }
    }
}

fn folder(painter: &Painter, rect: Rect, visuals: &Visuals, open: bool) {
    let fill = tone(
        visuals,
        Color32::from_rgb(0xC8, 0x92, 0x2E),
        Color32::from_rgb(0xE0, 0xA6, 0x4E),
    );
    let (l, t, r, b) = (rect.left(), rect.top(), rect.right(), rect.bottom());
    let h = rect.height();
    let w = rect.width();
    // Tab.
    painter.rect_filled(
        Rect::from_min_max(pos2(l, t + 0.08 * h), pos2(l + 0.45 * w, t + 0.40 * h)),
        CornerRadius::same(1),
        fill.gamma_multiply(0.85),
    );
    if open {
        // Back panel, then a tilted front flap.
        painter.rect_filled(
            Rect::from_min_max(pos2(l, t + 0.25 * h), pos2(r - 0.12 * w, b)),
            CornerRadius::same(1),
            fill.gamma_multiply(0.6),
        );
        painter.add(egui::Shape::convex_polygon(
            vec![
                pos2(l + 0.18 * w, t + 0.45 * h),
                pos2(r, t + 0.45 * h),
                pos2(r - 0.14 * w, b),
                pos2(l + 0.04 * w, b),
            ],
            fill,
            Stroke::NONE,
        ));
    } else {
        painter.rect_filled(
            Rect::from_min_max(pos2(l, t + 0.25 * h), pos2(r, b)),
            CornerRadius::same(1),
            fill,
        );
    }
}

fn krate(painter: &Painter, rect: Rect, visuals: &Visuals, open: bool) {
    let fill = tone(
        visuals,
        Color32::from_rgb(0xA8, 0x6A, 0x38),
        Color32::from_rgb(0xC8, 0x8A, 0x50),
    );
    let edge = Stroke::new(1.0, fill.gamma_multiply(0.55));
    let (l, r, b) = (rect.left(), rect.right(), rect.bottom());
    let h = rect.height();
    let w = rect.width();
    let box_top = rect.top() + if open { 0.35 } else { 0.18 } * h;
    let body = Rect::from_min_max(pos2(l, box_top), pos2(r, b));
    painter.rect_filled(body, CornerRadius::same(1), fill);
    // Slats.
    for f in [0.38, 0.72] {
        let y = body.top() + f * body.height();
        painter.line_segment([pos2(l + 1.0, y), pos2(r - 1.0, y)], edge);
    }
    painter.rect_stroke(body, CornerRadius::same(1), edge, StrokeKind::Inside);
    if open {
        // Lid flaps folded outward.
        for (x0, x1) in [(l, l + 0.42 * w), (r - 0.42 * w, r)] {
            painter.add(egui::Shape::convex_polygon(
                vec![
                    pos2(x0, box_top),
                    pos2(x1, box_top),
                    pos2((x0 + x1) * 0.5, box_top - 0.28 * h),
                ],
                fill.gamma_multiply(0.8),
                Stroke::NONE,
            ));
        }
    }
}

fn page(painter: &Painter, rect: Rect, visuals: &Visuals) {
    let outline = Stroke::new(1.0, visuals.widgets.noninteractive.fg_stroke.color);
    let fill = visuals.extreme_bg_color;
    let (l, t, r, b) = (rect.left(), rect.top(), rect.right(), rect.bottom());
    let inset = rect.width() * 0.10;
    let (l, r) = (l + inset, r - inset);
    let fold = (r - l) * 0.32;
    painter.add(egui::Shape::convex_polygon(
        vec![
            pos2(l, t),
            pos2(r - fold, t),
            pos2(r, t + fold),
            pos2(r, b),
            pos2(l, b),
        ],
        fill,
        outline,
    ));
    // Folded corner.
    painter.add(egui::Shape::convex_polygon(
        vec![
            pos2(r - fold, t),
            pos2(r, t + fold),
            pos2(r - fold, t + fold),
        ],
        outline.color.gamma_multiply(0.5),
        Stroke::NONE,
    ));
}

fn letter(painter: &Painter, rect: Rect, s: &str, color: Color32) {
    painter.text(
        rect.center() + vec2(0.0, rect.height() * 0.12),
        Align2::CENTER_CENTER,
        s,
        FontId::proportional(rect.height() * 0.55),
        color,
    );
}

fn chevrons(painter: &Painter, rect: Rect, color: Color32) {
    let c = rect.center() + vec2(0.0, rect.height() * 0.14);
    let s = rect.width() * 0.16;
    let stroke = Stroke::new(1.2, color);
    for (dir, off) in [(-1.0, -0.6), (1.0, 0.6)] {
        let apex = c + vec2(dir * s * 1.6 + off * s, 0.0);
        let back = -dir * s;
        painter.line_segment([apex + vec2(back, -s), apex], stroke);
        painter.line_segment([apex, apex + vec2(back, s)], stroke);
    }
}

fn picture(painter: &Painter, rect: Rect, visuals: &Visuals) {
    let green = tone(
        visuals,
        Color32::from_rgb(0x3C, 0x8C, 0x50),
        Color32::from_rgb(0x64, 0xB4, 0x78),
    );
    let b = rect.bottom() - rect.height() * 0.14;
    let l = rect.left() + rect.width() * 0.20;
    let r = rect.right() - rect.width() * 0.20;
    let h = rect.height() * 0.34;
    painter.add(egui::Shape::convex_polygon(
        vec![pos2(l, b), pos2((l + r) * 0.5, b - h), pos2(r, b)],
        green,
        Stroke::NONE,
    ));
    painter.circle_filled(
        pos2(r - rect.width() * 0.10, b - h),
        rect.width() * 0.08,
        green.gamma_multiply(0.8),
    );
}
