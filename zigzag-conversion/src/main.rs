use zigzag_conversion::zigzag;

fn main() {
    println!(
        "Result string: {}",
        zigzag::convert(String::from("PAYPALISHIRING"), 3)
    );
}
