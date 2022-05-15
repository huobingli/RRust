fn Shadowing() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}

fn error() {
    let mut s = "123";
    let s1 = s.len();
    println!("{}",s1)
}

fn main() {
    Shadowing();
    error();
}