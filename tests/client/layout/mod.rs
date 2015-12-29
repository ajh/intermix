use libintermix::client::layout::*;
use ::support::layout::*;
use ::support::layout_painter::*;

mod align;
mod wrap;

// Features todo:
//
// * [ ] padding
// * [ ] margin
// * [ ] title
// * [ ] border
// * [ ] use xml backend rather than node stuff?
// * [x] rethink widget vs leaf with id String

#[test]
fn it_draws_a_root_container() {
    ::setup_logging();
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(2), width: Some(2), ..Default::default()});
    let mut layout = Layout::new(Size { rows: 2, cols: 2}, leaf_a);

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.--.
|aa|
|aa|
.--.");
}

#[test]
fn it_draws_a_root_column() {
    ::setup_logging();
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(2), width: Some(2), ..Default::default()});
    let mut layout = Layout::new(
        Size { rows: 2, cols: 4},
        Node::col(6, Default::default(), vec![leaf_a])
    );

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.----.
|aa  |
|aa  |
.----.");
}

#[test]
fn it_draws_a_root_row() {
    ::setup_logging();
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(2), width: Some(2), ..Default::default()});
    let mut layout = Layout::new(
        Size { rows: 2, cols: 2},
        Node::row(Default::default(), vec![leaf_a])
    );

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.--.
|aa|
|aa|
.--.");
}

#[test]
fn it_draws_a_column_inside_a_row() {
    ::setup_logging();
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(2), width: Some(3), ..Default::default()});
    let mut layout = Layout::new(
        Size { rows: 2, cols: 4},
        Node::row(Default::default(), vec![
            Node::col(9, Default::default(), vec![leaf_a])
        ])
    );

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.----.
|aaa |
|aaa |
.----.");
}

#[test]
fn it_draws_a_row_inside_a_column() {
    ::setup_logging();
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(2), width: Some(3), ..Default::default()});
    let mut layout = Layout::new(
        Size { rows: 2, cols: 4},
        Node::col(9, Default::default(), vec![
            Node::row(Default::default(), vec![leaf_a])
        ])
    );

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.----.
|aaa |
|aaa |
.----.");
}

#[test]
fn it_draws_a_12_width_col() {
    ::setup_logging();
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});

    let mut layout = Layout::new(Size { rows: 2, cols: 4},
          Node::col(12, Default::default(), vec![leaf_a])
    );

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.----.
|aaaa|
|aaaa|
.----.");
}

#[test]
fn it_draws_a_9_and_3_width_col_evenly() {
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});
    let leaf_b = Node::leaf_v2("b".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});

    let mut layout = Layout::new(Size { rows: 2, cols: 4},
          Node::row(Default::default(), vec![
              Node::col(9, Default::default(), vec![leaf_a]),
              Node::col(3, Default::default(), vec![leaf_b]),
          ])
    );

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.----.
|aaab|
|aaab|
.----.");
}

#[test]
fn it_draws_a_9_and_3_width_col_unevenly() {
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});
    let leaf_b = Node::leaf_v2("b".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});

    let mut layout = Layout::new(Size { rows: 2, cols: 3},
          Node::row(Default::default(), vec![
              Node::col(9, Default::default(), vec![leaf_a]),
              Node::col(3, Default::default(), vec![leaf_b]),
          ])
    );

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.---.
|aab|
|aab|
.---.");
}

#[test]
fn it_draws_a_3_and_9_width_col_evenly() {
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});
    let leaf_b = Node::leaf_v2("b".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});

    let mut layout = Layout::new(Size { rows: 2, cols: 4},
          Node::row(Default::default(), vec![
              Node::col(3, Default::default(), vec![leaf_a]),
              Node::col(9, Default::default(), vec![leaf_b]),
          ])
    );

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.----.
|abbb|
|abbb|
.----.");
}

#[test]
fn it_draws_a_3_and_9_width_col_unevenly() {
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});
    let leaf_b = Node::leaf_v2("b".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});

    let mut layout = Layout::new(Size { rows: 2, cols: 3},
          Node::row(Default::default(), vec![
              Node::col(3, Default::default(), vec![leaf_a]),
              Node::col(9, Default::default(), vec![leaf_b]),
          ])
    );

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.---.
|abb|
|abb|
.---.");
}

#[test]
fn it_draws_a_pair_of_6_width_cols_evenly() {
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});
    let leaf_b = Node::leaf_v2("b".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});

    let mut layout = Layout::new(Size { rows: 2, cols: 4},
          Node::row(Default::default(), vec![
              Node::col(6, Default::default(), vec![leaf_a]),
              Node::col(6, Default::default(), vec![leaf_b]),
          ])
    );

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.----.
|aabb|
|aabb|
.----.");
}

#[test]
fn it_draws_a_pair_of_6_width_cols_unevenly() {
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});
    let leaf_b = Node::leaf_v2("b".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});

    let mut layout = Layout::new(Size { rows: 2, cols: 3},
          Node::row(Default::default(), vec![
              Node::col(6, Default::default(), vec![leaf_a]),
              Node::col(6, Default::default(), vec![leaf_b]),
          ])
    );

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.---.
|aab|
|aab|
.---.");
}

