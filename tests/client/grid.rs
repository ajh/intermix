use libintermix::client::grid::*;

#[test]
fn it_draws_an_empty_screen() {
    ::setup_logging();
    let mut screen = Screen::new(4, 2, vec![]);

    let actual = screen.display();

    assert_eq!(screen.display(), "    \n    ");
}

#[test]
fn it_draws_one_row_one_12_col() {
    ::setup_logging();
    let widget_a = Widget::new('a', 2);

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
fn it_draws_one_row_one_9_col() {
    ::setup_logging();
    let widget_a = Widget::new('a', 2);

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
    let widget_a = Widget::new('a', 2);

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
fn it_draws_one_row_one_3_col() {
    ::setup_logging();
    let widget_a = Widget::new('a', 2);

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
fn it_draws_one_row_9_col_and_3_col() {
    ::setup_logging();
    let widget_a = Widget::new('a', 2);
    let widget_b = Widget::new('b', 2);

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

//#[test]
//fn it_draws_two_rows() {
    //::setup_logging();
    //let widget_a = Widget::new('a', 2);
    //let widget_b = Widget::new('b', 2);

    //let mut screen = Screen::new(4, 2, vec![
        //Row::new(vec![Column::new(12, vec![widget_a])]),
        //Row::new(vec![Column::new(12, vec![widget_b])]),
    //]);

    //let actual = screen.display();

    //assert_eq!(
        //actual,
        //"\
//aaaa\n\
//aaaa\n\
//bbbb\n\
//bbbb");
//}

#[test]
fn it_draws_a_simple_scene() {
    ::setup_logging();
    let widget_a = Widget::new('a', 2);
    let widget_b = Widget::new('b', 2);

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
