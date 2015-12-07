use libintermix::client::grid::*;

#[test]
fn it_correctly_draws_empty_screen() {
    ::setup_logging();
    let widget_a = Widget::new('a', 2);
    let widget_b = Widget::new('b', 2);

    // screen is a 12 column grid
    let mut screen = Screen::new(4, 2, vec![
        Row::new(vec![
                 Column::new(6, vec![widget_a]),
                 Column::new(6, vec![widget_b]),
        ]),
    ]);

    let actual = screen.display();
    println!("{:?}", screen);

    assert_eq!(
        actual,
        "\
aabb
aabb");

}
