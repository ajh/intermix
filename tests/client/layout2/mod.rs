use libintermix::client::layout2::*;
use ::support::layout::*;
use ::support::layout2_painter::*;

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

    println!("{:#?}", screen.tree());

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

    println!("{:#?}", screen.tree());

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
·aaab·
·aaab·
······");
}

//#[test]
//fn it_draws_a_9_and_3_width_col_unevenly() {
    //let leaf_a = Node::leaf("a".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});
    //let leaf_b = Node::leaf("b".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});

    //let mut layout = Layout::new(Size { rows: 2, cols: 3},
          //Node::row(Default::default(), vec![
              //Node::col(9, Default::default(), vec![leaf_a]),
              //Node::col(3, Default::default(), vec![leaf_b]),
          //])
    //);

    //layout.calculate_layout();
    //assert_scene_eq(&draw_layout(&layout), "
//·····
//·aab·
//·aab·
//·····");
//}

//#[test]
//fn it_draws_a_3_and_9_width_col_evenly() {
    //let leaf_a = Node::leaf("a".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});
    //let leaf_b = Node::leaf("b".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});

    //let mut layout = Layout::new(Size { rows: 2, cols: 4},
          //Node::row(Default::default(), vec![
              //Node::col(3, Default::default(), vec![leaf_a]),
              //Node::col(9, Default::default(), vec![leaf_b]),
          //])
    //);

    //layout.calculate_layout();
    //assert_scene_eq(&draw_layout(&layout), "
//······
//·abbb·
//·abbb·
//······");
//}

//#[test]
//fn it_draws_a_3_and_9_width_col_unevenly() {
    //let leaf_a = Node::leaf("a".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});
    //let leaf_b = Node::leaf("b".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});

    //let mut layout = Layout::new(Size { rows: 2, cols: 3},
          //Node::row(Default::default(), vec![
              //Node::col(3, Default::default(), vec![leaf_a]),
              //Node::col(9, Default::default(), vec![leaf_b]),
          //])
    //);

    //layout.calculate_layout();
    //assert_scene_eq(&draw_layout(&layout), "
//·····
//·abb·
//·abb·
//·····");
//}

//#[test]
//fn it_draws_a_pair_of_6_width_cols_evenly() {
    //let leaf_a = Node::leaf("a".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});
    //let leaf_b = Node::leaf("b".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});

    //let mut layout = Layout::new(Size { rows: 2, cols: 4},
          //Node::row(Default::default(), vec![
              //Node::col(6, Default::default(), vec![leaf_a]),
              //Node::col(6, Default::default(), vec![leaf_b]),
          //])
    //);

    //layout.calculate_layout();
    //assert_scene_eq(&draw_layout(&layout), "
//······
//·aabb·
//·aabb·
//······");
//}

//#[test]
//fn it_draws_a_pair_of_6_width_cols_unevenly() {
    //let leaf_a = Node::leaf("a".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});
    //let leaf_b = Node::leaf("b".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});

    //let mut layout = Layout::new(Size { rows: 2, cols: 3},
          //Node::row(Default::default(), vec![
              //Node::col(6, Default::default(), vec![leaf_a]),
              //Node::col(6, Default::default(), vec![leaf_b]),
          //])
    //);

    //layout.calculate_layout();
    //assert_scene_eq(&draw_layout(&layout), "
//·····
//·aab·
//·aab·
//·····");
//}

//#[test]
//fn it_draws_a_bunch_of_columns() {
    //let leaf_a = Node::leaf("a".to_string(), NodeOptions { height: Some(1), width: Some(1), ..Default::default()});
    //let leaf_b = Node::leaf("b".to_string(), NodeOptions { height: Some(1), width: Some(1), ..Default::default()});
    //let leaf_c = Node::leaf("c".to_string(), NodeOptions { height: Some(1), width: Some(1), ..Default::default()});
    //let leaf_x = Node::leaf("x".to_string(), NodeOptions { height: Some(1), width: Some(1), ..Default::default()});
    //let leaf_y = Node::leaf("y".to_string(), NodeOptions { height: Some(1), width: Some(1), ..Default::default()});
    //let leaf_z = Node::leaf("z".to_string(), NodeOptions { height: Some(1), width: Some(1), ..Default::default()});

    //let mut layout = Layout::new(Size { rows: 1, cols: 6},
          //Node::row(Default::default(), vec![
              //Node::col(2, Default::default(), vec![leaf_a]),
              //Node::col(2, Default::default(), vec![leaf_b]),
              //Node::col(2, Default::default(), vec![leaf_c]),
              //Node::col(2, Default::default(), vec![leaf_x]),
              //Node::col(2, Default::default(), vec![leaf_y]),
              //Node::col(2, Default::default(), vec![leaf_z]),
          //])
    //);

    //layout.calculate_layout();
    //assert_scene_eq(&draw_layout(&layout), "
