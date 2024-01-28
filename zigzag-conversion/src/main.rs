pub mod zigzag;

fn main() {
    println!(
        "Result string: {}",
        zigzag::linear::convert(String::from("PAYPALISHIRING"), 3)
    );

    println!("expected PAHNAPLSIIGYIR");
}
