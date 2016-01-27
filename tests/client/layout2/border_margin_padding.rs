use libintermix::client::layout2::*;
use ::support::layout::*;
use ::support::layout2_painter::*;

#[test]
fn it_can_have_border_around_node() {
    let mut screen = Screen::new(Size { rows: 4, cols: 4});
    screen.tree_mut().root_mut().value().set_has_border(true);
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen), "
······
·┌──┐·
·│rr│·
·│rr│·
·└──┘·
······");
}

#[test]
fn it_can_have_margin_around_node() {
    let mut screen = Screen::new(Size { rows: 4, cols: 4});
    screen.tree_mut().root_mut().value().set_margin(1);
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen), "
······
·    ·
· rr ·
· rr ·
·    ·
······");
}

#[test]
fn it_can_have_padding_around_node() {
    let mut screen = Screen::new(Size { rows: 4, cols: 4});
    screen.tree_mut().root_mut().value().set_padding(1);
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen), "
······
·    ·
· rr ·
· rr ·
·    ·
······");
}

#[test]
fn it_can_have_border_marging_and_padding() {
    let mut screen = Screen::new(Size { rows: 8, cols: 8});
    screen.tree_mut().root_mut().value().set_has_border(true);
    screen.tree_mut().root_mut().value().set_margin(1);
    screen.tree_mut().root_mut().value().set_padding(1);
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen), "
··········
·        ·
· ┌────┐ ·
· │    │ ·
· │ aa │ ·
· │ aa │ ·
· │    │ ·
· └────┘ ·
·        ·
··········");
}

//#[test]
//fn it_can_layout_bordered_nodes_horizontally() {
    //let cols = vec![
        //WrapBuilder::col(6).name("a".to_string()).height(2).has_border(true).build(),
        //WrapBuilder::col(6).name("b".to_string()).height(2).has_border(true).build(),
    //];

    //let mut screen = Screen::new(Size { rows: 8, cols: 4});
    //for col in cols {
        //screen.tree_mut().root_mut().append(col);
    //}
    //screen.flush_changes();

    //assert_scene_eq(&draw_screen(&screen), "
//··········
//·┌──┐┌──┐·
//·│aa││bb│·
//·│aa││bb│·
//·└──┘└──┘·
//··········");
//}

//#[test]
//fn it_can_layout_bordered_nodes_vertically() {
    //let cols = vec![
        //WrapBuilder::row().name("a".to_string()).height(2).has_border(true).build(),
        //WrapBuilder::row().name("b".to_string()).height(2).has_border(true).build(),
    //];

    //let mut screen = Screen::new(Size { rows: 8, cols: 4});
    //for col in cols {
        //screen.tree_mut().root_mut().append(col);
    //}
    //screen.flush_changes();

    //assert_scene_eq(&draw_screen(&screen), "
//······
//·┌──┐·
//·│aa│·
//·│aa│·
//·└──┘·
//·┌──┐·
//·│bb│·
//·│bb│·
//·└──┘·
//······");
//}

//#[test]
//fn it_can_nest_nodes_with_borders_margins_and_paddings() {
    //::setup_logging();
    //let leaf_a = Node::leaf("a".to_string(), NodeOptions { height: Some(2), ..Default::default()});
    //let leaf_b = Node::leaf("b".to_string(), NodeOptions { height: Some(2), ..Default::default()});
    //let leaf_c = Node::leaf("c".to_string(), NodeOptions { height: Some(2), ..Default::default()});
    //let leaf_d = Node::leaf("d".to_string(), NodeOptions { height: Some(2), ..Default::default()});

    //let mut layout = Layout::new(
        //Size { rows: 12, cols: 20},
        //Node::row(NodeOptions { has_border: true, ..Default::default() }, vec![
            //Node::col(3, NodeOptions { margin: 1, ..Default::default() }, vec![leaf_a, leaf_b]),
            //Node::col(9, NodeOptions { has_border: true, ..Default::default() }, vec![leaf_c]),
            //Node::row(NodeOptions { has_border: true, ..Default::default() }, vec![leaf_d]),
        //])
    //);

    //layout.calculate_layout();
    //assert_scene_eq(&draw_layout(&layout), "
//······················
//·┌──────────────────┐·
//·│     ┌───────────┐│·
//·│ aaa │ccccccccccc││·
//·│ aaa │ccccccccccc││·
//·│ bbb └───────────┘│·
//·│ bbb              │·
//·│                  │·
//·│┌────────────────┐│·
//·││dddddddddddddddd││·
//·││dddddddddddddddd││·
//·│└────────────────┘│·
//·└──────────────────┘·
//······················");
//}
