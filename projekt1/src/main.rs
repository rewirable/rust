fn main() {
    let result = kalkulator("/", 40, 0);
    println!("{}", result);
}

fn kalkulator(action: &str, a: i32, b: i32) -> i32{
    return match action{
        "*" => a * b,
        "/" => if b == 0{ return 1 }else{ a/b },
        "+" => a + b,
        "-" => a - b,
        "%" => a % b,
        _ => -1
    
    };
    
}