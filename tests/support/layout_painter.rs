use libintermix::client::layout::*;

pub fn draw_layout(layout: &Layout) -> String {
    println!("{:#?}", layout);
    // scene is 2d vec organized rows then cols
    let mut scene: Vec<Vec<char>> = vec![vec![' '; layout.size.cols as usize]; layout.size.rows as usize];

    // draw widgets into scene
    for widget in layout.root.widgets() {
        if widget.get_pos().row as u16 >= layout.size.rows { continue }
        if widget.get_pos().col as u16 >= layout.size.cols { continue }

        let row_end = *[(widget.get_pos().row as u16) + widget.get_size().rows, layout.size.rows]
            .iter()
            .min()
            .unwrap();
        let col_end = *[(widget.get_pos().col as u16) + widget.get_size().cols, layout.size.cols]
            .iter()
            .min()
            .unwrap();

        for y in ((widget.get_pos().row as u16)..row_end) {
            for x in ((widget.get_pos().col as u16)..col_end) {
                scene[y as usize][x as usize] = widget.fill;
            }
        }
    }

    // draw border
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
