use egui_ailanthus::{Icon, Node, TreeView, TreeViewState};

fn show_frame(state: &mut TreeViewState<&'static str>) {
    egui::__run_test_ui(|ui| {
        let (_response, _actions) =
            TreeView::new(egui::Id::new("t")).show_state(ui, state, |tree| {
                tree.dir(Node::new("dir").label("dir").icon(Icon::folder()));
                tree.leaf(Node::new("dir/child").label("child").icon(Icon::FileRust));
                tree.close_dir();
                tree.leaf(Node::new("top").label("top").icon(Icon::FilePdf));
            });
    });
}

#[test]
fn renders_without_panicking() {
    let mut state = TreeViewState::new();
    show_frame(&mut state);
}

#[test]
fn openness_defaults_and_programmatic_toggle() {
    let mut state = TreeViewState::new();
    state.collapse("dir");
    show_frame(&mut state);
    assert_eq!(state.is_open(&"dir"), Some(false));

    state.expand("dir");
    show_frame(&mut state);
    assert_eq!(state.is_open(&"dir"), Some(true));
}

#[test]
fn reveal_expands_parents_and_selects() {
    let mut state = TreeViewState::new();
    state.collapse("dir");
    show_frame(&mut state);

    state.reveal("dir/child");
    show_frame(&mut state);

    assert_eq!(state.is_open(&"dir"), Some(true));
    assert!(state.is_selected(&"dir/child"));
    assert_eq!(state.cursor(), Some(&"dir/child"));
}

#[test]
fn expand_parents_of_does_not_select() {
    let mut state = TreeViewState::new();
    state.collapse("dir");
    show_frame(&mut state);

    state.expand_parents_of("dir/child");
    show_frame(&mut state);

    assert_eq!(state.is_open(&"dir"), Some(true));
    assert!(!state.is_selected(&"dir/child"));
}
