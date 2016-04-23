use libintermix::client::layout::*;
use ::support::*;
use vterm_sys::Size;

#[test]
fn it_can_have_border_around_node() {
    let mut layout = Layout::new(Size { height: 4, width: 4 });
    layout.tree_mut().root_mut().value().set_has_border(true);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
······
·┌──┐·
·│rr│·
·│rr│·
·└──┘·
······");
}

#[test]
fn it_can_have_margin_around_node() {
    let mut layout = Layout::new(Size { height: 4, width: 4 });
    layout.tree_mut().root_mut().value().set_margin(1);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
······
·    ·
· rr ·
· rr ·
·    ·
······");
}

#[test]
fn it_can_have_padding_around_node() {
    let mut layout = Layout::new(Size { height: 4, width: 4 });
    layout.tree_mut().root_mut().value().set_padding(1);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
······
·    ·
· rr ·
· rr ·
·    ·
······");
}

#[test]
fn it_can_have_border_marging_and_padding() {
    let mut layout = Layout::new(Size { height: 8, width: 8 });
    layout.tree_mut().root_mut().value().set_has_border(true);
    layout.tree_mut().root_mut().value().set_margin(1);
    layout.tree_mut().root_mut().value().set_padding(1);
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
··········
·        ·
· ┌────┐ ·
· │    │ ·
· │ rr │ ·
· │ rr │ ·
· │    │ ·
· └────┘ ·
·        ·
··········");
}

#[test]
fn it_can_layout_bordered_nodes_horizontally() {
    let cols = vec![
        WrapBuilder::col(6).name("a".to_string()).height(2).has_border(true).build(),
        WrapBuilder::col(6).name("b".to_string()).height(2).has_border(true).build(),
    ];

    let mut layout = Layout::new(Size { height: 4, width: 8 });
    for col in cols {
        layout.tree_mut().root_mut().append(col);
    }
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
··········
·┌──┐┌──┐·
·│aa││bb│·
·│aa││bb│·
·└──┘└──┘·
··········");
}

#[test]
fn it_can_layout_bordered_nodes_vertically() {
    let cols = vec![
        WrapBuilder::row().name("a".to_string()).height(2).has_border(true).build(),
        WrapBuilder::row().name("b".to_string()).height(2).has_border(true).build(),
    ];

    let mut layout = Layout::new(Size { height: 8, width: 4 });
    for col in cols {
        layout.tree_mut().root_mut().append(col);
    }
    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
······
·┌──┐·
·│aa│·
·│aa│·
·└──┘·
·┌──┐·
·│bb│·
·│bb│·
·└──┘·
······");
}

#[test]
fn it_can_nest_nodes_with_borders_margins_and_paddings() {
    let col1 = WrapBuilder::col(3).margin(1).build();
    let col2 = WrapBuilder::col(9).has_border(true).build();
    let row = WrapBuilder::row().has_border(true).build();

    let leaf_a = WrapBuilder::row().name("a".to_string()).height(2).build();
    let leaf_b = WrapBuilder::row().name("b".to_string()).height(2).build();
    let leaf_c = WrapBuilder::row().name("c".to_string()).height(2).build();
    let leaf_d = WrapBuilder::row().name("d".to_string()).height(2).build();

    let mut layout = Layout::new(Size {
        height: 12,
        width: 20,
    });
    layout.tree_mut().root_mut().value().set_has_border(true);
    {
        let mut root = layout.tree_mut().root_mut();
        let mut c = root.append(col1);
        c.append(leaf_a);
        c.append(leaf_b);
    }
    layout.tree_mut().root_mut().append(col2).append(leaf_c);
    layout.tree_mut().root_mut().append(row).append(leaf_d);

    layout.flush_changes();

    assert_scene_eq(&draw_layout(&layout),
                    "
······················
·┌──────────────────┐·
·│     ┌───────────┐│·
·│ aaa │ccccccccccc││·
·│ aaa │ccccccccccc││·
·│ bbb └───────────┘│·
·│ bbb              │·
·│                  │·
·│┌────────────────┐│·
·││dddddddddddddddd││·
·││dddddddddddddddd││·
·│└────────────────┘│·
·└──────────────────┘·
······················");
}
