fn my_shorter_function(
    _a: String,
    _b: String,
    _c: String,
) {
    unimplemented!()
}

fn my_function_with_a_very_long_name(a: &str, b: &str, c: &str, d: &str, e: &str, f: &str) {
    println!("A: {} | B: {} | C: {} | D: {} | E: {} | F: {}", a, b, c, d, e, f);
}

fn main() {
    my_function_with_a_very_long_name("a", "b", "c", "d", "e", "f");
}
