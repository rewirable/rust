fn main() {
    let name: &str = "alex";
    let mut age: i64 = 44;
    const PI: &f64 = &3.14159;
    if age >= 100 {
        return
    }
    loop {
        age += 1;
        if age >= 999{
            break
        }
    }
    println!("Name: {}", name);
    println!("Age: {}", age);
    let result = lol("brudas", &30, &50);
    println!("{}", result);
}
fn lol(name: &str, a: &i32, b: &i32) -> i32 {
    println!("Hello, {}", name);
    a + b
}

