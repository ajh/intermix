use libintermix::client::layout::*;
use ::support::layout::*;

#[test]
fn it_can_align_a_column_left() {
    ::setup_logging();
    let widget_a = Widget::new('a', Size { rows: 2, cols: 2});
    let mut layout = Layout::new(
        Size { rows: 2, cols: 4},
        Node::row(NodeOptions { align: Align::Left, ..Default::default() }, vec![
            Node::col(6, Default::default(), vec![Node::leaf(widget_a)])
        ])
    );

    assert_scene_eq(&layout.display(), "
.----.
|aa  |
|aa  |
.----.");
}

#[test]
fn it_can_align_columns_left() {
    ::setup_logging();
    let widget_a = Widget::new('a', Size { rows: 2, cols: 2});
    let widget_b = Widget::new('b', Size { rows: 2, cols: 2});
    let mut layout = Layout::new(
        Size { rows: 2, cols: 4},
        Node::row(NodeOptions { align: Align::Left, ..Default::default() }, vec![
            Node::col(3, Default::default(), vec![Node::leaf(widget_a)]),
            Node::col(3, Default::default(), vec![Node::leaf(widget_b)])
        ])
    );

    assert_scene_eq(&layout.display(), "
.----.
|ab  |
|ab  |
.----.");
}
#[test]
fn it_can_align_a_column_right() {
    ::setup_logging();
    let widget_a = Widget::new('a', Size { rows: 2, cols: 2});
    let mut layout = Layout::new(
        Size { rows: 2, cols: 4},
        Node::row(NodeOptions { align: Align::Right, ..Default::default() }, vec![
            Node::col(6, Default::default(), vec![Node::leaf(widget_a)])
        ])
    );

    assert_scene_eq(&layout.display(), "
.----.
|  aa|
|  aa|
.----.");
}

#[test]
fn it_can_align_columns_right() {
    ::setup_logging();
    let widget_a = Widget::new('a', Size { rows: 2, cols: 2});
    let widget_b = Widget::new('b', Size { rows: 2, cols: 2});
    let mut layout = Layout::new(
        Size { rows: 2, cols: 4},
        Node::row(NodeOptions { align: Align::Right, ..Default::default() }, vec![
            Node::col(3, Default::default(), vec![Node::leaf(widget_a)]),
            Node::col(3, Default::default(), vec![Node::leaf(widget_b)])
        ])
    );

    assert_scene_eq(&layout.display(), "
.----.
|  ab|
|  ab|
.----.");
}

#[test]
fn it_can_align_a_column_center() {
    ::setup_logging();
    let widget_a = Widget::new('a', Size { rows: 2, cols: 2});
    let mut layout = Layout::new(
        Size { rows: 2, cols: 4},
        Node::row(NodeOptions { align: Align::Center, ..Default::default() }, vec![
            Node::col(6, Default::default(), vec![Node::leaf(widget_a)])
        ])
    );

    assert_scene_eq(&layout.display(), "
.----.
| aa |
| aa |
.----.");
}

#[test]
fn it_can_align_columns_center() {
    ::setup_logging();
    let widget_a = Widget::new('a', Size { rows: 2, cols: 2});
    let widget_b = Widget::new('b', Size { rows: 2, cols: 2});
    let mut layout = Layout::new(
        Size { rows: 2, cols: 4},
        Node::row(NodeOptions { align: Align::Center, ..Default::default() }, vec![
            Node::col(3, Default::default(), vec![Node::leaf(widget_a)]),
            Node::col(3, Default::default(), vec![Node::leaf(widget_b)]),
        ])
    );

    assert_scene_eq(&layout.display(), "
.----.
| ab |
| ab |
.----.");
}
