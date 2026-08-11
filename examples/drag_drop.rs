//! Drag & drop and context menus: the tree reports `Action::Move`, the app
//! applies it to its own model. Also the demo target for the egui MCP
//! inspection server:
//!
//! ```sh
//! EGUI_INSPECTION=1 cargo run --example drag_drop
//! ```

use egui_ailanthus::{Action, DirPosition, Icon, Node, TreeView, TreeViewState};

fn main() -> eframe::Result {
    eframe::run_native(
        "egui_ailanthus — drag & drop",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(App::new()))),
    )
}

struct Entry {
    id: u32,
    name: String,
    is_dir: bool,
    children: Vec<Entry>,
}

impl Entry {
    fn leaf(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_owned(),
            is_dir: false,
            children: Vec::new(),
        }
    }

    fn dir(id: u32, name: &str, children: Vec<Entry>) -> Self {
        Self {
            id,
            name: name.to_owned(),
            is_dir: true,
            children,
        }
    }
}

struct App {
    roots: Vec<Entry>,
    state: TreeViewState<u32>,
    status: String,
}

impl App {
    fn new() -> Self {
        Self {
            roots: vec![
                Entry::dir(
                    1,
                    "projects",
                    vec![
                        Entry::dir(2, "ailanthus", vec![Entry::leaf(3, "lib.rs")]),
                        Entry::leaf(4, "notes.md"),
                    ],
                ),
                Entry::dir(5, "archive", vec![Entry::leaf(6, "report.pdf")]),
                Entry::leaf(7, "todo.md"),
            ],
            state: TreeViewState::new(),
            status: "Drag nodes around; right-click for menus.".to_owned(),
        }
    }

    /// Remove the entries with these ids (wherever they are) and return them.
    fn remove_ids(entries: &mut Vec<Entry>, ids: &[u32], removed: &mut Vec<Entry>) {
        let mut i = 0;
        while i < entries.len() {
            if ids.contains(&entries[i].id) {
                removed.push(entries.remove(i));
            } else {
                Self::remove_ids(&mut entries[i].children, ids, removed);
                i += 1;
            }
        }
    }

    fn find_dir_mut(entries: &mut Vec<Entry>, id: u32) -> Option<&mut Entry> {
        for entry in entries {
            if entry.id == id {
                return Some(entry);
            }
            if let Some(found) = Self::find_dir_mut(&mut entry.children, id) {
                return Some(found);
            }
        }
        None
    }

    fn apply_move(&mut self, dnd: &egui_ailanthus::DragAndDrop<u32>) {
        let ids: Vec<u32> = dnd.sources.iter().map(|s| s.id).collect();
        let mut moved = Vec::new();
        Self::remove_ids(&mut self.roots, &ids, &mut moved);

        let target = match &dnd.target_dir {
            Some(dir) => match Self::find_dir_mut(&mut self.roots, dir.id) {
                Some(entry) => &mut entry.children,
                None => &mut self.roots,
            },
            None => &mut self.roots,
        };
        let index = match &dnd.position {
            DirPosition::First => 0,
            DirPosition::Last => target.len(),
            DirPosition::Before(sibling) => target
                .iter()
                .position(|e| e.id == *sibling)
                .unwrap_or(target.len()),
            DirPosition::After(sibling) => target
                .iter()
                .position(|e| e.id == *sibling)
                .map(|i| i + 1)
                .unwrap_or(target.len()),
        };
        for (offset, entry) in moved.into_iter().enumerate() {
            target.insert((index + offset).min(target.len()), entry);
        }
        self.status = format!("Moved {ids:?} → {:?}", dnd.position);
    }

    fn build_level(tree: &mut egui_ailanthus::TreeBuilder<'_, '_, '_, u32>, entries: &[Entry]) {
        for entry in entries {
            if entry.is_dir {
                tree.dir(
                    Node::new(entry.id)
                        .label(&entry.name)
                        .icon(Icon::folder())
                        .context_menu(|ui| {
                            let _ = ui.button("New file…");
                            let _ = ui.button("Rename…");
                        }),
                );
                Self::build_level(tree, &entry.children);
                tree.close_dir();
            } else {
                tree.leaf(
                    Node::new(entry.id)
                        .label(&entry.name)
                        .icon(match entry.name.rsplit('.').next() {
                            Some("rs") => Icon::FileRust,
                            Some("pdf") => Icon::FilePdf,
                            Some("md") => Icon::FileMarkdown,
                            _ => Icon::File,
                        }),
                );
            }
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label(&self.status);
            ui.separator();
            let (_response, actions) = TreeView::new(egui::Id::new("dnd"))
                .fallback_context_menu(|ui| {
                    let _ = ui.button("Add root item…");
                })
                .show_state(ui, &mut self.state, |tree| {
                    Self::build_level(tree, &self.roots);
                });
            // Example veto: nothing may be dropped into "archive" (id 5).
            let into_archive = |dnd: &egui_ailanthus::DragAndDrop<u32>| {
                dnd.target_dir.as_ref().is_some_and(|t| t.id == 5)
            };
            for action in actions {
                match action {
                    Action::Move(dnd) if into_archive(&dnd) => {
                        self.status = "archive is read-only".to_owned();
                    }
                    Action::Move(dnd) => self.apply_move(&dnd),
                    Action::Drag(dnd) if into_archive(&dnd) => dnd.remove_marker(ui),
                    _ => {}
                }
            }
        });
    }
}
