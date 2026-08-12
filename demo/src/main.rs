//! The egui_ailanthus playground: every setting and decoration slot, with a
//! live action log. Runs natively (`cargo run -p egui_ailanthus_demo`) and on
//! the web (`trunk serve` in `demo/`).

use egui_ailanthus::{
    Action, Badge, DirPosition, Icon, Node, TreeView, TreeViewSettings, TreeViewState,
};

struct Entry {
    id: u32,
    name: String,
    is_dir: bool,
    is_crate: bool,
    children: Vec<Entry>,
}

impl Entry {
    fn leaf(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_owned(),
            is_dir: false,
            is_crate: false,
            children: Vec::new(),
        }
    }

    fn dir(id: u32, name: &str, children: Vec<Entry>) -> Self {
        Self {
            id,
            name: name.to_owned(),
            is_dir: true,
            is_crate: false,
            children,
        }
    }

    fn krate(id: u32, name: &str, children: Vec<Entry>) -> Self {
        Self {
            is_crate: true,
            ..Self::dir(id, name, children)
        }
    }

    fn all_dir_ids(&self, out: &mut Vec<u32>) {
        if self.is_dir {
            out.push(self.id);
        }
        for child in &self.children {
            child.all_dir_ids(out);
        }
    }
}

const DEEP_LEAF: u32 = 12;
const ARCHIVE_DIR: u32 = 20;

fn demo_model() -> Vec<Entry> {
    vec![
        Entry::dir(
            1,
            "workspace",
            vec![
                Entry::krate(
                    10,
                    "ailanthus",
                    vec![
                        Entry::dir(
                            11,
                            "src",
                            vec![Entry::leaf(DEEP_LEAF, "lib.rs"), Entry::leaf(13, "icon.rs")],
                        ),
                        Entry::leaf(14, "Cargo.toml"),
                    ],
                ),
                Entry::dir(
                    15,
                    "docs",
                    vec![
                        Entry::leaf(16, "report.pdf"),
                        Entry::leaf(17, "index.html"),
                        Entry::leaf(18, "notes.md"),
                        Entry::leaf(19, "logo.png"),
                    ],
                ),
            ],
        ),
        Entry::dir(
            ARCHIVE_DIR,
            "archive (read-only)",
            vec![Entry::leaf(21, "old.pdf")],
        ),
        Entry::leaf(22, "todo.md"),
    ]
}

fn icon_for(name: &str) -> Icon {
    match name.rsplit('.').next() {
        Some("rs") => Icon::FileRust,
        Some("pdf") => Icon::FilePdf,
        Some("html") => Icon::FileHtml,
        Some("md") => Icon::FileMarkdown,
        Some("png") => Icon::FileImage,
        _ => Icon::File,
    }
}

struct DemoApp {
    roots: Vec<Entry>,
    state: TreeViewState<u32>,
    settings: TreeViewSettings,
    log: Vec<String>,
}

impl DemoApp {
    fn new() -> Self {
        Self {
            roots: demo_model(),
            state: TreeViewState::new(),
            settings: TreeViewSettings {
                striped: true,
                ..Default::default()
            },
            log: vec!["Interact with the tree — actions appear here.".to_owned()],
        }
    }

    fn push_log(&mut self, line: String) {
        self.log.push(line);
        let excess = self.log.len().saturating_sub(10);
        if excess > 0 {
            self.log.drain(..excess);
        }
    }

    fn settings_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Settings");
        let s = &mut self.settings;
        ui.checkbox(&mut s.striped, "striped");
        ui.checkbox(&mut s.allow_multi_select, "multi-select");
        ui.checkbox(&mut s.allow_drag_and_drop, "drag & drop");

        let mut indent = s.indent.unwrap_or(13.5);
        if ui
            .add(egui::Slider::new(&mut indent, 4.0..=40.0).text("indent"))
            .changed()
        {
            s.indent = Some(indent);
        }
        let mut row_height = s.row_height.unwrap_or(18.0);
        if ui
            .add(egui::Slider::new(&mut row_height, 14.0..=40.0).text("row height"))
            .changed()
        {
            s.row_height = Some(row_height);
        }
        let mut icon_size = s.icon_size.unwrap_or(row_height * 0.7);
        if ui
            .add(egui::Slider::new(&mut icon_size, 8.0..=32.0).text("icon size"))
            .changed()
        {
            s.icon_size = Some(icon_size);
        }

        ui.separator();
        if ui.button("Reveal lib.rs").clicked() {
            self.state.reveal(DEEP_LEAF);
        }
        if ui.button("Expand all").clicked() {
            let mut ids = Vec::new();
            for root in &self.roots {
                root.all_dir_ids(&mut ids);
            }
            for id in ids {
                self.state.expand(id);
            }
        }
        if ui.button("Collapse all").clicked() {
            let mut ids = Vec::new();
            for root in &self.roots {
                root.all_dir_ids(&mut ids);
            }
            for id in ids {
                self.state.collapse(id);
            }
        }

