use libintermix::client::grid::*;

#[test]
fn it_draws_an_empty_screen() {
    ::setup_logging();
    let mut screen = Screen::new(4, 2, vec![]);

    let actual = screen.display();

    assert_eq!(screen.display(), "    \n    ");
}

#[test]
fn it_draws_row_and_12_width_col() {
    ::setup_logging();
    let widget_a = Widget::new('a', Size { rows: 2, cols: 4});

    let mut screen = Screen::new(4, 2, vec![
        Row::new(vec![Column::new(12, vec![widget_a])]),
    ]);

    let actual = screen.display();

    assert_eq!(
        actual,
        "\
aaaa
aaaa");
}

#[test]
fn it_draws_just_a_row_and_widget() {
    ::setup_logging();
    let widget_a = Widget::new('a', Size { rows: 2, cols: 4});

    let mut screen = Screen::new(4, 2, vec![
        Row::new(vec![Column::new(12, vec![widget_a])]),
    ]);

    let actual = screen.display();

    assert_eq!(
        actual,
        "\
aaaa
aaaa");
}

#[test]
fn it_draws_row_and_9_width_col() {
    ::setup_logging();
    let widget_a = Widget::new('a', Size { rows: 2, cols: 3});

    let mut screen = Screen::new(4, 2, vec![
        Row::new(vec![Column::new(9, vec![widget_a])]),
    ]);

    let actual = screen.display();

    assert_eq!(
        actual,
        "\
aaa \n\
aaa ");
}

#[test]
fn it_draws_one_row_one_6_col() {
    ::setup_logging();
    let widget_a = Widget::new('a', Size { rows: 2, cols: 2});

    let mut screen = Screen::new(4, 2, vec![
        Row::new(vec![Column::new(6, vec![widget_a])]),
    ]);

    let actual = screen.display();

    assert_eq!(
        actual,
        "\
aa  \n\
aa  ");
}

#[test]
fn it_draws_row_and_3_width_col() {
    ::setup_logging();
    let widget_a = Widget::new('a', Size { rows: 2, cols: 1});

    let mut screen = Screen::new(4, 2, vec![
        Row::new(vec![Column::new(3, vec![widget_a])]),
    ]);

    let actual = screen.display();

    assert_eq!(
        actual,
        "\
a   \n\
a   ");
}

#[test]
fn it_draws_row_and_9_and_3_width_cols() {
    ::setup_logging();
    let widget_a = Widget::new('a', Size { rows: 2, cols: 3});
    let widget_b = Widget::new('b', Size { rows: 2, cols: 1});

    let mut screen = Screen::new(4, 2, vec![
        Row::new(vec![
                 Column::new(9, vec![widget_a]),
                 Column::new(3, vec![widget_b]),
        ]),
    ]);

    let actual = screen.display();

    assert_eq!(
        actual,
        "\
aaab\n\
aaab");
}

#[test]
fn it_draws_two_rows() {
    ::setup_logging();
    let widget_a = Widget::new('a', Size { rows: 2, cols: 4});
    let widget_b = Widget::new('b', Size { rows: 2, cols: 4});

    let mut screen = Screen::new(4, 4, vec![
        Row::new(vec![Column::new(12, vec![widget_a])]),
        Row::new(vec![Column::new(12, vec![widget_b])]),
    ]);

    let actual = screen.display();

    assert_eq!(
        actual,
        "\
aaaa\n\
aaaa\n\
bbbb\n\
bbbb");
}

#[test]
fn it_draws_a_too_small_column() {
    ::setup_logging();
    let widget_a = Widget::new('a', Size { rows: 2, cols: 1});

    let mut screen = Screen::new(1, 1, vec![
        Row::new(vec![Column::new(12, vec![widget_a])]),
    ]);

    let actual = screen.display();

    assert_eq!(actual, "a");
}

#[test]
fn it_draws_a_too_small_row() {
    ::setup_logging();
    let widget_a = Widget::new('a', Size { rows: 1, cols: 2});

    let mut screen = Screen::new(1, 1, vec![
        Row::new(vec![Column::new(12, vec![widget_a])]),
    ]);

    let actual = screen.display();

    assert_eq!(actual, "a");
}

//#[test]
//fn it_draws_when_width_divides_unevenly_into_grid() {
    //::setup_logging();
    //let widget_a = Widget::new('a', Size { rows: 1, cols: 1});

    //let mut screen = Screen::new(5, 1, vec![
        //Row::new(vec![Column::new(12, vec![widget_a])]),
    //]);

    //let actual = screen.display();

    //assert_eq!(actual, "aaaaa");
//}

//#[test]
//fn it_draws_only_a_row() {
    //::setup_logging();
    //let widget_a = Widget::new('a', Size { rows: 2, cols: 2});
    //let mut screen = Screen::new(2, 2, vec![Row::new(vec![widget_a])]);

    //let actual = screen.display();
    //assert_eq!(
        //actual,
        //"\
//aa
//aa");
//}

#[test]
fn it_draws_a_simple_scene() {
    ::setup_logging();
    let widget_a = Widget::new('a', Size { rows: 2, cols: 2});
    let widget_b = Widget::new('b', Size { rows: 2, cols: 2});

    let mut screen = Screen::new(4, 2, vec![
        Row::new(vec![
                 Column::new(6, vec![widget_a]),
                 Column::new(6, vec![widget_b]),
        ]),
    ]);

    let actual = screen.display();

    assert_eq!(
        actual,
        "\
aabb
aabb");
}