//········
//·abcxyz·
//········");
//}

//#[test]
//fn it_draws_rows() {
    //let leaf_a = Node::leaf("a".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});
    //let leaf_b = Node::leaf("b".to_string(), NodeOptions { height: Some(2), width: Some(4), ..Default::default()});

    //let mut layout = Layout::new(Size { rows: 4, cols: 4},
          //Node::row(Default::default(), vec![
              //Node::row(Default::default(), vec![leaf_a]),
              //Node::row(Default::default(), vec![leaf_b]),
          //])
      //);
    //layout.calculate_layout();
    //assert_scene_eq(&draw_layout(&layout), "
//······
//·aaaa·
//·aaaa·
//·bbbb·
//·bbbb·
//······");
//}

//#[test]
//fn it_truncates_leaf_with_narrow_container() {
    //let leaf_a = Node::leaf("a".to_string(), NodeOptions { height: Some(1), width: Some(4), ..Default::default()});

    //let mut layout = Layout::new(Size { rows: 1, cols: 2}, leaf_a);

    //layout.calculate_layout();
    //assert_scene_eq(&draw_layout(&layout), "
//····
//·aa·
//····");
//}

//#[test]
//fn it_truncates_leaf_with_short_container() {
    //let leaf_a = Node::leaf("a".to_string(), NodeOptions { height: Some(4), width: Some(1), ..Default::default()});

    //let mut layout = Layout::new(Size { rows: 2, cols: 1}, leaf_a);

    //layout.calculate_layout();
    //assert_scene_eq(&draw_layout(&layout), "
//··.
//·a·
//·a·
//··.");
//}

//#[test]
//fn it_can_add_to_layout() {
    //let leaf_a = Node::leaf("a".to_string(), NodeOptions { height: Some(2), width: Some(2), ..Default::default()});
    //let mut layout = Layout::new(
        //Size { rows: 4, cols: 2},
        //Node::row(Default::default(), vec![leaf_a])
    //);

    //layout.calculate_layout();
    //assert_scene_eq(&draw_layout(&layout), "
//····
//·aa·
//·aa·
//·  ·
//·  ·
//····");

    //let leaf_b = Node::leaf("b".to_string(), NodeOptions { height: Some(2), width: Some(2), ..Default::default()});
    //layout.root
        //.children
        //.push(leaf_b);
    //layout.calculate_layout();

    //layout.calculate_layout();
    //assert_scene_eq(&draw_layout(&layout), "
//····
//·aa·
//·aa·
//·bb·
//·bb·
//····");
//}

//#[test]
//fn it_can_remove_from_layout() {
    //let leaf_a = Node::leaf("a".to_string(), NodeOptions { height: Some(2), width: Some(2), ..Default::default()});
    //let leaf_b = Node::leaf("b".to_string(), NodeOptions { height: Some(2), width: Some(2), ..Default::default()});

    //let mut layout = Layout::new(
        //Size { rows: 4, cols: 2},
        //Node::row(Default::default(), vec![
              //leaf_a,
              //leaf_b,
        //])
    //);
    //layout.calculate_layout();
    //assert_scene_eq(&draw_layout(&layout), "
//····
//·aa·
//·aa·
//·bb·
//·bb·
//····");

    //layout.root
        //.children
        //.remove(1);

    //layout.calculate_layout();
    //assert_scene_eq(&draw_layout(&layout), "
//····
//·aa·
//·aa·
//·  ·
//·  ·
//····");
//}

////#[test]
////fn it_draws_a_complicated_scene() {
    ////let leaf_a = Node::leaf("a".to_string(), Size { rows: 2, cols: 8});
    ////let leaf_b = Node::leaf("b".to_string(), Size { rows: 2, cols: 4});
    ////let leaf_c = Node::leaf("c".to_string(), Size { rows: 1, cols: 8});
    ////let leaf_x = Node::leaf("x".to_string(), Size { rows: 2, cols: 4});
    ////let leaf_y = Node::leaf("y".to_string(), Size { rows: 4, cols: 4});
    ////let leaf_z = Node::leaf("z".to_string(), Size { rows: 5, cols: 1});

    ////let mut layout = Layout::new(Size { rows: 10, cols: 12},
          ////Node::row(vec![
              ////Node::row(vec![
                  ////Node::col(4, vec![leaf_a, leaf_b]),
                  ////Node::col(8, vec![
                      ////Node::row(vec![leaf_c]),
                      ////Node::row(vec![leaf_x]),
                  ////]),
              ////]),
              ////Node::row(vec![
                  ////Node::col(4, vec![leaf_y]),
                  ////Node::col(2, vec![]),
                  ////Node::col(6, vec![leaf_z]),
              ////])
          ////])
    ////);
    ////
    ////blah blah blah
////}
