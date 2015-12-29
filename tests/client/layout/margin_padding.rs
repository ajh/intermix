use libintermix::client::layout::*;
use ::support::layout::*;
use ::support::layout_painter::*;

#[test]
fn it_adds_padding() {
    let widget_a = Widget::new("a".to_string(), Size { rows: 2, cols: 4});
    let mut layout = Layout::new(Size { rows: 4, cols: 4}, vec![Node::leaf(widget_a)]);

    layout.calculate_layout();
    assert_scene_eq(&draw_layout(&layout), "
.----.
| aa |
| aa |
.----.");
}
