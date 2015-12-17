use libintermix::client::layout::*;
use ::support::layout::*;

#[test]
fn it_wraps_content_in_leftmost_column() {
    let widget_a = Widget::new('a', Size { rows: 2, cols: 4});
    let widget_b = Widget::new('b', Size { rows: 2, cols: 4});

    let mut layout = Layout::new(Size { rows: 4, cols: 4},
          Node::col(12, Default::default(), vec![
              Node::col(9, Default::default(), vec![Node::leaf(widget_a)]),
              Node::col(6, Default::default(), vec![Node::leaf(widget_b)]),
          ])
      );
    assert_scene_eq(&layout.display(), "
.----.
|aaa |
|aaa |
|bb  |
|bb  |
.----.");
}

#[test]
fn it_wraps_content_in_rightmost_column() {
    let widget_a = Widget::new('a', Size { rows: 2, cols: 4});
    let widget_b = Widget::new('b', Size { rows: 2, cols: 4});

    let mut layout = Layout::new(Size { rows: 4, cols: 4}, Node::row(Default::default(), vec![
          Node::col(3, Default::default(), vec![]),
          Node::col(9, Default::default(), vec![
              Node::col(6, Default::default(), vec![Node::leaf(widget_a)]),
              Node::col(9, Default::default(), vec![Node::leaf(widget_b)]),
          ])
    ]));
    assert_scene_eq(&layout.display(), "
.----.
| aa |
| aa |
| bbb|
| bbb|
.----.");
}
