use libintermix::client::layout::*;

pub fn draw_layout(layout: &Layout) -> String {
    println!("{:#?}", layout);
    // scene is 2d vec organized rows then cols
    let mut scene: Vec<Vec<char>> = vec![vec![' '; layout.size.cols as usize]; layout.size.rows as usize];

    for node in layout.root.descendants().filter(|n| n.has_border) {
        let distance = node.padding + 1;
        let top      = (node.computed_pos.row as u16 - distance) as usize;
        let bottom   = (node.computed_pos.row as u16 + node.computed_size.rows - 1 + distance) as usize;
        let left     = (node.computed_pos.col as u16 - distance) as usize;
        let right    = (node.computed_pos.col as u16 + node.computed_size.cols - 1 + distance) as usize;

        scene[top][left] = '┌';
        scene[top][right] = '┐';
        scene[bottom][left] = '└';
        scene[bottom][right] = '┘';

        for x in (left + 1..right) {
            scene[top][x] = '─';
            scene[bottom][x] = '─';
        }

        for y in (top + 1..bottom) {
            scene[y][left] = '│';
            scene[y][right] = '│';
        }
    }

    // draw leafs into scene
    for leaf in layout.root.descendants().filter(|n| n.is_leaf()) {
        if leaf.computed_pos.row as u16 >= layout.size.rows { continue }
        if leaf.computed_pos.col as u16 >= layout.size.cols { continue }

        let row_end = *[(leaf.computed_pos.row as u16) + leaf.computed_size.rows, layout.size.rows]
            .iter()
            .min()
            .unwrap();
        let col_end = *[(leaf.computed_pos.col as u16) + leaf.computed_size.cols, layout.size.cols]
            .iter()
            .min()
            .unwrap();

        for y in ((leaf.computed_pos.row as u16)..row_end) {
            for x in ((leaf.computed_pos.col as u16)..col_end) {
                scene[y as usize][x as usize] = leaf.value.chars().next().unwrap();
            }
        }

    }

    // draw scene border
    let width = scene.first().unwrap().len();
    for line in scene.iter_mut() {
        line.insert(0, '|');
        line.push('|');
    }

    let mut top_bottom = vec!['-'; width];
    top_bottom.insert(0, '.');
    top_bottom.push('.');

    scene.insert(0, top_bottom.clone());
    scene.push(top_bottom);

    // convert 2d vec into a newline separated string
    scene.iter()
        .map(|row| row.iter().cloned().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}
