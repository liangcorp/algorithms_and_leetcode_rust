pub mod zigzag;

fn main() {
    println!(
        "Result string: {}",
        zigzag::calculate::convert(String::from("PAYPALISHIRING"), 3)
    );

    println!("expected PAHNAPLSIIGYIR");
}
