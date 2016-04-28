use libintermix::client::layout::*;
use ::support::*;
use vterm_sys::Size;

#[test]
fn it_draws_a_root() {
    let mut layout = Layout::new(Size { height: 2, width: 2 });
    layout.flush_changes();
    assert_scene_eq(&draw_layout(&layout),
                    "
····
·rr·
·rr·
····");
}

#[test]
fn it_draws_only_a_row() {
    let row = WrapBuilder::row()
                  .name("a".to_string())
                  .height(2)
                  .build();

    let mut layout = Layout::new(Size { height: 2, width: 2 });
    layout.tree_mut().root_mut().append(row);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
····
·aa·
·aa·
····");
}

#[test]
fn it_draws_only_a_col() {
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
fn it_draws_a_column_inside_a_row() {
    let row = WrapBuilder::row().build();

    let col = WrapBuilder::col(9)
                  .name("a".to_string())
                  .height(2)
                  .build();

    let mut layout = Layout::new(Size { height: 2, width: 4 });
    layout.tree_mut().root_mut().append(row).append(col);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
······
·aaa ·
·aaa ·
······");
}

#[test]
fn it_draws_a_row_inside_a_column() {
    let col = WrapBuilder::col(9).build();

    let row = WrapBuilder::row()
                  .name("a".to_string())
                  .height(2)
                  .build();

    let mut layout = Layout::new(Size { height: 2, width: 4 });
    layout.tree_mut().root_mut().append(col).append(row);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
······
·aaa ·
·aaa ·
······");
}

#[test]
fn it_draws_a_12_grid_width_col() {
    let col = WrapBuilder::col(12)
                  .name("a".to_string())
                  .height(2)
                  .build();

    let mut layout = Layout::new(Size { height: 2, width: 4 });
    layout.tree_mut().root_mut().append(col);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
······
·aaaa·
·aaaa·
······");
}

#[test]
fn it_draws_a_9_and_3_grid_width_col_evenly() {
    let col_a = WrapBuilder::col(9)
                    .name("a".to_string())
                    .height(2)
                    .build();

    let col_b = WrapBuilder::col(3)
                    .name("b".to_string())
                    .height(2)
                    .build();

    let mut layout = Layout::new(Size { height: 2, width: 4 });
    layout.tree_mut().root_mut().append(col_a);
    layout.tree_mut().root_mut().append(col_b);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
······
·aaab·
·aaab·
······");
}

#[test]
fn it_draws_a_9_and_3_grid_width_col_unevenly() {
    let col_a = WrapBuilder::col(9)
                    .name("a".to_string())
                    .height(2)
                    .build();

    let col_b = WrapBuilder::col(3)
                    .name("b".to_string())
                    .height(2)
                    .build();

    let mut layout = Layout::new(Size { height: 2, width: 3 });
    layout.tree_mut().root_mut().append(col_a);
    layout.tree_mut().root_mut().append(col_b);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
·····
·aab·
·aab·
·····");
}

#[test]
fn it_draws_a_3_and_9_grid_width_col_evenly() {
    let col_a = WrapBuilder::col(3)
                    .name("a".to_string())
                    .height(2)
                    .build();

    let col_b = WrapBuilder::col(9)
                    .name("b".to_string())
                    .height(2)
                    .build();

    let mut layout = Layout::new(Size { height: 2, width: 4 });
    layout.tree_mut().root_mut().append(col_a);
    layout.tree_mut().root_mut().append(col_b);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
······
·abbb·
·abbb·
······");
}

#[test]
fn it_draws_a_3_and_9_grid_width_col_unevenly() {
    let col_a = WrapBuilder::col(3)
                    .name("a".to_string())
                    .height(2)
                    .build();

    let col_b = WrapBuilder::col(9)
                    .name("b".to_string())
                    .height(2)
                    .build();

    let mut layout = Layout::new(Size { height: 2, width: 3 });
    layout.tree_mut().root_mut().append(col_a);
    layout.tree_mut().root_mut().append(col_b);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
·····
·abb·
·abb·
·····");
}

#[test]
fn it_draws_a_pair_of_6_grid_width_cols_evenly() {
    let col_a = WrapBuilder::col(6)
                    .name("a".to_string())
                    .height(2)
                    .build();

    let col_b = WrapBuilder::col(6)
                    .name("b".to_string())
                    .height(2)
                    .build();

    let mut layout = Layout::new(Size { height: 2, width: 4 });
    layout.tree_mut().root_mut().append(col_a);
    layout.tree_mut().root_mut().append(col_b);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
······
·aabb·
·aabb·
······");
}

#[test]
fn it_draws_a_pair_of_6_grid_width_cols_unevenly() {
    let col_a = WrapBuilder::col(6)
                    .name("a".to_string())
                    .height(2)
                    .build();

    let col_b = WrapBuilder::col(6)
                    .name("b".to_string())
                    .height(2)
                    .build();

    let mut layout = Layout::new(Size { height: 2, width: 3 });
    layout.tree_mut().root_mut().append(col_a);
    layout.tree_mut().root_mut().append(col_b);
    layout.flush_changes();

    // This outcome should probably be undefined
    assert_scene_eq(&draw_layout(&layout),
                    "
·····
·aab·
·aab·
·····");
}

#[test]
fn it_draws_a_bunch_of_columns() {
    let cols = vec![
        WrapBuilder::col(2).name("a".to_string()).height(1).build(),
        WrapBuilder::col(2).name("b".to_string()).height(1).build(),
        WrapBuilder::col(2).name("c".to_string()).height(1).build(),
        WrapBuilder::col(2).name("x".to_string()).height(1).build(),
        WrapBuilder::col(2).name("y".to_string()).height(1).build(),
        WrapBuilder::col(2).name("z".to_string()).height(1).build(),
    ];

    let mut layout = Layout::new(Size { height: 1, width: 6 });
    for col in cols {
        layout.tree_mut().root_mut().append(col);
    }
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
········
·abcxyz·
········");
}

#[test]
fn it_draws_a_bunch_of_rows() {
    let rows = vec![
        WrapBuilder::row().name("a".to_string()).height(1).build(),
        WrapBuilder::row().name("b".to_string()).height(1).build(),
        WrapBuilder::row().name("c".to_string()).height(1).build(),
        WrapBuilder::row().name("x".to_string()).height(1).build(),
        WrapBuilder::row().name("y".to_string()).height(1).build(),
        WrapBuilder::row().name("z".to_string()).height(1).build(),
    ];

    let mut layout = Layout::new(Size { height: 6, width: 1 });
    for row in rows {
        layout.tree_mut().root_mut().append(row);
    }
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
···
·a·
·b·
·c·
·x·
·y·
·z·
···");
}
