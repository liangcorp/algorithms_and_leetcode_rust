use regular_expression_matching::cache::is_match;

fn main() {
    println!("{}", is_match(String::from("aa"), String::from(".*")));
}
