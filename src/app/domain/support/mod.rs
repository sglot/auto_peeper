pub fn make_key(part_one: String, part_two: &str) -> String {
    let mut key = part_one;
    key.push_str("_");
    key.push_str(part_two);

    key
}

pub fn separated_string(number: f32) -> String {
    let m = (number / 1000000.0).trunc();

    let mut o: f32 = (number - m * 1000000.0).abs();
    let t = (o / 1000.0).trunc();
    o = (o - t * 1000.0).abs();

    let mut mm = format!("{}", m);
    if mm.eq("0") {
        mm = "".to_string();
    }

    let mut tt = format!("{:03}", t);
    if tt.eq("000") {
        tt = "".to_string();
    }

    let s = format!("{} {} {:.0}", mm, tt, o).trim().to_string();
    s
 }