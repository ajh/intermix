use libintermix::client::layout::*;
use ::support::layout::*;
use ::support::layout_painter::*;

#[test]
fn it_wraps_content_in_leftmost_column() {
    let leaf_a = Node::leaf("a".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});
    let leaf_b = Node::leaf("b".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});

    let mut layout = Layout::new(Size { rows: 4, cols: 4},
          Node::col(12, Default::default(), vec![
              Node::col(9, Default::default(), vec![leaf_a]),
              Node::col(6, Default::default(), vec![leaf_b]),
          ])
      );
    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.----.
|aaa |
|aaa |
|bb  |
|bb  |
.----.");
}

#[test]
fn it_wraps_content_in_rightmost_column() {
    let leaf_a = Node::leaf("a".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});
    let leaf_b = Node::leaf("b".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});

    let mut layout = Layout::new(Size { rows: 4, cols: 4}, Node::row(Default::default(), vec![
          Node::col(3, Default::default(), vec![]),
          Node::col(9, Default::default(), vec![
              Node::col(6, Default::default(), vec![leaf_a]),
              Node::col(9, Default::default(), vec![leaf_b]),
          ])
    ]));
    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.----.
| aa |
| aa |
| bbb|
| bbb|
.----.");
}
