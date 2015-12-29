use libintermix::client::layout::*;
use ::support::layout::*;
use ::support::layout_painter::*;

#[test]
fn it_can_align_a_column_left() {
    ::setup_logging();
    let leaf = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(2), width: Some(2), ..Default::default()});
    let mut layout = Layout::new(
        Size { rows: 2, cols: 4},
        Node::row(NodeOptions { align: Align::Left, ..Default::default() }, vec![
            Node::col(6, Default::default(), vec![leaf]),
        ])
    );

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.----.
|aa  |
|aa  |
.----.");
}

#[test]
fn it_can_align_columns_left() {
    ::setup_logging();
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(2), width: Some(2), ..Default::default()});
    let leaf_b = Node::leaf_v2("b".to_string(), NodeOptions { height: Some(2), width: Some(2), ..Default::default()});
    let mut layout = Layout::new(
        Size { rows: 2, cols: 4},
        Node::row(NodeOptions { align: Align::Left, ..Default::default() }, vec![
            Node::col(3, Default::default(), vec![leaf_a]),
            Node::col(3, Default::default(), vec![leaf_b])
        ])
    );

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.----.
|ab  |
|ab  |
.----.");
}
#[test]
fn it_can_align_a_column_right() {
    ::setup_logging();
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(2), width: Some(2), ..Default::default()});
    let mut layout = Layout::new(
        Size { rows: 2, cols: 4},
        Node::row(NodeOptions { align: Align::Right, ..Default::default() }, vec![
            Node::col(6, Default::default(), vec![leaf_a])
        ])
    );

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.----.
|  aa|
|  aa|
.----.");
}

#[test]
fn it_can_align_columns_right() {
    ::setup_logging();
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(2), width: Some(2), ..Default::default()});
    let leaf_b = Node::leaf_v2("b".to_string(), NodeOptions { height: Some(2), width: Some(2), ..Default::default()});
    let mut layout = Layout::new(
        Size { rows: 2, cols: 4},
        Node::row(NodeOptions { align: Align::Right, ..Default::default() }, vec![
            Node::col(3, Default::default(), vec![leaf_a]),
            Node::col(3, Default::default(), vec![leaf_b])
        ])
    );

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.----.
|  ab|
|  ab|
.----.");
}

#[test]
fn it_can_align_a_column_center() {
    ::setup_logging();
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(2), width: Some(2), ..Default::default()});
    let mut layout = Layout::new(
        Size { rows: 2, cols: 4},
        Node::row(NodeOptions { align: Align::Center, ..Default::default() }, vec![
            Node::col(6, Default::default(), vec![leaf_a])
        ])
    );

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.----.
| aa |
| aa |
.----.");
}

#[test]
fn it_can_align_columns_center() {
    ::setup_logging();
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(2), width: Some(2), ..Default::default()});
    let leaf_b = Node::leaf_v2("b".to_string(), NodeOptions { height: Some(2), width: Some(2), ..Default::default()});
    let mut layout = Layout::new(
        Size { rows: 2, cols: 4},
        Node::row(NodeOptions { align: Align::Center, ..Default::default() }, vec![
            Node::col(3, Default::default(), vec![leaf_a]),
            Node::col(3, Default::default(), vec![leaf_b]),
        ])
    );

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.----.
| ab |
| ab |
.----.");
}

#[test]
fn it_can_align_a_row_top() {
    ::setup_logging();
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(1), width: Some(4), ..Default::default()});
    let mut layout = Layout::new(
        Size { rows: 2, cols: 4},
        Node::row(NodeOptions { vertical_align: VerticalAlign::Top, ..Default::default() }, vec![
            Node::row(Default::default(), vec![leaf_a])
        ])
    );

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.----.
|aaaa|
|    |
.----.");
}

#[test]
fn it_can_align_rows_top() {
    ::setup_logging();
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(1), width: Some(4), ..Default::default()});
    let leaf_b = Node::leaf_v2("b".to_string(), NodeOptions { height: Some(1), width: Some(4), ..Default::default()});
    let mut layout = Layout::new(
        Size { rows: 3, cols: 4},
        Node::row(NodeOptions { vertical_align: VerticalAlign::Top, ..Default::default() }, vec![
            Node::row(Default::default(), vec![leaf_a]),
            Node::row(Default::default(), vec![leaf_b])
        ])
    );

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.----.
|aaaa|
|bbbb|
|    |
.----.");
}

#[test]
fn it_can_align_a_row_botton() {
    ::setup_logging();
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(1), width: Some(4), ..Default::default()});
    let mut layout = Layout::new(
        Size { rows: 2, cols: 4},
        Node::row(NodeOptions { vertical_align: VerticalAlign::Bottom, height: Some(2), ..Default::default() }, vec![
            Node::row(Default::default(), vec![leaf_a])
        ])
    );

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.----.
|    |
|aaaa|
.----.");
}

#[test]
fn it_can_align_rows_botton() {
    ::setup_logging();
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(1), width: Some(4), ..Default::default()});
    let leaf_b = Node::leaf_v2("b".to_string(), NodeOptions { height: Some(1), width: Some(4), ..Default::default()});
    let mut layout = Layout::new(
        Size { rows: 3, cols: 4},
        Node::row(NodeOptions { vertical_align: VerticalAlign::Bottom, height: Some(3), ..Default::default() }, vec![
            Node::row(Default::default(), vec![leaf_a]),
            Node::row(Default::default(), vec![leaf_b])
        ])
    );

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.----.
|    |
|aaaa|
|bbbb|
.----.");
}

#[test]
fn it_can_align_a_row_middle() {
    ::setup_logging();
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(1), width: Some(4), ..Default::default()});
    let mut layout = Layout::new(
        Size { rows: 3, cols: 4},
        Node::row(NodeOptions { vertical_align: VerticalAlign::Middle, height: Some(3), ..Default::default() }, vec![
            Node::row(Default::default(), vec![leaf_a])
        ])
    );

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.----.
|    |
|aaaa|
|    |
.----.");
}

#[test]
fn it_can_align_rows_center() {
    ::setup_logging();
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(1), width: Some(4), ..Default::default()});
    let leaf_b = Node::leaf_v2("b".to_string(), NodeOptions { height: Some(1), width: Some(4), ..Default::default()});
    let mut layout = Layout::new(
        Size { rows: 4, cols: 4},
        Node::row(NodeOptions { vertical_align: VerticalAlign::Middle, height: Some(4), ..Default::default() }, vec![
            Node::row(Default::default(), vec![leaf_a]),
            Node::row(Default::default(), vec![leaf_b])
        ])
    );

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.----.
|    |
|aaaa|
|bbbb|
|    |
.----.");
}
