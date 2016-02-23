use libintermix::client::layout::*;
use ::support::*;
use vterm_sys::ScreenSize;

#[test]
fn it_can_have_border_around_node() {
    let mut screen = Screen::new(ScreenSize { rows: 4, cols: 4 });
    screen.tree_mut().root_mut().value().set_has_border(true);
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen),
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
    let mut screen = Screen::new(ScreenSize { rows: 4, cols: 4 });
    screen.tree_mut().root_mut().value().set_margin(1);
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen),
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
    let mut screen = Screen::new(ScreenSize { rows: 4, cols: 4 });
    screen.tree_mut().root_mut().value().set_padding(1);
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen),
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
    let mut screen = Screen::new(ScreenSize { rows: 8, cols: 8 });
    screen.tree_mut().root_mut().value().set_has_border(true);
    screen.tree_mut().root_mut().value().set_margin(1);
    screen.tree_mut().root_mut().value().set_padding(1);
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen),
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

    let mut screen = Screen::new(ScreenSize { rows: 4, cols: 8 });
    for col in cols {
        screen.tree_mut().root_mut().append(col);
    }
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen),
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

    let mut screen = Screen::new(ScreenSize { rows: 8, cols: 4 });
    for col in cols {
        screen.tree_mut().root_mut().append(col);
    }
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen),
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

    let mut screen = Screen::new(ScreenSize {
        rows: 12,
        cols: 20,
    });
    screen.tree_mut().root_mut().value().set_has_border(true);
    {
        let mut root = screen.tree_mut().root_mut();
        let mut c = root.append(col1);
        c.append(leaf_a);
        c.append(leaf_b);
    }
    screen.tree_mut().root_mut().append(col2).append(leaf_c);
    screen.tree_mut().root_mut().append(row).append(leaf_d);

    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen),
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
