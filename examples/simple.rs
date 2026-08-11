//! The minimal tree: dirs, leaves, default look.

use egui_ailanthus::{Node, TreeView};

fn main() -> eframe::Result {
    eframe::run_native(
        "egui_ailanthus — simple",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(App))),
    )
}

struct App;

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            let (_response, actions) = TreeView::new(egui::Id::new("tree")).show(
                ui,
                |tree: &mut egui_ailanthus::TreeBuilder<'_, '_, &'static str>| {
                    tree.dir(Node::new("root").label("Root"));
                    tree.dir(Node::new("src").label("src"));
                    tree.leaf(Node::new("src/lib.rs").label("lib.rs"));
                    tree.leaf(Node::new("src/main.rs").label("main.rs"));
                    tree.close_dir();
                    tree.leaf(Node::new("Cargo.toml").label("Cargo.toml"));
                    tree.leaf(Node::new("README.md").label("README.md"));
                    tree.close_dir();
                },
            );
            for action in actions {
                println!("{action:?}");
            }
        });
    }
}
