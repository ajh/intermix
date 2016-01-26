use libintermix::client::layout2::*;
use ::support::layout::*;
use ::support::layout2_painter::*;

mod align;

// Features todo:
//
// * [x] padding
// * [x] margin
// * [ ] title
// * [x] border
// * [ ] use xml backend rather than node stuff?
// * [x] rethink widget vs leaf with id String

#[test]
fn it_draws_a_root() {
    let mut screen = Screen::new(Size { rows: 2, cols: 2});
    screen.flush_changes();
    assert_scene_eq(&draw_screen(&screen), "
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

    let mut screen = Screen::new(Size { rows: 2, cols: 2});
    screen.tree_mut().root_mut().append(row);
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen), "
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

    let mut screen = Screen::new(Size { rows: 2, cols: 4});
    screen.tree_mut().root_mut().append(col);
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen), "
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

    let mut screen = Screen::new(Size { rows: 2, cols: 4});
    screen.tree_mut().root_mut().append(row).append(col);
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen), "
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

    let mut screen = Screen::new(Size { rows: 2, cols: 4});
    screen.tree_mut().root_mut().append(col).append(row);
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen), "
······
·aaa ·
·aaa ·
······");
}

#[test]
fn it_draws_a_12_width_col() {
    let col = WrapBuilder::col(12)
        .name("a".to_string())
        .height(2)
        .build();

    let mut screen = Screen::new(Size { rows: 2, cols: 4});
    screen.tree_mut().root_mut().append(col);
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen), "
······
·aaaa·
·aaaa·
······");
}

#[test]
fn it_draws_a_9_and_3_width_col_evenly() {
    let col_a = WrapBuilder::col(9)
        .name("a".to_string())
        .height(2)
        .build();

    let col_b = WrapBuilder::col(3)
        .name("b".to_string())
        .height(2)
        .build();

    let mut screen = Screen::new(Size { rows: 2, cols: 4});
    screen.tree_mut().root_mut().append(col_a);
    screen.tree_mut().root_mut().append(col_b);
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen), "
······
·aaab·
·aaab·
······");
}

#[test]
fn it_draws_a_9_and_3_width_col_unevenly() {
    let col_a = WrapBuilder::col(9)
        .name("a".to_string())
        .height(2)
        .build();

    let col_b = WrapBuilder::col(3)
        .name("b".to_string())
        .height(2)
        .build();

    let mut screen = Screen::new(Size { rows: 2, cols: 3});
    screen.tree_mut().root_mut().append(col_a);
    screen.tree_mut().root_mut().append(col_b);
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen), "
·····
·aab·
·aab·
·····");
}

#[test]
fn it_draws_a_3_and_9_width_col_evenly() {
    let col_a = WrapBuilder::col(3)
        .name("a".to_string())
        .height(2)
        .build();

    let col_b = WrapBuilder::col(9)
        .name("b".to_string())
        .height(2)
        .build();

    let mut screen = Screen::new(Size { rows: 2, cols: 4});
    screen.tree_mut().root_mut().append(col_a);
    screen.tree_mut().root_mut().append(col_b);
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen), "
······
·abbb·
·abbb·
······");
}

#[test]
fn it_draws_a_3_and_9_width_col_unevenly() {
    let col_a = WrapBuilder::col(3)
        .name("a".to_string())
        .height(2)
        .build();

    let col_b = WrapBuilder::col(9)
        .name("b".to_string())
        .height(2)
        .build();

    let mut screen = Screen::new(Size { rows: 2, cols: 3});
    screen.tree_mut().root_mut().append(col_a);
    screen.tree_mut().root_mut().append(col_b);
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen), "
·····
·abb·
·abb·
·····");
}

#[test]
fn it_draws_a_pair_of_6_width_cols_evenly() {
    let col_a = WrapBuilder::col(6)
        .name("a".to_string())
        .height(2)
        .build();

    let col_b = WrapBuilder::col(6)
        .name("b".to_string())
        .height(2)
        .build();

    let mut screen = Screen::new(Size { rows: 2, cols: 4});
    screen.tree_mut().root_mut().append(col_a);
    screen.tree_mut().root_mut().append(col_b);
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen), "
······
·aabb·
·aabb·
······");
}

#[test]
fn it_draws_a_pair_of_6_width_cols_unevenly() {
    let col_a = WrapBuilder::col(6)
        .name("a".to_string())
        .height(2)
        .build();

    let col_b = WrapBuilder::col(6)
        .name("b".to_string())
        .height(2)
        .build();

    let mut screen = Screen::new(Size { rows: 2, cols: 3});
    screen.tree_mut().root_mut().append(col_a);
    screen.tree_mut().root_mut().append(col_b);
    screen.flush_changes();

    // This outcome should probably be undefined
    assert_scene_eq(&draw_screen(&screen), "
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

    let mut screen = Screen::new(Size { rows: 1, cols: 6});
    for col in cols {
        screen.tree_mut().root_mut().append(col);
    }
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen), "
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

    let mut screen = Screen::new(Size { rows: 6, cols: 1});
    for row in rows {
        screen.tree_mut().root_mut().append(row);
    }
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen), "
···
·a·
·b·
·c·
·x·
·y·
·z·
···");
}

#[test]
fn it_truncates_leaf_with_narrow_container() {
    let leaf = WrapBuilder::row().name("a".to_string()).width(3).height(1).build();

    let mut screen = Screen::new(Size { rows: 1, cols: 2});
    screen.tree_mut().root_mut().append(leaf);
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen), "
····
·aa·
····");
}

#[test]
fn it_truncates_leaf_with_short_container() {
    let leaf = WrapBuilder::row().name("a".to_string()).height(4).build();

    let mut screen = Screen::new(Size { rows: 2, cols: 1});
    screen.tree_mut().root_mut().append(leaf);
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen), "
···
·a·
·a·
···");
}
