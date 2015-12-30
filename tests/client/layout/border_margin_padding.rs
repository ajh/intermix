use libintermix::client::layout::*;
use ::support::layout::*;
use ::support::layout_painter::*;

#[test]
fn it_can_have_border_around_node() {
    ::setup_logging();
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});
    let mut layout = Layout::new(
        Size { rows: 4, cols: 4},
        Node::row(NodeOptions { has_border: true, ..Default::default() }, vec![leaf_a])
    );

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.----.
|┌──┐|
|│aa│|
|│aa│|
|└──┘|
.----.");
}

#[test]
fn it_can_have_margin_around_node() {
    ::setup_logging();
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});
    let mut layout = Layout::new(
        Size { rows: 4, cols: 4},
        Node::row(NodeOptions { margin: 1, ..Default::default() }, vec![leaf_a])
    );

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.----.
|    |
| aa |
| aa |
|    |
.----.");
}

#[test]
fn it_can_have_padding_around_children() {
    ::setup_logging();
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});
    let mut layout = Layout::new(
        Size { rows: 4, cols: 4},
        Node::row(NodeOptions { padding: 1, ..Default::default() }, vec![leaf_a])
    );

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.----.
|    |
| aa |
| aa |
|    |
.----.");
}

#[test]
fn it_can_have_border_marging_and_padding() {
    ::setup_logging();
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});
    let mut layout = Layout::new(
        Size { rows: 8, cols: 8},
        Node::row(NodeOptions { margin: 1, padding: 1, has_border: true, ..Default::default() }, vec![leaf_a])
    );

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.--------.
|        |
| ┌────┐ |
| │    │ |
| │ aa │ |
| │ aa │ |
| │    │ |
| └────┘ |
|        |
.--------.");
}

