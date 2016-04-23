use libintermix::client::layout::*;
use ::support::*;
use vterm_sys::Size;

#[test]
fn it_wraps_content_in_leftmost_column() {
    let leaf_a = WrapBuilder::col(9)
                     .name("a".to_string())
                     .height(2)
                     .build();

    let leaf_b = WrapBuilder::col(6)
                     .name("b".to_string())
                     .height(2)
                     .build();

    let mut layout = Layout::new(Size { height: 4, width: 4 });
    layout.tree_mut()
          .root_mut()
          .append(WrapBuilder::col(9).build())
          .append(leaf_a);
    layout.tree_mut()
          .root_mut()
          .append(WrapBuilder::col(6).build())
          .append(leaf_b);

    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
······
·aaa ·
·aaa ·
·bb  ·
·bb  ·
······");
}

#[test]
fn it_wraps_content_in_rightmost_column() {
    let leaf_a = WrapBuilder::col(6)
                     .name("a".to_string())
                     .height(2)
                     .build();

    let leaf_b = WrapBuilder::col(9)
                     .name("b".to_string())
                     .height(2)
                     .build();

    let mut layout = Layout::new(Size { height: 4, width: 4 });
    layout.tree_mut().root_mut().value().set_align(Align::Right);
    layout.tree_mut()
          .root_mut()
          .append(WrapBuilder::col(9).build())
          .append(leaf_a);
    layout.tree_mut()
          .root_mut()
          .append(WrapBuilder::col(9).build())
          .append(leaf_b);

    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
······
· aa ·
· aa ·
· bbb·
· bbb·
······");
}
