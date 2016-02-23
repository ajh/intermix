pub fn assert_scene_eq(actual: &str, expected: &str) {
    let actual = actual.trim();
    let expected = expected.trim();

    if actual != expected {
        panic!("scenes not equal.\nactual:\n{}\nexpected:\n{}",
               actual,
               expected);
    }
}