        ui.separator();
        ui.heading("Action log");
        for line in &self.log {
            ui.weak(line);
        }
    }

    fn build_level(tree: &mut egui_ailanthus::TreeBuilder<'_, '_, '_, u32>, entries: &[Entry]) {
        for entry in entries {
            if entry.is_dir {
                let icon = if entry.is_crate {
                    Icon::rust_crate()
                } else {
                    Icon::folder()
                };
                let mut node = Node::new(entry.id).label(&entry.name).icon(icon);
                if entry.id == ARCHIVE_DIR {
                    // Custom row decoration: a warning underline.
                    node = node.row_paint(|painter, row| {
                        painter.line_segment(
                            [row.rect.left_bottom(), row.rect.right_bottom()],
                            egui::Stroke::new(1.0, egui::Color32::from_rgb(0xC8, 0x8A, 0x3C)),
                        );
                    });
                }
                tree.dir(node.context_menu(|ui| {
                    let _ = ui.button("New file…");
                    let _ = ui.button("Rename…");
                }));
                Self::build_level(tree, &entry.children);
                tree.close_dir();
            } else {
                let mut node = Node::new(entry.id)
                    .label(&entry.name)
                    .icon(icon_for(&entry.name));
                if entry.name.ends_with(".md") {
                    node = node
                        .badge(Badge::dot(egui::Color32::ORANGE))
                        .trailing(|ui| {
                            ui.weak("edited");
                        });
                } else if entry.name.ends_with(".pdf") {
                    node = node.trailing(|ui| {
                        ui.weak("2.4 MB");
                    });
                }
                tree.leaf(node);
            }
        }
    }

    fn apply_move(&mut self, dnd: &egui_ailanthus::DragAndDrop<u32>) {
        let ids: Vec<u32> = dnd.sources.iter().map(|s| s.id).collect();
        let mut moved = Vec::new();
        remove_ids(&mut self.roots, &ids, &mut moved);
        let target = match &dnd.target_dir {
            Some(dir) => match find_dir_mut(&mut self.roots, dir.id) {
                Some(entry) => &mut entry.children,
                None => &mut self.roots,
            },
            None => &mut self.roots,
        };
        let index = match &dnd.position {
            DirPosition::First => 0,
            DirPosition::Last => target.len(),
            DirPosition::Before(sib) => target
                .iter()
                .position(|e| e.id == *sib)
                .unwrap_or(target.len()),
            DirPosition::After(sib) => target
                .iter()
                .position(|e| e.id == *sib)
                .map(|i| i + 1)
                .unwrap_or(target.len()),
        };
        for (offset, entry) in moved.into_iter().enumerate() {
            target.insert((index + offset).min(target.len()), entry);
        }
    }
}

fn remove_ids(entries: &mut Vec<Entry>, ids: &[u32], removed: &mut Vec<Entry>) {
    let mut i = 0;
    while i < entries.len() {
        if ids.contains(&entries[i].id) {
            removed.push(entries.remove(i));
        } else {
            remove_ids(&mut entries[i].children, ids, removed);
            i += 1;
        }
    }
}

fn find_dir_mut(entries: &mut Vec<Entry>, id: u32) -> Option<&mut Entry> {
    for entry in entries {
        if entry.id == id {
            return Some(entry);
        }
        if let Some(found) = find_dir_mut(&mut entry.children, id) {
            return Some(found);
        }
    }
    None
}

impl eframe::App for DemoApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::left(egui::Id::new("settings"))
            .default_size(260.0)
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| self.settings_panel(ui));
            });
        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::both()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    let (_response, actions) = TreeView::new(egui::Id::new("playground"))
                        .with_settings(self.settings.clone())
                        .fallback_context_menu(|ui| {
                            let _ = ui.button("Add root item…");
                        })
                        .show_state(ui, &mut self.state, |tree| {
                            Self::build_level(tree, &self.roots);
                        });

                    let read_only = |dnd: &egui_ailanthus::DragAndDrop<u32>| {
                        dnd.target_dir.as_ref().is_some_and(|t| t.id == ARCHIVE_DIR)
                    };
                    for action in actions {
                        match action {
                            Action::Move(dnd) if read_only(&dnd) => {
                                self.push_log("rejected: archive is read-only".to_owned());
                            }
                            Action::Move(dnd) => {
                                self.apply_move(&dnd);
                                self.push_log(format!(
                                    "Move {:?} → {:?} in {:?}",
                                    dnd.sources.iter().map(|s| s.id).collect::<Vec<_>>(),
                                    dnd.position,
                                    dnd.target_dir.as_ref().map(|t| t.id),
                                ));
                            }
                            Action::Drag(dnd) if read_only(&dnd) => dnd.remove_marker(ui),
                            Action::Drag(_) => {}
                            other => self.push_log(format!("{other:?}")),
                        }
                    }
                });
        });
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    eframe::run_native(
        "egui_ailanthus — playground",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(DemoApp::new()))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;
    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");
        let canvas = document
            .get_element_by_id("demo_canvas")
            .expect("no canvas element with id demo_canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("demo_canvas is not a canvas");
        eframe::WebRunner::new()
            .start(
                canvas,
                eframe::WebOptions::default(),
                Box::new(|_cc| Ok(Box::new(DemoApp::new()))),
            )
            .await
            .expect("failed to start eframe");
    });
}
