use libintermix::client::layout::*;
use ::support::*;
use vterm_sys::Size;

#[test]
fn it_draws_height_of_col_when_same_as_parent() {
    let col = WrapBuilder::col(12)
                  .name("a".to_string())
                  .height(4)
                  .build();

    let mut layout = Layout::new(Size { height: 4, width: 4 });
    layout.tree_mut().root_mut().append(col);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
······
·aaaa·
·aaaa·
·aaaa·
·aaaa·
······");
}

#[test]
fn it_draws_height_of_col_when_less_than_parent() {
    let col = WrapBuilder::col(12)
                  .name("a".to_string())
                  .height(2)
                  .build();

    let mut layout = Layout::new(Size { height: 4, width: 4 });
    layout.tree_mut().root_mut().append(col);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
······
·aaaa·
·aaaa·
·    ·
·    ·
······");
}

#[test]
fn it_draws_height_of_cols_on_row_with_different_heights() {
    let col_a = WrapBuilder::col(6)
                  .name("a".to_string())
                  .height(2)
                  .build();
    let col_b = WrapBuilder::col(6)
                  .name("b".to_string())
                  .height(3)
                  .build();

    let mut layout = Layout::new(Size { height: 4, width: 4 });
    layout.tree_mut().root_mut().append(col_a);
    layout.tree_mut().root_mut().append(col_b);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
······
·aabb·
·aabb·
·  bb·
·    ·
······");
}

#[test]
fn it_draws_col_without_a_height() {
    let col = WrapBuilder::col(6)
                  .name("a".to_string())
                  .build();

    let mut layout = Layout::new(Size { height: 4, width: 4 });
    layout.tree_mut().root_mut().append(col);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
······
·aa  ·
·aa  ·
·aa  ·
·aa  ·
······");
}

#[test]
fn it_draws_row_without_a_height() {
    let row = WrapBuilder::row()
                  .name("a".to_string())
                  .build();

    let mut layout = Layout::new(Size { height: 4, width: 2 });
    layout.tree_mut().root_mut().append(row);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
····
·aa·
·aa·
·aa·
·aa·
····");
}

#[test]
fn it_draws_rows_without_heights() {
    let row_a = WrapBuilder::row()
                  .name("a".to_string())
                  .build();
    let row_b = WrapBuilder::row()
                  .name("b".to_string())
                  .build();

    let mut layout = Layout::new(Size { height: 4, width: 2 });
    layout.tree_mut().root_mut().append(row_a);
    layout.tree_mut().root_mut().append(row_b);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
····
·aa·
·aa·
·bb·
·bb·
····");
}

#[test]
fn it_draws_rows_without_heights_with_are_applied_unevenly() {
    let row_a = WrapBuilder::row()
                  .name("a".to_string())
                  .build();
    let row_b = WrapBuilder::row()
                  .name("b".to_string())
                  .build();

    let mut layout = Layout::new(Size { height: 3, width: 2 });
    layout.tree_mut().root_mut().append(row_a);
    layout.tree_mut().root_mut().append(row_b);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
····
·aa·
·aa·
·bb·
····");
}

#[test]
fn it_draws_a_mix_or_rows_and_columns_vertically_without_heights() {
    let col = WrapBuilder::col(6)
                  .name("a".to_string())
                  .build();
    let row = WrapBuilder::row()
                  .name("b".to_string())
                  .build();

    let mut layout = Layout::new(Size { height: 4, width: 4 });
    layout.tree_mut().root_mut().append(col);
    layout.tree_mut().root_mut().append(row);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
······
·aa  ·
·aa  ·
·bbbb·
·bbbb·
······");
}

#[test]
fn it_draws_rows_when_only_one_has_height() {
    let col = WrapBuilder::col(6)
                  .name("a".to_string())
                  .height(1)
                  .build();
    let row = WrapBuilder::row()
                  .name("b".to_string())
                  .build();

    let mut layout = Layout::new(Size { height: 4, width: 4 });
    layout.tree_mut().root_mut().append(col);
    layout.tree_mut().root_mut().append(row);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
······
·aa  ·
·bbbb·
·bbbb·
·bbbb·
······");
}
