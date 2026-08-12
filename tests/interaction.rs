//! Headless interaction tests via egui_kittest: rows are located through
//! their AccessKit nodes and driven with synthesized pointer/keyboard input.

use egui_ailanthus::{Action, DirPosition, Icon, Node, TreeView, TreeViewState};
use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;

#[derive(Default)]
struct State {
    tree: TreeViewState<String>,
    actions: Vec<Action<String>>,
    deleted: bool,
}

fn harness<'a>() -> Harness<'a, State> {
    // Default step_dt is 0.25 s; a 60 fps step keeps double clicks inside
    // egui's 0.3 s double-click window.
    Harness::builder().with_step_dt(1.0 / 60.0).build_ui_state(
        |ui, s: &mut State| {
            let State {
                tree: tree_state,
                actions,
                deleted,
            } = s;
            let (_response, new_actions) = TreeView::new(egui::Id::new("t"))
                .fallback_context_menu(|ui| {
                    let _ = ui.button("Fallback entry");
                })
                .show_state(ui, tree_state, |tree| {
                    tree.dir(
                        Node::new("dir".to_owned())
                            .label("dir")
                            .icon(Icon::folder()),
                    );
                    tree.leaf(
                        Node::new("dir/a".to_owned())
                            .label("alpha")
                            .icon(Icon::FileRust)
                            .context_menu(|ui| {
                                if ui.button("Delete").clicked() {
                                    *deleted = true;
                                }
                            }),
                    );
                    tree.leaf(Node::new("dir/b".to_owned()).label("beta"));
                    tree.close_dir();
                    tree.leaf(Node::new("top".to_owned()).label("topfile"));
                });
            actions.extend(new_actions);
        },
        State::default(),
    )
}

#[test]
fn rows_are_accessible() {
    let mut harness = harness();
    harness.run();
    for label in ["dir", "alpha", "beta", "topfile"] {
        assert!(
            harness.query_by_label(label).is_some(),
            "row {label:?} should have an AccessKit node"
        );
    }
}

#[test]
fn click_selects_leaf() {
    let mut harness = harness();
    harness.run();
    harness.get_by_label("alpha").click();
    harness.run();

    let state = harness.state();
    assert_eq!(state.tree.selected(), ["dir/a".to_owned()]);
    assert!(
        state.actions.iter().any(|a| matches!(
            a,
            Action::SetSelected(nodes) if nodes.len() == 1
                && nodes[0].id == "dir/a" && !nodes[0].is_dir
        )),
        "expected a SetSelected action carrying is_dir=false, got {:?}",
        state.actions
    );
}

#[test]
fn click_toggles_dir() {
    let mut harness = harness();
    harness.run();
    harness.get_by_label("dir").click();
    harness.run();

    let state = harness.state();
    assert_eq!(state.tree.is_open(&"dir".to_owned()), Some(false));
    assert!(
        state
            .actions
            .iter()
            .any(|a| matches!(a, Action::DirClosed(n) if n.id == "dir")),
        "expected DirClosed, got {:?}",
        state.actions
    );
    // Children disappear from the accessibility tree once collapsed.
    assert!(harness.query_by_label("alpha").is_none());
}

#[test]
fn double_click_activates() {
    let mut harness = harness();
    harness.run();
    // Two clicks in one input batch land within the double-click window.
    harness.get_by_label("alpha").click();
    harness.get_by_label("alpha").click();
    harness.run();

    let state = harness.state();
    assert!(
        state.actions.iter().any(|a| matches!(
            a,
            Action::Activate { nodes, .. } if nodes.iter().any(|n| n.id == "dir/a")
        )),
        "expected Activate, got {:?}",
        state.actions
    );
}

#[test]
fn arrow_keys_move_selection() {
    let mut harness = harness();
    harness.run();
    harness.get_by_label("alpha").click();
    harness.run();
    harness.key_press(egui::Key::ArrowDown);
    harness.run();

    assert_eq!(harness.state().tree.selected(), ["dir/b".to_owned()]);

    harness.key_press(egui::Key::ArrowUp);
    harness.run();
    assert_eq!(harness.state().tree.selected(), ["dir/a".to_owned()]);
}

#[test]
fn context_menu_opens_and_clicks() {
    let mut harness = harness();
    harness.run();
    harness.get_by_label("alpha").click_secondary();
    harness.run();

    harness.get_by_label("Delete").click();
    harness.run();

    assert!(harness.state().deleted, "context menu action should fire");
    harness.run();
    assert!(
        harness.query_by_label("Delete").is_none(),
        "menu should close after click"
    );
}

#[test]
fn fallback_context_menu_on_plain_row() {
    let mut harness = harness();
    harness.run();
    harness.get_by_label("beta").click_secondary();
    harness.run();
    assert!(harness.query_by_label("Fallback entry").is_some());
}

#[test]
fn drag_reorders_via_move_action() {
    let mut harness = harness();
    harness.run();

    let from = harness.get_by_label("beta").rect().center();
    let to = harness.get_by_label("topfile").rect();
    // Aim at the lower half of the target row => After(top).
    let to = egui::pos2(to.center().x, to.center().y + to.height() * 0.3);

    harness.drag_at(from);
    harness.step();
    let mid = egui::pos2((from.x + to.x) * 0.5, (from.y + to.y) * 0.5);
    harness.hover_at(mid);
    harness.step();
    harness.hover_at(to);
    harness.step();
    harness.drop_at(to);
    harness.run();

    let state = harness.state();
    assert!(
        state.actions.iter().any(|a| matches!(
            a,
            Action::Move(dnd) if dnd.sources.len() == 1
                && dnd.sources[0].id == "dir/b"
                && dnd.target_dir.is_none()
                && dnd.position == DirPosition::After("top".to_owned())
        )),
        "expected Move after 'top' at root, got {:?}",
        state.actions
    );
}

#[test]
fn drag_into_dir_targets_dir() {
    let mut harness = harness();
    harness.run();

    let from = harness.get_by_label("topfile").rect().center();
    let to = harness.get_by_label("dir").rect().center();

    harness.drag_at(from);
    harness.step();
    harness.hover_at(egui::pos2((from.x + to.x) * 0.5, (from.y + to.y) * 0.5));
    harness.step();
    harness.hover_at(to);
    harness.step();
    harness.drop_at(to);
    harness.run();

    let state = harness.state();
    assert!(
        state.actions.iter().any(|a| matches!(
            a,
            Action::Move(dnd) if dnd.sources[0].id == "top"
                && dnd.target_dir.as_ref().is_some_and(|t| t.id == "dir")
                && dnd.position == DirPosition::Last
        )),
        "expected Move into 'dir', got {:?}",
        state.actions
    );
}
