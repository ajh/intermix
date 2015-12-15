use libintermix::client::layout::*;

#[test]
fn it_draws_an_empty_layout() {
    ::setup_logging();
    let mut layout = Layout::empty(Size { rows: 2, cols: 4 });

    let actual = layout.display();

    assert_eq!(layout.display(), "    \n    ");
}

#[test]
fn it_draws_a_root_container() {
    ::setup_logging();
    let widget_a = Widget::new('a', Size { rows: 2, cols: 2});
    let mut layout = Layout::new(Size { rows: 2, cols: 2}, Node::leaf(widget_a));

    let actual = layout.display();
    assert_eq!(
        actual,
        "\
aa
aa");
}

#[test]
fn it_draws_a_root_column() {
    ::setup_logging();
    let widget_a = Widget::new('a', Size { rows: 2, cols: 2});
    let mut layout = Layout::new(
        Size { rows: 2, cols: 4},
        Node::col(6, vec![Node::leaf(widget_a)])
    );

    let actual = layout.display();
    assert_eq!(
        actual,
        "\
aa  \n\
aa  ");
}

#[test]
fn it_draws_a_root_row() {
    ::setup_logging();
    let widget_a = Widget::new('a', Size { rows: 2, cols: 2});
    let mut layout = Layout::new(
        Size { rows: 2, cols: 2},
        Node::row(vec![Node::leaf(widget_a)])
    );

    let actual = layout.display();
    assert_eq!(
        actual,
        "\
aa
aa");
}

#[test]
fn it_draws_a_column_inside_a_row() {
    ::setup_logging();
    let widget_a = Widget::new('a', Size { rows: 2, cols: 3});
    let mut layout = Layout::new(
        Size { rows: 2, cols: 4},
        Node::row(vec![
            Node::col(9, vec![
                Node::leaf(widget_a)
            ])
        ])
    );

    let actual = layout.display();
    assert_eq!(
        actual,
        "\
aaa \n\
aaa ");
}

#[test]
fn it_draws_a_row_inside_a_column() {
    ::setup_logging();
    let widget_a = Widget::new('a', Size { rows: 2, cols: 3});
    let mut layout = Layout::new(
        Size { rows: 2, cols: 4},
        Node::col(9, vec![
            Node::row(vec![
                Node::leaf(widget_a)
            ])
        ])
    );

    let actual = layout.display();
    assert_eq!(
        actual,
        "\
aaa \n\
aaa ");
}

#[test]
fn it_draws_a_12_width_col() {
    ::setup_logging();
    let widget_a = Widget::new('a', Size { rows: 2, cols: 4});

    let mut layout = Layout::new(Size { rows: 2, cols: 4},
          Node::col(12, vec![
              Node::leaf(widget_a)
          ])
    );

    let actual = layout.display();

    assert_eq!(
        actual,
        "\
aaaa
aaaa");
}

#[test]
fn it_draws_a_9_and_3_width_col_evenly() {
    let widget_a = Widget::new('a', Size { rows: 2, cols: 4});
    let widget_b = Widget::new('b', Size { rows: 2, cols: 4});

    let mut layout = Layout::new(Size { rows: 2, cols: 4},
          Node::row(vec![
              Node::col(9, vec![Node::leaf(widget_a)]),
              Node::col(3, vec![Node::leaf(widget_b)]),
          ])
    );

    let actual = layout.display();

    assert_eq!(
        actual,
        "\
aaab
aaab");
}

#[test]
fn it_draws_a_9_and_3_width_col_unevenly() {
    let widget_a = Widget::new('a', Size { rows: 2, cols: 4});
    let widget_b = Widget::new('b', Size { rows: 2, cols: 4});

    let mut layout = Layout::new(Size { rows: 2, cols: 3},
          Node::row(vec![
              Node::col(9, vec![Node::leaf(widget_a)]),
              Node::col(3, vec![Node::leaf(widget_b)]),
          ])
    );

    let actual = layout.display();

    assert_eq!(
        actual,
        "\
aab
aab");
}

#[test]
fn it_draws_a_3_and_9_width_col_evenly() {
    let widget_a = Widget::new('a', Size { rows: 2, cols: 4});
    let widget_b = Widget::new('b', Size { rows: 2, cols: 4});

    let mut layout = Layout::new(Size { rows: 2, cols: 4},
          Node::row(vec![
              Node::col(3, vec![Node::leaf(widget_a)]),
              Node::col(9, vec![Node::leaf(widget_b)]),
          ])
    );

    let actual = layout.display();

    assert_eq!(
        actual,
        "\
abbb
abbb");
}

#[test]
fn it_draws_a_3_and_9_width_col_unevenly() {
    let widget_a = Widget::new('a', Size { rows: 2, cols: 4});
    let widget_b = Widget::new('b', Size { rows: 2, cols: 4});

    let mut layout = Layout::new(Size { rows: 2, cols: 3},
          Node::row(vec![
              Node::col(3, vec![Node::leaf(widget_a)]),
              Node::col(9, vec![Node::leaf(widget_b)]),
          ])
    );

    let actual = layout.display();

    assert_eq!(
        actual,
        "\
abb
abb");
}

#[test]
fn it_draws_a_pair_of_6_width_cols_evenly() {
    let widget_a = Widget::new('a', Size { rows: 2, cols: 4});
    let widget_b = Widget::new('b', Size { rows: 2, cols: 4});

    let mut layout = Layout::new(Size { rows: 2, cols: 4},
          Node::row(vec![
              Node::col(6, vec![Node::leaf(widget_a)]),
              Node::col(6, vec![Node::leaf(widget_b)]),
          ])
    );

    let actual = layout.display();

    assert_eq!(
        actual,
        "\
aabb
aabb");
}

#[test]
fn it_draws_a_pair_of_6_width_cols_unevenly() {
    let widget_a = Widget::new('a', Size { rows: 2, cols: 4});
    let widget_b = Widget::new('b', Size { rows: 2, cols: 4});

    let mut layout = Layout::new(Size { rows: 2, cols: 3},
          Node::row(vec![
              Node::col(6, vec![Node::leaf(widget_a)]),
              Node::col(6, vec![Node::leaf(widget_b)]),
          ])
    );

    let actual = layout.display();

    assert_eq!(
        actual,
        "\
aab
aab");
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
          Node::row(vec![
              Node::col(2, vec![Node::leaf(widget_a)]),
              Node::col(2, vec![Node::leaf(widget_b)]),
              Node::col(2, vec![Node::leaf(widget_c)]),
              Node::col(2, vec![Node::leaf(widget_x)]),
              Node::col(2, vec![Node::leaf(widget_y)]),
              Node::col(2, vec![Node::leaf(widget_z)]),
          ])
    );

    let actual = layout.display();

    assert_eq!(actual, "abcxyz");
}

#[test]
fn it_draws_rows() {
    let widget_a = Widget::new('a', Size { rows: 2, cols: 4});
    let widget_b = Widget::new('b', Size { rows: 2, cols: 4});

    let mut layout = Layout::new(Size { rows: 4, cols: 4},
          Node::row(vec![
              Node::row(vec![Node::leaf(widget_a)]),
              Node::row(vec![Node::leaf(widget_b)]),
          ])
      );
    let actual = layout.display();
    assert_eq!(
        actual,
        "\
aaaa
aaaa
bbbb
bbbb");
}

// it_draws_containers

#[test]
fn it_truncates_widget_with_narrow_container() {
    let widget_a = Widget::new('a', Size { rows: 1, cols: 4});

    let mut layout = Layout::new(Size { rows: 1, cols: 2}, Node::leaf(widget_a));

    let actual = layout.display();
    assert_eq!(actual, "aa");
}

#[test]
fn it_truncates_widget_with_short_container() {
    let widget_a = Widget::new('a', Size { rows: 4, cols: 1});

    let mut layout = Layout::new(Size { rows: 2, cols: 1}, Node::leaf(widget_a));

    let actual = layout.display();
    assert_eq!(actual, "a\na");
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

    //let actual = layout.display();
    //assert_eq!(
        //actual,
        //"\
//aaaacccccccc
//aaaaxxxx    \n\
//bbbbxxxx    \n\
//bbbb        \n\
//yyyy  z
//yyyy  z
//yyyy  z
//yyyy  z
      //z
       //");
//}