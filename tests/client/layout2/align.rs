use libintermix::client::layout2::*;
use ::support::layout::*;
use ::support::layout2_painter::*;

#[test]
fn it_can_align_a_column_left() {
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
fn it_can_align_columns_left() {
    let cols = vec![
        WrapBuilder::col(3).name("a".to_string()).height(2).build(),
        WrapBuilder::col(3).name("b".to_string()).height(2).build(),
    ];

    let mut screen = Screen::new(Size { rows: 2, cols: 4});
    for col in cols {
        screen.tree_mut().root_mut().append(col);
    }
    screen.flush_changes();

    assert_scene_eq(&draw_screen(&screen), "
······
·ab  ·
·ab  ·
······");
}

//#[test]
//fn it_can_align_a_column_right() {
    //::setup_logging();
    //let leaf_a = Node::leaf("a".to_string(), NodeOptions { height: Some(2), width: Some(2), ..Default::default()});
    //let mut layout = Layout::new(
        //Size { rows: 2, cols: 4},
        //Node::row(NodeOptions { align: Align::Right, ..Default::default() }, vec![
            //Node::col(6, Default::default(), vec![leaf_a])
        //])
    //);

    //layout.calculate_layout();
    //assert_scene_eq(&draw_layout(&layout), "
//······
//·  aa·
//·  aa·
//······");
//}

//#[test]
//fn it_can_align_columns_right() {
    //::setup_logging();
    //let leaf_a = Node::leaf("a".to_string(), NodeOptions { height: Some(2), width: Some(2), ..Default::default()});
    //let leaf_b = Node::leaf("b".to_string(), NodeOptions { height: Some(2), width: Some(2), ..Default::default()});
    //let mut layout = Layout::new(
        //Size { rows: 2, cols: 4},
        //Node::row(NodeOptions { align: Align::Right, ..Default::default() }, vec![
            //Node::col(3, Default::default(), vec![leaf_a]),
            //Node::col(3, Default::default(), vec![leaf_b])
        //])
    //);

    //layout.calculate_layout();
    //assert_scene_eq(&draw_layout(&layout), "
//······
//·  ab·
//·  ab·
//······");
//}

//#[test]
//fn it_can_align_a_column_center() {
    //::setup_logging();
    //let leaf_a = Node::leaf("a".to_string(), NodeOptions { height: Some(2), width: Some(2), ..Default::default()});
    //let mut layout = Layout::new(
        //Size { rows: 2, cols: 4},
        //Node::row(NodeOptions { align: Align::Center, ..Default::default() }, vec![
            //Node::col(6, Default::default(), vec![leaf_a])
        //])
    //);

    //layout.calculate_layout();
    //assert_scene_eq(&draw_layout(&layout), "
//······
//· aa ·
//· aa ·
//······");
//}

//#[test]
//fn it_can_align_columns_center() {
    //::setup_logging();
    //let leaf_a = Node::leaf("a".to_string(), NodeOptions { height: Some(2), width: Some(2), ..Default::default()});
    //let leaf_b = Node::leaf("b".to_string(), NodeOptions { height: Some(2), width: Some(2), ..Default::default()});
    //let mut layout = Layout::new(
        //Size { rows: 2, cols: 4},
        //Node::row(NodeOptions { align: Align::Center, ..Default::default() }, vec![
            //Node::col(3, Default::default(), vec![leaf_a]),
            //Node::col(3, Default::default(), vec![leaf_b]),
        //])
    //);

    //layout.calculate_layout();
    //assert_scene_eq(&draw_layout(&layout), "
//······
//· ab ·
//· ab ·
//······");
//}

//#[test]
//fn it_can_align_a_row_top() {
    //::setup_logging();
    //let leaf_a = Node::leaf("a".to_string(), NodeOptions { height: Some(1), width: Some(4), ..Default::default()});
    //let mut layout = Layout::new(
        //Size { rows: 2, cols: 4},
        //Node::row(NodeOptions { vertical_align: VerticalAlign::Top, ..Default::default() }, vec![
            //Node::row(Default::default(), vec![leaf_a])
        //])
    //);

    //layout.calculate_layout();
    //assert_scene_eq(&draw_layout(&layout), "
//······
//·aaaa·
//·    ·
//······");
//}

//#[test]
//fn it_can_align_rows_top() {
    //::setup_logging();
    //let leaf_a = Node::leaf("a".to_string(), NodeOptions { height: Some(1), width: Some(4), ..Default::default()});
    //let leaf_b = Node::leaf("b".to_string(), NodeOptions { height: Some(1), width: Some(4), ..Default::default()});
    //let mut layout = Layout::new(
        //Size { rows: 3, cols: 4},
        //Node::row(NodeOptions { vertical_align: VerticalAlign::Top, ..Default::default() }, vec![
            //Node::row(Default::default(), vec![leaf_a]),
            //Node::row(Default::default(), vec![leaf_b])
        //])
    //);

    //layout.calculate_layout();
    //assert_scene_eq(&draw_layout(&layout), "
//······
//·aaaa·
//·bbbb·
//·    ·
//······");
//}

//#[test]
//fn it_can_align_a_row_botton() {
    //::setup_logging();
    //let leaf_a = Node::leaf("a".to_string(), NodeOptions { height: Some(1), width: Some(4), ..Default::default()});
    //let mut layout = Layout::new(
        //Size { rows: 2, cols: 4},
        //Node::row(NodeOptions { vertical_align: VerticalAlign::Bottom, height: Some(2), ..Default::default() }, vec![
            //Node::row(Default::default(), vec![leaf_a])
        //])
    //);

    //layout.calculate_layout();
    //assert_scene_eq(&draw_layout(&layout), "
//······
//·    ·
//·aaaa·
//······");
//}

//#[test]
//fn it_can_align_rows_botton() {
    //::setup_logging();
    //let leaf_a = Node::leaf("a".to_string(), NodeOptions { height: Some(1), width: Some(4), ..Default::default()});
    //let leaf_b = Node::leaf("b".to_string(), NodeOptions { height: Some(1), width: Some(4), ..Default::default()});
    //let mut layout = Layout::new(
        //Size { rows: 3, cols: 4},
        //Node::row(NodeOptions { vertical_align: VerticalAlign::Bottom, height: Some(3), ..Default::default() }, vec![
            //Node::row(Default::default(), vec![leaf_a]),
            //Node::row(Default::default(), vec![leaf_b])
        //])
    //);

    //layout.calculate_layout();
    //assert_scene_eq(&draw_layout(&layout), "
//······
//·    ·
//·aaaa·
//·bbbb·
//······");
//}

//#[test]
//fn it_can_align_a_row_middle() {
    //::setup_logging();
    //let leaf_a = Node::leaf("a".to_string(), NodeOptions { height: Some(1), width: Some(4), ..Default::default()});
    //let mut layout = Layout::new(
        //Size { rows: 3, cols: 4},
        //Node::row(NodeOptions { vertical_align: VerticalAlign::Middle, height: Some(3), ..Default::default() }, vec![
            //Node::row(Default::default(), vec![leaf_a])
        //])
    //);

    //layout.calculate_layout();
    //assert_scene_eq(&draw_layout(&layout), "
//······
//·    ·
//·aaaa·
//·    ·
//······");
//}

//#[test]
//fn it_can_align_rows_center() {
    //::setup_logging();
    //let leaf_a = Node::leaf("a".to_string(), NodeOptions { height: Some(1), width: Some(4), ..Default::default()});
    //let leaf_b = Node::leaf("b".to_string(), NodeOptions { height: Some(1), width: Some(4), ..Default::default()});
    //let mut layout = Layout::new(
        //Size { rows: 4, cols: 4},
        //Node::row(NodeOptions { vertical_align: VerticalAlign::Middle, height: Some(4), ..Default::default() }, vec![
            //Node::row(Default::default(), vec![leaf_a]),
            //Node::row(Default::default(), vec![leaf_b])
        //])
    //);

    //layout.calculate_layout();
    //assert_scene_eq(&draw_layout(&layout), "
//······
//·    ·
//·aaaa·
//·bbbb·
//·    ·
//······");
//}
