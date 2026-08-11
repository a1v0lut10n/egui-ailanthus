//! The decoration showcase: file-type icons, open/closed folders and crates,
//! badges, trailing widgets, and a custom row-paint hook.

use egui::Color32;
use egui_ailanthus::{Badge, Icon, Node, TreeView, TreeViewState};

fn main() -> eframe::Result {
    eframe::run_native(
        "egui_ailanthus — decorated file tree",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(App::default()))),
    )
}

#[derive(Default)]
struct App {
    state: TreeViewState<&'static str>,
    last_activated: Option<&'static str>,
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::left(egui::Id::new("tree-panel"))
            .default_size(280.0)
            .show(ui, |ui| {
                egui::ScrollArea::both()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        self.tree(ui);
                    });
            });
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("egui_ailanthus");
            if ui.button("Reveal deep node").clicked() {
                self.state.reveal("crate/src/parser/expr.rs");
            }
            match self.last_activated {
                Some(id) => ui.label(format!("Activated: {id}")),
                None => ui.label("Double-click or press Enter on a file."),
            };
        });
    }
}

impl App {
    fn tree(&mut self, ui: &mut egui::Ui) {
        let (_response, actions) = TreeView::new(egui::Id::new("decorated"))
            .striped(true)
            .show_state(ui, &mut self.state, |tree| {
                tree.dir(Node::new("docs").label("docs").icon(Icon::folder()));
                tree.leaf(
                    Node::new("docs/report.pdf")
                        .label("report.pdf")
                        .icon(Icon::FilePdf)
                        .trailing(|ui| {
                            ui.weak("2.4 MB");
                        }),
                );
                tree.leaf(
                    Node::new("docs/index.html")
                        .label("index.html")
                        .icon(Icon::FileHtml),
                );
                tree.leaf(
                    Node::new("docs/notes.md")
                        .label("notes.md")
                        .icon(Icon::FileMarkdown)
                        .badge(Badge::dot(Color32::ORANGE)) // e.g. "modified"
                        .trailing(|ui| {
                            ui.weak("edited");
                        }),
                );
                tree.leaf(
                    Node::new("docs/logo.png")
                        .label("logo.png")
                        .icon(Icon::FileImage),
                );
                tree.close_dir();

                tree.dir(
                    Node::new("crate")
                        .label("ailanthus (crate)")
                        .icon(Icon::rust_crate()),
                );
                tree.dir(Node::new("crate/src").label("src").icon(Icon::folder()));
                tree.leaf(
                    Node::new("crate/src/lib.rs")
                        .label("lib.rs")
                        .icon(Icon::FileRust),
                );
                tree.dir(
                    Node::new("crate/src/parser")
                        .label("parser")
                        .icon(Icon::folder())
                        .default_open(false),
                );
                tree.leaf(
                    Node::new("crate/src/parser/expr.rs")
                        .label("expr.rs")
                        .icon(Icon::FileRust)
                        .badge(Badge::dot(Color32::LIGHT_GREEN)),
                );
                tree.close_dir();
                tree.close_dir();
                tree.leaf(
                    Node::new("crate/Cargo.toml")
                        .label("Cargo.toml")
                        .icon(Icon::File)
                        // A row-paint hook: underline this row with an accent.
                        .row_paint(|painter, row| {
                            painter.line_segment(
                                [row.rect.left_bottom(), row.rect.right_bottom()],
                                egui::Stroke::new(1.0, Color32::from_rgb(0x64, 0xA4, 0xE8)),
                            );
                        }),
                );
                tree.close_dir();
            });

        for action in actions {
            if let egui_ailanthus::Action::Activate { nodes, .. } = action
                && let Some(node) = nodes.iter().find(|n| !n.is_dir)
            {
                self.last_activated = Some(node.id);
            }
        }
    }
}
