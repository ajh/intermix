use libintermix::client::layout::*;
use ::support::*;

#[test]
fn it_can_align_a_column_left() {
    let col = WrapBuilder::col(6)
                  .name("a".to_string())
                  .height(2)
                  .build();

    let mut screen = Screen::new(Size { rows: 2, cols: 4 });
    screen.tree_mut().root_mut().append(col);
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen),
                    "
······
·aa  ·
·aa  ·
······");
}

#[test]
fn it_can_align_columns_left() {
    let cols = vec![
        WrapBuilder::col(3).name("a".to_string()).height(2).build(),
        WrapBuilder::col(3).name("b".to_string()).height(2).build(),
    ];

    let mut screen = Screen::new(Size { rows: 2, cols: 4 });
    for col in cols {
        screen.tree_mut().root_mut().append(col);
    }
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen),
                    "
······
·ab  ·
·ab  ·
······");
}

#[test]
fn it_can_align_a_column_right() {
    let col = WrapBuilder::col(6)
                  .name("a".to_string())
                  .height(2)
                  .build();

    let mut screen = Screen::new(Size { rows: 2, cols: 4 });
    screen.tree_mut().root_mut().value().set_align(Align::Right);
    screen.tree_mut().root_mut().append(col);
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen),
                    "
······
·  aa·
·  aa·
······");
}

#[test]
fn it_can_align_columns_right() {
    let cols = vec![
        WrapBuilder::col(3).name("a".to_string()).height(2).build(),
        WrapBuilder::col(3).name("b".to_string()).height(2).build(),
    ];

    let mut screen = Screen::new(Size { rows: 2, cols: 4 });
    screen.tree_mut().root_mut().value().set_align(Align::Right);
    for col in cols {
        screen.tree_mut().root_mut().append(col);
    }
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen),
                    "
······
·  ab·
·  ab·
······");
}

#[test]
fn it_can_align_a_column_center() {
    let col = WrapBuilder::col(6).name("a".to_string()).height(2).build();

    let mut screen = Screen::new(Size { rows: 2, cols: 4 });
    screen.tree_mut().root_mut().value().set_align(Align::Center);
    screen.tree_mut().root_mut().append(col);
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen),
                    "
······
· aa ·
· aa ·
······");
}

#[test]
fn it_can_align_columns_center() {
    let cols = vec![
        WrapBuilder::col(3).name("a".to_string()).height(2).build(),
        WrapBuilder::col(3).name("b".to_string()).height(2).build(),
    ];

    let mut screen = Screen::new(Size { rows: 2, cols: 4 });
    screen.tree_mut().root_mut().value().set_align(Align::Center);
    for col in cols {
        screen.tree_mut().root_mut().append(col);
    }
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen),
                    "
······
· ab ·
· ab ·
······");
}

#[test]
fn it_can_align_a_row_top() {
    let row = WrapBuilder::row().name("a".to_string()).height(1).build();

    let mut screen = Screen::new(Size { rows: 2, cols: 4 });
    screen.tree_mut().root_mut().value().set_vertical_align(VerticalAlign::Top);
    screen.tree_mut().root_mut().append(row);
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen),
                    "
······
·aaaa·
·    ·
······");
}

#[test]
fn it_can_align_rows_top() {
    let rows = vec![
        WrapBuilder::row().name("a".to_string()).height(1).build(),
        WrapBuilder::row().name("b".to_string()).height(1).build(),
    ];

    let mut screen = Screen::new(Size { rows: 3, cols: 4 });
    screen.tree_mut().root_mut().value().set_vertical_align(VerticalAlign::Top);
    for row in rows {
        screen.tree_mut().root_mut().append(row);
    }
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen),
                    "
······
·aaaa·
·bbbb·
·    ·
······");
}

#[test]
fn it_can_align_a_row_bottom() {
    let row = WrapBuilder::row().name("a".to_string()).height(1).build();

    let mut screen = Screen::new(Size { rows: 2, cols: 4 });
    screen.tree_mut().root_mut().value().set_vertical_align(VerticalAlign::Bottom);
    screen.tree_mut().root_mut().append(row);
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen),
                    "
······
·    ·
·aaaa·
······");
}

#[test]
fn it_can_align_rows_bottom() {
    let rows = vec![
        WrapBuilder::row().name("a".to_string()).height(1).build(),
        WrapBuilder::row().name("b".to_string()).height(1).build(),
    ];

    let mut screen = Screen::new(Size { rows: 3, cols: 4 });
    screen.tree_mut().root_mut().value().set_vertical_align(VerticalAlign::Bottom);
    for row in rows {
        screen.tree_mut().root_mut().append(row);
    }
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen),
                    "
······
·    ·
·aaaa·
·bbbb·
······");
}

#[test]
fn it_can_align_a_row_middle() {
    let row = WrapBuilder::row().name("a".to_string()).height(1).build();

    let mut screen = Screen::new(Size { rows: 3, cols: 4 });
    screen.tree_mut().root_mut().value().set_vertical_align(VerticalAlign::Middle);
    screen.tree_mut().root_mut().append(row);
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen),
                    "
······
·    ·
·aaaa·
·    ·
······");
}

#[test]
fn it_can_align_rows_middle() {
    let rows = vec![
        WrapBuilder::row().name("a".to_string()).height(1).build(),
        WrapBuilder::row().name("b".to_string()).height(1).build(),
    ];

    let mut screen = Screen::new(Size { rows: 4, cols: 4 });
    screen.tree_mut().root_mut().value().set_vertical_align(VerticalAlign::Middle);
    for row in rows {
        screen.tree_mut().root_mut().append(row);
    }
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen),
                    "
······
·    ·
·aaaa·
·bbbb·
·    ·
······");
}
