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

    println!("{:?}", layout);
    assert_scene_eq(&draw_layout(&layout),
                    "
······
·aabb·
·aabb·
·  bb·
·    ·
······");
}
