use libintermix::client::layout::*;
use ::support::*;
use vterm_sys::Size;

#[test]
fn it_draws_a_12_grid_width_column_as_wide_as_parent() {
    let parent = WrapBuilder::col(9).build();
    let col = WrapBuilder::col(12)
                  .name("a".to_string())
                  .height(2)
                  .build();

    let mut layout = Layout::new(Size { height: 2, width: 12 });
    layout.tree_mut().root_mut().append(parent).append(col);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
··············
·aaaaaaaaa   ·
·aaaaaaaaa   ·
··············");
}

#[test]
fn it_draws_a_8_grid_width_column_as_two_thirds_of_parents_width() {
    let parent = WrapBuilder::col(9).build();
    let col = WrapBuilder::col(8)
                  .name("a".to_string())
                  .height(2)
                  .build();

    let mut layout = Layout::new(Size { height: 2, width: 12 });
    layout.tree_mut().root_mut().append(parent).append(col);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
··············
·aaaaaa      ·
·aaaaaa      ·
··············");
}

#[test]
fn it_draws_a_6_grid_width_column_as_half_of_parents_width() {
    let parent = WrapBuilder::col(9).build();
    let col = WrapBuilder::col(6)
                  .name("a".to_string())
                  .height(2)
                  .build();

    let mut layout = Layout::new(Size { height: 2, width: 12 });
    layout.tree_mut().root_mut().append(parent).append(col);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
··············
·aaaaa       ·
·aaaaa       ·
··············");
}

#[test]
fn it_draws_a_4_grid_width_column_as_one_third_of_parents_width() {
    let parent = WrapBuilder::col(9).build();
    let col = WrapBuilder::col(4)
                  .name("a".to_string())
                  .height(2)
                  .build();

    let mut layout = Layout::new(Size { height: 2, width: 12 });
    layout.tree_mut().root_mut().append(parent).append(col);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
··············
·aaa         ·
·aaa         ·
··············");
}
