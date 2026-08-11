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
            let node = |id: &str| Node::new(id.to_owned()).label(id.rsplit('/').next().unwrap());
            let (_response, actions) = TreeView::new(egui::Id::new("tree")).show(
                ui,
                |tree: &mut egui_ailanthus::TreeBuilder<'_, '_, '_, String>| {
                    tree.dir(node("root"));
                    tree.dir(node("src"));
                    tree.leaf(node("src/lib.rs"));
                    tree.leaf(node("src/main.rs"));
                    tree.close_dir();
                    tree.leaf(node("Cargo.toml"));
                    tree.leaf(node("README.md"));
                    tree.close_dir();
                },
            );
            for action in actions {
                println!("{action:?}");
            }
        });
    }
}
