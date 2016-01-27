use libintermix::client::layout2::*;

pub fn draw_screen(screen: &Screen) -> String {
    // scene is 2d vec organized rows then cols
    let mut scene: Vec<Vec<char>> = vec![vec![' '; screen.size.cols as usize]; screen.size.rows as usize];

    // May include orphans? IDK
    let bordered: Vec<&Wrap> = screen.tree()
        .nodes()
        .map(|n| n.value())
        .filter(|w| w.has_border())
        .collect();

    for wrap in bordered {
        // this could be:
        //
        //     let rect = Rect { top: 0, left: 0, right: 3, bottom: 5 };
        //     rect.constrain(Rect { top: 0, left: 0, right: 3, bottom: 3 });

        let mut top = wrap.border_y().unwrap() as usize;
        if top < 0 { top = 0 }

        let mut bottom = (wrap.border_y().unwrap() + wrap.border_height().unwrap()) as usize;
        if bottom >= screen.size.rows as usize { bottom = screen.size.rows as usize - 1 }

        let mut left = wrap.border_x().unwrap() as usize;
        if left < 0 { left = 0 }

        let mut right = (wrap.border_x().unwrap() + wrap.border_width().unwrap()) as usize;
        if right >= screen.size.cols as usize { right = screen.size.cols as usize - 1 }

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

    let leafs: Vec<&Wrap> = screen.tree()
        .nodes()
        .filter(|n| !n.has_children())
        .map(|n| n.value())
        .collect();

    for leaf in leafs {
        if leaf.computed_x().unwrap() >= screen.size.cols as i16 { continue }
        if leaf.computed_y().unwrap() >= screen.size.rows as i16 { continue }

        let col_end = *[leaf.computed_x().unwrap() + leaf.computed_width().unwrap(), screen.size.cols as i16]
            .iter()
            .min()
            .unwrap();
        let row_end = *[leaf.computed_y().unwrap() + leaf.computed_height().unwrap(), screen.size.rows as i16]
            .iter()
            .min()
            .unwrap();

        for y in (leaf.computed_y().unwrap()..row_end) {
            for x in (leaf.computed_x().unwrap()..col_end) {
                scene[y as usize][x as usize] = leaf.name().chars().next().unwrap();
            }
        }
    }

    // draw scene border
    {
        let width = scene.first().unwrap().len();
        for line in scene.iter_mut() {
            line.insert(0, '·');
            line.push('·');
        }

        let mut top_bottom = vec!['·'; width];
        top_bottom.insert(0, '·');
        top_bottom.push('·');

        scene.insert(0, top_bottom.clone());
        scene.push(top_bottom);
    }

    // convert 2d vec into a newline separated string
    scene.iter()
        .map(|row| row.iter().cloned().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}
