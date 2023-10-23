// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.

// 定义一个函数来解析字符串为整数
fn parse_number(number: &str) -> i32 {
    match number {
        "ONE" => 1,
        "TWO" => 2,
        "THREE" => 3,
        _ => 0,
    }
}
fn main() {
    let mut number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    number = "3"; // don't rename this variable
    let number = parse_number(number); // 将字符串解析为整数类型
    println!("Number plus two is : {}", number + 2);
}
