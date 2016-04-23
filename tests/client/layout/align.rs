use libintermix::client::layout::*;
use ::support::*;
use vterm_sys::Size;

#[test]
fn it_can_align_a_column_left() {
    let col = WrapBuilder::col(6)
                  .name("a".to_string())
                  .height(2)
                  .build();

    let mut layout = Layout::new(Size { height: 2, width: 4 });
    layout.tree_mut().root_mut().append(col);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
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

    let mut layout = Layout::new(Size { height: 2, width: 4 });
    for col in cols {
        layout.tree_mut().root_mut().append(col);
    }
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
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

    let mut layout = Layout::new(Size { height: 2, width: 4 });
    layout.tree_mut().root_mut().value().set_align(Align::Right);
    layout.tree_mut().root_mut().append(col);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
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

    let mut layout = Layout::new(Size { height: 2, width: 4 });
    layout.tree_mut().root_mut().value().set_align(Align::Right);
    for col in cols {
        layout.tree_mut().root_mut().append(col);
    }
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
······
·  ab·
·  ab·
······");
}

#[test]
fn it_can_align_a_column_center() {
    let col = WrapBuilder::col(6).name("a".to_string()).height(2).build();

    let mut layout = Layout::new(Size { height: 2, width: 4 });
    layout.tree_mut().root_mut().value().set_align(Align::Center);
    layout.tree_mut().root_mut().append(col);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
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

    let mut layout = Layout::new(Size { height: 2, width: 4 });
    layout.tree_mut().root_mut().value().set_align(Align::Center);
    for col in cols {
        layout.tree_mut().root_mut().append(col);
    }
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
······
· ab ·
· ab ·
······");
}

#[test]
fn it_can_align_a_row_top() {
    let row = WrapBuilder::row().name("a".to_string()).height(1).build();

    let mut layout = Layout::new(Size { height: 2, width: 4 });
    layout.tree_mut().root_mut().value().set_vertical_align(VerticalAlign::Top);
    layout.tree_mut().root_mut().append(row);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
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

    let mut layout = Layout::new(Size { height: 3, width: 4 });
    layout.tree_mut().root_mut().value().set_vertical_align(VerticalAlign::Top);
    for row in rows {
        layout.tree_mut().root_mut().append(row);
    }
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
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

    let mut layout = Layout::new(Size { height: 2, width: 4 });
    layout.tree_mut().root_mut().value().set_vertical_align(VerticalAlign::Bottom);
    layout.tree_mut().root_mut().append(row);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
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

    let mut layout = Layout::new(Size { height: 3, width: 4 });
    layout.tree_mut().root_mut().value().set_vertical_align(VerticalAlign::Bottom);
    for row in rows {
        layout.tree_mut().root_mut().append(row);
    }
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
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

    let mut layout = Layout::new(Size { height: 3, width: 4 });
    layout.tree_mut().root_mut().value().set_vertical_align(VerticalAlign::Middle);
    layout.tree_mut().root_mut().append(row);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
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

    let mut layout = Layout::new(Size { height: 4, width: 4 });
    layout.tree_mut().root_mut().value().set_vertical_align(VerticalAlign::Middle);
    for row in rows {
        layout.tree_mut().root_mut().append(row);
    }
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
······
·    ·
·aaaa·
·bbbb·
·    ·
······");
}
