fn operation(optr: char, num1: i32, num2: i32) -> i32 {
    match optr {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => num1 / num2,
        _ => 0,
    }
}
pub fn eval_rpn(tokens: Vec<String>) -> i32 {

    let optr = "";
    let num1 = 0;
    let num2 = 0;


    0
}
