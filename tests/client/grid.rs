use libintermix::client::grid::*;

#[test]
fn it_draws_an_empty_screen() {
    ::setup_logging();
    let mut screen = Screen::new(Size { rows: 2, cols: 4 }, None);

    let actual = screen.display();

    assert_eq!(screen.display(), "    \n    ");
}

#[test]
fn it_draws_a_root_container() {
    ::setup_logging();
    let widget_a = Widget::new('a', Size { rows: 2, cols: 2});
    let mut screen = Screen::new(
        Size { rows: 2, cols: 2},
        Some(Node::container(widget_a)));

    let actual = screen.display();
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
    let mut screen = Screen::new(
        Size { rows: 2, cols: 4},
        Some(Node::col(6, vec![Node::container(widget_a)]))
    );

    let actual = screen.display();
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
    let mut screen = Screen::new(
        Size { rows: 2, cols: 2},
        Some(Node::row(vec![Node::container(widget_a)]))
    );

    let actual = screen.display();
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
    let mut screen = Screen::new(
        Size { rows: 2, cols: 4},
        Some(Node::row(vec![
            Node::col(9, vec![
                Node::container(widget_a)
            ])
        ]))
    );

    let actual = screen.display();
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
    let mut screen = Screen::new(
        Size { rows: 2, cols: 4},
        Some(Node::col(9, vec![
            Node::row(vec![
                Node::container(widget_a)
            ])
        ]))
    );

    let actual = screen.display();
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

    let mut screen = Screen::new(Size { rows: 2, cols: 4}, Some(
          Node::col(12, vec![
              Node::container(widget_a)
          ])
    ));

    let actual = screen.display();

    assert_eq!(
        actual,
        "\
aaaa
aaaa");
}

// it_draws_a_9_and_3_width_col_evenly
// it_draws_a_9_and_3_width_col_unevenly
// it_draws_a_3_and_9_width_col_evenly
// it_draws_a_3_and_9_width_col_unevenly
// it_draws_a_pair_of_6_width_cols_evenly
// it_draws_a_pair_of_6_width_cols_unevenly
// it_wraps_a_pair_of_9_width_cols
//
// it_draws_a_bunch_of_columns
// it_wraps_rows
// it_truncates_widget_with_narrow_container
// it_truncates_widget_with_short_container
// it_draws_a_complicated_scene
