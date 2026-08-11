//! Lazy child loading via `Action::DirOpened` — no mutable borrows inside the
//! build closure, no pending-list plumbing.

use std::collections::HashMap;

use egui_ailanthus::{Action, Icon, Node, TreeView, TreeViewState};

fn main() -> eframe::Result {
    eframe::run_native(
        "egui_ailanthus — lazy loading",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(App::new()))),
    )
}

struct App {
    state: TreeViewState<String>,
    /// dir id -> loaded children (name, is_dir). `None` = not loaded yet.
    children: HashMap<String, Vec<(String, bool)>>,
}

impl App {
    fn new() -> Self {
        let mut children = HashMap::new();
        children.insert(
            "/".to_owned(),
            vec![("a".to_owned(), true), ("b".to_owned(), true), ("readme".to_owned(), false)],
        );
        Self {
            state: TreeViewState::new(),
            children,
        }
    }

    /// Pretend to hit the filesystem/network for a directory's children.
    fn load(&mut self, dir: &str) {
        let depth = dir.chars().filter(|c| *c == '/').count();
        let entries = if depth >= 4 {
            vec![("leaf".to_owned(), false)]
        } else {
            vec![
                ("x".to_owned(), true),
                ("y".to_owned(), true),
                ("data".to_owned(), false),
            ]
        };
        self.children.insert(dir.to_owned(), entries);
    }

    fn build_level(
        tree: &mut egui_ailanthus::TreeBuilder<'_, '_, String>,
        children: &HashMap<String, Vec<(String, bool)>>,
        dir: &str,
    ) {
        let Some(entries) = children.get(dir) else {
            // Not loaded yet: show a placeholder row until DirOpened loads it.
            tree.leaf(Node::new(format!("{dir}…")).label("loading…"));
            return;
        };
        for (name, is_dir) in entries {
            let id = format!("{dir}{name}");
            if *is_dir {
                let id_slash = format!("{id}/");
                tree.dir(
                    Node::new(id_slash.clone())
                        .label(name)
                        .icon(Icon::folder())
                        .default_open(false),
                );
                Self::build_level(tree, children, &id_slash);
                tree.close_dir();
            } else {
                tree.leaf(Node::new(id).label(name).icon(Icon::File));
            }
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label("Every directory loads its children the first time it is opened.");
            let (_response, actions) =
                TreeView::new(egui::Id::new("lazy")).show_state(ui, &mut self.state, |tree| {
                    tree.dir(Node::new("/".to_owned()).label("root").icon(Icon::folder()));
                    Self::build_level(tree, &self.children, "/");
                    tree.close_dir();
                });

            for action in actions {
                if let Action::DirOpened(node) = action
                    && !self.children.contains_key(&node.id)
                {
                    self.load(&node.id);
                }
            }
        });
    }
}
