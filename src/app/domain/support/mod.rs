pub fn make_key(part_one: String, part_two: &str) -> String {
    let mut key = part_one;
    key.push_str("_");
    key.push_str(part_two);

    key
}
