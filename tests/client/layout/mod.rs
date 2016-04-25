use libintermix::client::layout::*;
use ::support::*;
use vterm_sys::Size;

mod align;
mod border_margin_padding;
mod layout;
mod line_wrap;
mod nesting;

// Features todo:
//
// * [ ] title
// * [ ] serialization

#[test]
fn it_truncates_leaf_with_narrow_container() {
    let leaf = WrapBuilder::row().name("a".to_string()).width(3).height(1).build();

    let mut layout = Layout::new(Size { height: 1, width: 2 });
    layout.tree_mut().root_mut().append(leaf);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
····
·aa·
····");

    assert_eq!(layout.tree().root().first_child().unwrap().value().computed_width(), Some(2));
}

#[test]
fn it_truncates_leaf_with_short_container() {
    let leaf = WrapBuilder::row().name("a".to_string()).height(4).build();

    let mut layout = Layout::new(Size { height: 2, width: 1 });
    layout.tree_mut().root_mut().append(leaf);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
···
·a·
·a·
···");

    //assert_eq!(layout.tree().root().first_child().unwrap().value().computed_height(), Some(2));
}