#[test]
fn it_draws_a_bunch_of_columns() {
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(1), width: Some(1), ..Default::default()});
    let leaf_b = Node::leaf_v2("b".to_string(), NodeOptions { height: Some(1), width: Some(1), ..Default::default()});
    let leaf_c = Node::leaf_v2("c".to_string(), NodeOptions { height: Some(1), width: Some(1), ..Default::default()});
    let leaf_x = Node::leaf_v2("x".to_string(), NodeOptions { height: Some(1), width: Some(1), ..Default::default()});
    let leaf_y = Node::leaf_v2("y".to_string(), NodeOptions { height: Some(1), width: Some(1), ..Default::default()});
    let leaf_z = Node::leaf_v2("z".to_string(), NodeOptions { height: Some(1), width: Some(1), ..Default::default()});

    let mut layout = Layout::new(Size { rows: 1, cols: 6},
          Node::row(Default::default(), vec![
              Node::col(2, Default::default(), vec![leaf_a]),
              Node::col(2, Default::default(), vec![leaf_b]),
              Node::col(2, Default::default(), vec![leaf_c]),
              Node::col(2, Default::default(), vec![leaf_x]),
              Node::col(2, Default::default(), vec![leaf_y]),
              Node::col(2, Default::default(), vec![leaf_z]),
          ])
    );

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.------.
|abcxyz|
.------.");
}

#[test]
fn it_draws_rows() {
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});
    let leaf_b = Node::leaf_v2("b".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});

    let mut layout = Layout::new(Size { rows: 4, cols: 4},
          Node::row(Default::default(), vec![
              Node::row(Default::default(), vec![leaf_a]),
              Node::row(Default::default(), vec![leaf_b]),
          ])
      );
    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.----.
|aaaa|
|aaaa|
|bbbb|
|bbbb|
.----.");
}

#[test]
fn it_truncates_leaf_with_narrow_container() {
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(1), width: Some(4), ..Default::default()});

    let mut layout = Layout::new(Size { rows: 1, cols: 2}, leaf_a);

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.--.
|aa|
.--.");
}

#[test]
fn it_truncates_leaf_with_short_container() {
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(4), width: Some(1), ..Default::default()});

    let mut layout = Layout::new(Size { rows: 2, cols: 1}, leaf_a);

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.-.
|a|
|a|
.-.");
}

#[test]
fn it_can_add_to_layout() {
    ::setup_logging();
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(2), width: Some(2), ..Default::default()});
    let mut layout = Layout::new(
        Size { rows: 4, cols: 2},
        Node::row(Default::default(), vec![leaf_a])
    );

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.--.
|aa|
|aa|
|  |
|  |
.--.");

    let leaf_b = Node::leaf_v2("b".to_string(), NodeOptions { height: Some(2), width: Some(2), ..Default::default()});
    layout.root
        .children
        .push(leaf_b);
    layout.calculate_layout();

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
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
    let leaf_a = Node::leaf_v2("a".to_string(), NodeOptions { height: Some(2), width: Some(2), ..Default::default()});
    let leaf_b = Node::leaf_v2("b".to_string(), NodeOptions { height: Some(2), width: Some(2), ..Default::default()});

    let mut layout = Layout::new(
        Size { rows: 4, cols: 2},
        Node::row(Default::default(), vec![
              leaf_a,
              leaf_b,
        ])
    );
    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.--.
|aa|
|aa|
|bb|
|bb|
.--.");

    layout.root
        .children
        .remove(1);

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.--.
|aa|
|aa|
|  |
|  |
.--.");
}

//#[test]
//fn it_draws_a_complicated_scene() {
    //let leaf_a = Node::leaf_v2("a".to_string(), Size { rows: 2, cols: 8});
    //let leaf_b = Node::leaf_v2("b".to_string(), Size { rows: 2, cols: 4});
    //let leaf_c = Node::leaf_v2("c".to_string(), Size { rows: 1, cols: 8});
    //let leaf_x = Node::leaf_v2("x".to_string(), Size { rows: 2, cols: 4});
    //let leaf_y = Node::leaf_v2("y".to_string(), Size { rows: 4, cols: 4});
    //let leaf_z = Node::leaf_v2("z".to_string(), Size { rows: 5, cols: 1});

    //let mut layout = Layout::new(Size { rows: 10, cols: 12},
          //Node::row(vec![
              //Node::row(vec![
                  //Node::col(4, vec![leaf_a, leaf_b]),
                  //Node::col(8, vec![
                      //Node::row(vec![leaf_c]),
                      //Node::row(vec![leaf_x]),
                  //]),
              //]),
              //Node::row(vec![
                  //Node::col(4, vec![leaf_y]),
                  //Node::col(2, vec![]),
                  //Node::col(6, vec![leaf_z]),
              //])
          //])
    //);
    //
    //blah blah blah
//}
