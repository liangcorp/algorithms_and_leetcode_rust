pub mod zigzag;

fn main() {
    println!(
        "Result string: {}",
        zigzag::brute::convert(String::from("PAYPALISHIRING"), 3)
    );

    println!("expected PAHNAPLSIIGYIR");

    println!(
        "Result string: {}",
        zigzag::brute::convert(String::from("A"), 1)
    );

    println!("expected A");
}
