use libintermix::client::layout::*;
use ::support::layout::*;

mod align;
mod wrap;

// Features todo:
//
// * horizontal align
// * vertical align
// * max height
// * padding
// * margin
// * title
// * border

#[test]
fn it_draws_a_root_container() {
    ::setup_logging();
    let widget_a = Widget::new('a', Size { rows: 2, cols: 2});
    let mut layout = Layout::new(Size { rows: 2, cols: 2}, Node::leaf(widget_a));

    assert_scene_eq(&layout.display(), "
.--.
|aa|
|aa|
.--.");
}

#[test]
fn it_draws_a_root_column() {
    ::setup_logging();
    let widget_a = Widget::new('a', Size { rows: 2, cols: 2});
    let mut layout = Layout::new(
        Size { rows: 2, cols: 4},
        Node::col(6, Default::default(), vec![Node::leaf(widget_a)])
    );

    assert_scene_eq(&layout.display(), "
.----.
|aa  |
|aa  |
.----.");
}

#[test]
fn it_draws_a_root_row() {
    ::setup_logging();
    let widget_a = Widget::new('a', Size { rows: 2, cols: 2});
    let mut layout = Layout::new(
        Size { rows: 2, cols: 2},
        Node::row(Default::default(), vec![Node::leaf(widget_a)])
    );

    assert_scene_eq(&layout.display(), "
.--.
|aa|
|aa|
.--.");
}

#[test]
fn it_draws_a_column_inside_a_row() {
    ::setup_logging();
    let widget_a = Widget::new('a', Size { rows: 2, cols: 3});
    let mut layout = Layout::new(
        Size { rows: 2, cols: 4},
        Node::row(Default::default(), vec![
            Node::col(9, Default::default(), vec![
                Node::leaf(widget_a)
            ])
        ])
    );

    assert_scene_eq(&layout.display(), "
.----.
|aaa |
|aaa |
.----.");
}

#[test]
fn it_draws_a_row_inside_a_column() {
    ::setup_logging();
    let widget_a = Widget::new('a', Size { rows: 2, cols: 3});
    let mut layout = Layout::new(
        Size { rows: 2, cols: 4},
        Node::col(9, Default::default(), vec![
            Node::row(Default::default(), vec![
                Node::leaf(widget_a)
            ])
        ])
    );

    assert_scene_eq(&layout.display(), "
.----.
|aaa |
|aaa |
.----.");
}

#[test]
fn it_draws_a_12_width_col() {
    ::setup_logging();
    let widget_a = Widget::new('a', Size { rows: 2, cols: 4});

    let mut layout = Layout::new(Size { rows: 2, cols: 4},
          Node::col(12, Default::default(), vec![
              Node::leaf(widget_a)
          ])
    );

    assert_scene_eq(&layout.display(), "
.----.
|aaaa|
|aaaa|
.----.");
}

#[test]
fn it_draws_a_9_and_3_width_col_evenly() {
    let widget_a = Widget::new('a', Size { rows: 2, cols: 4});
    let widget_b = Widget::new('b', Size { rows: 2, cols: 4});

    let mut layout = Layout::new(Size { rows: 2, cols: 4},
          Node::row(Default::default(), vec![
              Node::col(9, Default::default(), vec![Node::leaf(widget_a)]),
              Node::col(3, Default::default(), vec![Node::leaf(widget_b)]),
          ])
    );

    assert_scene_eq(&layout.display(), "
.----.
|aaab|
|aaab|
.----.");
}

#[test]
fn it_draws_a_9_and_3_width_col_unevenly() {
    let widget_a = Widget::new('a', Size { rows: 2, cols: 4});
    let widget_b = Widget::new('b', Size { rows: 2, cols: 4});

    let mut layout = Layout::new(Size { rows: 2, cols: 3},
          Node::row(Default::default(), vec![
              Node::col(9, Default::default(), vec![Node::leaf(widget_a)]),
              Node::col(3, Default::default(), vec![Node::leaf(widget_b)]),
          ])
    );

    assert_scene_eq(&layout.display(), "
.---.
|aab|
|aab|
.---.");
}

#[test]
fn it_draws_a_3_and_9_width_col_evenly() {
    let widget_a = Widget::new('a', Size { rows: 2, cols: 4});
    let widget_b = Widget::new('b', Size { rows: 2, cols: 4});

    let mut layout = Layout::new(Size { rows: 2, cols: 4},
          Node::row(Default::default(), vec![
              Node::col(3, Default::default(), vec![Node::leaf(widget_a)]),
              Node::col(9, Default::default(), vec![Node::leaf(widget_b)]),
          ])
    );

    assert_scene_eq(&layout.display(), "
.----.
|abbb|
|abbb|
.----.");
}

#[test]
fn it_draws_a_3_and_9_width_col_unevenly() {
    let widget_a = Widget::new('a', Size { rows: 2, cols: 4});
    let widget_b = Widget::new('b', Size { rows: 2, cols: 4});

    let mut layout = Layout::new(Size { rows: 2, cols: 3},
          Node::row(Default::default(), vec![
              Node::col(3, Default::default(), vec![Node::leaf(widget_a)]),
              Node::col(9, Default::default(), vec![Node::leaf(widget_b)]),
          ])
    );

    assert_scene_eq(&layout.display(), "
.---.
|abb|
|abb|
.---.");
}

#[test]
fn it_draws_a_pair_of_6_width_cols_evenly() {
    let widget_a = Widget::new('a', Size { rows: 2, cols: 4});
    let widget_b = Widget::new('b', Size { rows: 2, cols: 4});

    let mut layout = Layout::new(Size { rows: 2, cols: 4},
          Node::row(Default::default(), vec![
              Node::col(6, Default::default(), vec![Node::leaf(widget_a)]),
              Node::col(6, Default::default(), vec![Node::leaf(widget_b)]),
          ])
    );

    assert_scene_eq(&layout.display(), "
.----.
|aabb|
|aabb|
.----.");
}

#[test]
fn it_draws_a_pair_of_6_width_cols_unevenly() {
    let widget_a = Widget::new('a', Size { rows: 2, cols: 4});
    let widget_b = Widget::new('b', Size { rows: 2, cols: 4});

    let mut layout = Layout::new(Size { rows: 2, cols: 3},
          Node::row(Default::default(), vec![
              Node::col(6, Default::default(), vec![Node::leaf(widget_a)]),
              Node::col(6, Default::default(), vec![Node::leaf(widget_b)]),
          ])
    );

    assert_scene_eq(&layout.display(), "
.---.
|aab|
|aab|
.---.");
}

#[test]
fn it_draws_a_bunch_of_columns() {
    let widget_a = Widget::new('a', Size { rows: 1, cols: 1});
    let widget_b = Widget::new('b', Size { rows: 1, cols: 1});
    let widget_c = Widget::new('c', Size { rows: 1, cols: 1});
    let widget_x = Widget::new('x', Size { rows: 1, cols: 1});
    let widget_y = Widget::new('y', Size { rows: 1, cols: 1});
    let widget_z = Widget::new('z', Size { rows: 1, cols: 1});

    let mut layout = Layout::new(Size { rows: 1, cols: 6},
          Node::row(Default::default(), vec![
              Node::col(2, Default::default(), vec![Node::leaf(widget_a)]),
              Node::col(2, Default::default(), vec![Node::leaf(widget_b)]),
              Node::col(2, Default::default(), vec![Node::leaf(widget_c)]),
              Node::col(2, Default::default(), vec![Node::leaf(widget_x)]),
              Node::col(2, Default::default(), vec![Node::leaf(widget_y)]),
              Node::col(2, Default::default(), vec![Node::leaf(widget_z)]),
          ])
    );

    assert_scene_eq(&layout.display(), "
.------.
|abcxyz|
.------.");
}

#[test]
fn it_draws_rows() {
    let widget_a = Widget::new('a', Size { rows: 2, cols: 4});
    let widget_b = Widget::new('b', Size { rows: 2, cols: 4});

    let mut layout = Layout::new(Size { rows: 4, cols: 4},
          Node::row(Default::default(), vec![
              Node::row(Default::default(), vec![Node::leaf(widget_a)]),
              Node::row(Default::default(), vec![Node::leaf(widget_b)]),
          ])
      );
    assert_scene_eq(&layout.display(), "
.----.
|aaaa|
|aaaa|
|bbbb|
|bbbb|
.----.");
}

#[test]
fn it_truncates_widget_with_narrow_container() {
    let widget_a = Widget::new('a', Size { rows: 1, cols: 4});

    let mut layout = Layout::new(Size { rows: 1, cols: 2}, Node::leaf(widget_a));

    assert_scene_eq(&layout.display(), "
.--.
|aa|
.--.");
}

#[test]
fn it_truncates_widget_with_short_container() {
    let widget_a = Widget::new('a', Size { rows: 4, cols: 1});

    let mut layout = Layout::new(Size { rows: 2, cols: 1}, Node::leaf(widget_a));

    assert_scene_eq(&layout.display(), "
.-.
|a|
|a|
.-.");
}

#[test]
fn it_can_add_to_layout() {
    ::setup_logging();
    let widget_a = Widget::new('a', Size { rows: 2, cols: 2});
    let mut layout = Layout::new(
        Size { rows: 4, cols: 2},
        Node::row(Default::default(), vec![Node::leaf(widget_a)])
    );

    assert_scene_eq(&layout.display(), "
.--.
|aa|
|aa|
|  |
|  |
.--.");

    let widget_b = Widget::new('b', Size { rows: 2, cols: 2});
    layout.root
        .children
        .as_mut()
        .unwrap()
        .push(Node::leaf(widget_b));
    layout.calculate_layout();

    assert_scene_eq(&layout.display(), "
.--.
|aa|
|aa|
|bb|
|bb|
.--.");
}

#[test]
fn it_can_remove_from_layout() {
    ::setup_logging();
    let widget_a = Widget::new('a', Size { rows: 2, cols: 2});
    let widget_b = Widget::new('b', Size { rows: 2, cols: 2});

    let mut layout = Layout::new(
        Size { rows: 4, cols: 2},
        Node::row(Default::default(), vec![
              Node::leaf(widget_a),
              Node::leaf(widget_b),
        ])
    );
    assert_scene_eq(&layout.display(), "
.--.
|aa|
|aa|
|bb|
|bb|
.--.");

    layout.root
        .children
        .as_mut()
        .unwrap()
        .remove(1);

    assert_scene_eq(&layout.display(), "
.--.
|aa|
|aa|
|  |
|  |
.--.");
}

//#[test]
//fn it_draws_a_complicated_scene() {
    //let widget_a = Widget::new('a', Size { rows: 2, cols: 8});
    //let widget_b = Widget::new('b', Size { rows: 2, cols: 4});
    //let widget_c = Widget::new('c', Size { rows: 1, cols: 8});
    //let widget_x = Widget::new('x', Size { rows: 2, cols: 4});
    //let widget_y = Widget::new('y', Size { rows: 4, cols: 4});
    //let widget_z = Widget::new('z', Size { rows: 5, cols: 1});

    //let mut layout = Layout::new(Size { rows: 10, cols: 12},
          //Node::row(vec![
              //Node::row(vec![
                  //Node::col(4, vec![Node::leaf(widget_a), Node::leaf(widget_b)]),
                  //Node::col(8, vec![
                      //Node::row(vec![Node::leaf(widget_c)]),
                      //Node::row(vec![Node::leaf(widget_x)]),
                  //]),
              //]),
              //Node::row(vec![
                  //Node::col(4, vec![Node::leaf(widget_y)]),
                  //Node::col(2, vec![]),
                  //Node::col(6, vec![Node::leaf(widget_z)]),
              //])
          //])
    //);
    //
    //blah blah blah
//}
