//! State persistence across app restarts (`persistence` feature).
//!
//! Run with: `cargo run --example persistence --features persistence`
//! Open/close some directories, select a node, quit, and restart — the tree
//! comes back exactly as you left it.

use egui_ailanthus::{Icon, Node, TreeView};

fn main() -> eframe::Result {
    eframe::run_native(
        "egui_ailanthus — persistence",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(App))),
    )
}

struct App;

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label("Toggle and select, then restart the app — state persists.");
            // `show` with the persistence feature stores state persistently;
            // node ids just need to be serde-serializable (String is).
            let (_response, _actions) = TreeView::new(egui::Id::new("persist")).show(
                ui,
                |tree: &mut egui_ailanthus::TreeBuilder<'_, '_, '_, String>| {
                    for top in ["alpha", "beta", "gamma"] {
                        tree.dir(Node::new(top.to_owned()).label(top).icon(Icon::folder()));
                        for child in ["one", "two", "three"] {
                            tree.leaf(
                                Node::new(format!("{top}/{child}"))
                                    .label(child)
                                    .icon(Icon::File),
                            );
                        }
                        tree.close_dir();
                    }
                },
            );
        });
    }
}
